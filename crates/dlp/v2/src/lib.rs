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
    pub mod info_types {
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
                    #[doc = "filter to only return infoTypes supported by certain parts of the API. Defaults to supported_by=INSPECT."]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "languageCode")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "BCP-47 language code for localized infoType friendly names. If omitted, or if localized strings are not available, en-US strings will be returned."]
                    pub language_code: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "locationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated. This field has no effect."]
                    pub location_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "parent")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The parent resource name. The format of this value is as follows: locations/ LOCATION_ID"]
                    pub parent: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod locations {
        pub mod resources {
            pub mod info_types {
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
                            #[doc = "filter to only return infoTypes supported by certain parts of the API. Defaults to supported_by=INSPECT."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "languageCode")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "BCP-47 language code for localized infoType friendly names. If omitted, or if localized strings are not available, en-US strings will be returned."]
                            pub language_code: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "locationId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Deprecated. This field has no effect."]
                            pub location_id: ::std::option::Option<::std::string::String>,
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
    pub mod organizations {
        pub mod resources {
            pub mod deidentify_templates {
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
                            #[serde(rename = "locationId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Deprecated. This field has no effect."]
                            pub location_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Comma separated list of fields to order by, followed by `asc` or `desc` postfix. This list is case-insensitive, default sorting order is ascending, redundant space characters are insignificant. Example: `name asc,update_time, create_time desc` Supported fields are: - `create_time`: corresponds to time the template was created. - `update_time`: corresponds to time the template was last updated. - `name`: corresponds to template's name. - `display_name`: corresponds to template's display name."]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Size of the page, can be limited by server. If zero server returns a page of max size 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Page token to continue retrieval. Comes from previous call to `ListDeidentifyTemplates`."]
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
            pub mod inspect_templates {
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
                            #[serde(rename = "locationId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Deprecated. This field has no effect."]
                            pub location_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Comma separated list of fields to order by, followed by `asc` or `desc` postfix. This list is case-insensitive, default sorting order is ascending, redundant space characters are insignificant. Example: `name asc,update_time, create_time desc` Supported fields are: - `create_time`: corresponds to time the template was created. - `update_time`: corresponds to time the template was last updated. - `name`: corresponds to template's name. - `display_name`: corresponds to template's display name."]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Size of the page, can be limited by server. If zero server returns a page of max size 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Page token to continue retrieval. Comes from previous call to `ListInspectTemplates`."]
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
            pub mod locations {
                pub mod resources {
                    pub mod deidentify_templates {
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
                                    #[serde(rename = "locationId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. This field has no effect."]
                                    pub location_id: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "orderBy")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Comma separated list of fields to order by, followed by `asc` or `desc` postfix. This list is case-insensitive, default sorting order is ascending, redundant space characters are insignificant. Example: `name asc,update_time, create_time desc` Supported fields are: - `create_time`: corresponds to time the template was created. - `update_time`: corresponds to time the template was last updated. - `name`: corresponds to template's name. - `display_name`: corresponds to template's display name."]
                                    pub order_by: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Size of the page, can be limited by server. If zero server returns a page of max size 100."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Page token to continue retrieval. Comes from previous call to `ListDeidentifyTemplates`."]
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
                    pub mod dlp_jobs {
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
                                    #[doc = "Allows filtering. Supported syntax: * Filter expressions are made up of one or more restrictions. * Restrictions can be combined by `AND` or `OR` logical operators. A sequence of restrictions implicitly uses `AND`. * A restriction has the form of `{field} {operator} {value}`. * Supported fields/values for inspect jobs: - `state` - PENDING|RUNNING|CANCELED|FINISHED|FAILED - `inspected_storage` - DATASTORE|CLOUD_STORAGE|BIGQUERY - `trigger_name` - The resource name of the trigger that created job. - 'end_time` - Corresponds to time the job finished. - 'start_time` - Corresponds to time the job finished. * Supported fields for risk analysis jobs: - `state` - RUNNING|CANCELED|FINISHED|FAILED - 'end_time` - Corresponds to time the job finished. - 'start_time` - Corresponds to time the job finished. * The operator must be `=` or `!=`. Examples: * inspected_storage = cloud_storage AND state = done * inspected_storage = cloud_storage OR inspected_storage = bigquery * inspected_storage = cloud_storage AND (state = done OR state = canceled) * end_time > \\\"2017-12-12T00:00:00+00:00\\\" The length of this field should be no more than 500 characters."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "locationId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. This field has no effect."]
                                    pub location_id: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "orderBy")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Comma separated list of fields to order by, followed by `asc` or `desc` postfix. This list is case-insensitive, default sorting order is ascending, redundant space characters are insignificant. Example: `name asc, end_time asc, create_time desc` Supported fields are: - `create_time`: corresponds to time the job was created. - `end_time`: corresponds to time the job ended. - `name`: corresponds to job's name. - `state`: corresponds to `state`"]
                                    pub order_by: ::std::option::Option<::std::string::String>,
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "type")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The type of job. Defaults to `DlpJobType.INSPECT`"]
                                    pub _type: ::std::option::Option<QueryParametersTypeEnum>,
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
                                #[doc = "The type of job. Defaults to `DlpJobType.INSPECT`"]
                                pub enum QueryParametersTypeEnum {
                                    #[serde(rename = "DLP_JOB_TYPE_UNSPECIFIED")]
                                    #[doc = "Unused"]
                                    DlpJobTypeUnspecified,
                                    #[serde(rename = "INSPECT_JOB")]
                                    #[doc = "The job inspected Google Cloud for sensitive data."]
                                    InspectJob,
                                    #[serde(rename = "RISK_ANALYSIS_JOB")]
                                    #[doc = "The job executed a Risk Analysis computation."]
                                    RiskAnalysisJob,
                                }
                                impl ::std::default::Default for QueryParametersTypeEnum {
                                    fn default() -> Self {
                                        Self::DlpJobTypeUnspecified
                                    }
                                }
                            }
                        }
                    }
                    pub mod inspect_templates {
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
                                    #[serde(rename = "locationId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. This field has no effect."]
                                    pub location_id: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "orderBy")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Comma separated list of fields to order by, followed by `asc` or `desc` postfix. This list is case-insensitive, default sorting order is ascending, redundant space characters are insignificant. Example: `name asc,update_time, create_time desc` Supported fields are: - `create_time`: corresponds to time the template was created. - `update_time`: corresponds to time the template was last updated. - `name`: corresponds to template's name. - `display_name`: corresponds to template's display name."]
                                    pub order_by: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Size of the page, can be limited by server. If zero server returns a page of max size 100."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Page token to continue retrieval. Comes from previous call to `ListInspectTemplates`."]
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
                    pub mod job_triggers {
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
                                    #[doc = "Allows filtering. Supported syntax: * Filter expressions are made up of one or more restrictions. * Restrictions can be combined by `AND` or `OR` logical operators. A sequence of restrictions implicitly uses `AND`. * A restriction has the form of `{field} {operator} {value}`. * Supported fields/values for inspect triggers: - `status` - HEALTHY|PAUSED|CANCELLED - `inspected_storage` - DATASTORE|CLOUD_STORAGE|BIGQUERY - 'last_run_time` - RFC 3339 formatted timestamp, surrounded by quotation marks. Nanoseconds are ignored. - 'error_count' - Number of errors that have occurred while running. * The operator must be `=` or `!=` for status and inspected_storage. Examples: * inspected_storage = cloud_storage AND status = HEALTHY * inspected_storage = cloud_storage OR inspected_storage = bigquery * inspected_storage = cloud_storage AND (state = PAUSED OR state = HEALTHY) * last_run_time > \\\"2017-12-12T00:00:00+00:00\\\" The length of this field should be no more than 500 characters."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "locationId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. This field has no effect."]
                                    pub location_id: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "orderBy")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Comma separated list of triggeredJob fields to order by, followed by `asc` or `desc` postfix. This list is case-insensitive, default sorting order is ascending, redundant space characters are insignificant. Example: `name asc,update_time, create_time desc` Supported fields are: - `create_time`: corresponds to time the JobTrigger was created. - `update_time`: corresponds to time the JobTrigger was last updated. - `last_run_time`: corresponds to the last time the JobTrigger ran. - `name`: corresponds to JobTrigger's name. - `display_name`: corresponds to JobTrigger's display name. - `status`: corresponds to JobTrigger's status."]
                                    pub order_by: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Size of the page, can be limited by a server."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Page token to continue retrieval. Comes from previous call to ListJobTriggers. `order_by` field must not change for subsequent calls."]
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
                    pub mod stored_info_types {
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
                                    #[serde(rename = "locationId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. This field has no effect."]
                                    pub location_id: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "orderBy")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Comma separated list of fields to order by, followed by `asc` or `desc` postfix. This list is case-insensitive, default sorting order is ascending, redundant space characters are insignificant. Example: `name asc, display_name, create_time desc` Supported fields are: - `create_time`: corresponds to time the most recent version of the resource was created. - `state`: corresponds to the state of the resource. - `name`: corresponds to resource name. - `display_name`: corresponds to info type's display name."]
                                    pub order_by: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Size of the page, can be limited by server. If zero server returns a page of max size 100."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Page token to continue retrieval. Comes from previous call to `ListStoredInfoTypes`."]
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
            pub mod stored_info_types {
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
                            #[serde(rename = "locationId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Deprecated. This field has no effect."]
                            pub location_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Comma separated list of fields to order by, followed by `asc` or `desc` postfix. This list is case-insensitive, default sorting order is ascending, redundant space characters are insignificant. Example: `name asc, display_name, create_time desc` Supported fields are: - `create_time`: corresponds to time the most recent version of the resource was created. - `state`: corresponds to the state of the resource. - `name`: corresponds to resource name. - `display_name`: corresponds to info type's display name."]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Size of the page, can be limited by server. If zero server returns a page of max size 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Page token to continue retrieval. Comes from previous call to `ListStoredInfoTypes`."]
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
    pub mod projects {
        pub mod resources {
            pub mod deidentify_templates {
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
                            #[serde(rename = "locationId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Deprecated. This field has no effect."]
                            pub location_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Comma separated list of fields to order by, followed by `asc` or `desc` postfix. This list is case-insensitive, default sorting order is ascending, redundant space characters are insignificant. Example: `name asc,update_time, create_time desc` Supported fields are: - `create_time`: corresponds to time the template was created. - `update_time`: corresponds to time the template was last updated. - `name`: corresponds to template's name. - `display_name`: corresponds to template's display name."]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Size of the page, can be limited by server. If zero server returns a page of max size 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Page token to continue retrieval. Comes from previous call to `ListDeidentifyTemplates`."]
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
            pub mod dlp_jobs {
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
                            #[doc = "Allows filtering. Supported syntax: * Filter expressions are made up of one or more restrictions. * Restrictions can be combined by `AND` or `OR` logical operators. A sequence of restrictions implicitly uses `AND`. * A restriction has the form of `{field} {operator} {value}`. * Supported fields/values for inspect jobs: - `state` - PENDING|RUNNING|CANCELED|FINISHED|FAILED - `inspected_storage` - DATASTORE|CLOUD_STORAGE|BIGQUERY - `trigger_name` - The resource name of the trigger that created job. - 'end_time` - Corresponds to time the job finished. - 'start_time` - Corresponds to time the job finished. * Supported fields for risk analysis jobs: - `state` - RUNNING|CANCELED|FINISHED|FAILED - 'end_time` - Corresponds to time the job finished. - 'start_time` - Corresponds to time the job finished. * The operator must be `=` or `!=`. Examples: * inspected_storage = cloud_storage AND state = done * inspected_storage = cloud_storage OR inspected_storage = bigquery * inspected_storage = cloud_storage AND (state = done OR state = canceled) * end_time > \\\"2017-12-12T00:00:00+00:00\\\" The length of this field should be no more than 500 characters."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "locationId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Deprecated. This field has no effect."]
                            pub location_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Comma separated list of fields to order by, followed by `asc` or `desc` postfix. This list is case-insensitive, default sorting order is ascending, redundant space characters are insignificant. Example: `name asc, end_time asc, create_time desc` Supported fields are: - `create_time`: corresponds to time the job was created. - `end_time`: corresponds to time the job ended. - `name`: corresponds to job's name. - `state`: corresponds to `state`"]
                            pub order_by: ::std::option::Option<::std::string::String>,
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "type")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The type of job. Defaults to `DlpJobType.INSPECT`"]
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
                        #[doc = "The type of job. Defaults to `DlpJobType.INSPECT`"]
                        pub enum QueryParametersTypeEnum {
                            #[serde(rename = "DLP_JOB_TYPE_UNSPECIFIED")]
                            #[doc = "Unused"]
                            DlpJobTypeUnspecified,
                            #[serde(rename = "INSPECT_JOB")]
                            #[doc = "The job inspected Google Cloud for sensitive data."]
                            InspectJob,
                            #[serde(rename = "RISK_ANALYSIS_JOB")]
                            #[doc = "The job executed a Risk Analysis computation."]
                            RiskAnalysisJob,
                        }
                        impl ::std::default::Default for QueryParametersTypeEnum {
                            fn default() -> Self {
                                Self::DlpJobTypeUnspecified
                            }
                        }
                    }
                }
            }
            pub mod inspect_templates {
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
                            #[serde(rename = "locationId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Deprecated. This field has no effect."]
                            pub location_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Comma separated list of fields to order by, followed by `asc` or `desc` postfix. This list is case-insensitive, default sorting order is ascending, redundant space characters are insignificant. Example: `name asc,update_time, create_time desc` Supported fields are: - `create_time`: corresponds to time the template was created. - `update_time`: corresponds to time the template was last updated. - `name`: corresponds to template's name. - `display_name`: corresponds to template's display name."]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Size of the page, can be limited by server. If zero server returns a page of max size 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Page token to continue retrieval. Comes from previous call to `ListInspectTemplates`."]
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
            pub mod job_triggers {
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
                            #[doc = "Allows filtering. Supported syntax: * Filter expressions are made up of one or more restrictions. * Restrictions can be combined by `AND` or `OR` logical operators. A sequence of restrictions implicitly uses `AND`. * A restriction has the form of `{field} {operator} {value}`. * Supported fields/values for inspect triggers: - `status` - HEALTHY|PAUSED|CANCELLED - `inspected_storage` - DATASTORE|CLOUD_STORAGE|BIGQUERY - 'last_run_time` - RFC 3339 formatted timestamp, surrounded by quotation marks. Nanoseconds are ignored. - 'error_count' - Number of errors that have occurred while running. * The operator must be `=` or `!=` for status and inspected_storage. Examples: * inspected_storage = cloud_storage AND status = HEALTHY * inspected_storage = cloud_storage OR inspected_storage = bigquery * inspected_storage = cloud_storage AND (state = PAUSED OR state = HEALTHY) * last_run_time > \\\"2017-12-12T00:00:00+00:00\\\" The length of this field should be no more than 500 characters."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "locationId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Deprecated. This field has no effect."]
                            pub location_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Comma separated list of triggeredJob fields to order by, followed by `asc` or `desc` postfix. This list is case-insensitive, default sorting order is ascending, redundant space characters are insignificant. Example: `name asc,update_time, create_time desc` Supported fields are: - `create_time`: corresponds to time the JobTrigger was created. - `update_time`: corresponds to time the JobTrigger was last updated. - `last_run_time`: corresponds to the last time the JobTrigger ran. - `name`: corresponds to JobTrigger's name. - `display_name`: corresponds to JobTrigger's display name. - `status`: corresponds to JobTrigger's status."]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Size of the page, can be limited by a server."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Page token to continue retrieval. Comes from previous call to ListJobTriggers. `order_by` field must not change for subsequent calls."]
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
            pub mod locations {
                pub mod resources {
                    pub mod deidentify_templates {
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
                                    #[serde(rename = "locationId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. This field has no effect."]
                                    pub location_id: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "orderBy")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Comma separated list of fields to order by, followed by `asc` or `desc` postfix. This list is case-insensitive, default sorting order is ascending, redundant space characters are insignificant. Example: `name asc,update_time, create_time desc` Supported fields are: - `create_time`: corresponds to time the template was created. - `update_time`: corresponds to time the template was last updated. - `name`: corresponds to template's name. - `display_name`: corresponds to template's display name."]
                                    pub order_by: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Size of the page, can be limited by server. If zero server returns a page of max size 100."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Page token to continue retrieval. Comes from previous call to `ListDeidentifyTemplates`."]
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
                    pub mod dlp_jobs {
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
                                    #[doc = "Allows filtering. Supported syntax: * Filter expressions are made up of one or more restrictions. * Restrictions can be combined by `AND` or `OR` logical operators. A sequence of restrictions implicitly uses `AND`. * A restriction has the form of `{field} {operator} {value}`. * Supported fields/values for inspect jobs: - `state` - PENDING|RUNNING|CANCELED|FINISHED|FAILED - `inspected_storage` - DATASTORE|CLOUD_STORAGE|BIGQUERY - `trigger_name` - The resource name of the trigger that created job. - 'end_time` - Corresponds to time the job finished. - 'start_time` - Corresponds to time the job finished. * Supported fields for risk analysis jobs: - `state` - RUNNING|CANCELED|FINISHED|FAILED - 'end_time` - Corresponds to time the job finished. - 'start_time` - Corresponds to time the job finished. * The operator must be `=` or `!=`. Examples: * inspected_storage = cloud_storage AND state = done * inspected_storage = cloud_storage OR inspected_storage = bigquery * inspected_storage = cloud_storage AND (state = done OR state = canceled) * end_time > \\\"2017-12-12T00:00:00+00:00\\\" The length of this field should be no more than 500 characters."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "locationId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. This field has no effect."]
                                    pub location_id: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "orderBy")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Comma separated list of fields to order by, followed by `asc` or `desc` postfix. This list is case-insensitive, default sorting order is ascending, redundant space characters are insignificant. Example: `name asc, end_time asc, create_time desc` Supported fields are: - `create_time`: corresponds to time the job was created. - `end_time`: corresponds to time the job ended. - `name`: corresponds to job's name. - `state`: corresponds to `state`"]
                                    pub order_by: ::std::option::Option<::std::string::String>,
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "type")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The type of job. Defaults to `DlpJobType.INSPECT`"]
                                    pub _type: ::std::option::Option<QueryParametersTypeEnum>,
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
                                #[doc = "The type of job. Defaults to `DlpJobType.INSPECT`"]
                                pub enum QueryParametersTypeEnum {
                                    #[serde(rename = "DLP_JOB_TYPE_UNSPECIFIED")]
                                    #[doc = "Unused"]
                                    DlpJobTypeUnspecified,
                                    #[serde(rename = "INSPECT_JOB")]
                                    #[doc = "The job inspected Google Cloud for sensitive data."]
                                    InspectJob,
                                    #[serde(rename = "RISK_ANALYSIS_JOB")]
                                    #[doc = "The job executed a Risk Analysis computation."]
                                    RiskAnalysisJob,
                                }
                                impl ::std::default::Default for QueryParametersTypeEnum {
                                    fn default() -> Self {
                                        Self::DlpJobTypeUnspecified
                                    }
                                }
                            }
                        }
                    }
                    pub mod inspect_templates {
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
                                    #[serde(rename = "locationId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. This field has no effect."]
                                    pub location_id: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "orderBy")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Comma separated list of fields to order by, followed by `asc` or `desc` postfix. This list is case-insensitive, default sorting order is ascending, redundant space characters are insignificant. Example: `name asc,update_time, create_time desc` Supported fields are: - `create_time`: corresponds to time the template was created. - `update_time`: corresponds to time the template was last updated. - `name`: corresponds to template's name. - `display_name`: corresponds to template's display name."]
                                    pub order_by: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Size of the page, can be limited by server. If zero server returns a page of max size 100."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Page token to continue retrieval. Comes from previous call to `ListInspectTemplates`."]
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
                    pub mod job_triggers {
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
                                    #[doc = "Allows filtering. Supported syntax: * Filter expressions are made up of one or more restrictions. * Restrictions can be combined by `AND` or `OR` logical operators. A sequence of restrictions implicitly uses `AND`. * A restriction has the form of `{field} {operator} {value}`. * Supported fields/values for inspect triggers: - `status` - HEALTHY|PAUSED|CANCELLED - `inspected_storage` - DATASTORE|CLOUD_STORAGE|BIGQUERY - 'last_run_time` - RFC 3339 formatted timestamp, surrounded by quotation marks. Nanoseconds are ignored. - 'error_count' - Number of errors that have occurred while running. * The operator must be `=` or `!=` for status and inspected_storage. Examples: * inspected_storage = cloud_storage AND status = HEALTHY * inspected_storage = cloud_storage OR inspected_storage = bigquery * inspected_storage = cloud_storage AND (state = PAUSED OR state = HEALTHY) * last_run_time > \\\"2017-12-12T00:00:00+00:00\\\" The length of this field should be no more than 500 characters."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "locationId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. This field has no effect."]
                                    pub location_id: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "orderBy")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Comma separated list of triggeredJob fields to order by, followed by `asc` or `desc` postfix. This list is case-insensitive, default sorting order is ascending, redundant space characters are insignificant. Example: `name asc,update_time, create_time desc` Supported fields are: - `create_time`: corresponds to time the JobTrigger was created. - `update_time`: corresponds to time the JobTrigger was last updated. - `last_run_time`: corresponds to the last time the JobTrigger ran. - `name`: corresponds to JobTrigger's name. - `display_name`: corresponds to JobTrigger's display name. - `status`: corresponds to JobTrigger's status."]
                                    pub order_by: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Size of the page, can be limited by a server."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Page token to continue retrieval. Comes from previous call to ListJobTriggers. `order_by` field must not change for subsequent calls."]
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
                    pub mod stored_info_types {
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
                                    #[serde(rename = "locationId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. This field has no effect."]
                                    pub location_id: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "orderBy")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Comma separated list of fields to order by, followed by `asc` or `desc` postfix. This list is case-insensitive, default sorting order is ascending, redundant space characters are insignificant. Example: `name asc, display_name, create_time desc` Supported fields are: - `create_time`: corresponds to time the most recent version of the resource was created. - `state`: corresponds to the state of the resource. - `name`: corresponds to resource name. - `display_name`: corresponds to info type's display name."]
                                    pub order_by: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Size of the page, can be limited by server. If zero server returns a page of max size 100."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Page token to continue retrieval. Comes from previous call to `ListStoredInfoTypes`."]
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
            pub mod stored_info_types {
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
                            #[serde(rename = "locationId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Deprecated. This field has no effect."]
                            pub location_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Comma separated list of fields to order by, followed by `asc` or `desc` postfix. This list is case-insensitive, default sorting order is ascending, redundant space characters are insignificant. Example: `name asc, display_name, create_time desc` Supported fields are: - `create_time`: corresponds to time the most recent version of the resource was created. - `state`: corresponds to the state of the resource. - `name`: corresponds to resource name. - `display_name`: corresponds to info type's display name."]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Size of the page, can be limited by server. If zero server returns a page of max size 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Page token to continue retrieval. Comes from previous call to `ListStoredInfoTypes`."]
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
    #[doc = "A task to execute on the completion of a job. See https://cloud.google.com/dlp/docs/concepts-actions to learn more."]
    pub struct GooglePrivacyDlpV2Action {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobNotificationEmails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enable email notification for project owners and editors on job's completion/failure."]
        pub job_notification_emails:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2JobNotificationEmails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pubSub")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Publish a notification to a pubsub topic."]
        pub pub_sub: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PublishToPubSub>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishFindingsToCloudDataCatalog")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Publish findings to Cloud Datahub."]
        pub publish_findings_to_cloud_data_catalog: ::std::option::Option<
            ::std::boxed::Box<GooglePrivacyDlpV2PublishFindingsToCloudDataCatalog>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishSummaryToCscc")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Publish summary to Cloud Security Command Center (Alpha)."]
        pub publish_summary_to_cscc:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PublishSummaryToCscc>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishToStackdriver")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enable Stackdriver metric dlp.googleapis.com/finding_count."]
        pub publish_to_stackdriver:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PublishToStackdriver>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "saveFindings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Save resulting findings in a provided location."]
        pub save_findings: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2SaveFindings>>,
    }
    impl GooglePrivacyDlpV2Action {
        pub fn builder() -> GooglePrivacyDlpV2ActionBuilder {
            GooglePrivacyDlpV2ActionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for ActivateJobTrigger."]
    pub struct GooglePrivacyDlpV2ActivateJobTriggerRequest {}
    impl GooglePrivacyDlpV2ActivateJobTriggerRequest {
        pub fn builder() -> GooglePrivacyDlpV2ActivateJobTriggerRequestBuilder {
            GooglePrivacyDlpV2ActivateJobTriggerRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Result of a risk analysis operation request."]
    pub struct GooglePrivacyDlpV2AnalyzeDataSourceRiskDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categoricalStatsResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Categorical stats result"]
        pub categorical_stats_result:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CategoricalStatsResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deltaPresenceEstimationResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Delta-presence result"]
        pub delta_presence_estimation_result: ::std::option::Option<
            ::std::boxed::Box<GooglePrivacyDlpV2DeltaPresenceEstimationResult>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kAnonymityResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "K-anonymity result"]
        pub k_anonymity_result:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2KAnonymityResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kMapEstimationResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "K-map result"]
        pub k_map_estimation_result:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2KMapEstimationResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lDiversityResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "L-divesity result"]
        pub l_diversity_result:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2LDiversityResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numericalStatsResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Numerical stats result"]
        pub numerical_stats_result:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2NumericalStatsResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestedOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The configuration used for this job."]
        pub requested_options: ::std::option::Option<
            ::std::boxed::Box<GooglePrivacyDlpV2RequestedRiskAnalysisOptions>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestedPrivacyMetric")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Privacy metric to compute."]
        pub requested_privacy_metric:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PrivacyMetric>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestedSourceTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input dataset to compute metrics over."]
        pub requested_source_table:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryTable>>,
    }
    impl GooglePrivacyDlpV2AnalyzeDataSourceRiskDetails {
        pub fn builder() -> GooglePrivacyDlpV2AnalyzeDataSourceRiskDetailsBuilder {
            GooglePrivacyDlpV2AnalyzeDataSourceRiskDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An auxiliary table contains statistical information on the relative frequency of different quasi-identifiers values. It has one or several quasi-identifiers columns, and one column that indicates the relative frequency of each quasi-identifier tuple. If a tuple is present in the data but not in the auxiliary table, the corresponding relative frequency is assumed to be zero (and thus, the tuple is highly reidentifiable)."]
    pub struct GooglePrivacyDlpV2AuxiliaryTable {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quasiIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Quasi-identifier columns."]
        pub quasi_ids: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2QuasiIdField>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relativeFrequency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The relative frequency column must contain a floating-point number between 0 and 1 (inclusive). Null values are assumed to be zero."]
        pub relative_frequency: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "table")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Auxiliary table location."]
        pub table: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryTable>>,
    }
    impl GooglePrivacyDlpV2AuxiliaryTable {
        pub fn builder() -> GooglePrivacyDlpV2AuxiliaryTableBuilder {
            GooglePrivacyDlpV2AuxiliaryTableBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message defining a field of a BigQuery table."]
    pub struct GooglePrivacyDlpV2BigQueryField {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Designated field in the BigQuery table."]
        pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "table")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Source table of the field."]
        pub table: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryTable>>,
    }
    impl GooglePrivacyDlpV2BigQueryField {
        pub fn builder() -> GooglePrivacyDlpV2BigQueryFieldBuilder {
            GooglePrivacyDlpV2BigQueryFieldBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Row key for identifying a record in BigQuery table."]
    pub struct GooglePrivacyDlpV2BigQueryKey {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Row number inferred at the time the table was scanned. This value is nondeterministic, cannot be queried, and may be null for inspection jobs. To locate findings within a table, specify `inspect_job.storage_config.big_query_options.identifying_fields` in `CreateDlpJobRequest`."]
        pub row_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Complete BigQuery table reference."]
        pub table_reference:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryTable>>,
    }
    impl GooglePrivacyDlpV2BigQueryKey {
        pub fn builder() -> GooglePrivacyDlpV2BigQueryKeyBuilder {
            GooglePrivacyDlpV2BigQueryKeyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options defining BigQuery table and row identifiers."]
    pub struct GooglePrivacyDlpV2BigQueryOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludedFields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "References to fields excluded from scanning. This allows you to skip inspection of entire columns which you know have no findings."]
        pub excluded_fields:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "identifyingFields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Table fields that may uniquely identify a row within the table. When `actions.saveFindings.outputConfig.table` is specified, the values of columns specified here are available in the output table under `location.content_locations.record_location.record_key.id_values`. Nested fields such as `person.birthdate.year` are allowed."]
        pub identifying_fields:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowsLimit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Max number of rows to scan. If the table has more rows than this value, the rest of the rows are omitted. If not set, or if set to 0, all rows will be scanned. Only one of rows_limit and rows_limit_percent can be specified. Cannot be used in conjunction with TimespanConfig."]
        pub rows_limit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowsLimitPercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Max percentage of rows to scan. The rest are omitted. The number of rows scanned is rounded down. Must be between 0 and 100, inclusively. Both 0 and 100 means no limit. Defaults to 0. Only one of rows_limit and rows_limit_percent can be specified. Cannot be used in conjunction with TimespanConfig."]
        pub rows_limit_percent: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sampleMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub sample_method: ::std::option::Option<GooglePrivacyDlpV2BigQueryOptionsSampleMethodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Complete BigQuery table reference."]
        pub table_reference:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryTable>>,
    }
    impl GooglePrivacyDlpV2BigQueryOptions {
        pub fn builder() -> GooglePrivacyDlpV2BigQueryOptionsBuilder {
            GooglePrivacyDlpV2BigQueryOptionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GooglePrivacyDlpV2BigQueryOptionsSampleMethodEnum {
        #[serde(rename = "SAMPLE_METHOD_UNSPECIFIED")]
        #[doc = ""]
        SampleMethodUnspecified,
        #[serde(rename = "TOP")]
        #[doc = "Scan groups of rows in the order BigQuery provides (default). Multiple groups of rows may be scanned in parallel, so results may not appear in the same order the rows are read."]
        Top,
        #[serde(rename = "RANDOM_START")]
        #[doc = "Randomly pick groups of rows to scan."]
        RandomStart,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2BigQueryOptionsSampleMethodEnum {
        fn default() -> Self {
            Self::SampleMethodUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message defining the location of a BigQuery table. A table is uniquely identified by its project_id, dataset_id, and table_name. Within a query a table is often referenced with a string in the format of: `:.` or `..`."]
    pub struct GooglePrivacyDlpV2BigQueryTable {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datasetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dataset ID of the table."]
        pub dataset_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Cloud Platform project ID of the project containing the table. If omitted, project ID is inferred from the API call."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the table."]
        pub table_id: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2BigQueryTable {
        pub fn builder() -> GooglePrivacyDlpV2BigQueryTableBuilder {
            GooglePrivacyDlpV2BigQueryTableBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Bounding box encompassing detected text within an image."]
    pub struct GooglePrivacyDlpV2BoundingBox {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Height of the bounding box in pixels."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "left")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Left coordinate of the bounding box. (0,0) is upper left."]
        pub left: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "top")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Top coordinate of the bounding box. (0,0) is upper left."]
        pub top: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Width of the bounding box in pixels."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl GooglePrivacyDlpV2BoundingBox {
        pub fn builder() -> GooglePrivacyDlpV2BoundingBoxBuilder {
            GooglePrivacyDlpV2BoundingBoxBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Bucket is represented as a range, along with replacement values."]
    pub struct GooglePrivacyDlpV2Bucket {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "max")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Upper bound of the range, exclusive; type must match min."]
        pub max: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "min")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Lower bound of the range, inclusive. Type should be the same as max if used."]
        pub min: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replacementValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Replacement value for this bucket."]
        pub replacement_value: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
    }
    impl GooglePrivacyDlpV2Bucket {
        pub fn builder() -> GooglePrivacyDlpV2BucketBuilder {
            GooglePrivacyDlpV2BucketBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Generalization function that buckets values based on ranges. The ranges and replacement values are dynamically provided by the user for custom behavior, such as 1-30 -> LOW 31-65 -> MEDIUM 66-100 -> HIGH This can be used on data of type: number, long, string, timestamp. If the bound `Value` type differs from the type of data being transformed, we will first attempt converting the type of the data to be transformed to match the type of the bound before comparing. See https://cloud.google.com/dlp/docs/concepts-bucketing to learn more."]
    pub struct GooglePrivacyDlpV2BucketingConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buckets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of buckets. Ranges must be non-overlapping."]
        pub buckets:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Bucket>>>,
    }
    impl GooglePrivacyDlpV2BucketingConfig {
        pub fn builder() -> GooglePrivacyDlpV2BucketingConfigBuilder {
            GooglePrivacyDlpV2BucketingConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Container for bytes to inspect or redact."]
    pub struct GooglePrivacyDlpV2ByteContentItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Content data to inspect or redact."]
        pub data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of data stored in the bytes string. Default will be TEXT_UTF8."]
        pub _type: ::std::option::Option<GooglePrivacyDlpV2ByteContentItemTypeEnum>,
    }
    impl GooglePrivacyDlpV2ByteContentItem {
        pub fn builder() -> GooglePrivacyDlpV2ByteContentItemBuilder {
            GooglePrivacyDlpV2ByteContentItemBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of data stored in the bytes string. Default will be TEXT_UTF8."]
    pub enum GooglePrivacyDlpV2ByteContentItemTypeEnum {
        #[serde(rename = "BYTES_TYPE_UNSPECIFIED")]
        #[doc = "Unused"]
        BytesTypeUnspecified,
        #[serde(rename = "IMAGE")]
        #[doc = "Any image type."]
        Image,
        #[serde(rename = "IMAGE_JPEG")]
        #[doc = "jpeg"]
        ImageJpeg,
        #[serde(rename = "IMAGE_BMP")]
        #[doc = "bmp"]
        ImageBmp,
        #[serde(rename = "IMAGE_PNG")]
        #[doc = "png"]
        ImagePng,
        #[serde(rename = "IMAGE_SVG")]
        #[doc = "svg"]
        ImageSvg,
        #[serde(rename = "TEXT_UTF8")]
        #[doc = "plain text"]
        TextUtf8,
        #[serde(rename = "WORD_DOCUMENT")]
        #[doc = "docx, docm, dotx, dotm"]
        WordDocument,
        #[serde(rename = "PDF")]
        #[doc = "pdf"]
        Pdf,
        #[serde(rename = "AVRO")]
        #[doc = "avro"]
        Avro,
        #[serde(rename = "CSV")]
        #[doc = "csv"]
        Csv,
        #[serde(rename = "TSV")]
        #[doc = "tsv"]
        Tsv,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2ByteContentItemTypeEnum {
        fn default() -> Self {
            Self::BytesTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for canceling a DLP job."]
    pub struct GooglePrivacyDlpV2CancelDlpJobRequest {}
    impl GooglePrivacyDlpV2CancelDlpJobRequest {
        pub fn builder() -> GooglePrivacyDlpV2CancelDlpJobRequestBuilder {
            GooglePrivacyDlpV2CancelDlpJobRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Compute numerical stats over an individual column, including number of distinct values and value count distribution."]
    pub struct GooglePrivacyDlpV2CategoricalStatsConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Field to compute categorical stats on. All column types are supported except for arrays and structs. However, it may be more informative to use NumericalStats when the field type is supported, depending on the data."]
        pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
    }
    impl GooglePrivacyDlpV2CategoricalStatsConfig {
        pub fn builder() -> GooglePrivacyDlpV2CategoricalStatsConfigBuilder {
            GooglePrivacyDlpV2CategoricalStatsConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Histogram of value frequencies in the column."]
    pub struct GooglePrivacyDlpV2CategoricalStatsHistogramBucket {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of values in this bucket."]
        pub bucket_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketValueCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of distinct values in this bucket."]
        pub bucket_value_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sample of value frequencies in this bucket. The total number of values returned per bucket is capped at 20."]
        pub bucket_values: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2ValueFrequency>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueFrequencyLowerBound")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Lower bound on the value frequency of the values in this bucket."]
        pub value_frequency_lower_bound: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueFrequencyUpperBound")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Upper bound on the value frequency of the values in this bucket."]
        pub value_frequency_upper_bound: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2CategoricalStatsHistogramBucket {
        pub fn builder() -> GooglePrivacyDlpV2CategoricalStatsHistogramBucketBuilder {
            GooglePrivacyDlpV2CategoricalStatsHistogramBucketBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Result of the categorical stats computation."]
    pub struct GooglePrivacyDlpV2CategoricalStatsResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueFrequencyHistogramBuckets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Histogram of value frequencies in the column."]
        pub value_frequency_histogram_buckets: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2CategoricalStatsHistogramBucket>>,
        >,
    }
    impl GooglePrivacyDlpV2CategoricalStatsResult {
        pub fn builder() -> GooglePrivacyDlpV2CategoricalStatsResultBuilder {
            GooglePrivacyDlpV2CategoricalStatsResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Partially mask a string by replacing a given number of characters with a fixed character. Masking can start from the beginning or end of the string. This can be used on data of any type (numbers, longs, and so on) and when de-identifying structured data we'll attempt to preserve the original data's type. (This allows you to take a long like 123 and modify it to a string like **3."]
    pub struct GooglePrivacyDlpV2CharacterMaskConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "charactersToIgnore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When masking a string, items in this list will be skipped when replacing characters. For example, if the input string is `555-555-5555` and you instruct Cloud DLP to skip `-` and mask 5 characters with `*`, Cloud DLP returns `***-**5-5555`."]
        pub characters_to_ignore: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2CharsToIgnore>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maskingCharacter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Character to use to mask the sensitive values—for example, `*` for an alphabetic string such as a name, or `0` for a numeric string such as ZIP code or credit card number. This string must have a length of 1. If not supplied, this value defaults to `*` for strings, and `0` for digits."]
        pub masking_character: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numberToMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of characters to mask. If not set, all matching chars will be masked. Skipped characters do not count towards this tally."]
        pub number_to_mask: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reverseOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mask characters in reverse order. For example, if `masking_character` is `0`, `number_to_mask` is `14`, and `reverse_order` is `false`, then the input string `1234-5678-9012-3456` is masked as `00000000000000-3456`. If `masking_character` is `*`, `number_to_mask` is `3`, and `reverse_order` is `true`, then the string `12345` is masked as `12***`."]
        pub reverse_order: ::std::option::Option<::std::primitive::bool>,
    }
    impl GooglePrivacyDlpV2CharacterMaskConfig {
        pub fn builder() -> GooglePrivacyDlpV2CharacterMaskConfigBuilder {
            GooglePrivacyDlpV2CharacterMaskConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Characters to skip when doing deidentification of a value. These will be left alone and skipped."]
    pub struct GooglePrivacyDlpV2CharsToIgnore {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "charactersToSkip")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Characters to not transform when masking."]
        pub characters_to_skip: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonCharactersToIgnore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Common characters to not transform when masking. Useful to avoid removing punctuation."]
        pub common_characters_to_ignore:
            ::std::option::Option<GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum>,
    }
    impl GooglePrivacyDlpV2CharsToIgnore {
        pub fn builder() -> GooglePrivacyDlpV2CharsToIgnoreBuilder {
            GooglePrivacyDlpV2CharsToIgnoreBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Common characters to not transform when masking. Useful to avoid removing punctuation."]
    pub enum GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum {
        #[serde(rename = "COMMON_CHARS_TO_IGNORE_UNSPECIFIED")]
        #[doc = "Unused."]
        CommonCharsToIgnoreUnspecified,
        #[serde(rename = "NUMERIC")]
        #[doc = "0-9"]
        Numeric,
        #[serde(rename = "ALPHA_UPPER_CASE")]
        #[doc = "A-Z"]
        AlphaUpperCase,
        #[serde(rename = "ALPHA_LOWER_CASE")]
        #[doc = "a-z"]
        AlphaLowerCase,
        #[serde(rename = "PUNCTUATION")]
        #[doc = "US Punctuation, one of !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~"]
        Punctuation,
        #[serde(rename = "WHITESPACE")]
        #[doc = "Whitespace character, one of [ \\t\\n\\x0B\\f\\r]"]
        Whitespace,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum {
        fn default() -> Self {
            Self::CommonCharsToIgnoreUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message representing a set of files in Cloud Storage."]
    pub struct GooglePrivacyDlpV2CloudStorageFileSet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The url, in the format `gs:///`. Trailing wildcard in the path is allowed."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2CloudStorageFileSet {
        pub fn builder() -> GooglePrivacyDlpV2CloudStorageFileSetBuilder {
            GooglePrivacyDlpV2CloudStorageFileSetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options defining a file or a set of files within a Google Cloud Storage bucket."]
    pub struct GooglePrivacyDlpV2CloudStorageOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bytesLimitPerFile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Max number of bytes to scan from a file. If a scanned file's size is bigger than this value then the rest of the bytes are omitted. Only one of bytes_limit_per_file and bytes_limit_per_file_percent can be specified. Cannot be set if de-identification is requested."]
        pub bytes_limit_per_file: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bytesLimitPerFilePercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Max percentage of bytes to scan from a file. The rest are omitted. The number of bytes scanned is rounded down. Must be between 0 and 100, inclusively. Both 0 and 100 means no limit. Defaults to 0. Only one of bytes_limit_per_file and bytes_limit_per_file_percent can be specified. Cannot be set if de-identification is requested."]
        pub bytes_limit_per_file_percent: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of one or more files to scan."]
        pub file_set: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FileSet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of file type groups to include in the scan. If empty, all files are scanned and available data format processors are applied. In addition, the binary content of the selected files is always scanned as well. Images are scanned only as binary if the specified region does not support image inspection and no file_types were specified. Image inspection is restricted to 'global', 'us', 'asia', and 'europe'."]
        pub file_types: ::std::option::Option<
            ::std::vec::Vec<GooglePrivacyDlpV2CloudStorageOptionsFileTypesEnum>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filesLimitPercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Limits the number of files to scan to this percentage of the input FileSet. Number of files scanned is rounded down. Must be between 0 and 100, inclusively. Both 0 and 100 means no limit. Defaults to 0."]
        pub files_limit_percent: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sampleMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub sample_method:
            ::std::option::Option<GooglePrivacyDlpV2CloudStorageOptionsSampleMethodEnum>,
    }
    impl GooglePrivacyDlpV2CloudStorageOptions {
        pub fn builder() -> GooglePrivacyDlpV2CloudStorageOptionsBuilder {
            GooglePrivacyDlpV2CloudStorageOptionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GooglePrivacyDlpV2CloudStorageOptionsFileTypesEnum {
        #[serde(rename = "FILE_TYPE_UNSPECIFIED")]
        #[doc = "Includes all files."]
        FileTypeUnspecified,
        #[serde(rename = "BINARY_FILE")]
        #[doc = "Includes all file extensions not covered by another entry. Binary scanning attempts to convert the content of the file to utf_8 to scan the file. If you wish to avoid this fall back, specify one or more of the other FileType's in your storage scan."]
        BinaryFile,
        #[serde(rename = "TEXT_FILE")]
        #[doc = "Included file extensions: asc,asp, aspx, brf, c, cc,cfm, cgi, cpp, csv, cxx, c++, cs, css, dart, dat, dot, eml,, epbub, ged, go, h, hh, hpp, hxx, h++, hs, html, htm, mkd, markdown, m, ml, mli, perl, pl, plist, pm, php, phtml, pht, properties, py, pyw, rb, rbw, rs, rss, rc, scala, sh, sql, swift, tex, shtml, shtm, xhtml, lhs, ics, ini, java, js, json, kix, kml, ocaml, md, txt, text, tsv, vb, vcard, vcs, wml, xcodeproj, xml, xsl, xsd, yml, yaml."]
        TextFile,
        #[serde(rename = "IMAGE")]
        #[doc = "Included file extensions: bmp, gif, jpg, jpeg, jpe, png. bytes_limit_per_file has no effect on image files. Image inspection is restricted to 'global', 'us', 'asia', and 'europe'."]
        Image,
        #[serde(rename = "WORD")]
        #[doc = "Word files >30 MB will be scanned as binary files. Included file extensions: docx, dotx, docm, dotm"]
        Word,
        #[serde(rename = "PDF")]
        #[doc = "PDF files >30 MB will be scanned as binary files. Included file extensions: pdf"]
        Pdf,
        #[serde(rename = "AVRO")]
        #[doc = "Included file extensions: avro"]
        Avro,
        #[serde(rename = "CSV")]
        #[doc = "Included file extensions: csv"]
        Csv,
        #[serde(rename = "TSV")]
        #[doc = "Included file extensions: tsv"]
        Tsv,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2CloudStorageOptionsFileTypesEnum {
        fn default() -> Self {
            Self::FileTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GooglePrivacyDlpV2CloudStorageOptionsSampleMethodEnum {
        #[serde(rename = "SAMPLE_METHOD_UNSPECIFIED")]
        #[doc = ""]
        SampleMethodUnspecified,
        #[serde(rename = "TOP")]
        #[doc = "Scan from the top (default)."]
        Top,
        #[serde(rename = "RANDOM_START")]
        #[doc = "For each file larger than bytes_limit_per_file, randomly pick the offset to start scanning. The scanned bytes are contiguous."]
        RandomStart,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2CloudStorageOptionsSampleMethodEnum {
        fn default() -> Self {
            Self::SampleMethodUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message representing a single file or path in Cloud Storage."]
    pub struct GooglePrivacyDlpV2CloudStoragePath {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A url representing a file or path (no wildcards) in Cloud Storage. Example: gs://[BUCKET_NAME]/dictionary.txt"]
        pub path: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2CloudStoragePath {
        pub fn builder() -> GooglePrivacyDlpV2CloudStoragePathBuilder {
            GooglePrivacyDlpV2CloudStoragePathBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message representing a set of files in a Cloud Storage bucket. Regular expressions are used to allow fine-grained control over which files in the bucket to include. Included files are those that match at least one item in `include_regex` and do not match any items in `exclude_regex`. Note that a file that matches items from both lists will _not_ be included. For a match to occur, the entire file path (i.e., everything in the url after the bucket name) must match the regular expression. For example, given the input `{bucket_name: \"mybucket\", include_regex: [\"directory1/.*\"], exclude_regex: [\"directory1/excluded.*\"]}`: * `gs://mybucket/directory1/myfile` will be included * `gs://mybucket/directory1/directory2/myfile` will be included (`.*` matches across `/`) * `gs://mybucket/directory0/directory1/myfile` will _not_ be included (the full path doesn't match any items in `include_regex`) * `gs://mybucket/directory1/excludedfile` will _not_ be included (the path matches an item in `exclude_regex`) If `include_regex` is left empty, it will match all files by default (this is equivalent to setting `include_regex: [\".*\"]`). Some other common use cases: * `{bucket_name: \"mybucket\", exclude_regex: [\".*\\.pdf\"]}` will include all files in `mybucket` except for .pdf files * `{bucket_name: \"mybucket\", include_regex: [\"directory/[^/]+\"]}` will include all files directly under `gs://mybucket/directory/`, without matching across `/`"]
    pub struct GooglePrivacyDlpV2CloudStorageRegexFileSet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of a Cloud Storage bucket. Required."]
        pub bucket_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludeRegex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of regular expressions matching file paths to exclude. All files in the bucket that match at least one of these regular expressions will be excluded from the scan. Regular expressions use RE2 [syntax](https://github.com/google/re2/wiki/Syntax); a guide can be found under the google/re2 repository on GitHub."]
        pub exclude_regex: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeRegex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of regular expressions matching file paths to include. All files in the bucket that match at least one of these regular expressions will be included in the set of files, except for those that also match an item in `exclude_regex`. Leaving this field empty will match all files by default (this is equivalent to including `.*` in the list). Regular expressions use RE2 [syntax](https://github.com/google/re2/wiki/Syntax); a guide can be found under the google/re2 repository on GitHub."]
        pub include_regex: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GooglePrivacyDlpV2CloudStorageRegexFileSet {
        pub fn builder() -> GooglePrivacyDlpV2CloudStorageRegexFileSetBuilder {
            GooglePrivacyDlpV2CloudStorageRegexFileSetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a color in the RGB color space."]
    pub struct GooglePrivacyDlpV2Color {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of blue in the color as a value in the interval [0, 1]."]
        pub blue: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "green")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of green in the color as a value in the interval [0, 1]."]
        pub green: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "red")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of red in the color as a value in the interval [0, 1]."]
        pub red: ::std::option::Option<::std::primitive::f64>,
    }
    impl GooglePrivacyDlpV2Color {
        pub fn builder() -> GooglePrivacyDlpV2ColorBuilder {
            GooglePrivacyDlpV2ColorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The field type of `value` and `field` do not need to match to be considered equal, but not all comparisons are possible. EQUAL_TO and NOT_EQUAL_TO attempt to compare even with incompatible types, but all other comparisons are invalid with incompatible types. A `value` of type: - `string` can be compared against all other types - `boolean` can only be compared against other booleans - `integer` can be compared against doubles or a string if the string value can be parsed as an integer. - `double` can be compared against integers or a string if the string can be parsed as a double. - `Timestamp` can be compared against strings in RFC 3339 date string format. - `TimeOfDay` can be compared against timestamps and strings in the format of 'HH:mm:ss'. If we fail to compare do to type mismatch, a warning will be given and the condition will evaluate to false."]
    pub struct GooglePrivacyDlpV2Condition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Field within the record this condition is evaluated against."]
        pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Operator used to compare the field or infoType to the value."]
        pub operator: ::std::option::Option<GooglePrivacyDlpV2ConditionOperatorEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value to compare against. [Mandatory, except for `EXISTS` tests.]"]
        pub value: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
    }
    impl GooglePrivacyDlpV2Condition {
        pub fn builder() -> GooglePrivacyDlpV2ConditionBuilder {
            GooglePrivacyDlpV2ConditionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Operator used to compare the field or infoType to the value."]
    pub enum GooglePrivacyDlpV2ConditionOperatorEnum {
        #[serde(rename = "RELATIONAL_OPERATOR_UNSPECIFIED")]
        #[doc = "Unused"]
        RelationalOperatorUnspecified,
        #[serde(rename = "EQUAL_TO")]
        #[doc = "Equal. Attempts to match even with incompatible types."]
        EqualTo,
        #[serde(rename = "NOT_EQUAL_TO")]
        #[doc = "Not equal to. Attempts to match even with incompatible types."]
        NotEqualTo,
        #[serde(rename = "GREATER_THAN")]
        #[doc = "Greater than."]
        GreaterThan,
        #[serde(rename = "LESS_THAN")]
        #[doc = "Less than."]
        LessThan,
        #[serde(rename = "GREATER_THAN_OR_EQUALS")]
        #[doc = "Greater than or equals."]
        GreaterThanOrEquals,
        #[serde(rename = "LESS_THAN_OR_EQUALS")]
        #[doc = "Less than or equals."]
        LessThanOrEquals,
        #[serde(rename = "EXISTS")]
        #[doc = "Exists"]
        Exists,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2ConditionOperatorEnum {
        fn default() -> Self {
            Self::RelationalOperatorUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A collection of conditions."]
    pub struct GooglePrivacyDlpV2Conditions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conditions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A collection of conditions."]
        pub conditions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Condition>>>,
    }
    impl GooglePrivacyDlpV2Conditions {
        pub fn builder() -> GooglePrivacyDlpV2ConditionsBuilder {
            GooglePrivacyDlpV2ConditionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a container that may contain DLP findings. Examples of a container include a file, table, or database record."]
    pub struct GooglePrivacyDlpV2Container {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A string representation of the full container name. Examples: - BigQuery: 'Project:DataSetId.TableId' - Google Cloud Storage: 'gs://Bucket/folders/filename.txt'"]
        pub full_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Project where the finding was found. Can be different from the project that owns the finding."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relativePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rest of the path after the root. Examples: - For BigQuery table `project_id:dataset_id.table_id`, the relative path is `table_id` - Google Cloud Storage file `gs://bucket/folder/filename.txt`, the relative path is `folder/filename.txt`"]
        pub relative_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rootPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The root of the container. Examples: - For BigQuery table `project_id:dataset_id.table_id`, the root is `dataset_id` - For Google Cloud Storage file `gs://bucket/folder/filename.txt`, the root is `gs://bucket`"]
        pub root_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Container type, for example BigQuery or Google Cloud Storage."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Findings container modification timestamp, if applicable. For Google Cloud Storage contains last file modification timestamp. For BigQuery table contains last_modified_time property. For Datastore - not populated."]
        pub update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Findings container version, if available (\"generation\" for Google Cloud Storage)."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2Container {
        pub fn builder() -> GooglePrivacyDlpV2ContainerBuilder {
            GooglePrivacyDlpV2ContainerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Container structure for the content to inspect."]
    pub struct GooglePrivacyDlpV2ContentItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "byteItem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Content data to inspect or redact. Replaces `type` and `data`."]
        pub byte_item: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ByteContentItem>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "table")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Structured content for inspection. See https://cloud.google.com/dlp/docs/inspecting-text#inspecting_a_table to learn more."]
        pub table: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Table>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "String data to inspect or redact."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2ContentItem {
        pub fn builder() -> GooglePrivacyDlpV2ContentItemBuilder {
            GooglePrivacyDlpV2ContentItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Precise location of the finding within a document, record, image, or metadata container."]
    pub struct GooglePrivacyDlpV2ContentLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the container where the finding is located. The top level name is the source file name or table name. Names of some common storage containers are formatted as follows: * BigQuery tables: `{project_id}:{dataset_id}.{table_id}` * Cloud Storage files: `gs://{bucket}/{path}` * Datastore namespace: {namespace} Nested names could be absent if the embedded object has no string identifier (for an example an image contained within a document)."]
        pub container_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Findings container modification timestamp, if applicable. For Google Cloud Storage contains last file modification timestamp. For BigQuery table contains last_modified_time property. For Datastore - not populated."]
        pub container_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Findings container version, if available (\"generation\" for Google Cloud Storage)."]
        pub container_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "documentLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location data for document files."]
        pub document_location:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DocumentLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location within an image's pixels."]
        pub image_location:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ImageLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadataLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location within the metadata for inspected content."]
        pub metadata_location:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2MetadataLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recordLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location within a row or record of a database table."]
        pub record_location:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RecordLocation>>,
    }
    impl GooglePrivacyDlpV2ContentLocation {
        pub fn builder() -> GooglePrivacyDlpV2ContentLocationBuilder {
            GooglePrivacyDlpV2ContentLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CreateDeidentifyTemplate."]
    pub struct GooglePrivacyDlpV2CreateDeidentifyTemplateRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deidentifyTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The DeidentifyTemplate to create."]
        pub deidentify_template:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DeidentifyTemplate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This field has no effect."]
        pub location_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "templateId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The template id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one."]
        pub template_id: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2CreateDeidentifyTemplateRequest {
        pub fn builder() -> GooglePrivacyDlpV2CreateDeidentifyTemplateRequestBuilder {
            GooglePrivacyDlpV2CreateDeidentifyTemplateRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CreateDlpJobRequest. Used to initiate long running jobs such as calculating risk metrics or inspecting Google Cloud Storage."]
    pub struct GooglePrivacyDlpV2CreateDlpJobRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inspectJob")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An inspection job scans a storage repository for InfoTypes."]
        pub inspect_job:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectJobConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The job id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one."]
        pub job_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This field has no effect."]
        pub location_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "riskJob")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A risk analysis job calculates re-identification risk metrics for a BigQuery table."]
        pub risk_job:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RiskAnalysisJobConfig>>,
    }
    impl GooglePrivacyDlpV2CreateDlpJobRequest {
        pub fn builder() -> GooglePrivacyDlpV2CreateDlpJobRequestBuilder {
            GooglePrivacyDlpV2CreateDlpJobRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CreateInspectTemplate."]
    pub struct GooglePrivacyDlpV2CreateInspectTemplateRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inspectTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The InspectTemplate to create."]
        pub inspect_template:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectTemplate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This field has no effect."]
        pub location_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "templateId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The template id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one."]
        pub template_id: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2CreateInspectTemplateRequest {
        pub fn builder() -> GooglePrivacyDlpV2CreateInspectTemplateRequestBuilder {
            GooglePrivacyDlpV2CreateInspectTemplateRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CreateJobTrigger."]
    pub struct GooglePrivacyDlpV2CreateJobTriggerRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobTrigger")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The JobTrigger to create."]
        pub job_trigger: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2JobTrigger>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This field has no effect."]
        pub location_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The trigger id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one."]
        pub trigger_id: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2CreateJobTriggerRequest {
        pub fn builder() -> GooglePrivacyDlpV2CreateJobTriggerRequestBuilder {
            GooglePrivacyDlpV2CreateJobTriggerRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CreateStoredInfoType."]
    pub struct GooglePrivacyDlpV2CreateStoredInfoTypeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "config")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Configuration of the storedInfoType to create."]
        pub config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2StoredInfoTypeConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This field has no effect."]
        pub location_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storedInfoTypeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The storedInfoType ID can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one."]
        pub stored_info_type_id: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2CreateStoredInfoTypeRequest {
        pub fn builder() -> GooglePrivacyDlpV2CreateStoredInfoTypeRequestBuilder {
            GooglePrivacyDlpV2CreateStoredInfoTypeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Pseudonymization method that generates deterministic encryption for the given input. Outputs a base64 encoded representation of the encrypted output. Uses AES-SIV based on the RFC https://tools.ietf.org/html/rfc5297."]
    pub struct GooglePrivacyDlpV2CryptoDeterministicConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A context may be used for higher security and maintaining referential integrity such that the same identifier in two different contexts will be given a distinct surrogate. The context is appended to plaintext value being encrypted. On decryption the provided context is validated against the value used during encryption. If a context was provided during encryption, same context must be provided during decryption as well. If the context is not set, plaintext would be used as is for encryption. If the context is set but: 1. there is no record present when transforming a given value or 2. the field is not present when transforming a given value, plaintext would be used as is for encryption. Note that case (1) is expected when an `InfoTypeTransformation` is applied to both structured and non-structured `ContentItem`s."]
        pub context: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cryptoKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key used by the encryption function."]
        pub crypto_key: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CryptoKey>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "surrogateInfoType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The custom info type to annotate the surrogate with. This annotation will be applied to the surrogate by prefixing it with the name of the custom info type followed by the number of characters comprising the surrogate. The following scheme defines the format: {info type name}({surrogate character count}):{surrogate} For example, if the name of custom info type is 'MY_TOKEN_INFO_TYPE' and the surrogate is 'abc', the full replacement value will be: 'MY_TOKEN_INFO_TYPE(3):abc' This annotation identifies the surrogate when inspecting content using the custom info type 'Surrogate'. This facilitates reversal of the surrogate when it occurs in free text. Note: For record transformations where the entire cell in a table is being transformed, surrogates are not mandatory. Surrogates are used to denote the location of the token and are necessary for re-identification in free form text. In order for inspection to work properly, the name of this info type must not occur naturally anywhere in your data; otherwise, inspection may either - reverse a surrogate that does not correspond to an actual identifier - be unable to parse the surrogate and result in an error Therefore, choose your custom info type name carefully after considering what your data looks like. One way to select a name that has a high chance of yielding reliable detection is to include one or more unicode characters that are highly improbable to exist in your data. For example, assuming your data is entered from a regular ASCII keyboard, the symbol with the hex code point 29DD might be used like so: ⧝MY_TOKEN_TYPE."]
        pub surrogate_info_type:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
    }
    impl GooglePrivacyDlpV2CryptoDeterministicConfig {
        pub fn builder() -> GooglePrivacyDlpV2CryptoDeterministicConfigBuilder {
            GooglePrivacyDlpV2CryptoDeterministicConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Pseudonymization method that generates surrogates via cryptographic hashing. Uses SHA-256. The key size must be either 32 or 64 bytes. Outputs a base64 encoded representation of the hashed output (for example, L7k0BHmF1ha5U3NfGykjro4xWi1MPVQPjhMAZbSV9mM=). Currently, only string and integer values can be hashed. See https://cloud.google.com/dlp/docs/pseudonymization to learn more."]
    pub struct GooglePrivacyDlpV2CryptoHashConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cryptoKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key used by the hash function."]
        pub crypto_key: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CryptoKey>>,
    }
    impl GooglePrivacyDlpV2CryptoHashConfig {
        pub fn builder() -> GooglePrivacyDlpV2CryptoHashConfigBuilder {
            GooglePrivacyDlpV2CryptoHashConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This is a data encryption key (DEK) (as opposed to a key encryption key (KEK) stored by KMS). When using KMS to wrap/unwrap DEKs, be sure to set an appropriate IAM policy on the KMS CryptoKey (KEK) to ensure an attacker cannot unwrap the data crypto key."]
    pub struct GooglePrivacyDlpV2CryptoKey {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmsWrapped")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Kms wrapped key"]
        pub kms_wrapped:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2KmsWrappedCryptoKey>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transient")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transient crypto key"]
        pub transient:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TransientCryptoKey>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unwrapped")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unwrapped crypto key"]
        pub unwrapped:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2UnwrappedCryptoKey>>,
    }
    impl GooglePrivacyDlpV2CryptoKey {
        pub fn builder() -> GooglePrivacyDlpV2CryptoKeyBuilder {
            GooglePrivacyDlpV2CryptoKeyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Replaces an identifier with a surrogate using Format Preserving Encryption (FPE) with the FFX mode of operation; however when used in the `ReidentifyContent` API method, it serves the opposite function by reversing the surrogate back into the original identifier. The identifier must be encoded as ASCII. For a given crypto key and context, the same identifier will be replaced with the same surrogate. Identifiers must be at least two characters long. In the case that the identifier is the empty string, it will be skipped. See https://cloud.google.com/dlp/docs/pseudonymization to learn more. Note: We recommend using CryptoDeterministicConfig for all use cases which do not require preserving the input alphabet space and size, plus warrant referential integrity."]
    pub struct GooglePrivacyDlpV2CryptoReplaceFfxFpeConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonAlphabet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Common alphabets."]
        pub common_alphabet:
            ::std::option::Option<GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The 'tweak', a context may be used for higher security since the same identifier in two different contexts won't be given the same surrogate. If the context is not set, a default tweak will be used. If the context is set but: 1. there is no record present when transforming a given value or 1. the field is not present when transforming a given value, a default tweak will be used. Note that case (1) is expected when an `InfoTypeTransformation` is applied to both structured and non-structured `ContentItem`s. Currently, the referenced field may be of value type integer or string. The tweak is constructed as a sequence of bytes in big endian byte order such that: - a 64 bit integer is encoded followed by a single byte of value 1 - a string is encoded in UTF-8 format followed by a single byte of value 2"]
        pub context: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cryptoKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The key used by the encryption algorithm."]
        pub crypto_key: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CryptoKey>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customAlphabet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is supported by mapping these to the alphanumeric characters that the FFX mode natively supports. This happens before/after encryption/decryption. Each character listed must appear only once. Number of characters must be in the range [2, 95]. This must be encoded as ASCII. The order of characters does not matter. The full list of allowed characters is: 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz ~`!@#$%^&*()_-+={[}]|\\:;\"'<,>.?/"]
        pub custom_alphabet: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "radix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The native way to select the alphabet. Must be in the range [2, 95]."]
        pub radix: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "surrogateInfoType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The custom infoType to annotate the surrogate with. This annotation will be applied to the surrogate by prefixing it with the name of the custom infoType followed by the number of characters comprising the surrogate. The following scheme defines the format: info_type_name(surrogate_character_count):surrogate For example, if the name of custom infoType is 'MY_TOKEN_INFO_TYPE' and the surrogate is 'abc', the full replacement value will be: 'MY_TOKEN_INFO_TYPE(3):abc' This annotation identifies the surrogate when inspecting content using the custom infoType [`SurrogateType`](https://cloud.google.com/dlp/docs/reference/rest/v2/InspectConfig#surrogatetype). This facilitates reversal of the surrogate when it occurs in free text. In order for inspection to work properly, the name of this infoType must not occur naturally anywhere in your data; otherwise, inspection may find a surrogate that does not correspond to an actual identifier. Therefore, choose your custom infoType name carefully after considering what your data looks like. One way to select a name that has a high chance of yielding reliable detection is to include one or more unicode characters that are highly improbable to exist in your data. For example, assuming your data is entered from a regular ASCII keyboard, the symbol with the hex code point 29DD might be used like so: ⧝MY_TOKEN_TYPE"]
        pub surrogate_info_type:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
    }
    impl GooglePrivacyDlpV2CryptoReplaceFfxFpeConfig {
        pub fn builder() -> GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigBuilder {
            GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Common alphabets."]
    pub enum GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum {
        #[serde(rename = "FFX_COMMON_NATIVE_ALPHABET_UNSPECIFIED")]
        #[doc = "Unused."]
        FfxCommonNativeAlphabetUnspecified,
        #[serde(rename = "NUMERIC")]
        #[doc = "`[0-9]` (radix of 10)"]
        Numeric,
        #[serde(rename = "HEXADECIMAL")]
        #[doc = "`[0-9A-F]` (radix of 16)"]
        Hexadecimal,
        #[serde(rename = "UPPER_CASE_ALPHA_NUMERIC")]
        #[doc = "`[0-9A-Z]` (radix of 36)"]
        UpperCaseAlphaNumeric,
        #[serde(rename = "ALPHA_NUMERIC")]
        #[doc = "`[0-9A-Za-z]` (radix of 62)"]
        AlphaNumeric,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum {
        fn default() -> Self {
            Self::FfxCommonNativeAlphabetUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Custom information type provided by the user. Used to find domain-specific sensitive information configurable to the data in question."]
    pub struct GooglePrivacyDlpV2CustomInfoType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectionRules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of detection rules to apply to all findings of this CustomInfoType. Rules are applied in order that they are specified. Not supported for the `surrogate_type` CustomInfoType."]
        pub detection_rules: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2DetectionRule>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dictionary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of phrases to detect as a CustomInfoType."]
        pub dictionary: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Dictionary>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exclusionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set to EXCLUSION_TYPE_EXCLUDE this infoType will not cause a finding to be returned. It still can be used for rules matching."]
        pub exclusion_type:
            ::std::option::Option<GooglePrivacyDlpV2CustomInfoTypeExclusionTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infoType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "CustomInfoType can either be a new infoType, or an extension of built-in infoType, when the name matches one of existing infoTypes and that infoType is specified in `InspectContent.info_types` field. Specifying the latter adds findings to the one detected by the system. If built-in info type is not specified in `InspectContent.info_types` list then the name is treated as a custom info type."]
        pub info_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "likelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood to return for this CustomInfoType. This base value can be altered by a detection rule if the finding meets the criteria specified by the rule. Defaults to `VERY_LIKELY` if not specified."]
        pub likelihood: ::std::option::Option<GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Regular expression based CustomInfoType."]
        pub regex: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Regex>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storedType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Load an existing `StoredInfoType` resource for use in `InspectDataSource`. Not currently supported in `InspectContent`."]
        pub stored_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2StoredType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "surrogateType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Message for detecting output from deidentification transformations that support reversing."]
        pub surrogate_type:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2SurrogateType>>,
    }
    impl GooglePrivacyDlpV2CustomInfoType {
        pub fn builder() -> GooglePrivacyDlpV2CustomInfoTypeBuilder {
            GooglePrivacyDlpV2CustomInfoTypeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "If set to EXCLUSION_TYPE_EXCLUDE this infoType will not cause a finding to be returned. It still can be used for rules matching."]
    pub enum GooglePrivacyDlpV2CustomInfoTypeExclusionTypeEnum {
        #[serde(rename = "EXCLUSION_TYPE_UNSPECIFIED")]
        #[doc = "A finding of this custom info type will not be excluded from results."]
        ExclusionTypeUnspecified,
        #[serde(rename = "EXCLUSION_TYPE_EXCLUDE")]
        #[doc = "A finding of this custom info type will be excluded from final results, but can still affect rule execution."]
        ExclusionTypeExclude,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2CustomInfoTypeExclusionTypeEnum {
        fn default() -> Self {
            Self::ExclusionTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood to return for this CustomInfoType. This base value can be altered by a detection rule if the finding meets the criteria specified by the rule. Defaults to `VERY_LIKELY` if not specified."]
    pub enum GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum {
        #[serde(rename = "LIKELIHOOD_UNSPECIFIED")]
        #[doc = "Default value; same as POSSIBLE."]
        LikelihoodUnspecified,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "Few matching elements."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = ""]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "Some matching elements."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = ""]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "Many matching elements."]
        VeryLikely,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum {
        fn default() -> Self {
            Self::LikelihoodUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Record key for a finding in Cloud Datastore."]
    pub struct GooglePrivacyDlpV2DatastoreKey {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Datastore entity key."]
        pub entity_key: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Key>>,
    }
    impl GooglePrivacyDlpV2DatastoreKey {
        pub fn builder() -> GooglePrivacyDlpV2DatastoreKeyBuilder {
            GooglePrivacyDlpV2DatastoreKeyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options defining a data set within Google Cloud Datastore."]
    pub struct GooglePrivacyDlpV2DatastoreOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind to process."]
        pub kind: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2KindExpression>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partitionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A partition ID identifies a grouping of entities. The grouping is always by project and namespace, however the namespace ID may be empty."]
        pub partition_id: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PartitionId>>,
    }
    impl GooglePrivacyDlpV2DatastoreOptions {
        pub fn builder() -> GooglePrivacyDlpV2DatastoreOptionsBuilder {
            GooglePrivacyDlpV2DatastoreOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Shifts dates by random number of days, with option to be consistent for the same context. See https://cloud.google.com/dlp/docs/concepts-date-shifting to learn more."]
    pub struct GooglePrivacyDlpV2DateShiftConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Points to the field that contains the context, for example, an entity id. If set, must also set cryptoKey. If set, shift will be consistent for the given context."]
        pub context: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cryptoKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Causes the shift to be computed based on this key and the context. This results in the same shift for the same context and crypto_key. If set, must also set context. Can only be applied to table items."]
        pub crypto_key: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CryptoKey>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lowerBoundDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. For example, -5 means shift date to at most 5 days back in the past."]
        pub lower_bound_days: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upperBoundDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Range of shift in days. Actual shift will be selected at random within this range (inclusive ends). Negative means shift to earlier in time. Must not be more than 365250 days (1000 years) each direction. For example, 3 means shift date to at most 3 days into the future."]
        pub upper_bound_days: ::std::option::Option<::std::primitive::i64>,
    }
    impl GooglePrivacyDlpV2DateShiftConfig {
        pub fn builder() -> GooglePrivacyDlpV2DateShiftConfigBuilder {
            GooglePrivacyDlpV2DateShiftConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message for a date time object. e.g. 2018-01-01, 5th August."]
    pub struct GooglePrivacyDlpV2DateTime {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "date")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "One or more of the following must be set. Must be a valid date or time value."]
        pub date: ::std::option::Option<::std::boxed::Box<GoogleTypeDate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dayOfWeek")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Day of week"]
        pub day_of_week: ::std::option::Option<GooglePrivacyDlpV2DateTimeDayOfWeekEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "time")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time of day"]
        pub time: ::std::option::Option<::std::boxed::Box<GoogleTypeTimeOfDay>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time zone"]
        pub time_zone: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TimeZone>>,
    }
    impl GooglePrivacyDlpV2DateTime {
        pub fn builder() -> GooglePrivacyDlpV2DateTimeBuilder {
            GooglePrivacyDlpV2DateTimeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Day of week"]
    pub enum GooglePrivacyDlpV2DateTimeDayOfWeekEnum {
        #[serde(rename = "DAY_OF_WEEK_UNSPECIFIED")]
        #[doc = "The day of the week is unspecified."]
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
    impl ::std::default::Default for GooglePrivacyDlpV2DateTimeDayOfWeekEnum {
        fn default() -> Self {
            Self::DayOfWeekUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The configuration that controls how the data will change."]
    pub struct GooglePrivacyDlpV2DeidentifyConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infoTypeTransformations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Treat the dataset as free-form text and apply the same free text transformation everywhere."]
        pub info_type_transformations:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoTypeTransformations>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recordTransformations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Treat the dataset as structured. Transformations can be applied to specific locations within structured datasets, such as transforming a column within a table."]
        pub record_transformations:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RecordTransformations>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transformationErrorHandling")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mode for handling transformation errors. If left unspecified, the default mode is `TransformationErrorHandling.ThrowError`."]
        pub transformation_error_handling:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TransformationErrorHandling>>,
    }
    impl GooglePrivacyDlpV2DeidentifyConfig {
        pub fn builder() -> GooglePrivacyDlpV2DeidentifyConfigBuilder {
            GooglePrivacyDlpV2DeidentifyConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to de-identify a list of items."]
    pub struct GooglePrivacyDlpV2DeidentifyContentRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deidentifyConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for the de-identification of the content item. Items specified here will override the template referenced by the deidentify_template_name argument."]
        pub deidentify_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DeidentifyConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deidentifyTemplateName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Template to use. Any configuration directly specified in deidentify_config will override those set in the template. Singular fields that are set in this request will replace their corresponding fields in the template. Repeated fields are appended. Singular sub-messages and groups are recursively merged."]
        pub deidentify_template_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inspectConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for the inspector. Items specified here will override the template referenced by the inspect_template_name argument."]
        pub inspect_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inspectTemplateName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Template to use. Any configuration directly specified in inspect_config will override those set in the template. Singular fields that are set in this request will replace their corresponding fields in the template. Repeated fields are appended. Singular sub-messages and groups are recursively merged."]
        pub inspect_template_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "item")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The item to de-identify. Will be treated as text."]
        pub item: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ContentItem>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This field has no effect."]
        pub location_id: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2DeidentifyContentRequest {
        pub fn builder() -> GooglePrivacyDlpV2DeidentifyContentRequestBuilder {
            GooglePrivacyDlpV2DeidentifyContentRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results of de-identifying a ContentItem."]
    pub struct GooglePrivacyDlpV2DeidentifyContentResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "item")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The de-identified item."]
        pub item: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ContentItem>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "overview")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An overview of the changes that were made on the `item`."]
        pub overview:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TransformationOverview>>,
    }
    impl GooglePrivacyDlpV2DeidentifyContentResponse {
        pub fn builder() -> GooglePrivacyDlpV2DeidentifyContentResponseBuilder {
            GooglePrivacyDlpV2DeidentifyContentResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "DeidentifyTemplates contains instructions on how to de-identify content. See https://cloud.google.com/dlp/docs/concepts-templates to learn more."]
    pub struct GooglePrivacyDlpV2DeidentifyTemplate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The creation timestamp of an inspectTemplate."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deidentifyConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "///////////// // The core content of the template // ///////////////"]
        pub deidentify_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DeidentifyConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Short description (max 256 chars)."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display name (max 256 chars)."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The template name. The template will have one of the following formats: `projects/PROJECT_ID/deidentifyTemplates/TEMPLATE_ID` OR `organizations/ORGANIZATION_ID/deidentifyTemplates/TEMPLATE_ID`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The last update timestamp of an inspectTemplate."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2DeidentifyTemplate {
        pub fn builder() -> GooglePrivacyDlpV2DeidentifyTemplateBuilder {
            GooglePrivacyDlpV2DeidentifyTemplateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "δ-presence metric, used to estimate how likely it is for an attacker to figure out that one given individual appears in a de-identified dataset. Similarly to the k-map metric, we cannot compute δ-presence exactly without knowing the attack dataset, so we use a statistical model instead."]
    pub struct GooglePrivacyDlpV2DeltaPresenceEstimationConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "auxiliaryTables")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Several auxiliary tables can be used in the analysis. Each custom_tag used to tag a quasi-identifiers field must appear in exactly one field of one auxiliary table."]
        pub auxiliary_tables: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2StatisticalTable>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quasiIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Fields considered to be quasi-identifiers. No two fields can have the same tag."]
        pub quasi_ids:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2QuasiId>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ISO 3166-1 alpha-2 region code to use in the statistical modeling. Set if no column is tagged with a region-specific InfoType (like US_ZIP_5) or a region code."]
        pub region_code: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2DeltaPresenceEstimationConfig {
        pub fn builder() -> GooglePrivacyDlpV2DeltaPresenceEstimationConfigBuilder {
            GooglePrivacyDlpV2DeltaPresenceEstimationConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A DeltaPresenceEstimationHistogramBucket message with the following values: min_probability: 0.1 max_probability: 0.2 frequency: 42 means that there are 42 records for which δ is in [0.1, 0.2). An important particular case is when min_probability = max_probability = 1: then, every individual who shares this quasi-identifier combination is in the dataset."]
    pub struct GooglePrivacyDlpV2DeltaPresenceEstimationHistogramBucket {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of records within these probability bounds."]
        pub bucket_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketValueCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of distinct quasi-identifier tuple values in this bucket."]
        pub bucket_value_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sample of quasi-identifier tuple values in this bucket. The total number of classes returned per bucket is capped at 20."]
        pub bucket_values: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GooglePrivacyDlpV2DeltaPresenceEstimationQuasiIdValues>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxProbability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always greater than or equal to min_probability."]
        pub max_probability: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minProbability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Between 0 and 1."]
        pub min_probability: ::std::option::Option<::std::primitive::f64>,
    }
    impl GooglePrivacyDlpV2DeltaPresenceEstimationHistogramBucket {
        pub fn builder() -> GooglePrivacyDlpV2DeltaPresenceEstimationHistogramBucketBuilder {
            GooglePrivacyDlpV2DeltaPresenceEstimationHistogramBucketBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A tuple of values for the quasi-identifier columns."]
    pub struct GooglePrivacyDlpV2DeltaPresenceEstimationQuasiIdValues {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "estimatedProbability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The estimated probability that a given individual sharing these quasi-identifier values is in the dataset. This value, typically called δ, is the ratio between the number of records in the dataset with these quasi-identifier values, and the total number of individuals (inside *and* outside the dataset) with these quasi-identifier values. For example, if there are 15 individuals in the dataset who share the same quasi-identifier values, and an estimated 100 people in the entire population with these values, then δ is 0.15."]
        pub estimated_probability: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quasiIdsValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quasi-identifier values."]
        pub quasi_ids_values:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Value>>>,
    }
    impl GooglePrivacyDlpV2DeltaPresenceEstimationQuasiIdValues {
        pub fn builder() -> GooglePrivacyDlpV2DeltaPresenceEstimationQuasiIdValuesBuilder {
            GooglePrivacyDlpV2DeltaPresenceEstimationQuasiIdValuesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Result of the δ-presence computation. Note that these results are an estimation, not exact values."]
    pub struct GooglePrivacyDlpV2DeltaPresenceEstimationResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deltaPresenceEstimationHistogram")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The intervals [min_probability, max_probability) do not overlap. If a value doesn't correspond to any such interval, the associated frequency is zero. For example, the following records: {min_probability: 0, max_probability: 0.1, frequency: 17} {min_probability: 0.2, max_probability: 0.3, frequency: 42} {min_probability: 0.3, max_probability: 0.4, frequency: 99} mean that there are no record with an estimated probability in [0.1, 0.2) nor larger or equal to 0.4."]
        pub delta_presence_estimation_histogram: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GooglePrivacyDlpV2DeltaPresenceEstimationHistogramBucket>,
            >,
        >,
    }
    impl GooglePrivacyDlpV2DeltaPresenceEstimationResult {
        pub fn builder() -> GooglePrivacyDlpV2DeltaPresenceEstimationResultBuilder {
            GooglePrivacyDlpV2DeltaPresenceEstimationResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deprecated; use `InspectionRuleSet` instead. Rule for modifying a `CustomInfoType` to alter behavior under certain circumstances, depending on the specific details of the rule. Not supported for the `surrogate_type` custom infoType."]
    pub struct GooglePrivacyDlpV2DetectionRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hotwordRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hotword-based detection rule."]
        pub hotword_rule: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2HotwordRule>>,
    }
    impl GooglePrivacyDlpV2DetectionRule {
        pub fn builder() -> GooglePrivacyDlpV2DetectionRuleBuilder {
            GooglePrivacyDlpV2DetectionRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Custom information type based on a dictionary of words or phrases. This can be used to match sensitive information specific to the data, such as a list of employee IDs or job titles. Dictionary words are case-insensitive and all characters other than letters and digits in the unicode [Basic Multilingual Plane](https://en.wikipedia.org/wiki/Plane_%28Unicode%29#Basic_Multilingual_Plane) will be replaced with whitespace when scanning for matches, so the dictionary phrase \"Sam Johnson\" will match all three phrases \"sam johnson\", \"Sam, Johnson\", and \"Sam (Johnson)\". Additionally, the characters surrounding any match must be of a different type than the adjacent characters within the word, so letters must be next to non-letters and digits next to non-digits. For example, the dictionary word \"jen\" will match the first three letters of the text \"jen123\" but will return no matches for \"jennifer\". Dictionary words containing a large number of characters that are not letters or digits may result in unexpected findings because such characters are treated as whitespace. The [limits](https://cloud.google.com/dlp/limits) page contains details about the size limits of dictionaries. For dictionaries that do not fit within these constraints, consider using `LargeCustomDictionaryConfig` in the `StoredInfoType` API."]
    pub struct GooglePrivacyDlpV2Dictionary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudStoragePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Newline-delimited file of words in Cloud Storage. Only a single file is accepted."]
        pub cloud_storage_path:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CloudStoragePath>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "wordList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of words or phrases to search for."]
        pub word_list: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2WordList>>,
    }
    impl GooglePrivacyDlpV2Dictionary {
        pub fn builder() -> GooglePrivacyDlpV2DictionaryBuilder {
            GooglePrivacyDlpV2DictionaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Combines all of the information about a DLP job."]
    pub struct GooglePrivacyDlpV2DlpJob {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time when the job was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time when the job finished."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A stream of errors encountered running the job."]
        pub errors:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Error>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inspectDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Results from inspecting a data source."]
        pub inspect_details:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectDataSourceDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobTriggerName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If created by a job trigger, the resource name of the trigger that instantiated the job."]
        pub job_trigger_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The server-assigned name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "riskDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Results from analyzing risk of a data source."]
        pub risk_details: ::std::option::Option<
            ::std::boxed::Box<GooglePrivacyDlpV2AnalyzeDataSourceRiskDetails>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time when the job started."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "State of a job."]
        pub state: ::std::option::Option<GooglePrivacyDlpV2DlpJobStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of job."]
        pub _type: ::std::option::Option<GooglePrivacyDlpV2DlpJobTypeEnum>,
    }
    impl GooglePrivacyDlpV2DlpJob {
        pub fn builder() -> GooglePrivacyDlpV2DlpJobBuilder {
            GooglePrivacyDlpV2DlpJobBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "State of a job."]
    pub enum GooglePrivacyDlpV2DlpJobStateEnum {
        #[serde(rename = "JOB_STATE_UNSPECIFIED")]
        #[doc = "Unused."]
        JobStateUnspecified,
        #[serde(rename = "PENDING")]
        #[doc = "The job has not yet started."]
        Pending,
        #[serde(rename = "RUNNING")]
        #[doc = "The job is currently running. Once a job has finished it will transition to FAILED or DONE."]
        Running,
        #[serde(rename = "DONE")]
        #[doc = "The job is no longer running."]
        Done,
        #[serde(rename = "CANCELED")]
        #[doc = "The job was canceled before it could complete."]
        Canceled,
        #[serde(rename = "FAILED")]
        #[doc = "The job had an error and did not complete."]
        Failed,
        #[serde(rename = "ACTIVE")]
        #[doc = "The job is currently accepting findings via hybridInspect. A hybrid job in ACTIVE state may continue to have findings added to it through calling of hybridInspect. After the job has finished no more calls to hybridInspect may be made. ACTIVE jobs can transition to DONE."]
        Active,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2DlpJobStateEnum {
        fn default() -> Self {
            Self::JobStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of job."]
    pub enum GooglePrivacyDlpV2DlpJobTypeEnum {
        #[serde(rename = "DLP_JOB_TYPE_UNSPECIFIED")]
        #[doc = "Unused"]
        DlpJobTypeUnspecified,
        #[serde(rename = "INSPECT_JOB")]
        #[doc = "The job inspected Google Cloud for sensitive data."]
        InspectJob,
        #[serde(rename = "RISK_ANALYSIS_JOB")]
        #[doc = "The job executed a Risk Analysis computation."]
        RiskAnalysisJob,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2DlpJobTypeEnum {
        fn default() -> Self {
            Self::DlpJobTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Location of a finding within a document."]
    pub struct GooglePrivacyDlpV2DocumentLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Offset of the line, from the beginning of the file, where the finding is located."]
        pub file_offset: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2DocumentLocation {
        pub fn builder() -> GooglePrivacyDlpV2DocumentLocationBuilder {
            GooglePrivacyDlpV2DocumentLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An entity in a dataset is a field or set of fields that correspond to a single person. For example, in medical records the `EntityId` might be a patient identifier, or for financial records it might be an account identifier. This message is used when generalizations or analysis must take into account that multiple rows correspond to the same entity."]
    pub struct GooglePrivacyDlpV2EntityId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Composite key indicating which field contains the entity identifier."]
        pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
    }
    impl GooglePrivacyDlpV2EntityId {
        pub fn builder() -> GooglePrivacyDlpV2EntityIdBuilder {
            GooglePrivacyDlpV2EntityIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details information about an error encountered during job execution or the results of an unsuccessful activation of the JobTrigger."]
    pub struct GooglePrivacyDlpV2Error {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "details")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detailed error codes and messages."]
        pub details: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The times the error occurred."]
        pub timestamps: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GooglePrivacyDlpV2Error {
        pub fn builder() -> GooglePrivacyDlpV2ErrorBuilder {
            GooglePrivacyDlpV2ErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "List of exclude infoTypes."]
    pub struct GooglePrivacyDlpV2ExcludeInfoTypes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infoTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "InfoType list in ExclusionRule rule drops a finding when it overlaps or contained within with a finding of an infoType from this list. For example, for `InspectionRuleSet.info_types` containing \"PHONE_NUMBER\"` and `exclusion_rule` containing `exclude_info_types.info_types` with \"EMAIL_ADDRESS\" the phone number findings are dropped if they overlap with EMAIL_ADDRESS finding. That leads to \"555-222-2222@example.org\" to generate only a single finding, namely email address."]
        pub info_types:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>>,
    }
    impl GooglePrivacyDlpV2ExcludeInfoTypes {
        pub fn builder() -> GooglePrivacyDlpV2ExcludeInfoTypesBuilder {
            GooglePrivacyDlpV2ExcludeInfoTypesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The rule that specifies conditions when findings of infoTypes specified in `InspectionRuleSet` are removed from results."]
    pub struct GooglePrivacyDlpV2ExclusionRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dictionary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dictionary which defines the rule."]
        pub dictionary: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Dictionary>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludeInfoTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of infoTypes for which findings would affect this rule."]
        pub exclude_info_types:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ExcludeInfoTypes>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchingType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How the rule is applied, see MatchingType documentation for details."]
        pub matching_type: ::std::option::Option<GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Regular expression which defines the rule."]
        pub regex: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Regex>>,
    }
    impl GooglePrivacyDlpV2ExclusionRule {
        pub fn builder() -> GooglePrivacyDlpV2ExclusionRuleBuilder {
            GooglePrivacyDlpV2ExclusionRuleBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How the rule is applied, see MatchingType documentation for details."]
    pub enum GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum {
        #[serde(rename = "MATCHING_TYPE_UNSPECIFIED")]
        #[doc = "Invalid."]
        MatchingTypeUnspecified,
        #[serde(rename = "MATCHING_TYPE_FULL_MATCH")]
        #[doc = "Full match. - Dictionary: join of Dictionary results matched complete finding quote - Regex: all regex matches fill a finding quote start to end - Exclude info type: completely inside affecting info types findings"]
        MatchingTypeFullMatch,
        #[serde(rename = "MATCHING_TYPE_PARTIAL_MATCH")]
        #[doc = "Partial match. - Dictionary: at least one of the tokens in the finding matches - Regex: substring of the finding matches - Exclude info type: intersects with affecting info types findings"]
        MatchingTypePartialMatch,
        #[serde(rename = "MATCHING_TYPE_INVERSE_MATCH")]
        #[doc = "Inverse match. - Dictionary: no tokens in the finding match the dictionary - Regex: finding doesn't match the regex - Exclude info type: no intersection with affecting info types findings"]
        MatchingTypeInverseMatch,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum {
        fn default() -> Self {
            Self::MatchingTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An expression, consisting or an operator and conditions."]
    pub struct GooglePrivacyDlpV2Expressions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conditions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Conditions to apply to the expression."]
        pub conditions: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Conditions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logicalOperator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operator to apply to the result of conditions. Default and currently only supported value is `AND`."]
        pub logical_operator:
            ::std::option::Option<GooglePrivacyDlpV2ExpressionsLogicalOperatorEnum>,
    }
    impl GooglePrivacyDlpV2Expressions {
        pub fn builder() -> GooglePrivacyDlpV2ExpressionsBuilder {
            GooglePrivacyDlpV2ExpressionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The operator to apply to the result of conditions. Default and currently only supported value is `AND`."]
    pub enum GooglePrivacyDlpV2ExpressionsLogicalOperatorEnum {
        #[serde(rename = "LOGICAL_OPERATOR_UNSPECIFIED")]
        #[doc = "Unused"]
        LogicalOperatorUnspecified,
        #[serde(rename = "AND")]
        #[doc = "Conditional AND"]
        And,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2ExpressionsLogicalOperatorEnum {
        fn default() -> Self {
            Self::LogicalOperatorUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "General identifier of a data field in a storage service."]
    pub struct GooglePrivacyDlpV2FieldId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name describing the field."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2FieldId {
        pub fn builder() -> GooglePrivacyDlpV2FieldIdBuilder {
            GooglePrivacyDlpV2FieldIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The transformation to apply to the field."]
    pub struct GooglePrivacyDlpV2FieldTransformation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only apply the transformation if the condition evaluates to true for the given `RecordCondition`. The conditions are allowed to reference fields that are not used in the actual transformation. Example Use Cases: - Apply a different bucket transformation to an age column if the zip code column for the same record is within a specific range. - Redact a field if the date of birth field is greater than 85."]
        pub condition: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RecordCondition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input field(s) to apply the transformation to."]
        pub fields:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infoTypeTransformations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Treat the contents of the field as free text, and selectively transform content that matches an `InfoType`."]
        pub info_type_transformations:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoTypeTransformations>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primitiveTransformation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Apply the transformation to the entire field."]
        pub primitive_transformation:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PrimitiveTransformation>>,
    }
    impl GooglePrivacyDlpV2FieldTransformation {
        pub fn builder() -> GooglePrivacyDlpV2FieldTransformationBuilder {
            GooglePrivacyDlpV2FieldTransformationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of files to scan."]
    pub struct GooglePrivacyDlpV2FileSet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regexFileSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The regex-filtered set of files to scan. Exactly one of `url` or `regex_file_set` must be set."]
        pub regex_file_set:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CloudStorageRegexFileSet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud Storage url of the file(s) to scan, in the format `gs:///`. Trailing wildcard in the path is allowed. If the url ends in a trailing slash, the bucket or directory represented by the url will be scanned non-recursively (content in sub-directories will not be scanned). This means that `gs://mybucket/` is equivalent to `gs://mybucket/*`, and `gs://mybucket/directory/` is equivalent to `gs://mybucket/directory/*`. Exactly one of `url` or `regex_file_set` must be set."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2FileSet {
        pub fn builder() -> GooglePrivacyDlpV2FileSetBuilder {
            GooglePrivacyDlpV2FileSetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a piece of potentially sensitive content."]
    pub struct GooglePrivacyDlpV2Finding {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp when finding was detected."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "findingId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique finding id."]
        pub finding_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infoType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of content that might have been found. Provided if `excluded_types` is false."]
        pub info_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobCreateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the job started that produced this finding."]
        pub job_create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The job that stored the finding."]
        pub job_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The labels associated with this `Finding`. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?`. Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. No more than 10 labels can be associated with a given finding. Examples: * `\"environment\" : \"production\"` * `\"pipeline\" : \"etl\"`"]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "likelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of how likely it is that the `info_type` is correct."]
        pub likelihood: ::std::option::Option<GooglePrivacyDlpV2FindingLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Where the content was found."]
        pub location: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Location>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name in format projects/{project}/locations/{location}/findings/{finding} Populated only when viewing persisted findings."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quote")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content that was found. Even if the content is not textual, it may be converted to a textual representation here. Provided if `include_quote` is true and the finding is less than or equal to 4096 bytes long. If the finding exceeds 4096 bytes in length, the quote may be omitted."]
        pub quote: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quoteInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains data parsed from quotes. Only populated if include_quote was set to true and a supported infoType was requested. Currently supported infoTypes: DATE, DATE_OF_BIRTH and TIME."]
        pub quote_info: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2QuoteInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The job that stored the finding."]
        pub resource_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggerName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Job trigger name, if applicable, for this finding."]
        pub trigger_name: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2Finding {
        pub fn builder() -> GooglePrivacyDlpV2FindingBuilder {
            GooglePrivacyDlpV2FindingBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Confidence of how likely it is that the `info_type` is correct."]
    pub enum GooglePrivacyDlpV2FindingLikelihoodEnum {
        #[serde(rename = "LIKELIHOOD_UNSPECIFIED")]
        #[doc = "Default value; same as POSSIBLE."]
        LikelihoodUnspecified,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "Few matching elements."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = ""]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "Some matching elements."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = ""]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "Many matching elements."]
        VeryLikely,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2FindingLikelihoodEnum {
        fn default() -> Self {
            Self::LikelihoodUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration to control the number of findings returned. Cannot be set if de-identification is requested."]
    pub struct GooglePrivacyDlpV2FindingLimits {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxFindingsPerInfoType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration of findings limit given for specified infoTypes."]
        pub max_findings_per_info_type: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InfoTypeLimit>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxFindingsPerItem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Max number of findings that will be returned for each item scanned. When set within `InspectJobConfig`, the maximum returned is 2000 regardless if this is set higher. When set within `InspectContentRequest`, this field is ignored."]
        pub max_findings_per_item: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxFindingsPerRequest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Max number of findings that will be returned per request/job. When set within `InspectContentRequest`, the maximum returned is 2000 regardless if this is set higher."]
        pub max_findings_per_request: ::std::option::Option<::std::primitive::i64>,
    }
    impl GooglePrivacyDlpV2FindingLimits {
        pub fn builder() -> GooglePrivacyDlpV2FindingLimitsBuilder {
            GooglePrivacyDlpV2FindingLimitsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for finishing a DLP hybrid job."]
    pub struct GooglePrivacyDlpV2FinishDlpJobRequest {}
    impl GooglePrivacyDlpV2FinishDlpJobRequest {
        pub fn builder() -> GooglePrivacyDlpV2FinishDlpJobRequestBuilder {
            GooglePrivacyDlpV2FinishDlpJobRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Buckets values based on fixed size ranges. The Bucketing transformation can provide all of this functionality, but requires more configuration. This message is provided as a convenience to the user for simple bucketing strategies. The transformed value will be a hyphenated string of {lower_bound}-{upper_bound}, i.e if lower_bound = 10 and upper_bound = 20 all values that are within this bucket will be replaced with \"10-20\". This can be used on data of type: double, long. If the bound Value type differs from the type of data being transformed, we will first attempt converting the type of the data to be transformed to match the type of the bound before comparing. See https://cloud.google.com/dlp/docs/concepts-bucketing to learn more."]
    pub struct GooglePrivacyDlpV2FixedSizeBucketingConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Size of each bucket (except for minimum and maximum buckets). So if `lower_bound` = 10, `upper_bound` = 89, and `bucket_size` = 10, then the following buckets would be used: -10, 10-20, 20-30, 30-40, 40-50, 50-60, 60-70, 70-80, 80-89, 89+. Precision up to 2 decimals works."]
        pub bucket_size: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lowerBound")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Lower bound value of buckets. All values less than `lower_bound` are grouped together into a single bucket; for example if `lower_bound` = 10, then all values less than 10 are replaced with the value \"-10\"."]
        pub lower_bound: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upperBound")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Upper bound value of buckets. All values greater than upper_bound are grouped together into a single bucket; for example if `upper_bound` = 89, then all values greater than 89 are replaced with the value \"89+\"."]
        pub upper_bound: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
    }
    impl GooglePrivacyDlpV2FixedSizeBucketingConfig {
        pub fn builder() -> GooglePrivacyDlpV2FixedSizeBucketingConfigBuilder {
            GooglePrivacyDlpV2FixedSizeBucketingConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The rule that adjusts the likelihood of findings within a certain proximity of hotwords."]
    pub struct GooglePrivacyDlpV2HotwordRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hotwordRegex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Regular expression pattern defining what qualifies as a hotword."]
        pub hotword_regex: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Regex>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "likelihoodAdjustment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood adjustment to apply to all matching findings."]
        pub likelihood_adjustment:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2LikelihoodAdjustment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "proximity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Proximity of the finding within which the entire hotword must reside. The total length of the window cannot exceed 1000 characters. Note that the finding itself will be included in the window, so that hotwords may be used to match substrings of the finding itself. For example, the certainty of a phone number regex \"\\(\\d{3}\\) \\d{3}-\\d{4}\" could be adjusted upwards if the area code is known to be the local area code of a company office using the hotword regex \"\\(xxx\\)\", where \"xxx\" is the area code in question."]
        pub proximity: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Proximity>>,
    }
    impl GooglePrivacyDlpV2HotwordRule {
        pub fn builder() -> GooglePrivacyDlpV2HotwordRuleBuilder {
            GooglePrivacyDlpV2HotwordRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An individual hybrid item to inspect. Will be stored temporarily during processing."]
    pub struct GooglePrivacyDlpV2HybridContentItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "findingDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Supplementary information that will be added to each finding."]
        pub finding_details:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2HybridFindingDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "item")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The item to inspect."]
        pub item: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ContentItem>>,
    }
    impl GooglePrivacyDlpV2HybridContentItem {
        pub fn builder() -> GooglePrivacyDlpV2HybridContentItemBuilder {
            GooglePrivacyDlpV2HybridContentItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Populate to associate additional data with each finding."]
    pub struct GooglePrivacyDlpV2HybridFindingDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details about the container where the content being inspected is from."]
        pub container_details:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Container>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Offset in bytes of the line, from the beginning of the file, where the finding is located. Populate if the item being scanned is only part of a bigger item, such as a shard of a file and you want to track the absolute position of the finding."]
        pub file_offset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Labels to represent user provided metadata about the data being inspected. If configured by the job, some key values may be required. The labels associated with `Finding`'s produced by hybrid inspection. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?`. Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. No more than 10 labels can be associated with a given finding. Examples: * `\"environment\" : \"production\"` * `\"pipeline\" : \"etl\"`"]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Offset of the row for tables. Populate if the row(s) being scanned are part of a bigger dataset and you want to keep track of their absolute position."]
        pub row_offset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the container is a table, additional information to make findings meaningful such as the columns that are primary keys. If not known ahead of time, can also be set within each inspect hybrid call and the two will be merged. Note that identifying_fields will only be stored to BigQuery, and only if the BigQuery action has been included."]
        pub table_options: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TableOptions>>,
    }
    impl GooglePrivacyDlpV2HybridFindingDetails {
        pub fn builder() -> GooglePrivacyDlpV2HybridFindingDetailsBuilder {
            GooglePrivacyDlpV2HybridFindingDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to search for potentially sensitive info in a custom location."]
    pub struct GooglePrivacyDlpV2HybridInspectDlpJobRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hybridItem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The item to inspect."]
        pub hybrid_item:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2HybridContentItem>>,
    }
    impl GooglePrivacyDlpV2HybridInspectDlpJobRequest {
        pub fn builder() -> GooglePrivacyDlpV2HybridInspectDlpJobRequestBuilder {
            GooglePrivacyDlpV2HybridInspectDlpJobRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to search for potentially sensitive info in a custom location."]
    pub struct GooglePrivacyDlpV2HybridInspectJobTriggerRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hybridItem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The item to inspect."]
        pub hybrid_item:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2HybridContentItem>>,
    }
    impl GooglePrivacyDlpV2HybridInspectJobTriggerRequest {
        pub fn builder() -> GooglePrivacyDlpV2HybridInspectJobTriggerRequestBuilder {
            GooglePrivacyDlpV2HybridInspectJobTriggerRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Quota exceeded errors will be thrown once quota has been met."]
    pub struct GooglePrivacyDlpV2HybridInspectResponse {}
    impl GooglePrivacyDlpV2HybridInspectResponse {
        pub fn builder() -> GooglePrivacyDlpV2HybridInspectResponseBuilder {
            GooglePrivacyDlpV2HybridInspectResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Statistics related to processing hybrid inspect requests."]
    pub struct GooglePrivacyDlpV2HybridInspectStatistics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "abortedCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of hybrid inspection requests aborted because the job ran out of quota or was ended before they could be processed."]
        pub aborted_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pendingCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of hybrid requests currently being processed. Only populated when called via method `getDlpJob`. A burst of traffic may cause hybrid inspect requests to be enqueued. Processing will take place as quickly as possible, but resource limitations may impact how long a request is enqueued for."]
        pub pending_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processedCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of hybrid inspection requests processed within this job."]
        pub processed_count: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2HybridInspectStatistics {
        pub fn builder() -> GooglePrivacyDlpV2HybridInspectStatisticsBuilder {
            GooglePrivacyDlpV2HybridInspectStatisticsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration to control jobs where the content being inspected is outside of Google Cloud Platform."]
    pub struct GooglePrivacyDlpV2HybridOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short description of where the data is coming from. Will be stored once in the job. 256 max length."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "To organize findings, these labels will be added to each finding. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?`. Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. No more than 10 labels can be associated with a given finding. Examples: * `\"environment\" : \"production\"` * `\"pipeline\" : \"etl\"`"]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requiredFindingLabelKeys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "These are labels that each inspection request must include within their 'finding_labels' map. Request may contain others, but any missing one of these will be rejected. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?`. No more than 10 keys can be required."]
        pub required_finding_label_keys:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the container is a table, additional information to make findings meaningful such as the columns that are primary keys."]
        pub table_options: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TableOptions>>,
    }
    impl GooglePrivacyDlpV2HybridOptions {
        pub fn builder() -> GooglePrivacyDlpV2HybridOptionsBuilder {
            GooglePrivacyDlpV2HybridOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Location of the finding within an image."]
    pub struct GooglePrivacyDlpV2ImageLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBoxes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bounding boxes locating the pixels within the image containing the finding."]
        pub bounding_boxes: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2BoundingBox>>,
        >,
    }
    impl GooglePrivacyDlpV2ImageLocation {
        pub fn builder() -> GooglePrivacyDlpV2ImageLocationBuilder {
            GooglePrivacyDlpV2ImageLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for determining how redaction of images should occur."]
    pub struct GooglePrivacyDlpV2ImageRedactionConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infoType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only one per info_type should be provided per request. If not specified, and redact_all_text is false, the DLP API will redact all text that it matches against all info_types that are found, but not specified in another ImageRedactionConfig."]
        pub info_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redactAllText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, all text found in the image, regardless whether it matches an info_type, is redacted. Only one should be provided."]
        pub redact_all_text: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redactionColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color to use when redacting content from an image. If not specified, the default is black."]
        pub redaction_color: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Color>>,
    }
    impl GooglePrivacyDlpV2ImageRedactionConfig {
        pub fn builder() -> GooglePrivacyDlpV2ImageRedactionConfigBuilder {
            GooglePrivacyDlpV2ImageRedactionConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Type of information detected by the API."]
    pub struct GooglePrivacyDlpV2InfoType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed at https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type. When sending Cloud DLP results to Data Catalog, infoType names should conform to the pattern `[A-Za-z0-9$-_]{1,64}`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2InfoType {
        pub fn builder() -> GooglePrivacyDlpV2InfoTypeBuilder {
            GooglePrivacyDlpV2InfoTypeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "InfoType description."]
    pub struct GooglePrivacyDlpV2InfoTypeDescription {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the infotype. Translated when language is provided in the request."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human readable form of the infoType name."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Internal name of the infoType."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportedBy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which parts of the API supports this InfoType."]
        pub supported_by: ::std::option::Option<
            ::std::vec::Vec<GooglePrivacyDlpV2InfoTypeDescriptionSupportedByEnum>,
        >,
    }
    impl GooglePrivacyDlpV2InfoTypeDescription {
        pub fn builder() -> GooglePrivacyDlpV2InfoTypeDescriptionBuilder {
            GooglePrivacyDlpV2InfoTypeDescriptionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GooglePrivacyDlpV2InfoTypeDescriptionSupportedByEnum {
        #[serde(rename = "ENUM_TYPE_UNSPECIFIED")]
        #[doc = "Unused."]
        EnumTypeUnspecified,
        #[serde(rename = "INSPECT")]
        #[doc = "Supported by the inspect operations."]
        Inspect,
        #[serde(rename = "RISK_ANALYSIS")]
        #[doc = "Supported by the risk analysis operations."]
        RiskAnalysis,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2InfoTypeDescriptionSupportedByEnum {
        fn default() -> Self {
            Self::EnumTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Max findings configuration per infoType, per content item or long running DlpJob."]
    pub struct GooglePrivacyDlpV2InfoTypeLimit {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infoType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of information the findings limit applies to. Only one limit per info_type should be provided. If InfoTypeLimit does not have an info_type, the DLP API applies the limit against all info_types that are found but not specified in another InfoTypeLimit."]
        pub info_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxFindings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Max findings limit for the given infoType."]
        pub max_findings: ::std::option::Option<::std::primitive::i64>,
    }
    impl GooglePrivacyDlpV2InfoTypeLimit {
        pub fn builder() -> GooglePrivacyDlpV2InfoTypeLimitBuilder {
            GooglePrivacyDlpV2InfoTypeLimitBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Statistics regarding a specific InfoType."]
    pub struct GooglePrivacyDlpV2InfoTypeStats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of findings for this infoType."]
        pub count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infoType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of finding this stat is for."]
        pub info_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
    }
    impl GooglePrivacyDlpV2InfoTypeStats {
        pub fn builder() -> GooglePrivacyDlpV2InfoTypeStatsBuilder {
            GooglePrivacyDlpV2InfoTypeStatsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A transformation to apply to text that is identified as a specific info_type."]
    pub struct GooglePrivacyDlpV2InfoTypeTransformation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infoTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "InfoTypes to apply the transformation to. An empty list will cause this transformation to apply to all findings that correspond to infoTypes that were requested in `InspectConfig`."]
        pub info_types:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primitiveTransformation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Primitive transformation to apply to the infoType."]
        pub primitive_transformation:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PrimitiveTransformation>>,
    }
    impl GooglePrivacyDlpV2InfoTypeTransformation {
        pub fn builder() -> GooglePrivacyDlpV2InfoTypeTransformationBuilder {
            GooglePrivacyDlpV2InfoTypeTransformationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A type of transformation that will scan unstructured text and apply various `PrimitiveTransformation`s to each finding, where the transformation is applied to only values that were identified as a specific info_type."]
    pub struct GooglePrivacyDlpV2InfoTypeTransformations {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transformations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Transformation for each infoType. Cannot specify more than one for a given infoType."]
        pub transformations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InfoTypeTransformation>>,
        >,
    }
    impl GooglePrivacyDlpV2InfoTypeTransformations {
        pub fn builder() -> GooglePrivacyDlpV2InfoTypeTransformationsBuilder {
            GooglePrivacyDlpV2InfoTypeTransformationsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration description of the scanning process. When used with redactContent only info_types and min_likelihood are currently used."]
    pub struct GooglePrivacyDlpV2InspectConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of options defining data content to scan. If empty, text, images, and other content will be included."]
        pub content_options: ::std::option::Option<
            ::std::vec::Vec<GooglePrivacyDlpV2InspectConfigContentOptionsEnum>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customInfoTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "CustomInfoTypes provided by the user. See https://cloud.google.com/dlp/docs/creating-custom-infotypes to learn more."]
        pub custom_info_types: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2CustomInfoType>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludeInfoTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When true, excludes type information of the findings."]
        pub exclude_info_types: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeQuote")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When true, a contextual quote from the data that triggered a finding is included in the response; see Finding.quote."]
        pub include_quote: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infoTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricts what info_types to look for. The values must correspond to InfoType values returned by ListInfoTypes or listed at https://cloud.google.com/dlp/docs/infotypes-reference. When no InfoTypes or CustomInfoTypes are specified in a request, the system may automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated. If you need precise control and predictability as to what detectors are run you should specify specific InfoTypes listed in the reference, otherwise a default list will be used, which may change over time."]
        pub info_types:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "limits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration to control the number of findings returned."]
        pub limits: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FindingLimits>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only returns findings equal or above this threshold. The default is POSSIBLE. See https://cloud.google.com/dlp/docs/likelihood to learn more."]
        pub min_likelihood: ::std::option::Option<GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ruleSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of rules to apply to the findings for this InspectConfig. Exclusion rules, contained in the set are executed in the end, other rules are executed in the order they are specified for each info type."]
        pub rule_set: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InspectionRuleSet>>,
        >,
    }
    impl GooglePrivacyDlpV2InspectConfig {
        pub fn builder() -> GooglePrivacyDlpV2InspectConfigBuilder {
            GooglePrivacyDlpV2InspectConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GooglePrivacyDlpV2InspectConfigContentOptionsEnum {
        #[serde(rename = "CONTENT_UNSPECIFIED")]
        #[doc = "Includes entire content of a file or a data stream."]
        ContentUnspecified,
        #[serde(rename = "CONTENT_TEXT")]
        #[doc = "Text content within the data, excluding any metadata."]
        ContentText,
        #[serde(rename = "CONTENT_IMAGE")]
        #[doc = "Images found in the data."]
        ContentImage,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2InspectConfigContentOptionsEnum {
        fn default() -> Self {
            Self::ContentUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Only returns findings equal or above this threshold. The default is POSSIBLE. See https://cloud.google.com/dlp/docs/likelihood to learn more."]
    pub enum GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum {
        #[serde(rename = "LIKELIHOOD_UNSPECIFIED")]
        #[doc = "Default value; same as POSSIBLE."]
        LikelihoodUnspecified,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "Few matching elements."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = ""]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "Some matching elements."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = ""]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "Many matching elements."]
        VeryLikely,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum {
        fn default() -> Self {
            Self::LikelihoodUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to search for potentially sensitive info in a ContentItem."]
    pub struct GooglePrivacyDlpV2InspectContentRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inspectConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for the inspector. What specified here will override the template referenced by the inspect_template_name argument."]
        pub inspect_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inspectTemplateName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Template to use. Any configuration directly specified in inspect_config will override those set in the template. Singular fields that are set in this request will replace their corresponding fields in the template. Repeated fields are appended. Singular sub-messages and groups are recursively merged."]
        pub inspect_template_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "item")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The item to inspect."]
        pub item: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ContentItem>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This field has no effect."]
        pub location_id: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2InspectContentRequest {
        pub fn builder() -> GooglePrivacyDlpV2InspectContentRequestBuilder {
            GooglePrivacyDlpV2InspectContentRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results of inspecting an item."]
    pub struct GooglePrivacyDlpV2InspectContentResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "result")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The findings."]
        pub result: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectResult>>,
    }
    impl GooglePrivacyDlpV2InspectContentResponse {
        pub fn builder() -> GooglePrivacyDlpV2InspectContentResponseBuilder {
            GooglePrivacyDlpV2InspectContentResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The results of an inspect DataSource job."]
    pub struct GooglePrivacyDlpV2InspectDataSourceDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestedOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The configuration used for this job."]
        pub requested_options:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RequestedOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "result")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A summary of the outcome of this inspection job."]
        pub result: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Result>>,
    }
    impl GooglePrivacyDlpV2InspectDataSourceDetails {
        pub fn builder() -> GooglePrivacyDlpV2InspectDataSourceDetailsBuilder {
            GooglePrivacyDlpV2InspectDataSourceDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Controls what and how to inspect for findings."]
    pub struct GooglePrivacyDlpV2InspectJobConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Actions to execute at the completion of the job."]
        pub actions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Action>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inspectConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How and what to scan for."]
        pub inspect_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inspectTemplateName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If provided, will be used as the default for all values in InspectConfig. `inspect_config` will be merged into the values persisted as part of the template."]
        pub inspect_template_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storageConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data to scan."]
        pub storage_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2StorageConfig>>,
    }
    impl GooglePrivacyDlpV2InspectJobConfig {
        pub fn builder() -> GooglePrivacyDlpV2InspectJobConfigBuilder {
            GooglePrivacyDlpV2InspectJobConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "All the findings for a single scanned item."]
    pub struct GooglePrivacyDlpV2InspectResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "findings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of findings for an item."]
        pub findings:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Finding>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "findingsTruncated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, then this item might have more findings than were returned, and the findings returned are an arbitrary subset of all findings. The findings list might be truncated because the input items were too large, or because the server reached the maximum amount of resources allowed for a single API call. For best results, divide the input into smaller batches."]
        pub findings_truncated: ::std::option::Option<::std::primitive::bool>,
    }
    impl GooglePrivacyDlpV2InspectResult {
        pub fn builder() -> GooglePrivacyDlpV2InspectResultBuilder {
            GooglePrivacyDlpV2InspectResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The inspectTemplate contains a configuration (set of types of sensitive data to be detected) to be used anywhere you otherwise would normally specify InspectConfig. See https://cloud.google.com/dlp/docs/concepts-templates to learn more."]
    pub struct GooglePrivacyDlpV2InspectTemplate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The creation timestamp of an inspectTemplate."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Short description (max 256 chars)."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display name (max 256 chars)."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inspectConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The core content of the template. Configuration of the scanning process."]
        pub inspect_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The template name. The template will have one of the following formats: `projects/PROJECT_ID/inspectTemplates/TEMPLATE_ID` OR `organizations/ORGANIZATION_ID/inspectTemplates/TEMPLATE_ID`;"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The last update timestamp of an inspectTemplate."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2InspectTemplate {
        pub fn builder() -> GooglePrivacyDlpV2InspectTemplateBuilder {
            GooglePrivacyDlpV2InspectTemplateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single inspection rule to be applied to infoTypes, specified in `InspectionRuleSet`."]
    pub struct GooglePrivacyDlpV2InspectionRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exclusionRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Exclusion rule."]
        pub exclusion_rule:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ExclusionRule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hotwordRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hotword-based detection rule."]
        pub hotword_rule: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2HotwordRule>>,
    }
    impl GooglePrivacyDlpV2InspectionRule {
        pub fn builder() -> GooglePrivacyDlpV2InspectionRuleBuilder {
            GooglePrivacyDlpV2InspectionRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Rule set for modifying a set of infoTypes to alter behavior under certain circumstances, depending on the specific details of the rules within the set."]
    pub struct GooglePrivacyDlpV2InspectionRuleSet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infoTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of infoTypes this rule set is applied to."]
        pub info_types:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of rules to be applied to infoTypes. The rules are applied in order."]
        pub rules: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InspectionRule>>,
        >,
    }
    impl GooglePrivacyDlpV2InspectionRuleSet {
        pub fn builder() -> GooglePrivacyDlpV2InspectionRuleSetBuilder {
            GooglePrivacyDlpV2InspectionRuleSetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Enable email notification to project owners and editors on jobs's completion/failure."]
    pub struct GooglePrivacyDlpV2JobNotificationEmails {}
    impl GooglePrivacyDlpV2JobNotificationEmails {
        pub fn builder() -> GooglePrivacyDlpV2JobNotificationEmailsBuilder {
            GooglePrivacyDlpV2JobNotificationEmailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains a configuration to make dlp api calls on a repeating basis. See https://cloud.google.com/dlp/docs/concepts-job-triggers to learn more."]
    pub struct GooglePrivacyDlpV2JobTrigger {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The creation timestamp of a triggeredJob."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User provided description (max 256 chars)"]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display name (max 100 chars)"]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A stream of errors encountered when the trigger was activated. Repeated errors may result in the JobTrigger automatically being paused. Will return the last 100 errors. Whenever the JobTrigger is modified this list will be cleared."]
        pub errors:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Error>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inspectJob")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For inspect jobs, a snapshot of the configuration."]
        pub inspect_job:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectJobConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastRunTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The timestamp of the last time this trigger executed."]
        pub last_run_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique resource name for the triggeredJob, assigned by the service when the triggeredJob is created, for example `projects/dlp-test-project/jobTriggers/53234423`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A status for this trigger."]
        pub status: ::std::option::Option<GooglePrivacyDlpV2JobTriggerStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of triggers which will be OR'ed together. Only one in the list needs to trigger for a job to be started. The list may contain only a single Schedule trigger and must have at least one object."]
        pub triggers:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Trigger>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The last update timestamp of a triggeredJob."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2JobTrigger {
        pub fn builder() -> GooglePrivacyDlpV2JobTriggerBuilder {
            GooglePrivacyDlpV2JobTriggerBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. A status for this trigger."]
    pub enum GooglePrivacyDlpV2JobTriggerStatusEnum {
        #[serde(rename = "STATUS_UNSPECIFIED")]
        #[doc = "Unused."]
        StatusUnspecified,
        #[serde(rename = "HEALTHY")]
        #[doc = "Trigger is healthy."]
        Healthy,
        #[serde(rename = "PAUSED")]
        #[doc = "Trigger is temporarily paused."]
        Paused,
        #[serde(rename = "CANCELLED")]
        #[doc = "Trigger is cancelled and can not be resumed."]
        Cancelled,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2JobTriggerStatusEnum {
        fn default() -> Self {
            Self::StatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "k-anonymity metric, used for analysis of reidentification risk."]
    pub struct GooglePrivacyDlpV2KAnonymityConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Message indicating that multiple rows might be associated to a single individual. If the same entity_id is associated to multiple quasi-identifier tuples over distinct rows, we consider the entire collection of tuples as the composite quasi-identifier. This collection is a multiset: the order in which the different tuples appear in the dataset is ignored, but their frequency is taken into account. Important note: a maximum of 1000 rows can be associated to a single entity ID. If more rows are associated with the same entity ID, some might be ignored."]
        pub entity_id: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2EntityId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quasiIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of fields to compute k-anonymity over. When multiple fields are specified, they are considered a single composite key. Structs and repeated data types are not supported; however, nested fields are supported so long as they are not structs themselves or nested within a repeated field."]
        pub quasi_ids:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>>,
    }
    impl GooglePrivacyDlpV2KAnonymityConfig {
        pub fn builder() -> GooglePrivacyDlpV2KAnonymityConfigBuilder {
            GooglePrivacyDlpV2KAnonymityConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The set of columns' values that share the same ldiversity value"]
    pub struct GooglePrivacyDlpV2KAnonymityEquivalenceClass {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "equivalenceClassSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Size of the equivalence class, for example number of rows with the above set of values."]
        pub equivalence_class_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quasiIdsValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of values defining the equivalence class. One value per quasi-identifier column in the original KAnonymity metric message. The order is always the same as the original request."]
        pub quasi_ids_values:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Value>>>,
    }
    impl GooglePrivacyDlpV2KAnonymityEquivalenceClass {
        pub fn builder() -> GooglePrivacyDlpV2KAnonymityEquivalenceClassBuilder {
            GooglePrivacyDlpV2KAnonymityEquivalenceClassBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Histogram of k-anonymity equivalence classes."]
    pub struct GooglePrivacyDlpV2KAnonymityHistogramBucket {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of equivalence classes in this bucket."]
        pub bucket_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketValueCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of distinct equivalence classes in this bucket."]
        pub bucket_value_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sample of equivalence classes in this bucket. The total number of classes returned per bucket is capped at 20."]
        pub bucket_values: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2KAnonymityEquivalenceClass>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "equivalenceClassSizeLowerBound")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Lower bound on the size of the equivalence classes in this bucket."]
        pub equivalence_class_size_lower_bound: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "equivalenceClassSizeUpperBound")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Upper bound on the size of the equivalence classes in this bucket."]
        pub equivalence_class_size_upper_bound: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2KAnonymityHistogramBucket {
        pub fn builder() -> GooglePrivacyDlpV2KAnonymityHistogramBucketBuilder {
            GooglePrivacyDlpV2KAnonymityHistogramBucketBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Result of the k-anonymity computation."]
    pub struct GooglePrivacyDlpV2KAnonymityResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "equivalenceClassHistogramBuckets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Histogram of k-anonymity equivalence classes."]
        pub equivalence_class_histogram_buckets: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2KAnonymityHistogramBucket>>,
        >,
    }
    impl GooglePrivacyDlpV2KAnonymityResult {
        pub fn builder() -> GooglePrivacyDlpV2KAnonymityResultBuilder {
            GooglePrivacyDlpV2KAnonymityResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Reidentifiability metric. This corresponds to a risk model similar to what is called \"journalist risk\" in the literature, except the attack dataset is statistically modeled instead of being perfectly known. This can be done using publicly available data (like the US Census), or using a custom statistical model (indicated as one or several BigQuery tables), or by extrapolating from the distribution of values in the input dataset."]
    pub struct GooglePrivacyDlpV2KMapEstimationConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "auxiliaryTables")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Several auxiliary tables can be used in the analysis. Each custom_tag used to tag a quasi-identifiers column must appear in exactly one column of one auxiliary table."]
        pub auxiliary_tables: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2AuxiliaryTable>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quasiIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Fields considered to be quasi-identifiers. No two columns can have the same tag."]
        pub quasi_ids: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2TaggedField>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ISO 3166-1 alpha-2 region code to use in the statistical modeling. Set if no column is tagged with a region-specific InfoType (like US_ZIP_5) or a region code."]
        pub region_code: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2KMapEstimationConfig {
        pub fn builder() -> GooglePrivacyDlpV2KMapEstimationConfigBuilder {
            GooglePrivacyDlpV2KMapEstimationConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A KMapEstimationHistogramBucket message with the following values: min_anonymity: 3 max_anonymity: 5 frequency: 42 means that there are 42 records whose quasi-identifier values correspond to 3, 4 or 5 people in the overlying population. An important particular case is when min_anonymity = max_anonymity = 1: the frequency field then corresponds to the number of uniquely identifiable records."]
    pub struct GooglePrivacyDlpV2KMapEstimationHistogramBucket {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of records within these anonymity bounds."]
        pub bucket_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketValueCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of distinct quasi-identifier tuple values in this bucket."]
        pub bucket_value_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sample of quasi-identifier tuple values in this bucket. The total number of classes returned per bucket is capped at 20."]
        pub bucket_values: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2KMapEstimationQuasiIdValues>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxAnonymity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always greater than or equal to min_anonymity."]
        pub max_anonymity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minAnonymity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always positive."]
        pub min_anonymity: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2KMapEstimationHistogramBucket {
        pub fn builder() -> GooglePrivacyDlpV2KMapEstimationHistogramBucketBuilder {
            GooglePrivacyDlpV2KMapEstimationHistogramBucketBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A tuple of values for the quasi-identifier columns."]
    pub struct GooglePrivacyDlpV2KMapEstimationQuasiIdValues {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "estimatedAnonymity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The estimated anonymity for these quasi-identifier values."]
        pub estimated_anonymity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quasiIdsValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quasi-identifier values."]
        pub quasi_ids_values:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Value>>>,
    }
    impl GooglePrivacyDlpV2KMapEstimationQuasiIdValues {
        pub fn builder() -> GooglePrivacyDlpV2KMapEstimationQuasiIdValuesBuilder {
            GooglePrivacyDlpV2KMapEstimationQuasiIdValuesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Result of the reidentifiability analysis. Note that these results are an estimation, not exact values."]
    pub struct GooglePrivacyDlpV2KMapEstimationResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kMapEstimationHistogram")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The intervals [min_anonymity, max_anonymity] do not overlap. If a value doesn't correspond to any such interval, the associated frequency is zero. For example, the following records: {min_anonymity: 1, max_anonymity: 1, frequency: 17} {min_anonymity: 2, max_anonymity: 3, frequency: 42} {min_anonymity: 5, max_anonymity: 10, frequency: 99} mean that there are no record with an estimated anonymity of 4, 5, or larger than 10."]
        pub k_map_estimation_histogram: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2KMapEstimationHistogramBucket>>,
        >,
    }
    impl GooglePrivacyDlpV2KMapEstimationResult {
        pub fn builder() -> GooglePrivacyDlpV2KMapEstimationResultBuilder {
            GooglePrivacyDlpV2KMapEstimationResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A unique identifier for a Datastore entity. If a key's partition ID or any of its path kinds or names are reserved/read-only, the key is reserved/read-only. A reserved/read-only key is forbidden in certain documented contexts."]
    pub struct GooglePrivacyDlpV2Key {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partitionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entities are partitioned into subsets, currently identified by a project ID and namespace ID. Queries are scoped to a single partition."]
        pub partition_id: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PartitionId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entity path. An entity path consists of one or more elements composed of a kind and a string or numerical identifier, which identify entities. The first element identifies a _root entity_, the second element identifies a _child_ of the root entity, the third element identifies a child of the second entity, and so forth. The entities identified by all prefixes of the path are called the element's _ancestors_. A path can never be empty, and a path can have at most 100 elements."]
        pub path: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2PathElement>>,
        >,
    }
    impl GooglePrivacyDlpV2Key {
        pub fn builder() -> GooglePrivacyDlpV2KeyBuilder {
            GooglePrivacyDlpV2KeyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A representation of a Datastore kind."]
    pub struct GooglePrivacyDlpV2KindExpression {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the kind."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2KindExpression {
        pub fn builder() -> GooglePrivacyDlpV2KindExpressionBuilder {
            GooglePrivacyDlpV2KindExpressionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Include to use an existing data crypto key wrapped by KMS. The wrapped key must be a 128/192/256 bit key. Authorization requires the following IAM permissions when sending a request to perform a crypto transformation using a kms-wrapped crypto key: dlp.kms.encrypt"]
    pub struct GooglePrivacyDlpV2KmsWrappedCryptoKey {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cryptoKeyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The resource name of the KMS CryptoKey to use for unwrapping."]
        pub crypto_key_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "wrappedKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The wrapped data crypto key."]
        pub wrapped_key: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2KmsWrappedCryptoKey {
        pub fn builder() -> GooglePrivacyDlpV2KmsWrappedCryptoKeyBuilder {
            GooglePrivacyDlpV2KmsWrappedCryptoKeyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "l-diversity metric, used for analysis of reidentification risk."]
    pub struct GooglePrivacyDlpV2LDiversityConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quasiIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of quasi-identifiers indicating how equivalence classes are defined for the l-diversity computation. When multiple fields are specified, they are considered a single composite key."]
        pub quasi_ids:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sensitiveAttribute")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sensitive field for computing the l-value."]
        pub sensitive_attribute:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
    }
    impl GooglePrivacyDlpV2LDiversityConfig {
        pub fn builder() -> GooglePrivacyDlpV2LDiversityConfigBuilder {
            GooglePrivacyDlpV2LDiversityConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The set of columns' values that share the same ldiversity value."]
    pub struct GooglePrivacyDlpV2LDiversityEquivalenceClass {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "equivalenceClassSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Size of the k-anonymity equivalence class."]
        pub equivalence_class_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numDistinctSensitiveValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of distinct sensitive values in this equivalence class."]
        pub num_distinct_sensitive_values: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quasiIdsValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Quasi-identifier values defining the k-anonymity equivalence class. The order is always the same as the original request."]
        pub quasi_ids_values:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Value>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topSensitiveValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Estimated frequencies of top sensitive values."]
        pub top_sensitive_values: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2ValueFrequency>>,
        >,
    }
    impl GooglePrivacyDlpV2LDiversityEquivalenceClass {
        pub fn builder() -> GooglePrivacyDlpV2LDiversityEquivalenceClassBuilder {
            GooglePrivacyDlpV2LDiversityEquivalenceClassBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Histogram of l-diversity equivalence class sensitive value frequencies."]
    pub struct GooglePrivacyDlpV2LDiversityHistogramBucket {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of equivalence classes in this bucket."]
        pub bucket_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketValueCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of distinct equivalence classes in this bucket."]
        pub bucket_value_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sample of equivalence classes in this bucket. The total number of classes returned per bucket is capped at 20."]
        pub bucket_values: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2LDiversityEquivalenceClass>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sensitiveValueFrequencyLowerBound")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Lower bound on the sensitive value frequencies of the equivalence classes in this bucket."]
        pub sensitive_value_frequency_lower_bound: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sensitiveValueFrequencyUpperBound")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Upper bound on the sensitive value frequencies of the equivalence classes in this bucket."]
        pub sensitive_value_frequency_upper_bound: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2LDiversityHistogramBucket {
        pub fn builder() -> GooglePrivacyDlpV2LDiversityHistogramBucketBuilder {
            GooglePrivacyDlpV2LDiversityHistogramBucketBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Result of the l-diversity computation."]
    pub struct GooglePrivacyDlpV2LDiversityResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sensitiveValueFrequencyHistogramBuckets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Histogram of l-diversity equivalence class sensitive value frequencies."]
        pub sensitive_value_frequency_histogram_buckets: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2LDiversityHistogramBucket>>,
        >,
    }
    impl GooglePrivacyDlpV2LDiversityResult {
        pub fn builder() -> GooglePrivacyDlpV2LDiversityResultBuilder {
            GooglePrivacyDlpV2LDiversityResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for a custom dictionary created from a data source of any size up to the maximum size defined in the [limits](https://cloud.google.com/dlp/limits) page. The artifacts of dictionary creation are stored in the specified Google Cloud Storage location. Consider using `CustomInfoType.Dictionary` for smaller dictionaries that satisfy the size requirements."]
    pub struct GooglePrivacyDlpV2LargeCustomDictionaryConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bigQueryField")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Field in a BigQuery table where each cell represents a dictionary phrase."]
        pub big_query_field:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryField>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudStorageFileSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of files containing newline-delimited lists of dictionary phrases."]
        pub cloud_storage_file_set:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CloudStorageFileSet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location to store dictionary artifacts in Google Cloud Storage. These files will only be accessible by project owners and the DLP API. If any of these artifacts are modified, the dictionary is considered invalid and can no longer be used."]
        pub output_path:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CloudStoragePath>>,
    }
    impl GooglePrivacyDlpV2LargeCustomDictionaryConfig {
        pub fn builder() -> GooglePrivacyDlpV2LargeCustomDictionaryConfigBuilder {
            GooglePrivacyDlpV2LargeCustomDictionaryConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Summary statistics of a custom dictionary."]
    pub struct GooglePrivacyDlpV2LargeCustomDictionaryStats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "approxNumPhrases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Approximate number of distinct phrases in the dictionary."]
        pub approx_num_phrases: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2LargeCustomDictionaryStats {
        pub fn builder() -> GooglePrivacyDlpV2LargeCustomDictionaryStatsBuilder {
            GooglePrivacyDlpV2LargeCustomDictionaryStatsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Skips the data without modifying it if the requested transformation would cause an error. For example, if a `DateShift` transformation were applied an an IP address, this mode would leave the IP address unchanged in the response."]
    pub struct GooglePrivacyDlpV2LeaveUntransformed {}
    impl GooglePrivacyDlpV2LeaveUntransformed {
        pub fn builder() -> GooglePrivacyDlpV2LeaveUntransformedBuilder {
            GooglePrivacyDlpV2LeaveUntransformedBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message for specifying an adjustment to the likelihood of a finding as part of a detection rule."]
    pub struct GooglePrivacyDlpV2LikelihoodAdjustment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fixedLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set the likelihood of a finding to a fixed value."]
        pub fixed_likelihood:
            ::std::option::Option<GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relativeLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Increase or decrease the likelihood by the specified number of levels. For example, if a finding would be `POSSIBLE` without the detection rule and `relative_likelihood` is 1, then it is upgraded to `LIKELY`, while a value of -1 would downgrade it to `UNLIKELY`. Likelihood may never drop below `VERY_UNLIKELY` or exceed `VERY_LIKELY`, so applying an adjustment of 1 followed by an adjustment of -1 when base likelihood is `VERY_LIKELY` will result in a final likelihood of `LIKELY`."]
        pub relative_likelihood: ::std::option::Option<::std::primitive::i64>,
    }
    impl GooglePrivacyDlpV2LikelihoodAdjustment {
        pub fn builder() -> GooglePrivacyDlpV2LikelihoodAdjustmentBuilder {
            GooglePrivacyDlpV2LikelihoodAdjustmentBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Set the likelihood of a finding to a fixed value."]
    pub enum GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum {
        #[serde(rename = "LIKELIHOOD_UNSPECIFIED")]
        #[doc = "Default value; same as POSSIBLE."]
        LikelihoodUnspecified,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "Few matching elements."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = ""]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "Some matching elements."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = ""]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "Many matching elements."]
        VeryLikely,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum {
        fn default() -> Self {
            Self::LikelihoodUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListDeidentifyTemplates."]
    pub struct GooglePrivacyDlpV2ListDeidentifyTemplatesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deidentifyTemplates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of deidentify templates, up to page_size in ListDeidentifyTemplatesRequest."]
        pub deidentify_templates: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2DeidentifyTemplate>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the next page is available then the next page token to be used in following ListDeidentifyTemplates request."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2ListDeidentifyTemplatesResponse {
        pub fn builder() -> GooglePrivacyDlpV2ListDeidentifyTemplatesResponseBuilder {
            GooglePrivacyDlpV2ListDeidentifyTemplatesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for listing DLP jobs."]
    pub struct GooglePrivacyDlpV2ListDlpJobsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of DlpJobs that matches the specified filter in the request."]
        pub jobs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2DlpJob>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2ListDlpJobsResponse {
        pub fn builder() -> GooglePrivacyDlpV2ListDlpJobsResponseBuilder {
            GooglePrivacyDlpV2ListDlpJobsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to the ListInfoTypes request."]
    pub struct GooglePrivacyDlpV2ListInfoTypesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infoTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of sensitive infoTypes."]
        pub info_types: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InfoTypeDescription>>,
        >,
    }
    impl GooglePrivacyDlpV2ListInfoTypesResponse {
        pub fn builder() -> GooglePrivacyDlpV2ListInfoTypesResponseBuilder {
            GooglePrivacyDlpV2ListInfoTypesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListInspectTemplates."]
    pub struct GooglePrivacyDlpV2ListInspectTemplatesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inspectTemplates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of inspectTemplates, up to page_size in ListInspectTemplatesRequest."]
        pub inspect_templates: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InspectTemplate>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the next page is available then the next page token to be used in following ListInspectTemplates request."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2ListInspectTemplatesResponse {
        pub fn builder() -> GooglePrivacyDlpV2ListInspectTemplatesResponseBuilder {
            GooglePrivacyDlpV2ListInspectTemplatesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListJobTriggers."]
    pub struct GooglePrivacyDlpV2ListJobTriggersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobTriggers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of triggeredJobs, up to page_size in ListJobTriggersRequest."]
        pub job_triggers:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2JobTrigger>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the next page is available then the next page token to be used in following ListJobTriggers request."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2ListJobTriggersResponse {
        pub fn builder() -> GooglePrivacyDlpV2ListJobTriggersResponseBuilder {
            GooglePrivacyDlpV2ListJobTriggersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListStoredInfoTypes."]
    pub struct GooglePrivacyDlpV2ListStoredInfoTypesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the next page is available then the next page token to be used in following ListStoredInfoTypes request."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storedInfoTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of storedInfoTypes, up to page_size in ListStoredInfoTypesRequest."]
        pub stored_info_types: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2StoredInfoType>>,
        >,
    }
    impl GooglePrivacyDlpV2ListStoredInfoTypesResponse {
        pub fn builder() -> GooglePrivacyDlpV2ListStoredInfoTypesResponseBuilder {
            GooglePrivacyDlpV2ListStoredInfoTypesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies the location of the finding."]
    pub struct GooglePrivacyDlpV2Location {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "byteRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Zero-based byte offsets delimiting the finding. These are relative to the finding's containing element. Note that when the content is not textual, this references the UTF-8 encoded textual representation of the content. Omitted if content is an image."]
        pub byte_range: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Range>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "codepointRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unicode character offsets delimiting the finding. These are relative to the finding's containing element. Provided when the content is text."]
        pub codepoint_range: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Range>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "container")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the container where this finding occurred, if available."]
        pub container: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Container>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentLocations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of nested objects pointing to the precise location of the finding within the file or record."]
        pub content_locations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2ContentLocation>>,
        >,
    }
    impl GooglePrivacyDlpV2Location {
        pub fn builder() -> GooglePrivacyDlpV2LocationBuilder {
            GooglePrivacyDlpV2LocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Job trigger option for hybrid jobs. Jobs must be manually created and finished."]
    pub struct GooglePrivacyDlpV2Manual {}
    impl GooglePrivacyDlpV2Manual {
        pub fn builder() -> GooglePrivacyDlpV2ManualBuilder {
            GooglePrivacyDlpV2ManualBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata Location"]
    pub struct GooglePrivacyDlpV2MetadataLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storageLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Storage metadata."]
        pub storage_label:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2StorageMetadataLabel>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of metadata containing the finding."]
        pub _type: ::std::option::Option<GooglePrivacyDlpV2MetadataLocationTypeEnum>,
    }
    impl GooglePrivacyDlpV2MetadataLocation {
        pub fn builder() -> GooglePrivacyDlpV2MetadataLocationBuilder {
            GooglePrivacyDlpV2MetadataLocationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of metadata containing the finding."]
    pub enum GooglePrivacyDlpV2MetadataLocationTypeEnum {
        #[serde(rename = "METADATATYPE_UNSPECIFIED")]
        #[doc = "Unused"]
        MetadatatypeUnspecified,
        #[serde(rename = "STORAGE_METADATA")]
        #[doc = "General file metadata provided by Cloud Storage."]
        StorageMetadata,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2MetadataLocationTypeEnum {
        fn default() -> Self {
            Self::MetadatatypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Compute numerical stats over an individual column, including min, max, and quantiles."]
    pub struct GooglePrivacyDlpV2NumericalStatsConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Field to compute numerical stats on. Supported types are integer, float, date, datetime, timestamp, time."]
        pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
    }
    impl GooglePrivacyDlpV2NumericalStatsConfig {
        pub fn builder() -> GooglePrivacyDlpV2NumericalStatsConfigBuilder {
            GooglePrivacyDlpV2NumericalStatsConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Result of the numerical stats computation."]
    pub struct GooglePrivacyDlpV2NumericalStatsResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum value appearing in the column."]
        pub max_value: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum value appearing in the column."]
        pub min_value: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantileValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of 99 values that partition the set of field values into 100 equal sized buckets."]
        pub quantile_values:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Value>>>,
    }
    impl GooglePrivacyDlpV2NumericalStatsResult {
        pub fn builder() -> GooglePrivacyDlpV2NumericalStatsResultBuilder {
            GooglePrivacyDlpV2NumericalStatsResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Cloud repository for storing output."]
    pub struct GooglePrivacyDlpV2OutputStorageConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputSchema")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Schema used for writing the findings for Inspect jobs. This field is only used for Inspect and must be unspecified for Risk jobs. Columns are derived from the `Finding` object. If appending to an existing table, any columns from the predefined schema that are missing will be added. No columns in the existing table will be deleted. If unspecified, then all available columns will be used for a new table or an (existing) table with no schema, and no changes will be made to an existing table that has a schema. Only for use with external storage."]
        pub output_schema:
            ::std::option::Option<GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "table")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Store findings in an existing table or a new table in an existing dataset. If table_id is not set a new one will be generated for you with the following format: dlp_googleapis_yyyy_mm_dd_[dlp_job_id]. Pacific timezone will be used for generating the date details. For Inspect, each column in an existing output table must have the same name, type, and mode of a field in the `Finding` object. For Risk, an existing output table should be the output of a previous Risk analysis job run on the same source table, with the same privacy metric and quasi-identifiers. Risk jobs that analyze the same table but compute a different privacy metric, or use different sets of quasi-identifiers, cannot store their results in the same table."]
        pub table: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryTable>>,
    }
    impl GooglePrivacyDlpV2OutputStorageConfig {
        pub fn builder() -> GooglePrivacyDlpV2OutputStorageConfigBuilder {
            GooglePrivacyDlpV2OutputStorageConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Schema used for writing the findings for Inspect jobs. This field is only used for Inspect and must be unspecified for Risk jobs. Columns are derived from the `Finding` object. If appending to an existing table, any columns from the predefined schema that are missing will be added. No columns in the existing table will be deleted. If unspecified, then all available columns will be used for a new table or an (existing) table with no schema, and no changes will be made to an existing table that has a schema. Only for use with external storage."]
    pub enum GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum {
        #[serde(rename = "OUTPUT_SCHEMA_UNSPECIFIED")]
        #[doc = "Unused."]
        OutputSchemaUnspecified,
        #[serde(rename = "BASIC_COLUMNS")]
        #[doc = "Basic schema including only `info_type`, `quote`, `certainty`, and `timestamp`."]
        BasicColumns,
        #[serde(rename = "GCS_COLUMNS")]
        #[doc = "Schema tailored to findings from scanning Google Cloud Storage."]
        GcsColumns,
        #[serde(rename = "DATASTORE_COLUMNS")]
        #[doc = "Schema tailored to findings from scanning Google Datastore."]
        DatastoreColumns,
        #[serde(rename = "BIG_QUERY_COLUMNS")]
        #[doc = "Schema tailored to findings from scanning Google BigQuery."]
        BigQueryColumns,
        #[serde(rename = "ALL_COLUMNS")]
        #[doc = "Schema containing all columns."]
        AllColumns,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum {
        fn default() -> Self {
            Self::OutputSchemaUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Datastore partition ID. A partition ID identifies a grouping of entities. The grouping is always by project and namespace, however the namespace ID may be empty. A partition ID contains several dimensions: project ID and namespace ID."]
    pub struct GooglePrivacyDlpV2PartitionId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namespaceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If not empty, the ID of the namespace to which the entities belong."]
        pub namespace_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the project to which the entities belong."]
        pub project_id: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2PartitionId {
        pub fn builder() -> GooglePrivacyDlpV2PartitionIdBuilder {
            GooglePrivacyDlpV2PartitionIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A (kind, ID/name) pair used to construct a key path. If either name or ID is set, the element is complete. If neither is set, the element is incomplete."]
    pub struct GooglePrivacyDlpV2PathElement {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The auto-allocated ID of the entity. Never equal to zero. Values less than zero are discouraged and may not be supported in the future."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of the entity. A kind matching regex `__.*__` is reserved/read-only. A kind must not contain more than 1500 bytes when UTF-8 encoded. Cannot be `\"\"`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the entity. A name matching regex `__.*__` is reserved/read-only. A name must not be more than 1500 bytes when UTF-8 encoded. Cannot be `\"\"`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2PathElement {
        pub fn builder() -> GooglePrivacyDlpV2PathElementBuilder {
            GooglePrivacyDlpV2PathElementBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A rule for transforming a value."]
    pub struct GooglePrivacyDlpV2PrimitiveTransformation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketingConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bucketing"]
        pub bucketing_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BucketingConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "characterMaskConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mask"]
        pub character_mask_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CharacterMaskConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cryptoDeterministicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deterministic Crypto"]
        pub crypto_deterministic_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CryptoDeterministicConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cryptoHashConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Crypto"]
        pub crypto_hash_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CryptoHashConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cryptoReplaceFfxFpeConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ffx-Fpe"]
        pub crypto_replace_ffx_fpe_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CryptoReplaceFfxFpeConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateShiftConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date Shift"]
        pub date_shift_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DateShiftConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fixedSizeBucketingConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fixed size bucketing"]
        pub fixed_size_bucketing_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FixedSizeBucketingConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redactConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Redact"]
        pub redact_config: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RedactConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replaceConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Replace"]
        pub replace_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ReplaceValueConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replaceWithInfoTypeConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Replace with infotype"]
        pub replace_with_info_type_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ReplaceWithInfoTypeConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timePartConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time extraction"]
        pub time_part_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TimePartConfig>>,
    }
    impl GooglePrivacyDlpV2PrimitiveTransformation {
        pub fn builder() -> GooglePrivacyDlpV2PrimitiveTransformationBuilder {
            GooglePrivacyDlpV2PrimitiveTransformationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Privacy metric to compute for reidentification risk analysis."]
    pub struct GooglePrivacyDlpV2PrivacyMetric {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categoricalStatsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Categorical stats"]
        pub categorical_stats_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CategoricalStatsConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deltaPresenceEstimationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "delta-presence"]
        pub delta_presence_estimation_config: ::std::option::Option<
            ::std::boxed::Box<GooglePrivacyDlpV2DeltaPresenceEstimationConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kAnonymityConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "K-anonymity"]
        pub k_anonymity_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2KAnonymityConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kMapEstimationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "k-map"]
        pub k_map_estimation_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2KMapEstimationConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lDiversityConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "l-diversity"]
        pub l_diversity_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2LDiversityConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numericalStatsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Numerical stats"]
        pub numerical_stats_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2NumericalStatsConfig>>,
    }
    impl GooglePrivacyDlpV2PrivacyMetric {
        pub fn builder() -> GooglePrivacyDlpV2PrivacyMetricBuilder {
            GooglePrivacyDlpV2PrivacyMetricBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message for specifying a window around a finding to apply a detection rule."]
    pub struct GooglePrivacyDlpV2Proximity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "windowAfter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of characters after the finding to consider."]
        pub window_after: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "windowBefore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of characters before the finding to consider."]
        pub window_before: ::std::option::Option<::std::primitive::i64>,
    }
    impl GooglePrivacyDlpV2Proximity {
        pub fn builder() -> GooglePrivacyDlpV2ProximityBuilder {
            GooglePrivacyDlpV2ProximityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Publish findings of a DlpJob to Cloud Data Catalog. Labels summarizing the results of the DlpJob will be applied to the entry for the resource scanned in Cloud Data Catalog. Any labels previously written by another DlpJob will be deleted. InfoType naming patterns are strictly enforced when using this feature. Note that the findings will be persisted in Cloud Data Catalog storage and are governed by Data Catalog service-specific policy, see https://cloud.google.com/terms/service-terms Only a single instance of this action can be specified and only allowed if all resources being scanned are BigQuery tables. Compatible with: Inspect"]
    pub struct GooglePrivacyDlpV2PublishFindingsToCloudDataCatalog {}
    impl GooglePrivacyDlpV2PublishFindingsToCloudDataCatalog {
        pub fn builder() -> GooglePrivacyDlpV2PublishFindingsToCloudDataCatalogBuilder {
            GooglePrivacyDlpV2PublishFindingsToCloudDataCatalogBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Publish the result summary of a DlpJob to the Cloud Security Command Center (CSCC Alpha). This action is only available for projects which are parts of an organization and whitelisted for the alpha Cloud Security Command Center. The action will publish count of finding instances and their info types. The summary of findings will be persisted in CSCC and are governed by CSCC service-specific policy, see https://cloud.google.com/terms/service-terms Only a single instance of this action can be specified. Compatible with: Inspect"]
    pub struct GooglePrivacyDlpV2PublishSummaryToCscc {}
    impl GooglePrivacyDlpV2PublishSummaryToCscc {
        pub fn builder() -> GooglePrivacyDlpV2PublishSummaryToCsccBuilder {
            GooglePrivacyDlpV2PublishSummaryToCsccBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Publish a message into given Pub/Sub topic when DlpJob has completed. The message contains a single field, `DlpJobName`, which is equal to the finished job's [`DlpJob.name`](https://cloud.google.com/dlp/docs/reference/rest/v2/projects.dlpJobs#DlpJob). Compatible with: Inspect, Risk"]
    pub struct GooglePrivacyDlpV2PublishToPubSub {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cloud Pub/Sub topic to send notifications to. The topic must have given publishing access rights to the DLP API service account executing the long running DlpJob sending the notifications. Format is projects/{project}/topics/{topic}."]
        pub topic: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2PublishToPubSub {
        pub fn builder() -> GooglePrivacyDlpV2PublishToPubSubBuilder {
            GooglePrivacyDlpV2PublishToPubSubBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Enable Stackdriver metric dlp.googleapis.com/finding_count. This will publish a metric to stack driver on each infotype requested and how many findings were found for it. CustomDetectors will be bucketed as 'Custom' under the Stackdriver label 'info_type'."]
    pub struct GooglePrivacyDlpV2PublishToStackdriver {}
    impl GooglePrivacyDlpV2PublishToStackdriver {
        pub fn builder() -> GooglePrivacyDlpV2PublishToStackdriverBuilder {
            GooglePrivacyDlpV2PublishToStackdriverBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A column with a semantic tag attached."]
    pub struct GooglePrivacyDlpV2QuasiId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A column can be tagged with a custom tag. In this case, the user must indicate an auxiliary table that contains statistical information on the possible values of this column (below)."]
        pub custom_tag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Identifies the column."]
        pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inferred")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If no semantic tag is indicated, we infer the statistical model from the distribution of values in the input data"]
        pub inferred: ::std::option::Option<::std::boxed::Box<GoogleProtobufEmpty>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infoType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A column can be tagged with a InfoType to use the relevant public dataset as a statistical model of population, if available. We currently support US ZIP codes, region codes, ages and genders. To programmatically obtain the list of supported InfoTypes, use ListInfoTypes with the supported_by=RISK_ANALYSIS filter."]
        pub info_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
    }
    impl GooglePrivacyDlpV2QuasiId {
        pub fn builder() -> GooglePrivacyDlpV2QuasiIdBuilder {
            GooglePrivacyDlpV2QuasiIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A quasi-identifier column has a custom_tag, used to know which column in the data corresponds to which column in the statistical model."]
    pub struct GooglePrivacyDlpV2QuasiIdField {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A auxiliary field."]
        pub custom_tag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies the column."]
        pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
    }
    impl GooglePrivacyDlpV2QuasiIdField {
        pub fn builder() -> GooglePrivacyDlpV2QuasiIdFieldBuilder {
            GooglePrivacyDlpV2QuasiIdFieldBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A quasi-identifier column has a custom_tag, used to know which column in the data corresponds to which column in the statistical model."]
    pub struct GooglePrivacyDlpV2QuasiIdentifierField {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A column can be tagged with a custom tag. In this case, the user must indicate an auxiliary table that contains statistical information on the possible values of this column (below)."]
        pub custom_tag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies the column."]
        pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
    }
    impl GooglePrivacyDlpV2QuasiIdentifierField {
        pub fn builder() -> GooglePrivacyDlpV2QuasiIdentifierFieldBuilder {
            GooglePrivacyDlpV2QuasiIdentifierFieldBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message for infoType-dependent details parsed from quote."]
    pub struct GooglePrivacyDlpV2QuoteInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date time indicated by the quote."]
        pub date_time: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DateTime>>,
    }
    impl GooglePrivacyDlpV2QuoteInfo {
        pub fn builder() -> GooglePrivacyDlpV2QuoteInfoBuilder {
            GooglePrivacyDlpV2QuoteInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Generic half-open interval [start, end)"]
    pub struct GooglePrivacyDlpV2Range {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "end")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Index of the last character of the range (exclusive)."]
        pub end: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "start")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Index of the first character of the range (inclusive)."]
        pub start: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2Range {
        pub fn builder() -> GooglePrivacyDlpV2RangeBuilder {
            GooglePrivacyDlpV2RangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A condition for determining whether a transformation should be applied to a field."]
    pub struct GooglePrivacyDlpV2RecordCondition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expressions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An expression."]
        pub expressions: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Expressions>>,
    }
    impl GooglePrivacyDlpV2RecordCondition {
        pub fn builder() -> GooglePrivacyDlpV2RecordConditionBuilder {
            GooglePrivacyDlpV2RecordConditionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message for a unique key indicating a record that contains a finding."]
    pub struct GooglePrivacyDlpV2RecordKey {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bigQueryKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub big_query_key: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryKey>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datastoreKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub datastore_key: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DatastoreKey>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Values of identifying columns in the given row. Order of values matches the order of `identifying_fields` specified in the scanning request."]
        pub id_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GooglePrivacyDlpV2RecordKey {
        pub fn builder() -> GooglePrivacyDlpV2RecordKeyBuilder {
            GooglePrivacyDlpV2RecordKeyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Location of a finding within a row or record."]
    pub struct GooglePrivacyDlpV2RecordLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Field id of the field containing the finding."]
        pub field_id: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recordKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key of the finding."]
        pub record_key: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RecordKey>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location within a `ContentItem.Table`."]
        pub table_location:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TableLocation>>,
    }
    impl GooglePrivacyDlpV2RecordLocation {
        pub fn builder() -> GooglePrivacyDlpV2RecordLocationBuilder {
            GooglePrivacyDlpV2RecordLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration to suppress records whose suppression conditions evaluate to true."]
    pub struct GooglePrivacyDlpV2RecordSuppression {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A condition that when it evaluates to true will result in the record being evaluated to be suppressed from the transformed content."]
        pub condition: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RecordCondition>>,
    }
    impl GooglePrivacyDlpV2RecordSuppression {
        pub fn builder() -> GooglePrivacyDlpV2RecordSuppressionBuilder {
            GooglePrivacyDlpV2RecordSuppressionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A type of transformation that is applied over structured data such as a table."]
    pub struct GooglePrivacyDlpV2RecordTransformations {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldTransformations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transform the record by applying various field transformations."]
        pub field_transformations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2FieldTransformation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recordSuppressions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration defining which records get suppressed entirely. Records that match any suppression rule are omitted from the output."]
        pub record_suppressions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2RecordSuppression>>,
        >,
    }
    impl GooglePrivacyDlpV2RecordTransformations {
        pub fn builder() -> GooglePrivacyDlpV2RecordTransformationsBuilder {
            GooglePrivacyDlpV2RecordTransformationsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Redact a given value. For example, if used with an `InfoTypeTransformation` transforming PHONE_NUMBER, and input 'My phone number is 206-555-0123', the output would be 'My phone number is '."]
    pub struct GooglePrivacyDlpV2RedactConfig {}
    impl GooglePrivacyDlpV2RedactConfig {
        pub fn builder() -> GooglePrivacyDlpV2RedactConfigBuilder {
            GooglePrivacyDlpV2RedactConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to search for potentially sensitive info in an image and redact it by covering it with a colored rectangle."]
    pub struct GooglePrivacyDlpV2RedactImageRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "byteItem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content must be PNG, JPEG, SVG or BMP."]
        pub byte_item: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ByteContentItem>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageRedactionConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The configuration for specifying what content to redact from images."]
        pub image_redaction_configs: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2ImageRedactionConfig>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeFindings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the response should include findings along with the redacted image."]
        pub include_findings: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inspectConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for the inspector."]
        pub inspect_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This field has no effect."]
        pub location_id: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2RedactImageRequest {
        pub fn builder() -> GooglePrivacyDlpV2RedactImageRequestBuilder {
            GooglePrivacyDlpV2RedactImageRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results of redacting an image."]
    pub struct GooglePrivacyDlpV2RedactImageResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "extractedText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If an image was being inspected and the InspectConfig's include_quote was set to true, then this field will include all text, if any, that was found in the image."]
        pub extracted_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inspectResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The findings. Populated when include_findings in the request is true."]
        pub inspect_result:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redactedImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The redacted image. The type will be the same as the original image."]
        pub redacted_image: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2RedactImageResponse {
        pub fn builder() -> GooglePrivacyDlpV2RedactImageResponseBuilder {
            GooglePrivacyDlpV2RedactImageResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message defining a custom regular expression."]
    pub struct GooglePrivacyDlpV2Regex {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupIndexes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the submatch to extract as findings. When not specified, the entire match is returned. No more than 3 may be included."]
        pub group_indexes: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pattern")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pattern defining the regular expression. Its syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
        pub pattern: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2Regex {
        pub fn builder() -> GooglePrivacyDlpV2RegexBuilder {
            GooglePrivacyDlpV2RegexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to re-identify an item."]
    pub struct GooglePrivacyDlpV2ReidentifyContentRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inspectConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for the inspector."]
        pub inspect_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inspectTemplateName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Template to use. Any configuration directly specified in `inspect_config` will override those set in the template. Singular fields that are set in this request will replace their corresponding fields in the template. Repeated fields are appended. Singular sub-messages and groups are recursively merged."]
        pub inspect_template_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "item")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The item to re-identify. Will be treated as text."]
        pub item: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ContentItem>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This field has no effect."]
        pub location_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reidentifyConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for the re-identification of the content item. This field shares the same proto message type that is used for de-identification, however its usage here is for the reversal of the previous de-identification. Re-identification is performed by examining the transformations used to de-identify the items and executing the reverse. This requires that only reversible transformations be provided here. The reversible transformations are: - `CryptoDeterministicConfig` - `CryptoReplaceFfxFpeConfig`"]
        pub reidentify_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DeidentifyConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reidentifyTemplateName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Template to use. References an instance of `DeidentifyTemplate`. Any configuration directly specified in `reidentify_config` or `inspect_config` will override those set in the template. The `DeidentifyTemplate` used must include only reversible transformations. Singular fields that are set in this request will replace their corresponding fields in the template. Repeated fields are appended. Singular sub-messages and groups are recursively merged."]
        pub reidentify_template_name: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2ReidentifyContentRequest {
        pub fn builder() -> GooglePrivacyDlpV2ReidentifyContentRequestBuilder {
            GooglePrivacyDlpV2ReidentifyContentRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results of re-identifying a item."]
    pub struct GooglePrivacyDlpV2ReidentifyContentResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "item")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The re-identified item."]
        pub item: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ContentItem>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "overview")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An overview of the changes that were made to the `item`."]
        pub overview:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TransformationOverview>>,
    }
    impl GooglePrivacyDlpV2ReidentifyContentResponse {
        pub fn builder() -> GooglePrivacyDlpV2ReidentifyContentResponseBuilder {
            GooglePrivacyDlpV2ReidentifyContentResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Replace each input value with a given `Value`."]
    pub struct GooglePrivacyDlpV2ReplaceValueConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value to replace it with."]
        pub new_value: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
    }
    impl GooglePrivacyDlpV2ReplaceValueConfig {
        pub fn builder() -> GooglePrivacyDlpV2ReplaceValueConfigBuilder {
            GooglePrivacyDlpV2ReplaceValueConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Replace each matching finding with the name of the info_type."]
    pub struct GooglePrivacyDlpV2ReplaceWithInfoTypeConfig {}
    impl GooglePrivacyDlpV2ReplaceWithInfoTypeConfig {
        pub fn builder() -> GooglePrivacyDlpV2ReplaceWithInfoTypeConfigBuilder {
            GooglePrivacyDlpV2ReplaceWithInfoTypeConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Snapshot of the inspection configuration."]
    pub struct GooglePrivacyDlpV2RequestedOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Inspect config."]
        pub job_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectJobConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snapshotInspectTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If run with an InspectTemplate, a snapshot of its state at the time of this run."]
        pub snapshot_inspect_template:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectTemplate>>,
    }
    impl GooglePrivacyDlpV2RequestedOptions {
        pub fn builder() -> GooglePrivacyDlpV2RequestedOptionsBuilder {
            GooglePrivacyDlpV2RequestedOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Risk analysis options."]
    pub struct GooglePrivacyDlpV2RequestedRiskAnalysisOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The job config for the risk job."]
        pub job_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RiskAnalysisJobConfig>>,
    }
    impl GooglePrivacyDlpV2RequestedRiskAnalysisOptions {
        pub fn builder() -> GooglePrivacyDlpV2RequestedRiskAnalysisOptionsBuilder {
            GooglePrivacyDlpV2RequestedRiskAnalysisOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "All result fields mentioned below are updated while the job is processing."]
    pub struct GooglePrivacyDlpV2Result {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hybridStats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Statistics related to the processing of hybrid inspect."]
        pub hybrid_stats:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2HybridInspectStatistics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infoTypeStats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Statistics of how many instances of each info type were found during inspect job."]
        pub info_type_stats: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InfoTypeStats>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processedBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total size in bytes that were processed."]
        pub processed_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalEstimatedBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Estimate of the number of bytes to process."]
        pub total_estimated_bytes: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2Result {
        pub fn builder() -> GooglePrivacyDlpV2ResultBuilder {
            GooglePrivacyDlpV2ResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for a risk analysis job. See https://cloud.google.com/dlp/docs/concepts-risk-analysis to learn more."]
    pub struct GooglePrivacyDlpV2RiskAnalysisJobConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Actions to execute at the completion of the job. Are executed in the order provided."]
        pub actions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Action>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privacyMetric")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Privacy metric to compute."]
        pub privacy_metric:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PrivacyMetric>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input dataset to compute metrics over."]
        pub source_table: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryTable>>,
    }
    impl GooglePrivacyDlpV2RiskAnalysisJobConfig {
        pub fn builder() -> GooglePrivacyDlpV2RiskAnalysisJobConfigBuilder {
            GooglePrivacyDlpV2RiskAnalysisJobConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Values of the row."]
    pub struct GooglePrivacyDlpV2Row {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Individual cells."]
        pub values:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Value>>>,
    }
    impl GooglePrivacyDlpV2Row {
        pub fn builder() -> GooglePrivacyDlpV2RowBuilder {
            GooglePrivacyDlpV2RowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "If set, the detailed findings will be persisted to the specified OutputStorageConfig. Only a single instance of this action can be specified. Compatible with: Inspect, Risk"]
    pub struct GooglePrivacyDlpV2SaveFindings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location to store findings outside of DLP."]
        pub output_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2OutputStorageConfig>>,
    }
    impl GooglePrivacyDlpV2SaveFindings {
        pub fn builder() -> GooglePrivacyDlpV2SaveFindingsBuilder {
            GooglePrivacyDlpV2SaveFindingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Schedule for inspect job triggers."]
    pub struct GooglePrivacyDlpV2Schedule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recurrencePeriodDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "With this option a job is started a regular periodic basis. For example: every day (86400 seconds). A scheduled start time will be skipped if the previous execution has not ended when its scheduled time occurs. This value must be set to a time duration greater than or equal to 1 day and can be no longer than 60 days."]
        pub recurrence_period_duration: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2Schedule {
        pub fn builder() -> GooglePrivacyDlpV2ScheduleBuilder {
            GooglePrivacyDlpV2ScheduleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An auxiliary table containing statistical information on the relative frequency of different quasi-identifiers values. It has one or several quasi-identifiers columns, and one column that indicates the relative frequency of each quasi-identifier tuple. If a tuple is present in the data but not in the auxiliary table, the corresponding relative frequency is assumed to be zero (and thus, the tuple is highly reidentifiable)."]
    pub struct GooglePrivacyDlpV2StatisticalTable {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quasiIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Quasi-identifier columns."]
        pub quasi_ids: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2QuasiIdentifierField>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relativeFrequency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The relative frequency column must contain a floating-point number between 0 and 1 (inclusive). Null values are assumed to be zero."]
        pub relative_frequency: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "table")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Auxiliary table location."]
        pub table: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryTable>>,
    }
    impl GooglePrivacyDlpV2StatisticalTable {
        pub fn builder() -> GooglePrivacyDlpV2StatisticalTableBuilder {
            GooglePrivacyDlpV2StatisticalTableBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Shared message indicating Cloud storage type."]
    pub struct GooglePrivacyDlpV2StorageConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bigQueryOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "BigQuery options."]
        pub big_query_options:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudStorageOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage options."]
        pub cloud_storage_options:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CloudStorageOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datastoreOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Datastore options."]
        pub datastore_options:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DatastoreOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hybridOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hybrid inspection options."]
        pub hybrid_options:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2HybridOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timespanConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub timespan_config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TimespanConfig>>,
    }
    impl GooglePrivacyDlpV2StorageConfig {
        pub fn builder() -> GooglePrivacyDlpV2StorageConfigBuilder {
            GooglePrivacyDlpV2StorageConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Storage metadata label to indicate which metadata entry contains findings."]
    pub struct GooglePrivacyDlpV2StorageMetadataLabel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub key: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2StorageMetadataLabel {
        pub fn builder() -> GooglePrivacyDlpV2StorageMetadataLabelBuilder {
            GooglePrivacyDlpV2StorageMetadataLabelBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "StoredInfoType resource message that contains information about the current version and any pending updates."]
    pub struct GooglePrivacyDlpV2StoredInfoType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Current version of the stored info type."]
        pub current_version:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2StoredInfoTypeVersion>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pendingVersions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pending versions of the stored info type. Empty if no versions are pending."]
        pub pending_versions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2StoredInfoTypeVersion>>,
        >,
    }
    impl GooglePrivacyDlpV2StoredInfoType {
        pub fn builder() -> GooglePrivacyDlpV2StoredInfoTypeBuilder {
            GooglePrivacyDlpV2StoredInfoTypeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for stored infoTypes. All fields and subfield are provided by the user. For more information, see https://cloud.google.com/dlp/docs/creating-custom-infotypes."]
    pub struct GooglePrivacyDlpV2StoredInfoTypeConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the StoredInfoType (max 256 characters)."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dictionary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Store dictionary-based CustomInfoType."]
        pub dictionary: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Dictionary>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display name of the StoredInfoType (max 256 characters)."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "largeCustomDictionary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "StoredInfoType where findings are defined by a dictionary of phrases."]
        pub large_custom_dictionary:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2LargeCustomDictionaryConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Store regular expression-based StoredInfoType."]
        pub regex: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Regex>>,
    }
    impl GooglePrivacyDlpV2StoredInfoTypeConfig {
        pub fn builder() -> GooglePrivacyDlpV2StoredInfoTypeConfigBuilder {
            GooglePrivacyDlpV2StoredInfoTypeConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Statistics for a StoredInfoType."]
    pub struct GooglePrivacyDlpV2StoredInfoTypeStats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "largeCustomDictionary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "StoredInfoType where findings are defined by a dictionary of phrases."]
        pub large_custom_dictionary:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2LargeCustomDictionaryStats>>,
    }
    impl GooglePrivacyDlpV2StoredInfoTypeStats {
        pub fn builder() -> GooglePrivacyDlpV2StoredInfoTypeStatsBuilder {
            GooglePrivacyDlpV2StoredInfoTypeStatsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Version of a StoredInfoType, including the configuration used to build it, create timestamp, and current state."]
    pub struct GooglePrivacyDlpV2StoredInfoTypeVersion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "config")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "StoredInfoType configuration."]
        pub config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2StoredInfoTypeConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Create timestamp of the version. Read-only, determined by the system when the version is created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Errors that occurred when creating this storedInfoType version, or anomalies detected in the storedInfoType data that render it unusable. Only the five most recent errors will be displayed, with the most recent error appearing first. For example, some of the data for stored custom dictionaries is put in the user's Google Cloud Storage bucket, and if this data is modified or deleted by the user or another system, the dictionary becomes invalid. If any errors occur, fix the problem indicated by the error message and use the UpdateStoredInfoType API method to create another version of the storedInfoType to continue using it, reusing the same `config` if it was not the source of the error."]
        pub errors:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Error>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Stored info type version state. Read-only, updated by the system during dictionary creation."]
        pub state: ::std::option::Option<GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Statistics about this storedInfoType version."]
        pub stats: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2StoredInfoTypeStats>>,
    }
    impl GooglePrivacyDlpV2StoredInfoTypeVersion {
        pub fn builder() -> GooglePrivacyDlpV2StoredInfoTypeVersionBuilder {
            GooglePrivacyDlpV2StoredInfoTypeVersionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Stored info type version state. Read-only, updated by the system during dictionary creation."]
    pub enum GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum {
        #[serde(rename = "STORED_INFO_TYPE_STATE_UNSPECIFIED")]
        #[doc = "Unused"]
        StoredInfoTypeStateUnspecified,
        #[serde(rename = "PENDING")]
        #[doc = "StoredInfoType version is being created."]
        Pending,
        #[serde(rename = "READY")]
        #[doc = "StoredInfoType version is ready for use."]
        Ready,
        #[serde(rename = "FAILED")]
        #[doc = "StoredInfoType creation failed. All relevant error messages are returned in the `StoredInfoTypeVersion` message."]
        Failed,
        #[serde(rename = "INVALID")]
        #[doc = "StoredInfoType is no longer valid because artifacts stored in user-controlled storage were modified. To fix an invalid StoredInfoType, use the `UpdateStoredInfoType` method to create a new version."]
        Invalid,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum {
        fn default() -> Self {
            Self::StoredInfoTypeStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A reference to a StoredInfoType to use with scanning."]
    pub struct GooglePrivacyDlpV2StoredType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp indicating when the version of the `StoredInfoType` used for inspection was created. Output-only field, populated by the system."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of the requested `StoredInfoType`, for example `organizations/433245324/storedInfoTypes/432452342` or `projects/project-id/storedInfoTypes/432452342`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2StoredType {
        pub fn builder() -> GooglePrivacyDlpV2StoredTypeBuilder {
            GooglePrivacyDlpV2StoredTypeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A collection that informs the user the number of times a particular `TransformationResultCode` and error details occurred."]
    pub struct GooglePrivacyDlpV2SummaryResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Outcome of the transformation."]
        pub code: ::std::option::Option<GooglePrivacyDlpV2SummaryResultCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of transformations counted by this result."]
        pub count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "details")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A place for warnings or errors to show up if a transformation didn't work as expected."]
        pub details: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2SummaryResult {
        pub fn builder() -> GooglePrivacyDlpV2SummaryResultBuilder {
            GooglePrivacyDlpV2SummaryResultBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Outcome of the transformation."]
    pub enum GooglePrivacyDlpV2SummaryResultCodeEnum {
        #[serde(rename = "TRANSFORMATION_RESULT_CODE_UNSPECIFIED")]
        #[doc = "Unused"]
        TransformationResultCodeUnspecified,
        #[serde(rename = "SUCCESS")]
        #[doc = "Transformation completed without an error."]
        Success,
        #[serde(rename = "ERROR")]
        #[doc = "Transformation had an error."]
        Error,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2SummaryResultCodeEnum {
        fn default() -> Self {
            Self::TransformationResultCodeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message for detecting output from deidentification transformations such as [`CryptoReplaceFfxFpeConfig`](https://cloud.google.com/dlp/docs/reference/rest/v2/organizations.deidentifyTemplates#cryptoreplaceffxfpeconfig). These types of transformations are those that perform pseudonymization, thereby producing a \"surrogate\" as output. This should be used in conjunction with a field on the transformation such as `surrogate_info_type`. This CustomInfoType does not support the use of `detection_rules`."]
    pub struct GooglePrivacyDlpV2SurrogateType {}
    impl GooglePrivacyDlpV2SurrogateType {
        pub fn builder() -> GooglePrivacyDlpV2SurrogateTypeBuilder {
            GooglePrivacyDlpV2SurrogateTypeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Structured content to inspect. Up to 50,000 `Value`s per request allowed. See https://cloud.google.com/dlp/docs/inspecting-text#inspecting_a_table to learn more."]
    pub struct GooglePrivacyDlpV2Table {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Headers of the table."]
        pub headers:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rows of the table."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Row>>>,
    }
    impl GooglePrivacyDlpV2Table {
        pub fn builder() -> GooglePrivacyDlpV2TableBuilder {
            GooglePrivacyDlpV2TableBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Location of a finding within a table."]
    pub struct GooglePrivacyDlpV2TableLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zero-based index of the row where the finding is located. Only populated for resources that have a natural ordering, not BigQuery. In BigQuery, to identify the row a finding came from, populate BigQueryOptions.identifying_fields with your primary key column names and when you store the findings the value of those columns will be stored inside of Finding."]
        pub row_index: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2TableLocation {
        pub fn builder() -> GooglePrivacyDlpV2TableLocationBuilder {
            GooglePrivacyDlpV2TableLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Instructions regarding the table content being inspected."]
    pub struct GooglePrivacyDlpV2TableOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "identifyingFields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The columns that are the primary keys for table objects included in ContentItem. A copy of this cell's value will stored alongside alongside each finding so that the finding can be traced to the specific row it came from. No more than 3 may be provided."]
        pub identifying_fields:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>>,
    }
    impl GooglePrivacyDlpV2TableOptions {
        pub fn builder() -> GooglePrivacyDlpV2TableOptionsBuilder {
            GooglePrivacyDlpV2TableOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A column with a semantic tag attached."]
    pub struct GooglePrivacyDlpV2TaggedField {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A column can be tagged with a custom tag. In this case, the user must indicate an auxiliary table that contains statistical information on the possible values of this column (below)."]
        pub custom_tag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Identifies the column."]
        pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inferred")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If no semantic tag is indicated, we infer the statistical model from the distribution of values in the input data"]
        pub inferred: ::std::option::Option<::std::boxed::Box<GoogleProtobufEmpty>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infoType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A column can be tagged with a InfoType to use the relevant public dataset as a statistical model of population, if available. We currently support US ZIP codes, region codes, ages and genders. To programmatically obtain the list of supported InfoTypes, use ListInfoTypes with the supported_by=RISK_ANALYSIS filter."]
        pub info_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
    }
    impl GooglePrivacyDlpV2TaggedField {
        pub fn builder() -> GooglePrivacyDlpV2TaggedFieldBuilder {
            GooglePrivacyDlpV2TaggedFieldBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Throw an error and fail the request when a transformation error occurs."]
    pub struct GooglePrivacyDlpV2ThrowError {}
    impl GooglePrivacyDlpV2ThrowError {
        pub fn builder() -> GooglePrivacyDlpV2ThrowErrorBuilder {
            GooglePrivacyDlpV2ThrowErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "For use with `Date`, `Timestamp`, and `TimeOfDay`, extract or preserve a portion of the value."]
    pub struct GooglePrivacyDlpV2TimePartConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partToExtract")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The part of the time to keep."]
        pub part_to_extract:
            ::std::option::Option<GooglePrivacyDlpV2TimePartConfigPartToExtractEnum>,
    }
    impl GooglePrivacyDlpV2TimePartConfig {
        pub fn builder() -> GooglePrivacyDlpV2TimePartConfigBuilder {
            GooglePrivacyDlpV2TimePartConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The part of the time to keep."]
    pub enum GooglePrivacyDlpV2TimePartConfigPartToExtractEnum {
        #[serde(rename = "TIME_PART_UNSPECIFIED")]
        #[doc = "Unused"]
        TimePartUnspecified,
        #[serde(rename = "YEAR")]
        #[doc = "[0-9999]"]
        Year,
        #[serde(rename = "MONTH")]
        #[doc = "[1-12]"]
        Month,
        #[serde(rename = "DAY_OF_MONTH")]
        #[doc = "[1-31]"]
        DayOfMonth,
        #[serde(rename = "DAY_OF_WEEK")]
        #[doc = "[1-7]"]
        DayOfWeek,
        #[serde(rename = "WEEK_OF_YEAR")]
        #[doc = "[1-53]"]
        WeekOfYear,
        #[serde(rename = "HOUR_OF_DAY")]
        #[doc = "[0-23]"]
        HourOfDay,
    }
    impl ::std::default::Default for GooglePrivacyDlpV2TimePartConfigPartToExtractEnum {
        fn default() -> Self {
            Self::TimePartUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Time zone of the date time object."]
    pub struct GooglePrivacyDlpV2TimeZone {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offsetMinutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set only if the offset can be determined. Positive for time ahead of UTC. E.g. For \"UTC-9\", this value is -540."]
        pub offset_minutes: ::std::option::Option<::std::primitive::i64>,
    }
    impl GooglePrivacyDlpV2TimeZone {
        pub fn builder() -> GooglePrivacyDlpV2TimeZoneBuilder {
            GooglePrivacyDlpV2TimeZoneBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration of the timespan of the items to include in scanning. Currently only supported when inspecting Google Cloud Storage and BigQuery."]
    pub struct GooglePrivacyDlpV2TimespanConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableAutoPopulationOfTimespanConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the job is started by a JobTrigger we will automatically figure out a valid start_time to avoid scanning files that have not been modified since the last time the JobTrigger executed. This will be based on the time of the execution of the last run of the JobTrigger."]
        pub enable_auto_population_of_timespan_config:
            ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Exclude files, tables, or rows newer than this value. If not set, no upper time limit is applied."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Exclude files, tables, or rows older than this value. If not set, no lower time limit is applied."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestampField")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specification of the field containing the timestamp of scanned items. Used for data sources like Datastore and BigQuery. For BigQuery: If this value is not specified and the table was modified between the given start and end times, the entire table will be scanned. If this value is specified, then rows are filtered based on the given start and end times. Rows with a `NULL` value in the provided BigQuery column are skipped. Valid data types of the provided BigQuery column are: `INTEGER`, `DATE`, `TIMESTAMP`, and `DATETIME`. For Datastore: If this value is specified, then entities are filtered based on the given start and end times. If an entity does not contain the provided timestamp property or contains empty or invalid values, then it is included. Valid data types of the provided timestamp property are: `TIMESTAMP`."]
        pub timestamp_field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
    }
    impl GooglePrivacyDlpV2TimespanConfig {
        pub fn builder() -> GooglePrivacyDlpV2TimespanConfigBuilder {
            GooglePrivacyDlpV2TimespanConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "How to handle transformation errors during de-identification. A transformation error occurs when the requested transformation is incompatible with the data. For example, trying to de-identify an IP address using a `DateShift` transformation would result in a transformation error, since date info cannot be extracted from an IP address. Information about any incompatible transformations, and how they were handled, is returned in the response as part of the `TransformationOverviews`."]
    pub struct GooglePrivacyDlpV2TransformationErrorHandling {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "leaveUntransformed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ignore errors"]
        pub leave_untransformed:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2LeaveUntransformed>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "throwError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Throw an error"]
        pub throw_error: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ThrowError>>,
    }
    impl GooglePrivacyDlpV2TransformationErrorHandling {
        pub fn builder() -> GooglePrivacyDlpV2TransformationErrorHandlingBuilder {
            GooglePrivacyDlpV2TransformationErrorHandlingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Overview of the modifications that occurred."]
    pub struct GooglePrivacyDlpV2TransformationOverview {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transformationSummaries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transformations applied to the dataset."]
        pub transformation_summaries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2TransformationSummary>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transformedBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total size in bytes that were transformed in some way."]
        pub transformed_bytes: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2TransformationOverview {
        pub fn builder() -> GooglePrivacyDlpV2TransformationOverviewBuilder {
            GooglePrivacyDlpV2TransformationOverviewBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Summary of a single transformation. Only one of 'transformation', 'field_transformation', or 'record_suppress' will be set."]
    pub struct GooglePrivacyDlpV2TransformationSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set if the transformation was limited to a specific FieldId."]
        pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldTransformations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The field transformation that was applied. If multiple field transformations are requested for a single field, this list will contain all of them; otherwise, only one is supplied."]
        pub field_transformations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2FieldTransformation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infoType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set if the transformation was limited to a specific InfoType."]
        pub info_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recordSuppress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The specific suppression option these stats apply to."]
        pub record_suppress:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RecordSuppression>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Collection of all transformations that took place or had an error."]
        pub results: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2SummaryResult>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transformation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The specific transformation these stats apply to."]
        pub transformation:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PrimitiveTransformation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transformedBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total size in bytes that were transformed in some way."]
        pub transformed_bytes: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2TransformationSummary {
        pub fn builder() -> GooglePrivacyDlpV2TransformationSummaryBuilder {
            GooglePrivacyDlpV2TransformationSummaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Use this to have a random data crypto key generated. It will be discarded after the request finishes."]
    pub struct GooglePrivacyDlpV2TransientCryptoKey {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Name of the key. This is an arbitrary string used to differentiate different keys. A unique key is generated per name: two separate `TransientCryptoKey` protos share the same generated key if their names are the same. When the data crypto key is generated, this name is not used in any way (repeating the api call will result in a different key being generated)."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2TransientCryptoKey {
        pub fn builder() -> GooglePrivacyDlpV2TransientCryptoKeyBuilder {
            GooglePrivacyDlpV2TransientCryptoKeyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "What event needs to occur for a new job to be started."]
    pub struct GooglePrivacyDlpV2Trigger {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manual")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For use with hybrid jobs. Jobs must be manually created and finished."]
        pub manual: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Manual>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schedule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Create a job on a repeating basis based on the elapse of time."]
        pub schedule: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Schedule>>,
    }
    impl GooglePrivacyDlpV2Trigger {
        pub fn builder() -> GooglePrivacyDlpV2TriggerBuilder {
            GooglePrivacyDlpV2TriggerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Using raw keys is prone to security risks due to accidentally leaking the key. Choose another type of key if possible."]
    pub struct GooglePrivacyDlpV2UnwrappedCryptoKey {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A 128/192/256 bit key."]
        pub key: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2UnwrappedCryptoKey {
        pub fn builder() -> GooglePrivacyDlpV2UnwrappedCryptoKeyBuilder {
            GooglePrivacyDlpV2UnwrappedCryptoKeyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for UpdateDeidentifyTemplate."]
    pub struct GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deidentifyTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "New DeidentifyTemplate value."]
        pub deidentify_template:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DeidentifyTemplate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mask to control which fields get updated."]
        pub update_mask: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest {
        pub fn builder() -> GooglePrivacyDlpV2UpdateDeidentifyTemplateRequestBuilder {
            GooglePrivacyDlpV2UpdateDeidentifyTemplateRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for UpdateInspectTemplate."]
    pub struct GooglePrivacyDlpV2UpdateInspectTemplateRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inspectTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "New InspectTemplate value."]
        pub inspect_template:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectTemplate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mask to control which fields get updated."]
        pub update_mask: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2UpdateInspectTemplateRequest {
        pub fn builder() -> GooglePrivacyDlpV2UpdateInspectTemplateRequestBuilder {
            GooglePrivacyDlpV2UpdateInspectTemplateRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for UpdateJobTrigger."]
    pub struct GooglePrivacyDlpV2UpdateJobTriggerRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobTrigger")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "New JobTrigger value."]
        pub job_trigger: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2JobTrigger>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mask to control which fields get updated."]
        pub update_mask: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2UpdateJobTriggerRequest {
        pub fn builder() -> GooglePrivacyDlpV2UpdateJobTriggerRequestBuilder {
            GooglePrivacyDlpV2UpdateJobTriggerRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for UpdateStoredInfoType."]
    pub struct GooglePrivacyDlpV2UpdateStoredInfoTypeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "config")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updated configuration for the storedInfoType. If not provided, a new version of the storedInfoType will be created with the existing configuration."]
        pub config:
            ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2StoredInfoTypeConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mask to control which fields get updated."]
        pub update_mask: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2UpdateStoredInfoTypeRequest {
        pub fn builder() -> GooglePrivacyDlpV2UpdateStoredInfoTypeRequestBuilder {
            GooglePrivacyDlpV2UpdateStoredInfoTypeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of primitive values supported by the system. Note that for the purposes of inspection or transformation, the number of bytes considered to comprise a 'Value' is based on its representation as a UTF-8 encoded string. For example, if 'integer_value' is set to 123456789, the number of bytes would be counted as 9, even though an int64 only holds up to 8 bytes of data."]
    pub struct GooglePrivacyDlpV2Value {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "booleanValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "boolean"]
        pub boolean_value: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "date"]
        pub date_value: ::std::option::Option<::std::boxed::Box<GoogleTypeDate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dayOfWeekValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "day of week"]
        pub day_of_week_value: ::std::option::Option<GooglePrivacyDlpV2ValueDayOfWeekValueEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "floatValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "float"]
        pub float_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "integerValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "integer"]
        pub integer_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "string"]
        pub string_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "time of day"]
        pub time_value: ::std::option::Option<::std::boxed::Box<GoogleTypeTimeOfDay>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestampValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "timestamp"]
        pub timestamp_value: ::std::option::Option<::std::string::String>,
    }
    impl GooglePrivacyDlpV2Value {
        pub fn builder() -> GooglePrivacyDlpV2ValueBuilder {
            GooglePrivacyDlpV2ValueBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "day of week"]
    pub enum GooglePrivacyDlpV2ValueDayOfWeekValueEnum {
        #[serde(rename = "DAY_OF_WEEK_UNSPECIFIED")]
        #[doc = "The day of the week is unspecified."]
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
    impl ::std::default::Default for GooglePrivacyDlpV2ValueDayOfWeekValueEnum {
        fn default() -> Self {
            Self::DayOfWeekUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A value of a field, including its frequency."]
    pub struct GooglePrivacyDlpV2ValueFrequency {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How many times the value is contained in the field."]
        pub count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A value contained in the field in question."]
        pub value: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
    }
    impl GooglePrivacyDlpV2ValueFrequency {
        pub fn builder() -> GooglePrivacyDlpV2ValueFrequencyBuilder {
            GooglePrivacyDlpV2ValueFrequencyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message defining a list of words or phrases to search for in the data."]
    pub struct GooglePrivacyDlpV2WordList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "words")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Words or phrases defining the dictionary. The dictionary must contain at least one phrase and every phrase must contain at least 2 characters that are letters or digits. [required]"]
        pub words: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GooglePrivacyDlpV2WordList {
        pub fn builder() -> GooglePrivacyDlpV2WordListBuilder {
            GooglePrivacyDlpV2WordListBuilder::default()
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
    #[doc = "Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."]
    pub struct GoogleTypeDate {
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
    impl GoogleTypeDate {
        pub fn builder() -> GoogleTypeDateBuilder {
            GoogleTypeDateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`."]
    pub struct GoogleTypeTimeOfDay {
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
    impl GoogleTypeTimeOfDay {
        pub fn builder() -> GoogleTypeTimeOfDayBuilder {
            GoogleTypeTimeOfDayBuilder::default()
        }
    }
}
