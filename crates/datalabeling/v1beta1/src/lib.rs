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
            pub mod annotation_spec_sets {
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
                            #[doc = "Optional. Filter is not supported at this moment."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Requested page size. Server may return fewer results than requested. Default value is 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. A token identifying a page of results for the server to return. Typically obtained by ListAnnotationSpecSetsResponse.next_page_token of the previous [DataLabelingService.ListAnnotationSpecSets] call. Return first page if empty."]
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
            pub mod datasets {
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
                            #[doc = "Optional. Filter on dataset is not supported at this moment."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Requested page size. Server may return fewer results than requested. Default value is 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. A token identifying a page of results for the server to return. Typically obtained by ListDatasetsResponse.next_page_token of the previous [DataLabelingService.ListDatasets] call. Returns the first page if empty."]
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
                    pub mod annotated_datasets {
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
                                    #[doc = "Optional. Filter is not supported at this moment."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. Requested page size. Server may return fewer results than requested. Default value is 100."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. A token identifying a page of results for the server to return. Typically obtained by ListAnnotatedDatasetsResponse.next_page_token of the previous [DataLabelingService.ListAnnotatedDatasets] call. Return first page if empty."]
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
                            pub mod data_items {
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
                                            #[doc = "Optional. Filter is not supported at this moment."]
                                            pub filter:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. Requested page size. Server may return fewer results than requested. Default value is 100."]
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
                                            #[doc = "Optional. A token identifying a page of results for the server to return. Typically obtained by ListDataItemsResponse.next_page_token of the previous [DataLabelingService.ListDataItems] call. Return first page if empty."]
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
                            pub mod examples {
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
                                            #[serde(rename = "filter")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. An expression for filtering Examples. Filter by annotation_spec.display_name is supported. Format \"annotation_spec.display_name = {display_name}\""]
                                            pub filter:
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
                                            #[serde(rename = "filter")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. An expression for filtering Examples. For annotated datasets that have annotation spec set, filter by annotation_spec.display_name is supported. Format \"annotation_spec.display_name = {display_name}\""]
                                            pub filter:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. Requested page size. Server may return fewer results than requested. Default value is 100."]
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
                                            #[doc = "Optional. A token identifying a page of results for the server to return. Typically obtained by ListExamplesResponse.next_page_token of the previous [DataLabelingService.ListExamples] call. Return first page if empty."]
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
                            pub mod feedback_threads {
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
                                            #[doc = "Optional. Requested page size. Server may return fewer results than requested. Default value is 100."]
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
                                            #[doc = "Optional. A token identifying a page of results for the server to return. Typically obtained by ListFeedbackThreads.next_page_token of the previous [DataLabelingService.ListFeedbackThreads] call. Return first page if empty."]
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
                                    pub mod feedback_messages {
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
                                                    #[doc = "Optional. Requested page size. Server may return fewer results than requested. Default value is 100."]
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
                                                    #[doc = "Optional. A token identifying a page of results for the server to return. Typically obtained by ListFeedbackMessages.next_page_token of the previous [DataLabelingService.ListFeedbackMessages] call. Return first page if empty."]
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
                        }
                    }
                    pub mod data_items {
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
                                    #[doc = "Optional. Filter is not supported at this moment."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. Requested page size. Server may return fewer results than requested. Default value is 100."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. A token identifying a page of results for the server to return. Typically obtained by ListDataItemsResponse.next_page_token of the previous [DataLabelingService.ListDataItems] call. Return first page if empty."]
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
            pub mod evaluation_jobs {
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
                            #[doc = "Optional. You can filter the jobs to list by model_id (also known as model_name, as described in EvaluationJob.modelVersion) or by evaluation job state (as described in EvaluationJob.state). To filter by both criteria, use the `AND` operator or the `OR` operator. For example, you can use the following string for your filter: \"evaluation_job.model_id = {model_name} AND evaluation_job.state = {evaluation_job_state}\""]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Requested page size. Server may return fewer results than requested. Default value is 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. A token identifying a page of results for the server to return. Typically obtained by the nextPageToken in the response to the previous request. The request returns the first page if this is empty."]
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
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Mask for which fields to update. You can only provide the following fields: * `evaluationJobConfig.humanAnnotationConfig.instruction` * `evaluationJobConfig.exampleCount` * `evaluationJobConfig.exampleSamplePercentage` You can provide more than one of these fields by separating them with commas."]
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
            pub mod evaluations {
                pub mod methods {
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
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. To search evaluations, you can filter by the following: * evaluation_job.evaluation_job_id (the last part of EvaluationJob.name) * evaluation_job.model_id (the {model_name} portion of EvaluationJob.modelVersion) * evaluation_job.evaluation_job_run_time_start (Minimum threshold for the evaluationJobRunTime that created the evaluation) * evaluation_job.evaluation_job_run_time_end (Maximum threshold for the evaluationJobRunTime that created the evaluation) * evaluation_job.job_state (EvaluationJob.state) * annotation_spec.display_name (the Evaluation contains a metric for the annotation spec with this displayName) To filter by multiple critiera, use the `AND` operator or the `OR` operator. The following examples shows a string that filters by several critiera: \"evaluation_job.evaluation_job_id = {evaluation_job_id} AND evaluation_job.model_id = {model_name} AND evaluation_job.evaluation_job_run_time_start = {timestamp_1} AND evaluation_job.evaluation_job_run_time_end = {timestamp_2} AND annotation_spec.display_name = {display_name}\""]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Requested page size. Server may return fewer results than requested. Default value is 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. A token identifying a page of results for the server to return. Typically obtained by the nextPageToken of the response to a previous search request. If you don't specify this field, the API call requests the first page of the search."]
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
            pub mod instructions {
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
                            #[doc = "Optional. Filter is not supported at this moment."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Requested page size. Server may return fewer results than requested. Default value is 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. A token identifying a page of results for the server to return. Typically obtained by ListInstructionsResponse.next_page_token of the previous [DataLabelingService.ListInstructions] call. Return first page if empty."]
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
    #[doc = "Metadata of a CreateInstruction operation."]
    pub struct GoogleCloudDatalabelingV1alpha1CreateInstructionMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp when create instruction request was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instruction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the created Instruction. projects/{project_id}/instructions/{instruction_id}"]
        pub instruction: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Partial failures encountered. E.g. single files that couldn't be read. Status details field will contain standard GCP error details."]
        pub partial_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    }
    impl GoogleCloudDatalabelingV1alpha1CreateInstructionMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1alpha1CreateInstructionMetadataBuilder {
            GoogleCloudDatalabelingV1alpha1CreateInstructionMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of an ExportData operation."]
    pub struct GoogleCloudDatalabelingV1alpha1ExportDataOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of annotated dataset in format \"projects/*/datasets/*/annotatedDatasets/*\"."]
        pub annotated_dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp when export dataset request was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of dataset to be exported. \"projects/*/datasets/*\""]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Partial failures encountered. E.g. single files that couldn't be read. Status details field will contain standard GCP error details."]
        pub partial_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    }
    impl GoogleCloudDatalabelingV1alpha1ExportDataOperationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1alpha1ExportDataOperationMetadataBuilder {
            GoogleCloudDatalabelingV1alpha1ExportDataOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response used for ExportDataset longrunning operation."]
    pub struct GoogleCloudDatalabelingV1alpha1ExportDataOperationResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of annotated dataset in format \"projects/*/datasets/*/annotatedDatasets/*\"."]
        pub annotated_dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ouptut only. The name of dataset. \"projects/*/datasets/*\""]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exportCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Number of examples exported successfully."]
        pub export_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelStats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Statistic infos of labels in the exported dataset."]
        pub label_stats:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1alpha1LabelStats>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. output_config in the ExportData request."]
        pub output_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1alpha1OutputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Total number of examples requested to export"]
        pub total_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDatalabelingV1alpha1ExportDataOperationResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1alpha1ExportDataOperationResponseBuilder {
            GoogleCloudDatalabelingV1alpha1ExportDataOperationResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Export destination of the data.Only gcs path is allowed in output_uri."]
    pub struct GoogleCloudDatalabelingV1alpha1GcsDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The format of the gcs destination. Only \"text/csv\" and \"application/json\" are supported."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The output uri of destination file."]
        pub output_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1alpha1GcsDestination {
        pub fn builder() -> GoogleCloudDatalabelingV1alpha1GcsDestinationBuilder {
            GoogleCloudDatalabelingV1alpha1GcsDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Export folder destination of the data."]
    pub struct GoogleCloudDatalabelingV1alpha1GcsFolderDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputFolderUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Cloud Storage directory to export data to."]
        pub output_folder_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1alpha1GcsFolderDestination {
        pub fn builder() -> GoogleCloudDatalabelingV1alpha1GcsFolderDestinationBuilder {
            GoogleCloudDatalabelingV1alpha1GcsFolderDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for how human labeling task should be done."]
    pub struct GoogleCloudDatalabelingV1alpha1HumanAnnotationConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDatasetDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A human-readable description for AnnotatedDataset. The description can be up to 10000 characters long."]
        pub annotated_dataset_description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDatasetDisplayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A human-readable name for AnnotatedDataset defined by users. Maximum of 64 characters ."]
        pub annotated_dataset_display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contributorEmails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If you want your own labeling contributors to manage and work on this labeling request, you can set these contributors here. We will give them access to the question types in crowdcompute. Note that these emails must be registered in crowdcompute worker UI: https://crowd-compute.appspot.com/"]
        pub contributor_emails: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instruction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Instruction resource name."]
        pub instruction: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A human-readable label used to logically group labeling tasks. This string must match the regular expression `[a-zA-Z\\\\d_-]{0,128}`."]
        pub label_group: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The Language of this question, as a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). Default value is en-US. Only need to set this when task is language related. For example, French text classification."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "questionDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Maximum duration for contributors to answer a question. Maximum is 3600 seconds. Default is 3600 seconds."]
        pub question_duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replicaCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Replication of questions. Each question will be sent to up to this number of contributors to label. Aggregated answers will be returned. Default is set to 1. For image related labeling, valid values are 1, 3, 5."]
        pub replica_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEmailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email of the user who started the labeling task and should be notified by email. If empty no notification will be sent."]
        pub user_email_address: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1alpha1HumanAnnotationConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1alpha1HumanAnnotationConfigBuilder {
            GoogleCloudDatalabelingV1alpha1HumanAnnotationConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of an ImportData operation."]
    pub struct GoogleCloudDatalabelingV1alpha1ImportDataOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp when import dataset request was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of imported dataset. \"projects/*/datasets/*\""]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Partial failures encountered. E.g. single files that couldn't be read. Status details field will contain standard GCP error details."]
        pub partial_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    }
    impl GoogleCloudDatalabelingV1alpha1ImportDataOperationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1alpha1ImportDataOperationMetadataBuilder {
            GoogleCloudDatalabelingV1alpha1ImportDataOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response used for ImportData longrunning operation."]
    pub struct GoogleCloudDatalabelingV1alpha1ImportDataOperationResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ouptut only. The name of imported dataset."]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Number of examples imported successfully."]
        pub import_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Total number of examples requested to import"]
        pub total_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDatalabelingV1alpha1ImportDataOperationResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1alpha1ImportDataOperationResponseBuilder {
            GoogleCloudDatalabelingV1alpha1ImportDataOperationResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelImageBoundingBox operation metadata."]
    pub struct GoogleCloudDatalabelingV1alpha1LabelImageBoundingBoxOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1alpha1LabelImageBoundingBoxOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1alpha1LabelImageBoundingBoxOperationMetadataBuilder {
            GoogleCloudDatalabelingV1alpha1LabelImageBoundingBoxOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of LabelImageBoundingPoly operation metadata."]
    pub struct GoogleCloudDatalabelingV1alpha1LabelImageBoundingPolyOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1alpha1LabelImageBoundingPolyOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1alpha1LabelImageBoundingPolyOperationMetadataBuilder {
            GoogleCloudDatalabelingV1alpha1LabelImageBoundingPolyOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of a LabelImageClassification operation."]
    pub struct GoogleCloudDatalabelingV1alpha1LabelImageClassificationOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1alpha1LabelImageClassificationOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1alpha1LabelImageClassificationOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1alpha1LabelImageClassificationOperationMetadataBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelImageOrientedBoundingBox operation metadata."]
    pub struct GoogleCloudDatalabelingV1alpha1LabelImageOrientedBoundingBoxOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1alpha1LabelImageOrientedBoundingBoxOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1alpha1LabelImageOrientedBoundingBoxOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1alpha1LabelImageOrientedBoundingBoxOperationMetadataBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of LabelImagePolyline operation metadata."]
    pub struct GoogleCloudDatalabelingV1alpha1LabelImagePolylineOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1alpha1LabelImagePolylineOperationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1alpha1LabelImagePolylineOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1alpha1LabelImagePolylineOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelImageSegmentation operation metadata."]
    pub struct GoogleCloudDatalabelingV1alpha1LabelImageSegmentationOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1alpha1LabelImageSegmentationOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1alpha1LabelImageSegmentationOperationMetadataBuilder {
            GoogleCloudDatalabelingV1alpha1LabelImageSegmentationOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of a labeling operation, such as LabelImage or LabelVideo. Next tag: 23"]
    pub struct GoogleCloudDatalabelingV1alpha1LabelOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of annotated dataset in format \"projects/*/datasets/*/annotatedDatasets/*\"."]
        pub annotated_dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp when labeling request was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of dataset to be labeled. \"projects/*/datasets/*\""]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageBoundingBoxDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image bounding box operation."]
        pub image_bounding_box_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1alpha1LabelImageBoundingBoxOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageBoundingPolyDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image bounding poly operation."]
        pub image_bounding_poly_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1alpha1LabelImageBoundingPolyOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageClassificationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image classification operation."]
        pub image_classification_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1alpha1LabelImageClassificationOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageOrientedBoundingBoxDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image oriented bounding box operation."]
        pub image_oriented_bounding_box_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1alpha1LabelImageOrientedBoundingBoxOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imagePolylineDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image polyline operation."]
        pub image_polyline_details: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1alpha1LabelImagePolylineOperationMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageSegmentationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image segmentation operation."]
        pub image_segmentation_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1alpha1LabelImageSegmentationOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Partial failures encountered. E.g. single files that couldn't be read. Status details field will contain standard GCP error details."]
        pub partial_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressPercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Progress of label operation. Range: [0, 100]."]
        pub progress_percent: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textClassificationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label text classification operation."]
        pub text_classification_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1alpha1LabelTextClassificationOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textEntityExtractionDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label text entity extraction operation."]
        pub text_entity_extraction_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1alpha1LabelTextEntityExtractionOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoClassificationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label video classification operation."]
        pub video_classification_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1alpha1LabelVideoClassificationOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoEventDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label video event operation."]
        pub video_event_details: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1alpha1LabelVideoEventOperationMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoObjectDetectionDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label video object detection operation."]
        pub video_object_detection_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1alpha1LabelVideoObjectDetectionOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoObjectTrackingDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label video object tracking operation."]
        pub video_object_tracking_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1alpha1LabelVideoObjectTrackingOperationMetadata,
            >,
        >,
    }
    impl GoogleCloudDatalabelingV1alpha1LabelOperationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1alpha1LabelOperationMetadataBuilder {
            GoogleCloudDatalabelingV1alpha1LabelOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Statistics about annotation specs."]
    pub struct GoogleCloudDatalabelingV1alpha1LabelStats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exampleCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map of each annotation spec's example count. Key is the annotation spec name and value is the number of examples for that annotation spec. If the annotated dataset does not have annotation spec, the map will return a pair where the key is empty string and value is the total number of annotations."]
        pub example_count:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl GoogleCloudDatalabelingV1alpha1LabelStats {
        pub fn builder() -> GoogleCloudDatalabelingV1alpha1LabelStatsBuilder {
            GoogleCloudDatalabelingV1alpha1LabelStatsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelTextClassification operation metadata."]
    pub struct GoogleCloudDatalabelingV1alpha1LabelTextClassificationOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1alpha1LabelTextClassificationOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1alpha1LabelTextClassificationOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1alpha1LabelTextClassificationOperationMetadataBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelTextEntityExtraction operation metadata."]
    pub struct GoogleCloudDatalabelingV1alpha1LabelTextEntityExtractionOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1alpha1LabelTextEntityExtractionOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1alpha1LabelTextEntityExtractionOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1alpha1LabelTextEntityExtractionOperationMetadataBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelVideoClassification operation metadata."]
    pub struct GoogleCloudDatalabelingV1alpha1LabelVideoClassificationOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1alpha1LabelVideoClassificationOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1alpha1LabelVideoClassificationOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1alpha1LabelVideoClassificationOperationMetadataBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelVideoEvent operation metadata."]
    pub struct GoogleCloudDatalabelingV1alpha1LabelVideoEventOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1alpha1LabelVideoEventOperationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1alpha1LabelVideoEventOperationMetadataBuilder {
            GoogleCloudDatalabelingV1alpha1LabelVideoEventOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelVideoObjectDetection operation metadata."]
    pub struct GoogleCloudDatalabelingV1alpha1LabelVideoObjectDetectionOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1alpha1LabelVideoObjectDetectionOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1alpha1LabelVideoObjectDetectionOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1alpha1LabelVideoObjectDetectionOperationMetadataBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelVideoObjectTracking operation metadata."]
    pub struct GoogleCloudDatalabelingV1alpha1LabelVideoObjectTrackingOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1alpha1LabelVideoObjectTrackingOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1alpha1LabelVideoObjectTrackingOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1alpha1LabelVideoObjectTrackingOperationMetadataBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The configuration of output data."]
    pub struct GoogleCloudDatalabelingV1alpha1OutputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output to a file in Cloud Storage. Should be used for labeling output other than image segmentation."]
        pub gcs_destination:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1alpha1GcsDestination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsFolderDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output to a folder in Cloud Storage. Should be used for image segmentation or document de-identification labeling outputs."]
        pub gcs_folder_destination: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1alpha1GcsFolderDestination>,
        >,
    }
    impl GoogleCloudDatalabelingV1alpha1OutputConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1alpha1OutputConfigBuilder {
            GoogleCloudDatalabelingV1alpha1OutputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "AnnotatedDataset is a set holding annotations for data in a Dataset. Each labeling task will generate an AnnotatedDataset under the Dataset that the task is requested for."]
    pub struct GoogleCloudDatalabelingV1beta1AnnotatedDataset {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Source of the annotation."]
        pub annotation_source: ::std::option::Option<
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationSourceEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Type of the annotation. It is specified when starting labeling task."]
        pub annotation_type:
            ::std::option::Option<GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockingResources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The names of any related resources that are blocking changes to the annotated dataset."]
        pub blocking_resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "completedExampleCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Number of examples that have annotation in the annotated dataset."]
        pub completed_example_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time the AnnotatedDataset was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The description of the AnnotatedDataset. It is specified in HumanAnnotationConfig when user starts a labeling task. Maximum of 10000 characters."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The display name of the AnnotatedDataset. It is specified in HumanAnnotationConfig when user starts a labeling task. Maximum of 64 characters."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exampleCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Number of examples in the annotated dataset."]
        pub example_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelStats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Per label statistics."]
        pub label_stats:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1LabelStats>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Additional information about AnnotatedDataset."]
        pub metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotatedDatasetMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. AnnotatedDataset resource name in format of: projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/ {annotated_dataset_id}"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1AnnotatedDataset {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1AnnotatedDatasetBuilder {
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Source of the annotation."]
    pub enum GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationSourceEnum {
        #[serde(rename = "ANNOTATION_SOURCE_UNSPECIFIED")]
        #[doc = ""]
        AnnotationSourceUnspecified,
        #[serde(rename = "OPERATOR")]
        #[doc = "Answer is provided by a human contributor."]
        Operator,
    }
    impl ::std::default::Default
        for GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationSourceEnum
    {
        fn default() -> Self {
            Self::AnnotationSourceUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Type of the annotation. It is specified when starting labeling task."]
    pub enum GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum {
        #[serde(rename = "ANNOTATION_TYPE_UNSPECIFIED")]
        #[doc = ""]
        AnnotationTypeUnspecified,
        #[serde(rename = "IMAGE_CLASSIFICATION_ANNOTATION")]
        #[doc = "Classification annotations in an image. Allowed for continuous evaluation."]
        ImageClassificationAnnotation,
        #[serde(rename = "IMAGE_BOUNDING_BOX_ANNOTATION")]
        #[doc = "Bounding box annotations in an image. A form of image object detection. Allowed for continuous evaluation."]
        ImageBoundingBoxAnnotation,
        #[serde(rename = "IMAGE_ORIENTED_BOUNDING_BOX_ANNOTATION")]
        #[doc = "Oriented bounding box. The box does not have to be parallel to horizontal line."]
        ImageOrientedBoundingBoxAnnotation,
        #[serde(rename = "IMAGE_BOUNDING_POLY_ANNOTATION")]
        #[doc = "Bounding poly annotations in an image."]
        ImageBoundingPolyAnnotation,
        #[serde(rename = "IMAGE_POLYLINE_ANNOTATION")]
        #[doc = "Polyline annotations in an image."]
        ImagePolylineAnnotation,
        #[serde(rename = "IMAGE_SEGMENTATION_ANNOTATION")]
        #[doc = "Segmentation annotations in an image."]
        ImageSegmentationAnnotation,
        #[serde(rename = "VIDEO_SHOTS_CLASSIFICATION_ANNOTATION")]
        #[doc = "Classification annotations in video shots."]
        VideoShotsClassificationAnnotation,
        #[serde(rename = "VIDEO_OBJECT_TRACKING_ANNOTATION")]
        #[doc = "Video object tracking annotation."]
        VideoObjectTrackingAnnotation,
        #[serde(rename = "VIDEO_OBJECT_DETECTION_ANNOTATION")]
        #[doc = "Video object detection annotation."]
        VideoObjectDetectionAnnotation,
        #[serde(rename = "VIDEO_EVENT_ANNOTATION")]
        #[doc = "Video event annotation."]
        VideoEventAnnotation,
        #[serde(rename = "TEXT_CLASSIFICATION_ANNOTATION")]
        #[doc = "Classification for text. Allowed for continuous evaluation."]
        TextClassificationAnnotation,
        #[serde(rename = "TEXT_ENTITY_EXTRACTION_ANNOTATION")]
        #[doc = "Entity extraction for text."]
        TextEntityExtractionAnnotation,
        #[serde(rename = "GENERAL_CLASSIFICATION_ANNOTATION")]
        #[doc = "General classification. Allowed for continuous evaluation."]
        GeneralClassificationAnnotation,
    }
    impl ::std::default::Default for GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum {
        fn default() -> Self {
            Self::AnnotationTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata on AnnotatedDataset."]
    pub struct GoogleCloudDatalabelingV1beta1AnnotatedDatasetMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPolyConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for image bounding box and bounding poly task."]
        pub bounding_poly_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1BoundingPolyConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for video event labeling task."]
        pub event_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1EventConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "humanAnnotationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HumanAnnotationConfig used when requesting the human labeling task for this AnnotatedDataset."]
        pub human_annotation_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageClassificationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for image classification task."]
        pub image_classification_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1ImageClassificationConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectDetectionConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for video object detection task."]
        pub object_detection_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1ObjectDetectionConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectTrackingConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for video object tracking task."]
        pub object_tracking_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1ObjectTrackingConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "polylineConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for image polyline task."]
        pub polyline_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1PolylineConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segmentationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for image segmentation task."]
        pub segmentation_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1SegmentationConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textClassificationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for text classification task."]
        pub text_classification_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1TextClassificationConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textEntityExtractionConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for text entity extraction task."]
        pub text_entity_extraction_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1TextEntityExtractionConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoClassificationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for video classification task."]
        pub video_classification_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1VideoClassificationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1AnnotatedDatasetMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1AnnotatedDatasetMetadataBuilder {
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation for Example. Each example may have one or more annotations. For example in image classification problem, each image might have one or more labels. We call labels binded with this image an Annotation."]
    pub struct GoogleCloudDatalabelingV1beta1Annotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Annotation metadata, including information like votes for labels."]
        pub annotation_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotationMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSentiment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Sentiment for this annotation."]
        pub annotation_sentiment:
            ::std::option::Option<GoogleCloudDatalabelingV1beta1AnnotationAnnotationSentimentEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The source of the annotation."]
        pub annotation_source:
            ::std::option::Option<GoogleCloudDatalabelingV1beta1AnnotationAnnotationSourceEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. This is the actual annotation value, e.g classification, bounding box values are stored here."]
        pub annotation_value:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotationValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Unique name of this annotation, format is: projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset}/examples/{example_id}/annotations/{annotation_id}"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1Annotation {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1AnnotationBuilder {
            GoogleCloudDatalabelingV1beta1AnnotationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Sentiment for this annotation."]
    pub enum GoogleCloudDatalabelingV1beta1AnnotationAnnotationSentimentEnum {
        #[serde(rename = "ANNOTATION_SENTIMENT_UNSPECIFIED")]
        #[doc = ""]
        AnnotationSentimentUnspecified,
        #[serde(rename = "NEGATIVE")]
        #[doc = "This annotation describes negatively about the data."]
        Negative,
        #[serde(rename = "POSITIVE")]
        #[doc = "This label describes positively about the data."]
        Positive,
    }
    impl ::std::default::Default for GoogleCloudDatalabelingV1beta1AnnotationAnnotationSentimentEnum {
        fn default() -> Self {
            Self::AnnotationSentimentUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The source of the annotation."]
    pub enum GoogleCloudDatalabelingV1beta1AnnotationAnnotationSourceEnum {
        #[serde(rename = "ANNOTATION_SOURCE_UNSPECIFIED")]
        #[doc = ""]
        AnnotationSourceUnspecified,
        #[serde(rename = "OPERATOR")]
        #[doc = "Answer is provided by a human contributor."]
        Operator,
    }
    impl ::std::default::Default for GoogleCloudDatalabelingV1beta1AnnotationAnnotationSourceEnum {
        fn default() -> Self {
            Self::AnnotationSourceUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional information associated with the annotation."]
    pub struct GoogleCloudDatalabelingV1beta1AnnotationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operatorMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata related to human labeling."]
        pub operator_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1OperatorMetadata>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1AnnotationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1AnnotationMetadataBuilder {
            GoogleCloudDatalabelingV1beta1AnnotationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Container of information related to one possible annotation that can be used in a labeling task. For example, an image classification task where images are labeled as `dog` or `cat` must reference an AnnotationSpec for `dog` and an AnnotationSpec for `cat`."]
    pub struct GoogleCloudDatalabelingV1beta1AnnotationSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. User-provided description of the annotation specification. The description can be up to 10,000 characters long."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The display name of the AnnotationSpec. Maximum of 64 characters."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. This is the integer index of the AnnotationSpec. The index for the whole AnnotationSpecSet is sequential starting from 0. For example, an AnnotationSpecSet with classes `dog` and `cat`, might contain one AnnotationSpec with `{ display_name: \"dog\", index: 0 }` and one AnnotationSpec with `{ display_name: \"cat\", index: 1 }`. This is especially useful for model training as it encodes the string labels into numeric values."]
        pub index: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDatalabelingV1beta1AnnotationSpec {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1AnnotationSpecBuilder {
            GoogleCloudDatalabelingV1beta1AnnotationSpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An AnnotationSpecSet is a collection of label definitions. For example, in image classification tasks, you define a set of possible labels for images as an AnnotationSpecSet. An AnnotationSpecSet is immutable upon creation."]
    pub struct GoogleCloudDatalabelingV1beta1AnnotationSpecSet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpecs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The array of AnnotationSpecs that you define when you create the AnnotationSpecSet. These are the possible labels for the labeling task."]
        pub annotation_specs: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotationSpec>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockingResources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The names of any related resources that are blocking changes to the annotation spec set."]
        pub blocking_resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. User-provided description of the annotation specification set. The description can be up to 10,000 characters long."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The display name for AnnotationSpecSet that you define when you create it. Maximum of 64 characters."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The AnnotationSpecSet resource name in the following format: \"projects/{project_id}/annotationSpecSets/{annotation_spec_set_id}\""]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1AnnotationSpecSet {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1AnnotationSpecSetBuilder {
            GoogleCloudDatalabelingV1beta1AnnotationSpecSetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation spec set with the setting of allowing multi labels or not."]
    pub struct GoogleCloudDatalabelingV1beta1AnnotationSpecSetConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowMultiLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If allow_multi_label is true, contributors are able to choose multiple labels from one annotation spec set."]
        pub allow_multi_label: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpecSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Annotation spec set resource name."]
        pub annotation_spec_set: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1AnnotationSpecSetConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1AnnotationSpecSetConfigBuilder {
            GoogleCloudDatalabelingV1beta1AnnotationSpecSetConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation value for an example."]
    pub struct GoogleCloudDatalabelingV1beta1AnnotationValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageBoundingPolyAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotation value for image bounding box, oriented bounding box and polygon cases."]
        pub image_bounding_poly_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1ImageBoundingPolyAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageClassificationAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotation value for image classification case."]
        pub image_classification_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1ImageClassificationAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imagePolylineAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotation value for image polyline cases. Polyline here is different from BoundingPoly. It is formed by line segments connected to each other but not closed form(Bounding Poly). The line segments can cross each other."]
        pub image_polyline_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1ImagePolylineAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageSegmentationAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotation value for image segmentation."]
        pub image_segmentation_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1ImageSegmentationAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textClassificationAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotation value for text classification case."]
        pub text_classification_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1TextClassificationAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textEntityExtractionAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotation value for text entity extraction case."]
        pub text_entity_extraction_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1TextEntityExtractionAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoClassificationAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotation value for video classification case."]
        pub video_classification_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1VideoClassificationAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoEventAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotation value for video event case."]
        pub video_event_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1VideoEventAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoObjectTrackingAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotation value for video object detection and tracking case."]
        pub video_object_tracking_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1VideoObjectTrackingAnnotation>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1AnnotationValue {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1AnnotationValueBuilder {
            GoogleCloudDatalabelingV1beta1AnnotationValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Records a failed evaluation job run."]
    pub struct GoogleCloudDatalabelingV1beta1Attempt {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attemptTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub attempt_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of errors that occurred."]
        pub partial_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    }
    impl GoogleCloudDatalabelingV1beta1Attempt {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1AttemptBuilder {
            GoogleCloudDatalabelingV1beta1AttemptBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The BigQuery location for input data. If used in an EvaluationJob, this is where the service saves the prediction input and output sampled from the model version."]
    pub struct GoogleCloudDatalabelingV1beta1BigQuerySource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. BigQuery URI to a table, up to 2,000 characters long. If you specify the URI of a table that does not exist, Data Labeling Service creates a table at the URI with the correct schema when you create your EvaluationJob. If you specify the URI of a table that already exists, it must have the [correct schema](/ml-engine/docs/continuous-evaluation/create-job#table-schema). Provide the table URI in the following format: \"bq://{your_project_id}/ {your_dataset_name}/{your_table_name}\" [Learn more](/ml-engine/docs/continuous-evaluation/create-job#table-schema)."]
        pub input_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1BigQuerySource {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1BigQuerySourceBuilder {
            GoogleCloudDatalabelingV1beta1BigQuerySourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options regarding evaluation between bounding boxes."]
    pub struct GoogleCloudDatalabelingV1beta1BoundingBoxEvaluationOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iouThreshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum [intersection-over-union (IOU)](/vision/automl/object-detection/docs/evaluate#intersection-over-union) required for 2 bounding boxes to be considered a match. This must be a number between 0 and 1."]
        pub iou_threshold: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudDatalabelingV1beta1BoundingBoxEvaluationOptions {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1BoundingBoxEvaluationOptionsBuilder {
            GoogleCloudDatalabelingV1beta1BoundingBoxEvaluationOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A bounding polygon in the image."]
    pub struct GoogleCloudDatalabelingV1beta1BoundingPoly {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon vertices."]
        pub vertices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1Vertex>>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1BoundingPoly {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1BoundingPolyBuilder {
            GoogleCloudDatalabelingV1beta1BoundingPolyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for image bounding poly (and bounding box) human labeling task."]
    pub struct GoogleCloudDatalabelingV1beta1BoundingPolyConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpecSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Annotation spec set resource name."]
        pub annotation_spec_set: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instructionMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Instruction message showed on contributors UI."]
        pub instruction_message: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1BoundingPolyConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1BoundingPolyConfigBuilder {
            GoogleCloudDatalabelingV1beta1BoundingPolyConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for classification annotations."]
    pub struct GoogleCloudDatalabelingV1beta1ClassificationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isMultiLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the classification task is multi-label or not."]
        pub is_multi_label: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudDatalabelingV1beta1ClassificationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ClassificationMetadataBuilder {
            GoogleCloudDatalabelingV1beta1ClassificationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metrics calculated for a classification model."]
    pub struct GoogleCloudDatalabelingV1beta1ClassificationMetrics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confusionMatrix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confusion matrix of predicted labels vs. ground truth labels."]
        pub confusion_matrix:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1ConfusionMatrix>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prCurve")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Precision-recall curve based on ground truth labels, predicted labels, and scores for the predicted labels."]
        pub pr_curve:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1PrCurve>>,
    }
    impl GoogleCloudDatalabelingV1beta1ClassificationMetrics {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ClassificationMetricsBuilder {
            GoogleCloudDatalabelingV1beta1ClassificationMetricsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudDatalabelingV1beta1ConfidenceMetricsEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidenceThreshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Threshold used for this entry. For classification tasks, this is a classification threshold: a predicted label is categorized as positive or negative (in the context of this point on the PR curve) based on whether the label's score meets this threshold. For image object detection (bounding box) tasks, this is the [intersection-over-union (IOU)](/vision/automl/object-detection/docs/evaluate#intersection-over-union) threshold for the context of this point on the PR curve."]
        pub confidence_threshold: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "f1Score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Harmonic mean of recall and precision."]
        pub f1_score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "f1ScoreAt1")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The harmonic mean of recall_at1 and precision_at1."]
        pub f1_score_at1: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "f1ScoreAt5")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The harmonic mean of recall_at5 and precision_at5."]
        pub f1_score_at5: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "precision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Precision value."]
        pub precision: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "precisionAt1")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Precision value for entries with label that has highest score."]
        pub precision_at1: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "precisionAt5")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Precision value for entries with label that has highest 5 scores."]
        pub precision_at5: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recall")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Recall value."]
        pub recall: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recallAt1")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Recall value for entries with label that has highest score."]
        pub recall_at1: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recallAt5")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Recall value for entries with label that has highest 5 scores."]
        pub recall_at5: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudDatalabelingV1beta1ConfidenceMetricsEntry {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ConfidenceMetricsEntryBuilder {
            GoogleCloudDatalabelingV1beta1ConfidenceMetricsEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Confusion matrix of the model running the classification. Only applicable when the metrics entry aggregates multiple labels. Not applicable when the entry is for a single label."]
    pub struct GoogleCloudDatalabelingV1beta1ConfusionMatrix {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "row")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub row: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1Row>>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1ConfusionMatrix {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ConfusionMatrixBuilder {
            GoogleCloudDatalabelingV1beta1ConfusionMatrixBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudDatalabelingV1beta1ConfusionMatrixEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The annotation spec of a predicted label."]
        pub annotation_spec:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotationSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of items predicted to have this label. (The ground truth label for these items is the `Row.annotationSpec` of this entry's parent.)"]
        pub item_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDatalabelingV1beta1ConfusionMatrixEntry {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ConfusionMatrixEntryBuilder {
            GoogleCloudDatalabelingV1beta1ConfusionMatrixEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CreateAnnotationSpecSet."]
    pub struct GoogleCloudDatalabelingV1beta1CreateAnnotationSpecSetRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpecSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Annotation spec set to create. Annotation specs must be included. Only one annotation spec will be accepted for annotation specs with same display_name."]
        pub annotation_spec_set: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotationSpecSet>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1CreateAnnotationSpecSetRequest {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1CreateAnnotationSpecSetRequestBuilder {
            GoogleCloudDatalabelingV1beta1CreateAnnotationSpecSetRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CreateDataset."]
    pub struct GoogleCloudDatalabelingV1beta1CreateDatasetRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The dataset to be created."]
        pub dataset:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1Dataset>>,
    }
    impl GoogleCloudDatalabelingV1beta1CreateDatasetRequest {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1CreateDatasetRequestBuilder {
            GoogleCloudDatalabelingV1beta1CreateDatasetRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CreateEvaluationJob."]
    pub struct GoogleCloudDatalabelingV1beta1CreateEvaluationJobRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "job")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The evaluation job to create."]
        pub job:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1EvaluationJob>>,
    }
    impl GoogleCloudDatalabelingV1beta1CreateEvaluationJobRequest {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1CreateEvaluationJobRequestBuilder {
            GoogleCloudDatalabelingV1beta1CreateEvaluationJobRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of a CreateInstruction operation."]
    pub struct GoogleCloudDatalabelingV1beta1CreateInstructionMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp when create instruction request was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instruction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the created Instruction. projects/{project_id}/instructions/{instruction_id}"]
        pub instruction: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Partial failures encountered. E.g. single files that couldn't be read. Status details field will contain standard GCP error details."]
        pub partial_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    }
    impl GoogleCloudDatalabelingV1beta1CreateInstructionMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1CreateInstructionMetadataBuilder {
            GoogleCloudDatalabelingV1beta1CreateInstructionMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CreateInstruction."]
    pub struct GoogleCloudDatalabelingV1beta1CreateInstructionRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instruction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Instruction of how to perform the labeling task."]
        pub instruction:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1Instruction>>,
    }
    impl GoogleCloudDatalabelingV1beta1CreateInstructionRequest {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1CreateInstructionRequestBuilder {
            GoogleCloudDatalabelingV1beta1CreateInstructionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deprecated: this instruction format is not supported any more. Instruction from a CSV file."]
    pub struct GoogleCloudDatalabelingV1beta1CsvInstruction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsFileUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "CSV file for the instruction. Only gcs path is allowed."]
        pub gcs_file_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1CsvInstruction {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1CsvInstructionBuilder {
            GoogleCloudDatalabelingV1beta1CsvInstructionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "DataItem is a piece of data, without annotation. For example, an image."]
    pub struct GoogleCloudDatalabelingV1beta1DataItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imagePayload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The image payload, a container of the image bytes/uri."]
        pub image_payload:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1ImagePayload>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Name of the data item, in format of: projects/{project_id}/datasets/{dataset_id}/dataItems/{data_item_id}"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textPayload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text payload, a container of text content."]
        pub text_payload:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1TextPayload>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoPayload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video payload, a container of the video uri."]
        pub video_payload:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1VideoPayload>>,
    }
    impl GoogleCloudDatalabelingV1beta1DataItem {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1DataItemBuilder {
            GoogleCloudDatalabelingV1beta1DataItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Dataset is the resource to hold your data. You can request multiple labeling tasks for a dataset while each one will generate an AnnotatedDataset."]
    pub struct GoogleCloudDatalabelingV1beta1Dataset {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockingResources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The names of any related resources that are blocking changes to the dataset."]
        pub blocking_resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time the dataset is created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataItemCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The number of data items in the dataset."]
        pub data_item_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. User-provided description of the annotation specification set. The description can be up to 10000 characters long."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The display name of the dataset. Maximum of 64 characters."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. This is populated with the original input configs where ImportData is called. It is available only after the clients import data to this dataset."]
        pub input_configs: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1InputConfig>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastMigrateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last time that the Dataset is migrated to AI Platform V2. If any of the AnnotatedDataset is migrated, the last_migration_time in Dataset is also updated."]
        pub last_migrate_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Dataset resource name, format is: projects/{project_id}/datasets/{dataset_id}"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1Dataset {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1DatasetBuilder {
            GoogleCloudDatalabelingV1beta1DatasetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes an evaluation between a machine learning model's predictions and ground truth labels. Created when an EvaluationJob runs successfully."]
    pub struct GoogleCloudDatalabelingV1beta1Evaluation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Type of task that the model version being evaluated performs, as defined in the evaluationJobConfig.inputConfig.annotationType field of the evaluation job that created this evaluation."]
        pub annotation_type:
            ::std::option::Option<GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "config")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Options used in the evaluation job that created this evaluation."]
        pub config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1EvaluationConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp for when this evaluation was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "evaluatedItemCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The number of items in the ground truth dataset that were used for this evaluation. Only populated when the evaulation is for certain AnnotationTypes."]
        pub evaluated_item_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "evaluationJobRunTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp for when the evaluation job that created this evaluation ran."]
        pub evaluation_job_run_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "evaluationMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Metrics comparing predictions to ground truth labels."]
        pub evaluation_metrics: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1EvaluationMetrics>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource name of an evaluation. The name has the following format: \"projects/{project_id}/datasets/{dataset_id}/evaluations/ {evaluation_id}'"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1Evaluation {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1EvaluationBuilder {
            GoogleCloudDatalabelingV1beta1EvaluationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Type of task that the model version being evaluated performs, as defined in the evaluationJobConfig.inputConfig.annotationType field of the evaluation job that created this evaluation."]
    pub enum GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum {
        #[serde(rename = "ANNOTATION_TYPE_UNSPECIFIED")]
        #[doc = ""]
        AnnotationTypeUnspecified,
        #[serde(rename = "IMAGE_CLASSIFICATION_ANNOTATION")]
        #[doc = "Classification annotations in an image. Allowed for continuous evaluation."]
        ImageClassificationAnnotation,
        #[serde(rename = "IMAGE_BOUNDING_BOX_ANNOTATION")]
        #[doc = "Bounding box annotations in an image. A form of image object detection. Allowed for continuous evaluation."]
        ImageBoundingBoxAnnotation,
        #[serde(rename = "IMAGE_ORIENTED_BOUNDING_BOX_ANNOTATION")]
        #[doc = "Oriented bounding box. The box does not have to be parallel to horizontal line."]
        ImageOrientedBoundingBoxAnnotation,
        #[serde(rename = "IMAGE_BOUNDING_POLY_ANNOTATION")]
        #[doc = "Bounding poly annotations in an image."]
        ImageBoundingPolyAnnotation,
        #[serde(rename = "IMAGE_POLYLINE_ANNOTATION")]
        #[doc = "Polyline annotations in an image."]
        ImagePolylineAnnotation,
        #[serde(rename = "IMAGE_SEGMENTATION_ANNOTATION")]
        #[doc = "Segmentation annotations in an image."]
        ImageSegmentationAnnotation,
        #[serde(rename = "VIDEO_SHOTS_CLASSIFICATION_ANNOTATION")]
        #[doc = "Classification annotations in video shots."]
        VideoShotsClassificationAnnotation,
        #[serde(rename = "VIDEO_OBJECT_TRACKING_ANNOTATION")]
        #[doc = "Video object tracking annotation."]
        VideoObjectTrackingAnnotation,
        #[serde(rename = "VIDEO_OBJECT_DETECTION_ANNOTATION")]
        #[doc = "Video object detection annotation."]
        VideoObjectDetectionAnnotation,
        #[serde(rename = "VIDEO_EVENT_ANNOTATION")]
        #[doc = "Video event annotation."]
        VideoEventAnnotation,
        #[serde(rename = "TEXT_CLASSIFICATION_ANNOTATION")]
        #[doc = "Classification for text. Allowed for continuous evaluation."]
        TextClassificationAnnotation,
        #[serde(rename = "TEXT_ENTITY_EXTRACTION_ANNOTATION")]
        #[doc = "Entity extraction for text."]
        TextEntityExtractionAnnotation,
        #[serde(rename = "GENERAL_CLASSIFICATION_ANNOTATION")]
        #[doc = "General classification. Allowed for continuous evaluation."]
        GeneralClassificationAnnotation,
    }
    impl ::std::default::Default for GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum {
        fn default() -> Self {
            Self::AnnotationTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration details used for calculating evaluation metrics and creating an Evaluation."]
    pub struct GoogleCloudDatalabelingV1beta1EvaluationConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBoxEvaluationOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only specify this field if the related model performs image object detection (`IMAGE_BOUNDING_BOX_ANNOTATION`). Describes how to evaluate bounding boxes."]
        pub bounding_box_evaluation_options: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1BoundingBoxEvaluationOptions>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1EvaluationConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1EvaluationConfigBuilder {
            GoogleCloudDatalabelingV1beta1EvaluationConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines an evaluation job that runs periodically to generate Evaluations. [Creating an evaluation job](/ml-engine/docs/continuous-evaluation/create-job) is the starting point for using continuous evaluation."]
    pub struct GoogleCloudDatalabelingV1beta1EvaluationJob {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpecSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Name of the AnnotationSpecSet describing all the labels that your machine learning model outputs. You must create this resource before you create an evaluation job and provide its name in the following format: \"projects/{project_id}/annotationSpecSets/{annotation_spec_set_id}\""]
        pub annotation_spec_set: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attempts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Every time the evaluation job runs and an error occurs, the failed attempt is appended to this array."]
        pub attempts: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1Attempt>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp of when this evaluation job was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Description of the job. The description can be up to 25,000 characters long."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "evaluationJobConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Configuration details for the evaluation job."]
        pub evaluation_job_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1EvaluationJobConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelMissingGroundTruth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Whether you want Data Labeling Service to provide ground truth labels for prediction input. If you want the service to assign human labelers to annotate your data, set this to `true`. If you want to provide your own ground truth labels in the evaluation job's BigQuery table, set this to `false`."]
        pub label_missing_ground_truth: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modelVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The [AI Platform Prediction model version](/ml-engine/docs/prediction-overview) to be evaluated. Prediction input and output is sampled from this model version. When creating an evaluation job, specify the model version in the following format: \"projects/{project_id}/models/{model_name}/versions/{version_name}\" There can only be one evaluation job per model version."]
        pub model_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. After you create a job, Data Labeling Service assigns a name to the job with the following format: \"projects/{project_id}/evaluationJobs/ {evaluation_job_id}\""]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schedule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Describes the interval at which the job runs. This interval must be at least 1 day, and it is rounded to the nearest day. For example, if you specify a 50-hour interval, the job runs every 2 days. You can provide the schedule in [crontab format](/scheduler/docs/configuring/cron-job-schedules) or in an [English-like format](/appengine/docs/standard/python/config/cronref#schedule_format). Regardless of what you specify, the job will run at 10:00 AM UTC. Only the interval from this schedule is used, not the specific time of day."]
        pub schedule: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Describes the current state of the job."]
        pub state: ::std::option::Option<GoogleCloudDatalabelingV1beta1EvaluationJobStateEnum>,
    }
    impl GoogleCloudDatalabelingV1beta1EvaluationJob {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1EvaluationJobBuilder {
            GoogleCloudDatalabelingV1beta1EvaluationJobBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Describes the current state of the job."]
    pub enum GoogleCloudDatalabelingV1beta1EvaluationJobStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = ""]
        StateUnspecified,
        #[serde(rename = "SCHEDULED")]
        #[doc = "The job is scheduled to run at the configured interval. You can pause or delete the job. When the job is in this state, it samples prediction input and output from your model version into your BigQuery table as predictions occur."]
        Scheduled,
        #[serde(rename = "RUNNING")]
        #[doc = "The job is currently running. When the job runs, Data Labeling Service does several things: 1. If you have configured your job to use Data Labeling Service for ground truth labeling, the service creates a Dataset and a labeling task for all data sampled since the last time the job ran. Human labelers provide ground truth labels for your data. Human labeling may take hours, or even days, depending on how much data has been sampled. The job remains in the `RUNNING` state during this time, and it can even be running multiple times in parallel if it gets triggered again (for example 24 hours later) before the earlier run has completed. When human labelers have finished labeling the data, the next step occurs. If you have configured your job to provide your own ground truth labels, Data Labeling Service still creates a Dataset for newly sampled data, but it expects that you have already added ground truth labels to the BigQuery table by this time. The next step occurs immediately. 2. Data Labeling Service creates an Evaluation by comparing your model version's predictions with the ground truth labels. If the job remains in this state for a long time, it continues to sample prediction data into your BigQuery table and will run again at the next interval, even if it causes the job to run multiple times in parallel."]
        Running,
        #[serde(rename = "PAUSED")]
        #[doc = "The job is not sampling prediction input and output into your BigQuery table and it will not run according to its schedule. You can resume the job."]
        Paused,
        #[serde(rename = "STOPPED")]
        #[doc = "The job has this state right before it is deleted."]
        Stopped,
    }
    impl ::std::default::Default for GoogleCloudDatalabelingV1beta1EvaluationJobStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides details for how an evaluation job sends email alerts based on the results of a run."]
    pub struct GoogleCloudDatalabelingV1beta1EvaluationJobAlertConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. An email address to send alerts to."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minAcceptableMeanAveragePrecision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A number between 0 and 1 that describes a minimum mean average precision threshold. When the evaluation job runs, if it calculates that your model version's predictions from the recent interval have meanAveragePrecision below this threshold, then it sends an alert to your specified email."]
        pub min_acceptable_mean_average_precision: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudDatalabelingV1beta1EvaluationJobAlertConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1EvaluationJobAlertConfigBuilder {
            GoogleCloudDatalabelingV1beta1EvaluationJobAlertConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configures specific details of how a continuous evaluation job works. Provide this configuration when you create an EvaluationJob."]
    pub struct GoogleCloudDatalabelingV1beta1EvaluationJobConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bigqueryImportKeys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Prediction keys that tell Data Labeling Service where to find the data for evaluation in your BigQuery table. When the service samples prediction input and output from your model version and saves it to BigQuery, the data gets stored as JSON strings in the BigQuery table. These keys tell Data Labeling Service how to parse the JSON. You can provide the following entries in this field: * `data_json_key`: the data key for prediction input. You must provide either this key or `reference_json_key`. * `reference_json_key`: the data reference key for prediction input. You must provide either this key or `data_json_key`. * `label_json_key`: the label key for prediction output. Required. * `label_score_json_key`: the score key for prediction output. Required. * `bounding_box_json_key`: the bounding box key for prediction output. Required if your model version perform image object detection. Learn [how to configure prediction keys](/ml-engine/docs/continuous-evaluation/create-job#prediction-keys)."]
        pub bigquery_import_keys:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPolyConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specify this field if your model version performs image object detection (bounding box detection). `annotationSpecSet` in this configuration must match EvaluationJob.annotationSpecSet."]
        pub bounding_poly_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1BoundingPolyConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "evaluationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Details for calculating evaluation metrics and creating Evaulations. If your model version performs image object detection, you must specify the `boundingBoxEvaluationOptions` field within this configuration. Otherwise, provide an empty object for this configuration."]
        pub evaluation_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1EvaluationConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "evaluationJobAlertConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Configuration details for evaluation job alerts. Specify this field if you want to receive email alerts if the evaluation job finds that your predictions have low mean average precision during a run."]
        pub evaluation_job_alert_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1EvaluationJobAlertConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exampleCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The maximum number of predictions to sample and save to BigQuery during each evaluation interval. This limit overrides `example_sample_percentage`: even if the service has not sampled enough predictions to fulfill `example_sample_perecentage` during an interval, it stops sampling predictions when it meets this limit."]
        pub example_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exampleSamplePercentage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Fraction of predictions to sample and save to BigQuery during each evaluation interval. For example, 0.1 means 10% of predictions served by your model version get saved to BigQuery."]
        pub example_sample_percentage: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "humanAnnotationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Details for human annotation of your data. If you set labelMissingGroundTruth to `true` for this evaluation job, then you must specify this field. If you plan to provide your own ground truth labels, then omit this field. Note that you must create an Instruction resource before you can specify this field. Provide the name of the instruction resource in the `instruction` field within this configuration."]
        pub human_annotation_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageClassificationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specify this field if your model version performs image classification or general classification. `annotationSpecSet` in this configuration must match EvaluationJob.annotationSpecSet. `allowMultiLabel` in this configuration must match `classificationMetadata.isMultiLabel` in input_config."]
        pub image_classification_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1ImageClassificationConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rquired. Details for the sampled prediction input. Within this configuration, there are requirements for several fields: * `dataType` must be one of `IMAGE`, `TEXT`, or `GENERAL_DATA`. * `annotationType` must be one of `IMAGE_CLASSIFICATION_ANNOTATION`, `TEXT_CLASSIFICATION_ANNOTATION`, `GENERAL_CLASSIFICATION_ANNOTATION`, or `IMAGE_BOUNDING_BOX_ANNOTATION` (image object detection). * If your machine learning model performs classification, you must specify `classificationMetadata.isMultiLabel`. * You must specify `bigquerySource` (not `gcsSource`)."]
        pub input_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1InputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textClassificationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specify this field if your model version performs text classification. `annotationSpecSet` in this configuration must match EvaluationJob.annotationSpecSet. `allowMultiLabel` in this configuration must match `classificationMetadata.isMultiLabel` in input_config."]
        pub text_classification_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1TextClassificationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1EvaluationJobConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1EvaluationJobConfigBuilder {
            GoogleCloudDatalabelingV1beta1EvaluationJobConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudDatalabelingV1beta1EvaluationMetrics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "classificationMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub classification_metrics: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1ClassificationMetrics>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectDetectionMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub object_detection_metrics: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1ObjectDetectionMetrics>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1EvaluationMetrics {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1EvaluationMetricsBuilder {
            GoogleCloudDatalabelingV1beta1EvaluationMetricsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for video event human labeling task."]
    pub struct GoogleCloudDatalabelingV1beta1EventConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpecSets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The list of annotation spec set resource name. Similar to video classification, we support selecting event from multiple AnnotationSpecSet at the same time."]
        pub annotation_spec_sets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clipLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Videos will be cut to smaller clips to make it easier for labelers to work on. Users can configure is field in seconds, if not set, default value is 60s."]
        pub clip_length: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "overlapLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The overlap length between different video clips. Users can configure is field in seconds, if not set, default value is 1s."]
        pub overlap_length: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDatalabelingV1beta1EventConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1EventConfigBuilder {
            GoogleCloudDatalabelingV1beta1EventConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Example is a piece of data and its annotation. For example, an image with label \"house\"."]
    pub struct GoogleCloudDatalabelingV1beta1Example {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Annotations for the piece of data in Example. One piece of data can have multiple annotations."]
        pub annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1Annotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imagePayload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The image payload, a container of the image bytes/uri."]
        pub image_payload:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1ImagePayload>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Name of the example, in format of: projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/ {annotated_dataset_id}/examples/{example_id}"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textPayload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text payload, a container of the text content."]
        pub text_payload:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1TextPayload>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoPayload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video payload, a container of the video uri."]
        pub video_payload:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1VideoPayload>>,
    }
    impl GoogleCloudDatalabelingV1beta1Example {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ExampleBuilder {
            GoogleCloudDatalabelingV1beta1ExampleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Example comparisons comparing ground truth output and predictions for a specific input."]
    pub struct GoogleCloudDatalabelingV1beta1ExampleComparison {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groundTruthExample")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ground truth output for the input."]
        pub ground_truth_example:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1Example>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modelCreatedExamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Predictions by the model for the input."]
        pub model_created_examples: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1Example>>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1ExampleComparison {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ExampleComparisonBuilder {
            GoogleCloudDatalabelingV1beta1ExampleComparisonBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of an ExportData operation."]
    pub struct GoogleCloudDatalabelingV1beta1ExportDataOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of annotated dataset in format \"projects/*/datasets/*/annotatedDatasets/*\"."]
        pub annotated_dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp when export dataset request was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of dataset to be exported. \"projects/*/datasets/*\""]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Partial failures encountered. E.g. single files that couldn't be read. Status details field will contain standard GCP error details."]
        pub partial_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    }
    impl GoogleCloudDatalabelingV1beta1ExportDataOperationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ExportDataOperationMetadataBuilder {
            GoogleCloudDatalabelingV1beta1ExportDataOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response used for ExportDataset longrunning operation."]
    pub struct GoogleCloudDatalabelingV1beta1ExportDataOperationResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of annotated dataset in format \"projects/*/datasets/*/annotatedDatasets/*\"."]
        pub annotated_dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ouptut only. The name of dataset. \"projects/*/datasets/*\""]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exportCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Number of examples exported successfully."]
        pub export_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelStats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Statistic infos of labels in the exported dataset."]
        pub label_stats:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1LabelStats>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. output_config in the ExportData request."]
        pub output_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1OutputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Total number of examples requested to export"]
        pub total_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDatalabelingV1beta1ExportDataOperationResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ExportDataOperationResponseBuilder {
            GoogleCloudDatalabelingV1beta1ExportDataOperationResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for ExportData API."]
    pub struct GoogleCloudDatalabelingV1beta1ExportDataRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Annotated dataset resource name. DataItem in Dataset and their annotations in specified annotated dataset will be exported. It's in format of projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/ {annotated_dataset_id}"]
        pub annotated_dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Filter is not supported at this moment."]
        pub filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Specify the output destination."]
        pub output_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1OutputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEmailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email of the user who started the export task and should be notified by email. If empty no notification will be sent."]
        pub user_email_address: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1ExportDataRequest {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ExportDataRequestBuilder {
            GoogleCloudDatalabelingV1beta1ExportDataRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A feedback message inside a feedback thread."]
    pub struct GoogleCloudDatalabelingV1beta1FeedbackMessage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "body")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "String content of the feedback. Maximum of 10000 characters."]
        pub body: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Create time."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The image storing this feedback if the feedback is an image representing operator's comments."]
        pub image: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the feedback message in a feedback thread. Format: 'project/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset_id}/feedbackThreads/{feedback_thread_id}/feedbackMessage/{feedback_message_id}'"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operatorFeedbackMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub operator_feedback_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1OperatorFeedbackMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requesterFeedbackMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub requester_feedback_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1RequesterFeedbackMetadata>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1FeedbackMessage {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1FeedbackMessageBuilder {
            GoogleCloudDatalabelingV1beta1FeedbackMessageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A feedback thread of a certain labeling task on a certain annotated dataset."]
    pub struct GoogleCloudDatalabelingV1beta1FeedbackThread {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feedbackThreadMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata regarding the feedback thread."]
        pub feedback_thread_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1FeedbackThreadMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the feedback thread. Format: 'project/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset_id}/feedbackThreads/{feedback_thread_id}'"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1FeedbackThread {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1FeedbackThreadBuilder {
            GoogleCloudDatalabelingV1beta1FeedbackThreadBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudDatalabelingV1beta1FeedbackThreadMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the thread is created"]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the thread is last updated."]
        pub last_update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub status:
            ::std::option::Option<GoogleCloudDatalabelingV1beta1FeedbackThreadMetadataStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An image thumbnail of this thread."]
        pub thumbnail: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1FeedbackThreadMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1FeedbackThreadMetadataBuilder {
            GoogleCloudDatalabelingV1beta1FeedbackThreadMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleCloudDatalabelingV1beta1FeedbackThreadMetadataStatusEnum {
        #[serde(rename = "FEEDBACK_THREAD_STATUS_UNSPECIFIED")]
        #[doc = ""]
        FeedbackThreadStatusUnspecified,
        #[serde(rename = "NEW")]
        #[doc = "Feedback thread is created with no reply;"]
        New,
        #[serde(rename = "REPLIED")]
        #[doc = "Feedback thread is replied at least once;"]
        Replied,
    }
    impl ::std::default::Default for GoogleCloudDatalabelingV1beta1FeedbackThreadMetadataStatusEnum {
        fn default() -> Self {
            Self::FeedbackThreadStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Export destination of the data.Only gcs path is allowed in output_uri."]
    pub struct GoogleCloudDatalabelingV1beta1GcsDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The format of the gcs destination. Only \"text/csv\" and \"application/json\" are supported."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The output uri of destination file."]
        pub output_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1GcsDestination {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1GcsDestinationBuilder {
            GoogleCloudDatalabelingV1beta1GcsDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Export folder destination of the data."]
    pub struct GoogleCloudDatalabelingV1beta1GcsFolderDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputFolderUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Cloud Storage directory to export data to."]
        pub output_folder_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1GcsFolderDestination {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1GcsFolderDestinationBuilder {
            GoogleCloudDatalabelingV1beta1GcsFolderDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Source of the Cloud Storage file to be imported."]
    pub struct GoogleCloudDatalabelingV1beta1GcsSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The input URI of source file. This must be a Cloud Storage path (`gs://...`)."]
        pub input_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The format of the source file. Only \"text/csv\" is supported."]
        pub mime_type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1GcsSource {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1GcsSourceBuilder {
            GoogleCloudDatalabelingV1beta1GcsSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for how human labeling task should be done."]
    pub struct GoogleCloudDatalabelingV1beta1HumanAnnotationConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDatasetDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A human-readable description for AnnotatedDataset. The description can be up to 10000 characters long."]
        pub annotated_dataset_description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDatasetDisplayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A human-readable name for AnnotatedDataset defined by users. Maximum of 64 characters ."]
        pub annotated_dataset_display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contributorEmails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If you want your own labeling contributors to manage and work on this labeling request, you can set these contributors here. We will give them access to the question types in crowdcompute. Note that these emails must be registered in crowdcompute worker UI: https://crowd-compute.appspot.com/"]
        pub contributor_emails: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instruction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Instruction resource name."]
        pub instruction: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A human-readable label used to logically group labeling tasks. This string must match the regular expression `[a-zA-Z\\\\d_-]{0,128}`."]
        pub label_group: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The Language of this question, as a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). Default value is en-US. Only need to set this when task is language related. For example, French text classification."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "questionDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Maximum duration for contributors to answer a question. Maximum is 3600 seconds. Default is 3600 seconds."]
        pub question_duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replicaCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Replication of questions. Each question will be sent to up to this number of contributors to label. Aggregated answers will be returned. Default is set to 1. For image related labeling, valid values are 1, 3, 5."]
        pub replica_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEmailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email of the user who started the labeling task and should be notified by email. If empty no notification will be sent."]
        pub user_email_address: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1HumanAnnotationConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1HumanAnnotationConfigBuilder {
            GoogleCloudDatalabelingV1beta1HumanAnnotationConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Image bounding poly annotation. It represents a polygon including bounding box in the image."]
    pub struct GoogleCloudDatalabelingV1beta1ImageBoundingPolyAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label of object in this bounding polygon."]
        pub annotation_spec:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotationSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedBoundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub normalized_bounding_poly: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1NormalizedBoundingPoly>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1ImageBoundingPolyAnnotation {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ImageBoundingPolyAnnotationBuilder {
            GoogleCloudDatalabelingV1beta1ImageBoundingPolyAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Image classification annotation definition."]
    pub struct GoogleCloudDatalabelingV1beta1ImageClassificationAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label of image."]
        pub annotation_spec:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotationSpec>>,
    }
    impl GoogleCloudDatalabelingV1beta1ImageClassificationAnnotation {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ImageClassificationAnnotationBuilder {
            GoogleCloudDatalabelingV1beta1ImageClassificationAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for image classification human labeling task."]
    pub struct GoogleCloudDatalabelingV1beta1ImageClassificationConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowMultiLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If allow_multi_label is true, contributors are able to choose multiple labels for one image."]
        pub allow_multi_label: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpecSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Annotation spec set resource name."]
        pub annotation_spec_set: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "answerAggregationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The type of how to aggregate answers."]
        pub answer_aggregation_type: ::std::option::Option<
            GoogleCloudDatalabelingV1beta1ImageClassificationConfigAnswerAggregationTypeEnum,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1ImageClassificationConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ImageClassificationConfigBuilder {
            GoogleCloudDatalabelingV1beta1ImageClassificationConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The type of how to aggregate answers."]
    pub enum GoogleCloudDatalabelingV1beta1ImageClassificationConfigAnswerAggregationTypeEnum {
        #[serde(rename = "STRING_AGGREGATION_TYPE_UNSPECIFIED")]
        #[doc = ""]
        StringAggregationTypeUnspecified,
        #[serde(rename = "MAJORITY_VOTE")]
        #[doc = "Majority vote to aggregate answers."]
        MajorityVote,
        #[serde(rename = "UNANIMOUS_VOTE")]
        #[doc = "Unanimous answers will be adopted."]
        UnanimousVote,
        #[serde(rename = "NO_AGGREGATION")]
        #[doc = "Preserve all answers by crowd compute."]
        NoAggregation,
    }
    impl ::std::default::Default
        for GoogleCloudDatalabelingV1beta1ImageClassificationConfigAnswerAggregationTypeEnum
    {
        fn default() -> Self {
            Self::StringAggregationTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Container of information about an image."]
    pub struct GoogleCloudDatalabelingV1beta1ImagePayload {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageThumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A byte string of a thumbnail image."]
        pub image_thumbnail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image uri from the user bucket."]
        pub image_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image format."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signedUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Signed uri of the image file in the service bucket."]
        pub signed_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1ImagePayload {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ImagePayloadBuilder {
            GoogleCloudDatalabelingV1beta1ImagePayloadBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A polyline for the image annotation."]
    pub struct GoogleCloudDatalabelingV1beta1ImagePolylineAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label of this polyline."]
        pub annotation_spec:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotationSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedPolyline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub normalized_polyline: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1NormalizedPolyline>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "polyline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub polyline:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1Polyline>>,
    }
    impl GoogleCloudDatalabelingV1beta1ImagePolylineAnnotation {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ImagePolylineAnnotationBuilder {
            GoogleCloudDatalabelingV1beta1ImagePolylineAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Image segmentation annotation."]
    pub struct GoogleCloudDatalabelingV1beta1ImageSegmentationAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationColors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The mapping between rgb color and annotation spec. The key is the rgb color represented in format of rgb(0, 0, 0). The value is the AnnotationSpec."]
        pub annotation_colors: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                ::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotationSpec>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A byte string of a full image's color map."]
        pub image_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image format."]
        pub mime_type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1ImageSegmentationAnnotation {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ImageSegmentationAnnotationBuilder {
            GoogleCloudDatalabelingV1beta1ImageSegmentationAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of an ImportData operation."]
    pub struct GoogleCloudDatalabelingV1beta1ImportDataOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp when import dataset request was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of imported dataset. \"projects/*/datasets/*\""]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Partial failures encountered. E.g. single files that couldn't be read. Status details field will contain standard GCP error details."]
        pub partial_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    }
    impl GoogleCloudDatalabelingV1beta1ImportDataOperationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ImportDataOperationMetadataBuilder {
            GoogleCloudDatalabelingV1beta1ImportDataOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response used for ImportData longrunning operation."]
    pub struct GoogleCloudDatalabelingV1beta1ImportDataOperationResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ouptut only. The name of imported dataset."]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Number of examples imported successfully."]
        pub import_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Total number of examples requested to import"]
        pub total_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDatalabelingV1beta1ImportDataOperationResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ImportDataOperationResponseBuilder {
            GoogleCloudDatalabelingV1beta1ImportDataOperationResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for ImportData API."]
    pub struct GoogleCloudDatalabelingV1beta1ImportDataRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Specify the input source of the data."]
        pub input_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1InputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEmailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email of the user who started the import task and should be notified by email. If empty no notification will be sent."]
        pub user_email_address: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1ImportDataRequest {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ImportDataRequestBuilder {
            GoogleCloudDatalabelingV1beta1ImportDataRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The configuration of input data, including data type, location, etc."]
    pub struct GoogleCloudDatalabelingV1beta1InputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The type of annotation to be performed on this data. You must specify this field if you are using this InputConfig in an EvaluationJob."]
        pub annotation_type:
            ::std::option::Option<GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bigquerySource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Source located in BigQuery. You must specify this field if you are using this InputConfig in an EvaluationJob."]
        pub bigquery_source:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1BigQuerySource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "classificationMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Metadata about annotations for the input. You must specify this field if you are using this InputConfig in an EvaluationJob for a model version that performs classification."]
        pub classification_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1ClassificationMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Data type must be specifed when user tries to import data."]
        pub data_type: ::std::option::Option<GoogleCloudDatalabelingV1beta1InputConfigDataTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Source located in Cloud Storage."]
        pub gcs_source:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1GcsSource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for text import, as language code must be specified."]
        pub text_metadata:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1TextMetadata>>,
    }
    impl GoogleCloudDatalabelingV1beta1InputConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1InputConfigBuilder {
            GoogleCloudDatalabelingV1beta1InputConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The type of annotation to be performed on this data. You must specify this field if you are using this InputConfig in an EvaluationJob."]
    pub enum GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum {
        #[serde(rename = "ANNOTATION_TYPE_UNSPECIFIED")]
        #[doc = ""]
        AnnotationTypeUnspecified,
        #[serde(rename = "IMAGE_CLASSIFICATION_ANNOTATION")]
        #[doc = "Classification annotations in an image. Allowed for continuous evaluation."]
        ImageClassificationAnnotation,
        #[serde(rename = "IMAGE_BOUNDING_BOX_ANNOTATION")]
        #[doc = "Bounding box annotations in an image. A form of image object detection. Allowed for continuous evaluation."]
        ImageBoundingBoxAnnotation,
        #[serde(rename = "IMAGE_ORIENTED_BOUNDING_BOX_ANNOTATION")]
        #[doc = "Oriented bounding box. The box does not have to be parallel to horizontal line."]
        ImageOrientedBoundingBoxAnnotation,
        #[serde(rename = "IMAGE_BOUNDING_POLY_ANNOTATION")]
        #[doc = "Bounding poly annotations in an image."]
        ImageBoundingPolyAnnotation,
        #[serde(rename = "IMAGE_POLYLINE_ANNOTATION")]
        #[doc = "Polyline annotations in an image."]
        ImagePolylineAnnotation,
        #[serde(rename = "IMAGE_SEGMENTATION_ANNOTATION")]
        #[doc = "Segmentation annotations in an image."]
        ImageSegmentationAnnotation,
        #[serde(rename = "VIDEO_SHOTS_CLASSIFICATION_ANNOTATION")]
        #[doc = "Classification annotations in video shots."]
        VideoShotsClassificationAnnotation,
        #[serde(rename = "VIDEO_OBJECT_TRACKING_ANNOTATION")]
        #[doc = "Video object tracking annotation."]
        VideoObjectTrackingAnnotation,
        #[serde(rename = "VIDEO_OBJECT_DETECTION_ANNOTATION")]
        #[doc = "Video object detection annotation."]
        VideoObjectDetectionAnnotation,
        #[serde(rename = "VIDEO_EVENT_ANNOTATION")]
        #[doc = "Video event annotation."]
        VideoEventAnnotation,
        #[serde(rename = "TEXT_CLASSIFICATION_ANNOTATION")]
        #[doc = "Classification for text. Allowed for continuous evaluation."]
        TextClassificationAnnotation,
        #[serde(rename = "TEXT_ENTITY_EXTRACTION_ANNOTATION")]
        #[doc = "Entity extraction for text."]
        TextEntityExtractionAnnotation,
        #[serde(rename = "GENERAL_CLASSIFICATION_ANNOTATION")]
        #[doc = "General classification. Allowed for continuous evaluation."]
        GeneralClassificationAnnotation,
    }
    impl ::std::default::Default for GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum {
        fn default() -> Self {
            Self::AnnotationTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Data type must be specifed when user tries to import data."]
    pub enum GoogleCloudDatalabelingV1beta1InputConfigDataTypeEnum {
        #[serde(rename = "DATA_TYPE_UNSPECIFIED")]
        #[doc = "Data type is unspecified."]
        DataTypeUnspecified,
        #[serde(rename = "IMAGE")]
        #[doc = "Allowed for continuous evaluation."]
        Image,
        #[serde(rename = "VIDEO")]
        #[doc = "Video data type."]
        Video,
        #[serde(rename = "TEXT")]
        #[doc = "Allowed for continuous evaluation."]
        Text,
        #[serde(rename = "GENERAL_DATA")]
        #[doc = "Allowed for continuous evaluation."]
        GeneralData,
    }
    impl ::std::default::Default for GoogleCloudDatalabelingV1beta1InputConfigDataTypeEnum {
        fn default() -> Self {
            Self::DataTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Instruction of how to perform the labeling task for human operators. Currently only PDF instruction is supported."]
    pub struct GoogleCloudDatalabelingV1beta1Instruction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockingResources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The names of any related resources that are blocking changes to the instruction."]
        pub blocking_resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Creation time of instruction."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "csvInstruction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated: this instruction format is not supported any more. Instruction from a CSV file, such as for classification task. The CSV file should have exact two columns, in the following format: * The first column is labeled data, such as an image reference, text. * The second column is comma separated labels associated with data."]
        pub csv_instruction:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1CsvInstruction>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The data type of this instruction."]
        pub data_type: ::std::option::Option<GoogleCloudDatalabelingV1beta1InstructionDataTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. User-provided description of the instruction. The description can be up to 10000 characters long."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The display name of the instruction. Maximum of 64 characters."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Instruction resource name, format: projects/{project_id}/instructions/{instruction_id}"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pdfInstruction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instruction from a PDF document. The PDF should be in a Cloud Storage bucket."]
        pub pdf_instruction:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1PdfInstruction>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Last update time of instruction."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1Instruction {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1InstructionBuilder {
            GoogleCloudDatalabelingV1beta1InstructionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The data type of this instruction."]
    pub enum GoogleCloudDatalabelingV1beta1InstructionDataTypeEnum {
        #[serde(rename = "DATA_TYPE_UNSPECIFIED")]
        #[doc = "Data type is unspecified."]
        DataTypeUnspecified,
        #[serde(rename = "IMAGE")]
        #[doc = "Allowed for continuous evaluation."]
        Image,
        #[serde(rename = "VIDEO")]
        #[doc = "Video data type."]
        Video,
        #[serde(rename = "TEXT")]
        #[doc = "Allowed for continuous evaluation."]
        Text,
        #[serde(rename = "GENERAL_DATA")]
        #[doc = "Allowed for continuous evaluation."]
        GeneralData,
    }
    impl ::std::default::Default for GoogleCloudDatalabelingV1beta1InstructionDataTypeEnum {
        fn default() -> Self {
            Self::DataTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelImageBoundingBox operation metadata."]
    pub struct GoogleCloudDatalabelingV1beta1LabelImageBoundingBoxOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1LabelImageBoundingBoxOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1beta1LabelImageBoundingBoxOperationMetadataBuilder {
            GoogleCloudDatalabelingV1beta1LabelImageBoundingBoxOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of LabelImageBoundingPoly operation metadata."]
    pub struct GoogleCloudDatalabelingV1beta1LabelImageBoundingPolyOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1LabelImageBoundingPolyOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1beta1LabelImageBoundingPolyOperationMetadataBuilder {
            GoogleCloudDatalabelingV1beta1LabelImageBoundingPolyOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of a LabelImageClassification operation."]
    pub struct GoogleCloudDatalabelingV1beta1LabelImageClassificationOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1LabelImageClassificationOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1beta1LabelImageClassificationOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1beta1LabelImageClassificationOperationMetadataBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelImageOrientedBoundingBox operation metadata."]
    pub struct GoogleCloudDatalabelingV1beta1LabelImageOrientedBoundingBoxOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1LabelImageOrientedBoundingBoxOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1beta1LabelImageOrientedBoundingBoxOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1beta1LabelImageOrientedBoundingBoxOperationMetadataBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of LabelImagePolyline operation metadata."]
    pub struct GoogleCloudDatalabelingV1beta1LabelImagePolylineOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1LabelImagePolylineOperationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1LabelImagePolylineOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1beta1LabelImagePolylineOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for starting an image labeling task."]
    pub struct GoogleCloudDatalabelingV1beta1LabelImageRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Basic human annotation config."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPolyConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for bounding box and bounding poly task. One of image_classification_config, bounding_poly_config, polyline_config and segmentation_config are required."]
        pub bounding_poly_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1BoundingPolyConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The type of image labeling task."]
        pub feature:
            ::std::option::Option<GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageClassificationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for image classification task. One of image_classification_config, bounding_poly_config, polyline_config and segmentation_config are required."]
        pub image_classification_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1ImageClassificationConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "polylineConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for polyline task. One of image_classification_config, bounding_poly_config, polyline_config and segmentation_config are required."]
        pub polyline_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1PolylineConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segmentationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for segmentation task. One of image_classification_config, bounding_poly_config, polyline_config and segmentation_config are required."]
        pub segmentation_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1SegmentationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1LabelImageRequest {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1LabelImageRequestBuilder {
            GoogleCloudDatalabelingV1beta1LabelImageRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The type of image labeling task."]
    pub enum GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum {
        #[serde(rename = "FEATURE_UNSPECIFIED")]
        #[doc = ""]
        FeatureUnspecified,
        #[serde(rename = "CLASSIFICATION")]
        #[doc = "Label whole image with one or more of labels."]
        Classification,
        #[serde(rename = "BOUNDING_BOX")]
        #[doc = "Label image with bounding boxes for labels."]
        BoundingBox,
        #[serde(rename = "ORIENTED_BOUNDING_BOX")]
        #[doc = "Label oriented bounding box. The box does not have to be parallel to horizontal line."]
        OrientedBoundingBox,
        #[serde(rename = "BOUNDING_POLY")]
        #[doc = "Label images with bounding poly. A bounding poly is a plane figure that is bounded by a finite chain of straight line segments closing in a loop."]
        BoundingPoly,
        #[serde(rename = "POLYLINE")]
        #[doc = "Label images with polyline. Polyline is formed by connected line segments which are not in closed form."]
        Polyline,
        #[serde(rename = "SEGMENTATION")]
        #[doc = "Label images with segmentation. Segmentation is different from bounding poly since it is more fine-grained, pixel level annotation."]
        Segmentation,
    }
    impl ::std::default::Default for GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum {
        fn default() -> Self {
            Self::FeatureUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelImageSegmentation operation metadata."]
    pub struct GoogleCloudDatalabelingV1beta1LabelImageSegmentationOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1LabelImageSegmentationOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1beta1LabelImageSegmentationOperationMetadataBuilder {
            GoogleCloudDatalabelingV1beta1LabelImageSegmentationOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of a labeling operation, such as LabelImage or LabelVideo. Next tag: 23"]
    pub struct GoogleCloudDatalabelingV1beta1LabelOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of annotated dataset in format \"projects/*/datasets/*/annotatedDatasets/*\"."]
        pub annotated_dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp when labeling request was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of dataset to be labeled. \"projects/*/datasets/*\""]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageBoundingBoxDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image bounding box operation."]
        pub image_bounding_box_details: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1LabelImageBoundingBoxOperationMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageBoundingPolyDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image bounding poly operation."]
        pub image_bounding_poly_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1beta1LabelImageBoundingPolyOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageClassificationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image classification operation."]
        pub image_classification_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1beta1LabelImageClassificationOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageOrientedBoundingBoxDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image oriented bounding box operation."]
        pub image_oriented_bounding_box_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1beta1LabelImageOrientedBoundingBoxOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imagePolylineDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image polyline operation."]
        pub image_polyline_details: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1LabelImagePolylineOperationMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageSegmentationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image segmentation operation."]
        pub image_segmentation_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1beta1LabelImageSegmentationOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Partial failures encountered. E.g. single files that couldn't be read. Status details field will contain standard GCP error details."]
        pub partial_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressPercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Progress of label operation. Range: [0, 100]."]
        pub progress_percent: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textClassificationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label text classification operation."]
        pub text_classification_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1beta1LabelTextClassificationOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textEntityExtractionDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label text entity extraction operation."]
        pub text_entity_extraction_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1beta1LabelTextEntityExtractionOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoClassificationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label video classification operation."]
        pub video_classification_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1beta1LabelVideoClassificationOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoEventDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label video event operation."]
        pub video_event_details: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1LabelVideoEventOperationMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoObjectDetectionDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label video object detection operation."]
        pub video_object_detection_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1beta1LabelVideoObjectDetectionOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoObjectTrackingDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label video object tracking operation."]
        pub video_object_tracking_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1beta1LabelVideoObjectTrackingOperationMetadata,
            >,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1LabelOperationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1LabelOperationMetadataBuilder {
            GoogleCloudDatalabelingV1beta1LabelOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Statistics about annotation specs."]
    pub struct GoogleCloudDatalabelingV1beta1LabelStats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exampleCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map of each annotation spec's example count. Key is the annotation spec name and value is the number of examples for that annotation spec. If the annotated dataset does not have annotation spec, the map will return a pair where the key is empty string and value is the total number of annotations."]
        pub example_count:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl GoogleCloudDatalabelingV1beta1LabelStats {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1LabelStatsBuilder {
            GoogleCloudDatalabelingV1beta1LabelStatsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelTextClassification operation metadata."]
    pub struct GoogleCloudDatalabelingV1beta1LabelTextClassificationOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1LabelTextClassificationOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1beta1LabelTextClassificationOperationMetadataBuilder {
            GoogleCloudDatalabelingV1beta1LabelTextClassificationOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelTextEntityExtraction operation metadata."]
    pub struct GoogleCloudDatalabelingV1beta1LabelTextEntityExtractionOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1LabelTextEntityExtractionOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1beta1LabelTextEntityExtractionOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1beta1LabelTextEntityExtractionOperationMetadataBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for LabelText."]
    pub struct GoogleCloudDatalabelingV1beta1LabelTextRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Basic human annotation config."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The type of text labeling task."]
        pub feature:
            ::std::option::Option<GoogleCloudDatalabelingV1beta1LabelTextRequestFeatureEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textClassificationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for text classification task. One of text_classification_config and text_entity_extraction_config is required."]
        pub text_classification_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1TextClassificationConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textEntityExtractionConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for entity extraction task. One of text_classification_config and text_entity_extraction_config is required."]
        pub text_entity_extraction_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1TextEntityExtractionConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1LabelTextRequest {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1LabelTextRequestBuilder {
            GoogleCloudDatalabelingV1beta1LabelTextRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The type of text labeling task."]
    pub enum GoogleCloudDatalabelingV1beta1LabelTextRequestFeatureEnum {
        #[serde(rename = "FEATURE_UNSPECIFIED")]
        #[doc = ""]
        FeatureUnspecified,
        #[serde(rename = "TEXT_CLASSIFICATION")]
        #[doc = "Label text content to one of more labels."]
        TextClassification,
        #[serde(rename = "TEXT_ENTITY_EXTRACTION")]
        #[doc = "Label entities and their span in text."]
        TextEntityExtraction,
    }
    impl ::std::default::Default for GoogleCloudDatalabelingV1beta1LabelTextRequestFeatureEnum {
        fn default() -> Self {
            Self::FeatureUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelVideoClassification operation metadata."]
    pub struct GoogleCloudDatalabelingV1beta1LabelVideoClassificationOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1LabelVideoClassificationOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1beta1LabelVideoClassificationOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1beta1LabelVideoClassificationOperationMetadataBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelVideoEvent operation metadata."]
    pub struct GoogleCloudDatalabelingV1beta1LabelVideoEventOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1LabelVideoEventOperationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1LabelVideoEventOperationMetadataBuilder {
            GoogleCloudDatalabelingV1beta1LabelVideoEventOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelVideoObjectDetection operation metadata."]
    pub struct GoogleCloudDatalabelingV1beta1LabelVideoObjectDetectionOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1LabelVideoObjectDetectionOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1beta1LabelVideoObjectDetectionOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1beta1LabelVideoObjectDetectionOperationMetadataBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelVideoObjectTracking operation metadata."]
    pub struct GoogleCloudDatalabelingV1beta1LabelVideoObjectTrackingOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1LabelVideoObjectTrackingOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1beta1LabelVideoObjectTrackingOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1beta1LabelVideoObjectTrackingOperationMetadataBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for LabelVideo."]
    pub struct GoogleCloudDatalabelingV1beta1LabelVideoRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Basic human annotation config."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1HumanAnnotationConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for video event task. One of video_classification_config, object_detection_config, object_tracking_config and event_config is required."]
        pub event_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1EventConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The type of video labeling task."]
        pub feature:
            ::std::option::Option<GoogleCloudDatalabelingV1beta1LabelVideoRequestFeatureEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectDetectionConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for video object detection task. One of video_classification_config, object_detection_config, object_tracking_config and event_config is required."]
        pub object_detection_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1ObjectDetectionConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectTrackingConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for video object tracking task. One of video_classification_config, object_detection_config, object_tracking_config and event_config is required."]
        pub object_tracking_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1ObjectTrackingConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoClassificationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for video classification task. One of video_classification_config, object_detection_config, object_tracking_config and event_config is required."]
        pub video_classification_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1VideoClassificationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1LabelVideoRequest {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1LabelVideoRequestBuilder {
            GoogleCloudDatalabelingV1beta1LabelVideoRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The type of video labeling task."]
    pub enum GoogleCloudDatalabelingV1beta1LabelVideoRequestFeatureEnum {
        #[serde(rename = "FEATURE_UNSPECIFIED")]
        #[doc = ""]
        FeatureUnspecified,
        #[serde(rename = "CLASSIFICATION")]
        #[doc = "Label whole video or video segment with one or more labels."]
        Classification,
        #[serde(rename = "OBJECT_DETECTION")]
        #[doc = "Label objects with bounding box on image frames extracted from the video."]
        ObjectDetection,
        #[serde(rename = "OBJECT_TRACKING")]
        #[doc = "Label and track objects in video."]
        ObjectTracking,
        #[serde(rename = "EVENT")]
        #[doc = "Label the range of video for the specified events."]
        Event,
    }
    impl ::std::default::Default for GoogleCloudDatalabelingV1beta1LabelVideoRequestFeatureEnum {
        fn default() -> Self {
            Self::FeatureUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results of listing annotated datasets for a dataset."]
    pub struct GoogleCloudDatalabelingV1beta1ListAnnotatedDatasetsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDatasets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of annotated datasets to return."]
        pub annotated_datasets: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotatedDataset>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1ListAnnotatedDatasetsResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ListAnnotatedDatasetsResponseBuilder {
            GoogleCloudDatalabelingV1beta1ListAnnotatedDatasetsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results of listing annotation spec set under a project."]
    pub struct GoogleCloudDatalabelingV1beta1ListAnnotationSpecSetsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpecSets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of annotation spec sets."]
        pub annotation_spec_sets: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotationSpecSet>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1ListAnnotationSpecSetsResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ListAnnotationSpecSetsResponseBuilder {
            GoogleCloudDatalabelingV1beta1ListAnnotationSpecSetsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results of listing data items in a dataset."]
    pub struct GoogleCloudDatalabelingV1beta1ListDataItemsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of data items to return."]
        pub data_items: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1DataItem>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1ListDataItemsResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ListDataItemsResponseBuilder {
            GoogleCloudDatalabelingV1beta1ListDataItemsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results of listing datasets within a project."]
    pub struct GoogleCloudDatalabelingV1beta1ListDatasetsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datasets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of datasets to return."]
        pub datasets: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1Dataset>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1ListDatasetsResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ListDatasetsResponseBuilder {
            GoogleCloudDatalabelingV1beta1ListDatasetsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results for listing evaluation jobs."]
    pub struct GoogleCloudDatalabelingV1beta1ListEvaluationJobsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "evaluationJobs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of evaluation jobs to return."]
        pub evaluation_jobs: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1EvaluationJob>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1ListEvaluationJobsResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ListEvaluationJobsResponseBuilder {
            GoogleCloudDatalabelingV1beta1ListEvaluationJobsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results of listing Examples in and annotated dataset."]
    pub struct GoogleCloudDatalabelingV1beta1ListExamplesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "examples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of examples to return."]
        pub examples: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1Example>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1ListExamplesResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ListExamplesResponseBuilder {
            GoogleCloudDatalabelingV1beta1ListExamplesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results for listing FeedbackMessages."]
    pub struct GoogleCloudDatalabelingV1beta1ListFeedbackMessagesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feedbackMessages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of feedback messages to return."]
        pub feedback_messages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1FeedbackMessage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1ListFeedbackMessagesResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ListFeedbackMessagesResponseBuilder {
            GoogleCloudDatalabelingV1beta1ListFeedbackMessagesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results for listing FeedbackThreads."]
    pub struct GoogleCloudDatalabelingV1beta1ListFeedbackThreadsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feedbackThreads")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of feedback threads to return."]
        pub feedback_threads: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1FeedbackThread>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1ListFeedbackThreadsResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ListFeedbackThreadsResponseBuilder {
            GoogleCloudDatalabelingV1beta1ListFeedbackThreadsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results of listing instructions under a project."]
    pub struct GoogleCloudDatalabelingV1beta1ListInstructionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instructions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Instructions to return."]
        pub instructions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1Instruction>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1ListInstructionsResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ListInstructionsResponseBuilder {
            GoogleCloudDatalabelingV1beta1ListInstructionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Normalized bounding polygon."]
    pub struct GoogleCloudDatalabelingV1beta1NormalizedBoundingPoly {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedVertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon normalized vertices."]
        pub normalized_vertices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1NormalizedVertex>>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1NormalizedBoundingPoly {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1NormalizedBoundingPolyBuilder {
            GoogleCloudDatalabelingV1beta1NormalizedBoundingPolyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Normalized polyline."]
    pub struct GoogleCloudDatalabelingV1beta1NormalizedPolyline {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedVertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normalized polyline vertices."]
        pub normalized_vertices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1NormalizedVertex>>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1NormalizedPolyline {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1NormalizedPolylineBuilder {
            GoogleCloudDatalabelingV1beta1NormalizedPolylineBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."]
    pub struct GoogleCloudDatalabelingV1beta1NormalizedVertex {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "x")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "X coordinate."]
        pub x: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "y")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Y coordinate."]
        pub y: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudDatalabelingV1beta1NormalizedVertex {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1NormalizedVertexBuilder {
            GoogleCloudDatalabelingV1beta1NormalizedVertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for video object detection human labeling task. Object detection will be conducted on the images extracted from the video, and those objects will be labeled with bounding boxes. User need to specify the number of images to be extracted per second as the extraction frame rate."]
    pub struct GoogleCloudDatalabelingV1beta1ObjectDetectionConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpecSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Annotation spec set resource name."]
        pub annotation_spec_set: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "extractionFrameRate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Number of frames per second to be extracted from the video."]
        pub extraction_frame_rate: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudDatalabelingV1beta1ObjectDetectionConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ObjectDetectionConfigBuilder {
            GoogleCloudDatalabelingV1beta1ObjectDetectionConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metrics calculated for an image object detection (bounding box) model."]
    pub struct GoogleCloudDatalabelingV1beta1ObjectDetectionMetrics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prCurve")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Precision-recall curve."]
        pub pr_curve:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1PrCurve>>,
    }
    impl GoogleCloudDatalabelingV1beta1ObjectDetectionMetrics {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ObjectDetectionMetricsBuilder {
            GoogleCloudDatalabelingV1beta1ObjectDetectionMetricsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for video object tracking human labeling task."]
    pub struct GoogleCloudDatalabelingV1beta1ObjectTrackingConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpecSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Annotation spec set resource name."]
        pub annotation_spec_set: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clipLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Videos will be cut to smaller clips to make it easier for labelers to work on. Users can configure is field in seconds, if not set, default value is 20s."]
        pub clip_length: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "overlapLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The overlap length between different video clips. Users can configure is field in seconds, if not set, default value is 0.3s."]
        pub overlap_length: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDatalabelingV1beta1ObjectTrackingConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ObjectTrackingConfigBuilder {
            GoogleCloudDatalabelingV1beta1ObjectTrackingConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotation for object detection and tracking."]
    pub struct GoogleCloudDatalabelingV1beta1ObjectTrackingFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedBoundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub normalized_bounding_poly: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1NormalizedBoundingPoly>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time offset of this frame relative to the beginning of the video."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1ObjectTrackingFrame {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ObjectTrackingFrameBuilder {
            GoogleCloudDatalabelingV1beta1ObjectTrackingFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata describing the feedback from the operator."]
    pub struct GoogleCloudDatalabelingV1beta1OperatorFeedbackMetadata {}
    impl GoogleCloudDatalabelingV1beta1OperatorFeedbackMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1OperatorFeedbackMetadataBuilder {
            GoogleCloudDatalabelingV1beta1OperatorFeedbackMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "General information useful for labels coming from contributors."]
    pub struct GoogleCloudDatalabelingV1beta1OperatorMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Comments from contributors."]
        pub comments: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelVotes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of contributors that choose this label."]
        pub label_votes: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence score corresponding to a label. For examle, if 3 contributors have answered the question and 2 of them agree on the final label, the confidence score will be 0.67 (2/3)."]
        pub score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalVotes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of contributors that answer this question."]
        pub total_votes: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDatalabelingV1beta1OperatorMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1OperatorMetadataBuilder {
            GoogleCloudDatalabelingV1beta1OperatorMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The configuration of output data."]
    pub struct GoogleCloudDatalabelingV1beta1OutputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output to a file in Cloud Storage. Should be used for labeling output other than image segmentation."]
        pub gcs_destination:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1GcsDestination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsFolderDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output to a folder in Cloud Storage. Should be used for image segmentation or document de-identification labeling outputs."]
        pub gcs_folder_destination: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1GcsFolderDestination>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1OutputConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1OutputConfigBuilder {
            GoogleCloudDatalabelingV1beta1OutputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for PauseEvaluationJob."]
    pub struct GoogleCloudDatalabelingV1beta1PauseEvaluationJobRequest {}
    impl GoogleCloudDatalabelingV1beta1PauseEvaluationJobRequest {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1PauseEvaluationJobRequestBuilder {
            GoogleCloudDatalabelingV1beta1PauseEvaluationJobRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Instruction from a PDF file."]
    pub struct GoogleCloudDatalabelingV1beta1PdfInstruction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsFileUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "PDF file for the instruction. Only gcs path is allowed."]
        pub gcs_file_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1PdfInstruction {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1PdfInstructionBuilder {
            GoogleCloudDatalabelingV1beta1PdfInstructionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A line with multiple line segments."]
    pub struct GoogleCloudDatalabelingV1beta1Polyline {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The polyline vertices."]
        pub vertices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1Vertex>>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1Polyline {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1PolylineBuilder {
            GoogleCloudDatalabelingV1beta1PolylineBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for image polyline human labeling task."]
    pub struct GoogleCloudDatalabelingV1beta1PolylineConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpecSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Annotation spec set resource name."]
        pub annotation_spec_set: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instructionMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Instruction message showed on contributors UI."]
        pub instruction_message: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1PolylineConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1PolylineConfigBuilder {
            GoogleCloudDatalabelingV1beta1PolylineConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudDatalabelingV1beta1PrCurve {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The annotation spec of the label for which the precision-recall curve calculated. If this field is empty, that means the precision-recall curve is an aggregate curve for all labels."]
        pub annotation_spec:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotationSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "areaUnderCurve")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Area under the precision-recall curve. Not to be confused with area under a receiver operating characteristic (ROC) curve."]
        pub area_under_curve: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidenceMetricsEntries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entries that make up the precision-recall graph. Each entry is a \"point\" on the graph drawn for a different `confidence_threshold`."]
        pub confidence_metrics_entries: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDatalabelingV1beta1ConfidenceMetricsEntry>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meanAveragePrecision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mean average prcision of this curve."]
        pub mean_average_precision: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudDatalabelingV1beta1PrCurve {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1PrCurveBuilder {
            GoogleCloudDatalabelingV1beta1PrCurveBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata describing the feedback from the labeling task requester."]
    pub struct GoogleCloudDatalabelingV1beta1RequesterFeedbackMetadata {}
    impl GoogleCloudDatalabelingV1beta1RequesterFeedbackMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1RequesterFeedbackMetadataBuilder {
            GoogleCloudDatalabelingV1beta1RequesterFeedbackMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message ResumeEvaluationJob."]
    pub struct GoogleCloudDatalabelingV1beta1ResumeEvaluationJobRequest {}
    impl GoogleCloudDatalabelingV1beta1ResumeEvaluationJobRequest {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1ResumeEvaluationJobRequestBuilder {
            GoogleCloudDatalabelingV1beta1ResumeEvaluationJobRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A row in the confusion matrix. Each entry in this row has the same ground truth label."]
    pub struct GoogleCloudDatalabelingV1beta1Row {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The annotation spec of the ground truth label for this row."]
        pub annotation_spec:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotationSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of the confusion matrix entries. One entry for each possible predicted label."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1ConfusionMatrixEntry>>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1Row {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1RowBuilder {
            GoogleCloudDatalabelingV1beta1RowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results of searching evaluations."]
    pub struct GoogleCloudDatalabelingV1beta1SearchEvaluationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "evaluations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of evaluations matching the search."]
        pub evaluations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1Evaluation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1SearchEvaluationsResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1SearchEvaluationsResponseBuilder {
            GoogleCloudDatalabelingV1beta1SearchEvaluationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message of SearchExampleComparisons."]
    pub struct GoogleCloudDatalabelingV1beta1SearchExampleComparisonsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Requested page size. Server may return fewer results than requested. Default value is 100."]
        pub page_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A token identifying a page of results for the server to return. Typically obtained by the nextPageToken of the response to a previous search rquest. If you don't specify this field, the API call requests the first page of the search."]
        pub page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1SearchExampleComparisonsRequest {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1SearchExampleComparisonsRequestBuilder {
            GoogleCloudDatalabelingV1beta1SearchExampleComparisonsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results of searching example comparisons."]
    pub struct GoogleCloudDatalabelingV1beta1SearchExampleComparisonsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exampleComparisons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of example comparisons matching the search criteria."]
        pub example_comparisons: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1ExampleComparison>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1SearchExampleComparisonsResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1SearchExampleComparisonsResponseBuilder {
            GoogleCloudDatalabelingV1beta1SearchExampleComparisonsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for image segmentation"]
    pub struct GoogleCloudDatalabelingV1beta1SegmentationConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpecSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Annotation spec set resource name. format: projects/{project_id}/annotationSpecSets/{annotation_spec_set_id}"]
        pub annotation_spec_set: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instructionMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instruction message showed on labelers UI."]
        pub instruction_message: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1SegmentationConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1SegmentationConfigBuilder {
            GoogleCloudDatalabelingV1beta1SegmentationConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for setting up sentiments."]
    pub struct GoogleCloudDatalabelingV1beta1SentimentConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableLabelSentimentSelection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set to true, contributors will have the option to select sentiment of the label they selected, to mark it as negative or positive label. Default is false."]
        pub enable_label_sentiment_selection: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudDatalabelingV1beta1SentimentConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1SentimentConfigBuilder {
            GoogleCloudDatalabelingV1beta1SentimentConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Start and end position in a sequence (e.g. text segment)."]
    pub struct GoogleCloudDatalabelingV1beta1SequentialSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "end")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End position (exclusive)."]
        pub end: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "start")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start position (inclusive)."]
        pub start: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDatalabelingV1beta1SequentialSegment {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1SequentialSegmentBuilder {
            GoogleCloudDatalabelingV1beta1SequentialSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Text classification annotation."]
    pub struct GoogleCloudDatalabelingV1beta1TextClassificationAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label of the text."]
        pub annotation_spec:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotationSpec>>,
    }
    impl GoogleCloudDatalabelingV1beta1TextClassificationAnnotation {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1TextClassificationAnnotationBuilder {
            GoogleCloudDatalabelingV1beta1TextClassificationAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for text classification human labeling task."]
    pub struct GoogleCloudDatalabelingV1beta1TextClassificationConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowMultiLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If allow_multi_label is true, contributors are able to choose multiple labels for one text segment."]
        pub allow_multi_label: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpecSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Annotation spec set resource name."]
        pub annotation_spec_set: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sentimentConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Configs for sentiment selection. We deprecate sentiment analysis in data labeling side as it is incompatible with uCAIP."]
        pub sentiment_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1SentimentConfig>>,
    }
    impl GoogleCloudDatalabelingV1beta1TextClassificationConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1TextClassificationConfigBuilder {
            GoogleCloudDatalabelingV1beta1TextClassificationConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Text entity extraction annotation."]
    pub struct GoogleCloudDatalabelingV1beta1TextEntityExtractionAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label of the text entities."]
        pub annotation_spec:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotationSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sequentialSegment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Position of the entity."]
        pub sequential_segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1beta1SequentialSegment>,
        >,
    }
    impl GoogleCloudDatalabelingV1beta1TextEntityExtractionAnnotation {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1TextEntityExtractionAnnotationBuilder {
            GoogleCloudDatalabelingV1beta1TextEntityExtractionAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for text entity extraction human labeling task."]
    pub struct GoogleCloudDatalabelingV1beta1TextEntityExtractionConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpecSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Annotation spec set resource name."]
        pub annotation_spec_set: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1TextEntityExtractionConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1TextEntityExtractionConfigBuilder {
            GoogleCloudDatalabelingV1beta1TextEntityExtractionConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for the text."]
    pub struct GoogleCloudDatalabelingV1beta1TextMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language of this text, as a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). Default value is en-US."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1TextMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1TextMetadataBuilder {
            GoogleCloudDatalabelingV1beta1TextMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Container of information about a piece of text."]
    pub struct GoogleCloudDatalabelingV1beta1TextPayload {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text content."]
        pub text_content: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1TextPayload {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1TextPayloadBuilder {
            GoogleCloudDatalabelingV1beta1TextPayloadBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A time period inside of an example that has a time dimension (e.g. video)."]
    pub struct GoogleCloudDatalabelingV1beta1TimeSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTimeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End of the time segment (exclusive), represented as the duration since the example start."]
        pub end_time_offset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTimeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start of the time segment (inclusive), represented as the duration since the example start."]
        pub start_time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1TimeSegment {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1TimeSegmentBuilder {
            GoogleCloudDatalabelingV1beta1TimeSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the vertex coordinates are in the same scale as the original image."]
    pub struct GoogleCloudDatalabelingV1beta1Vertex {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "x")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "X coordinate."]
        pub x: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "y")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Y coordinate."]
        pub y: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDatalabelingV1beta1Vertex {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1VertexBuilder {
            GoogleCloudDatalabelingV1beta1VertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video classification annotation."]
    pub struct GoogleCloudDatalabelingV1beta1VideoClassificationAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label of the segment specified by time_segment."]
        pub annotation_spec:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotationSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSegment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time segment of the video to which the annotation applies."]
        pub time_segment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1TimeSegment>>,
    }
    impl GoogleCloudDatalabelingV1beta1VideoClassificationAnnotation {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1VideoClassificationAnnotationBuilder {
            GoogleCloudDatalabelingV1beta1VideoClassificationAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for video classification human labeling task. Currently two types of video classification are supported: 1. Assign labels on the entire video. 2. Split the video into multiple video clips based on camera shot, and assign labels on each video clip."]
    pub struct GoogleCloudDatalabelingV1beta1VideoClassificationConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpecSetConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The list of annotation spec set configs. Since watching a video clip takes much longer time than an image, we support label with multiple AnnotationSpecSet at the same time. Labels in each AnnotationSpecSet will be shown in a group to contributors. Contributors can select one or more (depending on whether to allow multi label) from each group."]
        pub annotation_spec_set_configs: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotationSpecSetConfig>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applyShotDetection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Option to apply shot detection on the video."]
        pub apply_shot_detection: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudDatalabelingV1beta1VideoClassificationConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1VideoClassificationConfigBuilder {
            GoogleCloudDatalabelingV1beta1VideoClassificationConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video event annotation."]
    pub struct GoogleCloudDatalabelingV1beta1VideoEventAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label of the event in this annotation."]
        pub annotation_spec:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotationSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSegment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time segment of the video to which the annotation applies."]
        pub time_segment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1TimeSegment>>,
    }
    impl GoogleCloudDatalabelingV1beta1VideoEventAnnotation {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1VideoEventAnnotationBuilder {
            GoogleCloudDatalabelingV1beta1VideoEventAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video object tracking annotation."]
    pub struct GoogleCloudDatalabelingV1beta1VideoObjectTrackingAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label of the object tracked in this annotation."]
        pub annotation_spec:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1AnnotationSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectTrackingFrames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of frames where this object track appears."]
        pub object_tracking_frames: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1ObjectTrackingFrame>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSegment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time segment of the video to which object tracking applies."]
        pub time_segment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1beta1TimeSegment>>,
    }
    impl GoogleCloudDatalabelingV1beta1VideoObjectTrackingAnnotation {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1VideoObjectTrackingAnnotationBuilder {
            GoogleCloudDatalabelingV1beta1VideoObjectTrackingAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Container of information of a video."]
    pub struct GoogleCloudDatalabelingV1beta1VideoPayload {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frameRate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "FPS of the video."]
        pub frame_rate: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video format."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signedUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Signed uri of the video file in the service bucket."]
        pub signed_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoThumbnails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of video thumbnails."]
        pub video_thumbnails: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatalabelingV1beta1VideoThumbnail>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video uri from the user bucket."]
        pub video_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1VideoPayload {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1VideoPayloadBuilder {
            GoogleCloudDatalabelingV1beta1VideoPayloadBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Container of information of a video thumbnail."]
    pub struct GoogleCloudDatalabelingV1beta1VideoThumbnail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A byte string of the video frame."]
        pub thumbnail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time offset relative to the beginning of the video, corresponding to the video frame where the thumbnail has been extracted from."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1beta1VideoThumbnail {
        pub fn builder() -> GoogleCloudDatalabelingV1beta1VideoThumbnailBuilder {
            GoogleCloudDatalabelingV1beta1VideoThumbnailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of a CreateInstruction operation."]
    pub struct GoogleCloudDatalabelingV1p1alpha1CreateInstructionMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp when create instruction request was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instruction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the created Instruction. projects/{project_id}/instructions/{instruction_id}"]
        pub instruction: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Partial failures encountered. E.g. single files that couldn't be read. Status details field will contain standard GCP error details."]
        pub partial_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    }
    impl GoogleCloudDatalabelingV1p1alpha1CreateInstructionMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1p1alpha1CreateInstructionMetadataBuilder {
            GoogleCloudDatalabelingV1p1alpha1CreateInstructionMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of an ExportData operation."]
    pub struct GoogleCloudDatalabelingV1p1alpha1ExportDataOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of annotated dataset in format \"projects/*/datasets/*/annotatedDatasets/*\"."]
        pub annotated_dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp when export dataset request was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of dataset to be exported. \"projects/*/datasets/*\""]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Partial failures encountered. E.g. single files that couldn't be read. Status details field will contain standard GCP error details."]
        pub partial_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    }
    impl GoogleCloudDatalabelingV1p1alpha1ExportDataOperationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1p1alpha1ExportDataOperationMetadataBuilder {
            GoogleCloudDatalabelingV1p1alpha1ExportDataOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response used for ExportDataset longrunning operation."]
    pub struct GoogleCloudDatalabelingV1p1alpha1ExportDataOperationResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of annotated dataset in format \"projects/*/datasets/*/annotatedDatasets/*\"."]
        pub annotated_dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ouptut only. The name of dataset. \"projects/*/datasets/*\""]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exportCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Number of examples exported successfully."]
        pub export_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelStats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Statistic infos of labels in the exported dataset."]
        pub label_stats:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1p1alpha1LabelStats>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. output_config in the ExportData request."]
        pub output_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1p1alpha1OutputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Total number of examples requested to export"]
        pub total_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDatalabelingV1p1alpha1ExportDataOperationResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1p1alpha1ExportDataOperationResponseBuilder {
            GoogleCloudDatalabelingV1p1alpha1ExportDataOperationResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Export destination of the data.Only gcs path is allowed in output_uri."]
    pub struct GoogleCloudDatalabelingV1p1alpha1GcsDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The format of the gcs destination. Only \"text/csv\" and \"application/json\" are supported."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The output uri of destination file."]
        pub output_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1p1alpha1GcsDestination {
        pub fn builder() -> GoogleCloudDatalabelingV1p1alpha1GcsDestinationBuilder {
            GoogleCloudDatalabelingV1p1alpha1GcsDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Export folder destination of the data."]
    pub struct GoogleCloudDatalabelingV1p1alpha1GcsFolderDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputFolderUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Cloud Storage directory to export data to."]
        pub output_folder_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1p1alpha1GcsFolderDestination {
        pub fn builder() -> GoogleCloudDatalabelingV1p1alpha1GcsFolderDestinationBuilder {
            GoogleCloudDatalabelingV1p1alpha1GcsFolderDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of an GenerateAnalysisReport operation."]
    pub struct GoogleCloudDatalabelingV1p1alpha1GenerateAnalysisReportOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp when generate report request was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the dataset for which the analysis report is generated. Format: \"projects/*/datasets/*\""]
        pub dataset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1p1alpha1GenerateAnalysisReportOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p1alpha1GenerateAnalysisReportOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p1alpha1GenerateAnalysisReportOperationMetadataBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for how human labeling task should be done."]
    pub struct GoogleCloudDatalabelingV1p1alpha1HumanAnnotationConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDatasetDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A human-readable description for AnnotatedDataset. The description can be up to 10000 characters long."]
        pub annotated_dataset_description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDatasetDisplayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A human-readable name for AnnotatedDataset defined by users. Maximum of 64 characters ."]
        pub annotated_dataset_display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contributorEmails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If you want your own labeling contributors to manage and work on this labeling request, you can set these contributors here. We will give them access to the question types in crowdcompute. Note that these emails must be registered in crowdcompute worker UI: https://crowd-compute.appspot.com/"]
        pub contributor_emails: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instruction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Instruction resource name."]
        pub instruction: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A human-readable label used to logically group labeling tasks. This string must match the regular expression `[a-zA-Z\\\\d_-]{0,128}`."]
        pub label_group: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The Language of this question, as a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). Default value is en-US. Only need to set this when task is language related. For example, French text classification."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "questionDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Maximum duration for contributors to answer a question. Maximum is 3600 seconds. Default is 3600 seconds."]
        pub question_duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replicaCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Replication of questions. Each question will be sent to up to this number of contributors to label. Aggregated answers will be returned. Default is set to 1. For image related labeling, valid values are 1, 3, 5."]
        pub replica_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEmailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email of the user who started the labeling task and should be notified by email. If empty no notification will be sent."]
        pub user_email_address: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1p1alpha1HumanAnnotationConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1p1alpha1HumanAnnotationConfigBuilder {
            GoogleCloudDatalabelingV1p1alpha1HumanAnnotationConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of an ImportData operation."]
    pub struct GoogleCloudDatalabelingV1p1alpha1ImportDataOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp when import dataset request was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of imported dataset. \"projects/*/datasets/*\""]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Partial failures encountered. E.g. single files that couldn't be read. Status details field will contain standard GCP error details."]
        pub partial_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    }
    impl GoogleCloudDatalabelingV1p1alpha1ImportDataOperationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1p1alpha1ImportDataOperationMetadataBuilder {
            GoogleCloudDatalabelingV1p1alpha1ImportDataOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response used for ImportData longrunning operation."]
    pub struct GoogleCloudDatalabelingV1p1alpha1ImportDataOperationResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ouptut only. The name of imported dataset."]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Number of examples imported successfully."]
        pub import_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Total number of examples requested to import"]
        pub total_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDatalabelingV1p1alpha1ImportDataOperationResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1p1alpha1ImportDataOperationResponseBuilder {
            GoogleCloudDatalabelingV1p1alpha1ImportDataOperationResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelImageBoundingBox operation metadata."]
    pub struct GoogleCloudDatalabelingV1p1alpha1LabelImageBoundingBoxOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p1alpha1LabelImageBoundingBoxOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p1alpha1LabelImageBoundingBoxOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p1alpha1LabelImageBoundingBoxOperationMetadataBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of LabelImageBoundingPoly operation metadata."]
    pub struct GoogleCloudDatalabelingV1p1alpha1LabelImageBoundingPolyOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p1alpha1LabelImageBoundingPolyOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p1alpha1LabelImageBoundingPolyOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p1alpha1LabelImageBoundingPolyOperationMetadataBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of a LabelImageClassification operation."]
    pub struct GoogleCloudDatalabelingV1p1alpha1LabelImageClassificationOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p1alpha1LabelImageClassificationOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p1alpha1LabelImageClassificationOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p1alpha1LabelImageClassificationOperationMetadataBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelImageOrientedBoundingBox operation metadata."]
    pub struct GoogleCloudDatalabelingV1p1alpha1LabelImageOrientedBoundingBoxOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p1alpha1LabelImageOrientedBoundingBoxOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p1alpha1LabelImageOrientedBoundingBoxOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p1alpha1LabelImageOrientedBoundingBoxOperationMetadataBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of LabelImagePolyline operation metadata."]
    pub struct GoogleCloudDatalabelingV1p1alpha1LabelImagePolylineOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p1alpha1LabelImagePolylineOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p1alpha1LabelImagePolylineOperationMetadataBuilder {
            GoogleCloudDatalabelingV1p1alpha1LabelImagePolylineOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelImageSegmentation operation metadata."]
    pub struct GoogleCloudDatalabelingV1p1alpha1LabelImageSegmentationOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p1alpha1LabelImageSegmentationOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p1alpha1LabelImageSegmentationOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p1alpha1LabelImageSegmentationOperationMetadataBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of a labeling operation, such as LabelImage or LabelVideo. Next tag: 23"]
    pub struct GoogleCloudDatalabelingV1p1alpha1LabelOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of annotated dataset in format \"projects/*/datasets/*/annotatedDatasets/*\"."]
        pub annotated_dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp when labeling request was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of dataset to be labeled. \"projects/*/datasets/*\""]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageBoundingBoxDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image bounding box operation."]
        pub image_bounding_box_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p1alpha1LabelImageBoundingBoxOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageBoundingPolyDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image bounding poly operation."]
        pub image_bounding_poly_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p1alpha1LabelImageBoundingPolyOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageClassificationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image classification operation."]
        pub image_classification_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p1alpha1LabelImageClassificationOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageOrientedBoundingBoxDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image oriented bounding box operation."]
        pub image_oriented_bounding_box_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p1alpha1LabelImageOrientedBoundingBoxOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imagePolylineDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image polyline operation."]
        pub image_polyline_details: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p1alpha1LabelImagePolylineOperationMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageSegmentationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image segmentation operation."]
        pub image_segmentation_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p1alpha1LabelImageSegmentationOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Partial failures encountered. E.g. single files that couldn't be read. Status details field will contain standard GCP error details."]
        pub partial_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressPercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Progress of label operation. Range: [0, 100]."]
        pub progress_percent: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textClassificationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label text classification operation."]
        pub text_classification_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p1alpha1LabelTextClassificationOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textEntityExtractionDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label text entity extraction operation."]
        pub text_entity_extraction_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p1alpha1LabelTextEntityExtractionOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoClassificationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label video classification operation."]
        pub video_classification_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p1alpha1LabelVideoClassificationOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoEventDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label video event operation."]
        pub video_event_details: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p1alpha1LabelVideoEventOperationMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoObjectDetectionDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label video object detection operation."]
        pub video_object_detection_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p1alpha1LabelVideoObjectDetectionOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoObjectTrackingDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label video object tracking operation."]
        pub video_object_tracking_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p1alpha1LabelVideoObjectTrackingOperationMetadata,
            >,
        >,
    }
    impl GoogleCloudDatalabelingV1p1alpha1LabelOperationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1p1alpha1LabelOperationMetadataBuilder {
            GoogleCloudDatalabelingV1p1alpha1LabelOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Statistics about annotation specs."]
    pub struct GoogleCloudDatalabelingV1p1alpha1LabelStats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exampleCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map of each annotation spec's example count. Key is the annotation spec name and value is the number of examples for that annotation spec. If the annotated dataset does not have annotation spec, the map will return a pair where the key is empty string and value is the total number of annotations."]
        pub example_count:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl GoogleCloudDatalabelingV1p1alpha1LabelStats {
        pub fn builder() -> GoogleCloudDatalabelingV1p1alpha1LabelStatsBuilder {
            GoogleCloudDatalabelingV1p1alpha1LabelStatsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelTextClassification operation metadata."]
    pub struct GoogleCloudDatalabelingV1p1alpha1LabelTextClassificationOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p1alpha1LabelTextClassificationOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p1alpha1LabelTextClassificationOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p1alpha1LabelTextClassificationOperationMetadataBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelTextEntityExtraction operation metadata."]
    pub struct GoogleCloudDatalabelingV1p1alpha1LabelTextEntityExtractionOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p1alpha1LabelTextEntityExtractionOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p1alpha1LabelTextEntityExtractionOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p1alpha1LabelTextEntityExtractionOperationMetadataBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelVideoClassification operation metadata."]
    pub struct GoogleCloudDatalabelingV1p1alpha1LabelVideoClassificationOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p1alpha1LabelVideoClassificationOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p1alpha1LabelVideoClassificationOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p1alpha1LabelVideoClassificationOperationMetadataBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelVideoEvent operation metadata."]
    pub struct GoogleCloudDatalabelingV1p1alpha1LabelVideoEventOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p1alpha1LabelVideoEventOperationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1p1alpha1LabelVideoEventOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p1alpha1LabelVideoEventOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelVideoObjectDetection operation metadata."]
    pub struct GoogleCloudDatalabelingV1p1alpha1LabelVideoObjectDetectionOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p1alpha1LabelVideoObjectDetectionOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p1alpha1LabelVideoObjectDetectionOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p1alpha1LabelVideoObjectDetectionOperationMetadataBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelVideoObjectTracking operation metadata."]
    pub struct GoogleCloudDatalabelingV1p1alpha1LabelVideoObjectTrackingOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p1alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p1alpha1LabelVideoObjectTrackingOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p1alpha1LabelVideoObjectTrackingOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p1alpha1LabelVideoObjectTrackingOperationMetadataBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The configuration of output data."]
    pub struct GoogleCloudDatalabelingV1p1alpha1OutputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output to a file in Cloud Storage. Should be used for labeling output other than image segmentation."]
        pub gcs_destination: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p1alpha1GcsDestination>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsFolderDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output to a folder in Cloud Storage. Should be used for image segmentation or document de-identification labeling outputs."]
        pub gcs_folder_destination: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p1alpha1GcsFolderDestination>,
        >,
    }
    impl GoogleCloudDatalabelingV1p1alpha1OutputConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1p1alpha1OutputConfigBuilder {
            GoogleCloudDatalabelingV1p1alpha1OutputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of a CreateInstruction operation."]
    pub struct GoogleCloudDatalabelingV1p2alpha1CreateInstructionMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp when create instruction request was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instruction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the created Instruction. projects/{project_id}/instructions/{instruction_id}"]
        pub instruction: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Partial failures encountered. E.g. single files that couldn't be read. Status details field will contain standard GCP error details."]
        pub partial_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    }
    impl GoogleCloudDatalabelingV1p2alpha1CreateInstructionMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1p2alpha1CreateInstructionMetadataBuilder {
            GoogleCloudDatalabelingV1p2alpha1CreateInstructionMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of an ExportData operation."]
    pub struct GoogleCloudDatalabelingV1p2alpha1ExportDataOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of annotated dataset in format \"projects/*/datasets/*/annotatedDatasets/*\"."]
        pub annotated_dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp when export dataset request was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of dataset to be exported. \"projects/*/datasets/*\""]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Partial failures encountered. E.g. single files that couldn't be read. Status details field will contain standard GCP error details."]
        pub partial_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    }
    impl GoogleCloudDatalabelingV1p2alpha1ExportDataOperationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1p2alpha1ExportDataOperationMetadataBuilder {
            GoogleCloudDatalabelingV1p2alpha1ExportDataOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response used for ExportDataset longrunning operation."]
    pub struct GoogleCloudDatalabelingV1p2alpha1ExportDataOperationResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of annotated dataset in format \"projects/*/datasets/*/annotatedDatasets/*\"."]
        pub annotated_dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ouptut only. The name of dataset. \"projects/*/datasets/*\""]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exportCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Number of examples exported successfully."]
        pub export_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelStats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Statistic infos of labels in the exported dataset."]
        pub label_stats:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1p2alpha1LabelStats>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. output_config in the ExportData request."]
        pub output_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatalabelingV1p2alpha1OutputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Total number of examples requested to export"]
        pub total_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDatalabelingV1p2alpha1ExportDataOperationResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1p2alpha1ExportDataOperationResponseBuilder {
            GoogleCloudDatalabelingV1p2alpha1ExportDataOperationResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Export destination of the data.Only gcs path is allowed in output_uri."]
    pub struct GoogleCloudDatalabelingV1p2alpha1GcsDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The format of the gcs destination. Only \"text/csv\" and \"application/json\" are supported."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The output uri of destination file."]
        pub output_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1p2alpha1GcsDestination {
        pub fn builder() -> GoogleCloudDatalabelingV1p2alpha1GcsDestinationBuilder {
            GoogleCloudDatalabelingV1p2alpha1GcsDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Export folder destination of the data."]
    pub struct GoogleCloudDatalabelingV1p2alpha1GcsFolderDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputFolderUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Cloud Storage directory to export data to."]
        pub output_folder_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1p2alpha1GcsFolderDestination {
        pub fn builder() -> GoogleCloudDatalabelingV1p2alpha1GcsFolderDestinationBuilder {
            GoogleCloudDatalabelingV1p2alpha1GcsFolderDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for how human labeling task should be done."]
    pub struct GoogleCloudDatalabelingV1p2alpha1HumanAnnotationConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDatasetDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A human-readable description for AnnotatedDataset. The description can be up to 10000 characters long."]
        pub annotated_dataset_description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDatasetDisplayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A human-readable name for AnnotatedDataset defined by users. Maximum of 64 characters ."]
        pub annotated_dataset_display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contributorEmails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If you want your own labeling contributors to manage and work on this labeling request, you can set these contributors here. We will give them access to the question types in crowdcompute. Note that these emails must be registered in crowdcompute worker UI: https://crowd-compute.appspot.com/"]
        pub contributor_emails: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instruction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Instruction resource name."]
        pub instruction: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A human-readable label used to logically group labeling tasks. This string must match the regular expression `[a-zA-Z\\\\d_-]{0,128}`."]
        pub label_group: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The Language of this question, as a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). Default value is en-US. Only need to set this when task is language related. For example, French text classification."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "questionDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Maximum duration for contributors to answer a question. Maximum is 3600 seconds. Default is 3600 seconds."]
        pub question_duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replicaCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Replication of questions. Each question will be sent to up to this number of contributors to label. Aggregated answers will be returned. Default is set to 1. For image related labeling, valid values are 1, 3, 5."]
        pub replica_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEmailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email of the user who started the labeling task and should be notified by email. If empty no notification will be sent."]
        pub user_email_address: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatalabelingV1p2alpha1HumanAnnotationConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1p2alpha1HumanAnnotationConfigBuilder {
            GoogleCloudDatalabelingV1p2alpha1HumanAnnotationConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of an ImportData operation."]
    pub struct GoogleCloudDatalabelingV1p2alpha1ImportDataOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp when import dataset request was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of imported dataset. \"projects/*/datasets/*\""]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Partial failures encountered. E.g. single files that couldn't be read. Status details field will contain standard GCP error details."]
        pub partial_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    }
    impl GoogleCloudDatalabelingV1p2alpha1ImportDataOperationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1p2alpha1ImportDataOperationMetadataBuilder {
            GoogleCloudDatalabelingV1p2alpha1ImportDataOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response used for ImportData longrunning operation."]
    pub struct GoogleCloudDatalabelingV1p2alpha1ImportDataOperationResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ouptut only. The name of imported dataset."]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Number of examples imported successfully."]
        pub import_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Total number of examples requested to import"]
        pub total_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDatalabelingV1p2alpha1ImportDataOperationResponse {
        pub fn builder() -> GoogleCloudDatalabelingV1p2alpha1ImportDataOperationResponseBuilder {
            GoogleCloudDatalabelingV1p2alpha1ImportDataOperationResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelImageBoundingBox operation metadata."]
    pub struct GoogleCloudDatalabelingV1p2alpha1LabelImageBoundingBoxOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p2alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p2alpha1LabelImageBoundingBoxOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p2alpha1LabelImageBoundingBoxOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p2alpha1LabelImageBoundingBoxOperationMetadataBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of LabelImageBoundingPoly operation metadata."]
    pub struct GoogleCloudDatalabelingV1p2alpha1LabelImageBoundingPolyOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p2alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p2alpha1LabelImageBoundingPolyOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p2alpha1LabelImageBoundingPolyOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p2alpha1LabelImageBoundingPolyOperationMetadataBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of a LabelImageClassification operation."]
    pub struct GoogleCloudDatalabelingV1p2alpha1LabelImageClassificationOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p2alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p2alpha1LabelImageClassificationOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p2alpha1LabelImageClassificationOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p2alpha1LabelImageClassificationOperationMetadataBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelImageOrientedBoundingBox operation metadata."]
    pub struct GoogleCloudDatalabelingV1p2alpha1LabelImageOrientedBoundingBoxOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p2alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p2alpha1LabelImageOrientedBoundingBoxOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p2alpha1LabelImageOrientedBoundingBoxOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p2alpha1LabelImageOrientedBoundingBoxOperationMetadataBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of LabelImagePolyline operation metadata."]
    pub struct GoogleCloudDatalabelingV1p2alpha1LabelImagePolylineOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p2alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p2alpha1LabelImagePolylineOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p2alpha1LabelImagePolylineOperationMetadataBuilder {
            GoogleCloudDatalabelingV1p2alpha1LabelImagePolylineOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelImageSegmentation operation metadata."]
    pub struct GoogleCloudDatalabelingV1p2alpha1LabelImageSegmentationOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p2alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p2alpha1LabelImageSegmentationOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p2alpha1LabelImageSegmentationOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p2alpha1LabelImageSegmentationOperationMetadataBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of a labeling operation, such as LabelImage or LabelVideo. Next tag: 23"]
    pub struct GoogleCloudDatalabelingV1p2alpha1LabelOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotatedDataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of annotated dataset in format \"projects/*/datasets/*/annotatedDatasets/*\"."]
        pub annotated_dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamp when labeling request was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of dataset to be labeled. \"projects/*/datasets/*\""]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageBoundingBoxDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image bounding box operation."]
        pub image_bounding_box_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p2alpha1LabelImageBoundingBoxOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageBoundingPolyDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image bounding poly operation."]
        pub image_bounding_poly_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p2alpha1LabelImageBoundingPolyOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageClassificationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image classification operation."]
        pub image_classification_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p2alpha1LabelImageClassificationOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageOrientedBoundingBoxDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image oriented bounding box operation."]
        pub image_oriented_bounding_box_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p2alpha1LabelImageOrientedBoundingBoxOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imagePolylineDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image polyline operation."]
        pub image_polyline_details: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p2alpha1LabelImagePolylineOperationMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageSegmentationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label image segmentation operation."]
        pub image_segmentation_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p2alpha1LabelImageSegmentationOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Partial failures encountered. E.g. single files that couldn't be read. Status details field will contain standard GCP error details."]
        pub partial_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressPercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Progress of label operation. Range: [0, 100]."]
        pub progress_percent: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textClassificationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label text classification operation."]
        pub text_classification_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p2alpha1LabelTextClassificationOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textEntityExtractionDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label text entity extraction operation."]
        pub text_entity_extraction_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p2alpha1LabelTextEntityExtractionOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoClassificationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label video classification operation."]
        pub video_classification_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p2alpha1LabelVideoClassificationOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoEventDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label video event operation."]
        pub video_event_details: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p2alpha1LabelVideoEventOperationMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoObjectDetectionDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label video object detection operation."]
        pub video_object_detection_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p2alpha1LabelVideoObjectDetectionOperationMetadata,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoObjectTrackingDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of label video object tracking operation."]
        pub video_object_tracking_details: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDatalabelingV1p2alpha1LabelVideoObjectTrackingOperationMetadata,
            >,
        >,
    }
    impl GoogleCloudDatalabelingV1p2alpha1LabelOperationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1p2alpha1LabelOperationMetadataBuilder {
            GoogleCloudDatalabelingV1p2alpha1LabelOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Statistics about annotation specs."]
    pub struct GoogleCloudDatalabelingV1p2alpha1LabelStats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exampleCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map of each annotation spec's example count. Key is the annotation spec name and value is the number of examples for that annotation spec. If the annotated dataset does not have annotation spec, the map will return a pair where the key is empty string and value is the total number of annotations."]
        pub example_count:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl GoogleCloudDatalabelingV1p2alpha1LabelStats {
        pub fn builder() -> GoogleCloudDatalabelingV1p2alpha1LabelStatsBuilder {
            GoogleCloudDatalabelingV1p2alpha1LabelStatsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelTextClassification operation metadata."]
    pub struct GoogleCloudDatalabelingV1p2alpha1LabelTextClassificationOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p2alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p2alpha1LabelTextClassificationOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p2alpha1LabelTextClassificationOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p2alpha1LabelTextClassificationOperationMetadataBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelTextEntityExtraction operation metadata."]
    pub struct GoogleCloudDatalabelingV1p2alpha1LabelTextEntityExtractionOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p2alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p2alpha1LabelTextEntityExtractionOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p2alpha1LabelTextEntityExtractionOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p2alpha1LabelTextEntityExtractionOperationMetadataBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelVideoClassification operation metadata."]
    pub struct GoogleCloudDatalabelingV1p2alpha1LabelVideoClassificationOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p2alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p2alpha1LabelVideoClassificationOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p2alpha1LabelVideoClassificationOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p2alpha1LabelVideoClassificationOperationMetadataBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelVideoEvent operation metadata."]
    pub struct GoogleCloudDatalabelingV1p2alpha1LabelVideoEventOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p2alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p2alpha1LabelVideoEventOperationMetadata {
        pub fn builder() -> GoogleCloudDatalabelingV1p2alpha1LabelVideoEventOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p2alpha1LabelVideoEventOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelVideoObjectDetection operation metadata."]
    pub struct GoogleCloudDatalabelingV1p2alpha1LabelVideoObjectDetectionOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p2alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p2alpha1LabelVideoObjectDetectionOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p2alpha1LabelVideoObjectDetectionOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p2alpha1LabelVideoObjectDetectionOperationMetadataBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of a LabelVideoObjectTracking operation metadata."]
    pub struct GoogleCloudDatalabelingV1p2alpha1LabelVideoObjectTrackingOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic human annotation config used in labeling request."]
        pub basic_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p2alpha1HumanAnnotationConfig>,
        >,
    }
    impl GoogleCloudDatalabelingV1p2alpha1LabelVideoObjectTrackingOperationMetadata {
        pub fn builder(
        ) -> GoogleCloudDatalabelingV1p2alpha1LabelVideoObjectTrackingOperationMetadataBuilder
        {
            GoogleCloudDatalabelingV1p2alpha1LabelVideoObjectTrackingOperationMetadataBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The configuration of output data."]
    pub struct GoogleCloudDatalabelingV1p2alpha1OutputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output to a file in Cloud Storage. Should be used for labeling output other than image segmentation."]
        pub gcs_destination: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p2alpha1GcsDestination>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsFolderDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output to a folder in Cloud Storage. Should be used for image segmentation or document de-identification labeling outputs."]
        pub gcs_folder_destination: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatalabelingV1p2alpha1GcsFolderDestination>,
        >,
    }
    impl GoogleCloudDatalabelingV1p2alpha1OutputConfig {
        pub fn builder() -> GoogleCloudDatalabelingV1p2alpha1OutputConfigBuilder {
            GoogleCloudDatalabelingV1p2alpha1OutputConfigBuilder::default()
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
}
