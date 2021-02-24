#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
pub struct QueryParameters {
    #[builder(default = "{ query_parameters_defaults :: alt () }", setter(into))]
    #[serde(rename = "alt")]
    #[serde(default = "query_parameters_defaults :: alt")]
    #[doc = "Data format for the response."]
    pub alt: QueryParametersAltEnum,
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
    #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
    pub quota_user: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "userIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use quotaUser instead."]
    pub user_ip: ::std::option::Option<::std::string::String>,
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
#[doc = "Data format for the response."]
pub enum QueryParametersAltEnum {
    #[serde(rename = "json")]
    #[doc = "Responses with Content-Type of application/json"]
    Json,
}
impl ::std::default::Default for QueryParametersAltEnum {
    fn default() -> Self {
        Self::Json
    }
}
pub mod resources {
    pub mod datasets {
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
                    #[serde(rename = "deleteContents")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If True, delete all the tables in the dataset. If False and the dataset contains tables, the request will fail. Default is False"]
                    pub delete_contents: ::std::option::Option<::std::primitive::bool>,
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
                    #[serde(rename = "all")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to list all datasets, including hidden ones"]
                    pub all: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "An expression for filtering the results of the request by label. The syntax is \"labels.<name>[:<value>]\". Multiple filters can be ANDed together by connecting with a space. Example: \"labels.department:receiving labels.active\". See Filtering datasets using labels for details."]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of results to return"]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Page token, returned by a previous call, to request the next page of results"]
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
    pub mod jobs {
        pub mod methods {
            pub mod cancel {
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
                    #[serde(rename = "location")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The geographic location of the job. Required except for US and EU. See details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location."]
                    pub location: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "location")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The geographic location of the job. Required except for US and EU. See details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location."]
                    pub location: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod get_query_results {
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
                    #[serde(rename = "location")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The geographic location where the job should run. Required except for US and EU. See details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location."]
                    pub location: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of results to read"]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Page token, returned by a previous call, to request the next page of results"]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startIndex")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Zero-based index of the starting row"]
                    pub start_index: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timeoutMs")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "How long to wait for the query to complete, in milliseconds, before returning. Default is 10 seconds. If the timeout passes before the job completes, the 'jobComplete' field in the response will be false"]
                    pub timeout_ms: ::std::option::Option<::std::primitive::i64>,
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
                    #[serde(rename = "allUsers")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to display jobs owned by all users in the project. Default false"]
                    pub all_users: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxCreationTime")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Max value for job creation time, in milliseconds since the POSIX epoch. If set, only jobs created before or at this timestamp are returned"]
                    pub max_creation_time: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of results to return"]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "minCreationTime")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Min value for job creation time, in milliseconds since the POSIX epoch. If set, only jobs created after or at this timestamp are returned"]
                    pub min_creation_time: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Page token, returned by a previous call, to request the next page of results"]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "parentJobId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If set, retrieves only jobs whose parent is this job. Otherwise, retrieves only jobs which have no parent"]
                    pub parent_job_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restrict information returned to a set of selected fields"]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "stateFilter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter for job state"]
                    pub state_filter: ::std::option::Option<QueryParametersStateFilterEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Restrict information returned to a set of selected fields"]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "full")]
                    #[doc = "Includes all job data"]
                    Full,
                    #[serde(rename = "minimal")]
                    #[doc = "Does not include the job configuration"]
                    Minimal,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::Full
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Filter for job state"]
                pub enum QueryParametersStateFilterEnum {
                    #[serde(rename = "done")]
                    #[doc = "Finished jobs"]
                    Done,
                    #[serde(rename = "pending")]
                    #[doc = "Pending jobs"]
                    Pending,
                    #[serde(rename = "running")]
                    #[doc = "Running jobs"]
                    Running,
                }
                impl ::std::default::Default for QueryParametersStateFilterEnum {
                    fn default() -> Self {
                        Self::Done
                    }
                }
            }
        }
    }
    pub mod models {
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of results to return in a single response page. Leverage the page tokens to iterate through the entire collection."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Page token, returned by a previous call to request the next page of results"]
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
    pub mod projects {
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of results to return"]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Page token, returned by a previous call, to request the next page of results"]
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
    pub mod routines {
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
                    #[serde(rename = "readMask")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If set, only the Routine fields in the field mask are returned in the response. If unset, all Routine fields are returned."]
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If set, then only the Routines matching this filter are returned. The current supported form is either \"routine_type:\" or \"routineType:\", where is a RoutineType enum. Example: \"routineType:SCALAR_FUNCTION\"."]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of results to return in a single response page. Leverage the page tokens to iterate through the entire collection."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Page token, returned by a previous call, to request the next page of results"]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "readMask")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If set, then only the Routine fields in the field mask, as well as project_id, dataset_id and routine_id, are returned in the response. If unset, then the following Routine fields are returned: etag, project_id, dataset_id, routine_id, routine_type, creation_time, last_modified_time, and language."]
                    pub read_mask: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod row_access_policies {
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
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of results to return in a single response page. Leverage the page tokens to iterate through the entire collection."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Page token, returned by a previous call, to request the next page of results."]
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
    pub mod tabledata {
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of results to return"]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Page token, returned by a previous call, identifying the result set"]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "selectedFields")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "List of fields to return (comma-separated). If unspecified, all fields are returned"]
                    pub selected_fields: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startIndex")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Zero-based index of the starting row to read"]
                    pub start_index: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod tables {
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
                    #[serde(rename = "selectedFields")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "List of fields to return (comma-separated). If unspecified, all fields are returned"]
                    pub selected_fields: ::std::option::Option<::std::string::String>,
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
                    #[doc = "Maximum number of results to return"]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Page token, returned by a previous call, to request the next page of results"]
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
    #[doc = "Aggregate metrics for classification/classifier models. For multi-class models, the metrics are either macro-averaged or micro-averaged. When macro-averaged, the metrics are calculated for each label and then an unweighted average is taken of those values. When micro-averaged, the metric is calculated globally by counting the total number of correctly predicted rows."]
    pub struct AggregateClassificationMetrics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accuracy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Accuracy is the fraction of predictions given the correct label. For multiclass this is a micro-averaged metric."]
        pub accuracy: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "f1Score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The F1 score is an average of recall and precision. For multiclass this is a macro-averaged metric."]
        pub f1_score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logLoss")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Logarithmic Loss. For multiclass this is a macro-averaged metric."]
        pub log_loss: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "precision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Precision is the fraction of actual positive predictions that had positive actual labels. For multiclass this is a macro-averaged metric treating each class as a binary classifier."]
        pub precision: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recall")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Recall is the fraction of actual positive labels that were given a positive prediction. For multiclass this is a macro-averaged metric."]
        pub recall: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rocAuc")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Area Under a ROC Curve. For multiclass this is a macro-averaged metric."]
        pub roc_auc: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Threshold at which the metrics are computed. For binary classification models this is the positive class threshold. For multi-class classfication models this is the confidence threshold."]
        pub threshold: ::std::option::Option<::std::primitive::f64>,
    }
    impl AggregateClassificationMetrics {
        pub fn builder() -> AggregateClassificationMetricsBuilder {
            AggregateClassificationMetricsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Input/output argument of a function or a stored procedure."]
    pub struct Argument {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "argumentKind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Defaults to FIXED_TYPE."]
        pub argument_kind: ::std::option::Option<ArgumentArgumentKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required unless argument_kind = ANY_TYPE."]
        pub data_type: ::std::option::Option<::std::boxed::Box<StandardSqlDataType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Specifies whether the argument is input or output. Can be set for procedures only."]
        pub mode: ::std::option::Option<ArgumentModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The name of this argument. Can be absent for function return argument."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Argument {
        pub fn builder() -> ArgumentBuilder {
            ArgumentBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Defaults to FIXED_TYPE."]
    pub enum ArgumentArgumentKindEnum {
        #[serde(rename = "ARGUMENT_KIND_UNSPECIFIED")]
        #[doc = ""]
        ArgumentKindUnspecified,
        #[serde(rename = "FIXED_TYPE")]
        #[doc = "The argument is a variable with fully specified type, which can be a struct or an array, but not a table."]
        FixedType,
        #[serde(rename = "ANY_TYPE")]
        #[doc = "The argument is any type, including struct or array, but not a table. To be added: FIXED_TABLE, ANY_TABLE"]
        AnyType,
    }
    impl ::std::default::Default for ArgumentArgumentKindEnum {
        fn default() -> Self {
            Self::ArgumentKindUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Specifies whether the argument is input or output. Can be set for procedures only."]
    pub enum ArgumentModeEnum {
        #[serde(rename = "MODE_UNSPECIFIED")]
        #[doc = ""]
        ModeUnspecified,
        #[serde(rename = "IN")]
        #[doc = "The argument is input-only."]
        In,
        #[serde(rename = "OUT")]
        #[doc = "The argument is output-only."]
        Out,
        #[serde(rename = "INOUT")]
        #[doc = "The argument is both an input and an output."]
        Inout,
    }
    impl ::std::default::Default for ArgumentModeEnum {
        fn default() -> Self {
            Self::ModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Arima coefficients."]
    pub struct ArimaCoefficients {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoRegressiveCoefficients")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Auto-regressive coefficients, an array of double."]
        pub auto_regressive_coefficients:
            ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "interceptCoefficient")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Intercept coefficient, just a double not an array."]
        pub intercept_coefficient: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "movingAverageCoefficients")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Moving-average coefficients, an array of double."]
        pub moving_average_coefficients:
            ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
    }
    impl ArimaCoefficients {
        pub fn builder() -> ArimaCoefficientsBuilder {
            ArimaCoefficientsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "ARIMA model fitting metrics."]
    pub struct ArimaFittingMetrics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "AIC."]
        pub aic: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Log-likelihood."]
        pub log_likelihood: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "variance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Variance."]
        pub variance: ::std::option::Option<::std::primitive::f64>,
    }
    impl ArimaFittingMetrics {
        pub fn builder() -> ArimaFittingMetricsBuilder {
            ArimaFittingMetricsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Model evaluation metrics for ARIMA forecasting models."]
    pub struct ArimaForecastingMetrics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arimaFittingMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Arima model fitting metrics."]
        pub arima_fitting_metrics:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ArimaFittingMetrics>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arimaSingleModelForecastingMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Repeated as there can be many metric sets (one for each model) in auto-arima and the large-scale case."]
        pub arima_single_model_forecasting_metrics: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<ArimaSingleModelForecastingMetrics>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasDrift")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether Arima model fitted with drift or not. It is always false when d is not 1."]
        pub has_drift: ::std::option::Option<::std::vec::Vec<::std::primitive::bool>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonSeasonalOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Non-seasonal order."]
        pub non_seasonal_order:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ArimaOrder>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seasonalPeriods")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Seasonal periods. Repeated because multiple periods are supported for one time series."]
        pub seasonal_periods:
            ::std::option::Option<::std::vec::Vec<ArimaForecastingMetricsSeasonalPeriodsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSeriesId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Id to differentiate different time series for the large-scale case."]
        pub time_series_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ArimaForecastingMetrics {
        pub fn builder() -> ArimaForecastingMetricsBuilder {
            ArimaForecastingMetricsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum ArimaForecastingMetricsSeasonalPeriodsEnum {
        #[serde(rename = "SEASONAL_PERIOD_TYPE_UNSPECIFIED")]
        #[doc = ""]
        SeasonalPeriodTypeUnspecified,
        #[serde(rename = "NO_SEASONALITY")]
        #[doc = "No seasonality"]
        NoSeasonality,
        #[serde(rename = "DAILY")]
        #[doc = "Daily period, 24 hours."]
        Daily,
        #[serde(rename = "WEEKLY")]
        #[doc = "Weekly period, 7 days."]
        Weekly,
        #[serde(rename = "MONTHLY")]
        #[doc = "Monthly period, 30 days or irregular."]
        Monthly,
        #[serde(rename = "QUARTERLY")]
        #[doc = "Quarterly period, 90 days or irregular."]
        Quarterly,
        #[serde(rename = "YEARLY")]
        #[doc = "Yearly period, 365 days or irregular."]
        Yearly,
    }
    impl ::std::default::Default for ArimaForecastingMetricsSeasonalPeriodsEnum {
        fn default() -> Self {
            Self::SeasonalPeriodTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Arima model information."]
    pub struct ArimaModelInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arimaCoefficients")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Arima coefficients."]
        pub arima_coefficients: ::std::option::Option<::std::boxed::Box<ArimaCoefficients>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arimaFittingMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Arima fitting metrics."]
        pub arima_fitting_metrics: ::std::option::Option<::std::boxed::Box<ArimaFittingMetrics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasDrift")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether Arima model fitted with drift or not. It is always false when d is not 1."]
        pub has_drift: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonSeasonalOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Non-seasonal order."]
        pub non_seasonal_order: ::std::option::Option<::std::boxed::Box<ArimaOrder>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seasonalPeriods")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Seasonal periods. Repeated because multiple periods are supported for one time series."]
        pub seasonal_periods:
            ::std::option::Option<::std::vec::Vec<ArimaModelInfoSeasonalPeriodsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSeriesId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id to indicate different time series."]
        pub time_series_id: ::std::option::Option<::std::string::String>,
    }
    impl ArimaModelInfo {
        pub fn builder() -> ArimaModelInfoBuilder {
            ArimaModelInfoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum ArimaModelInfoSeasonalPeriodsEnum {
        #[serde(rename = "SEASONAL_PERIOD_TYPE_UNSPECIFIED")]
        #[doc = ""]
        SeasonalPeriodTypeUnspecified,
        #[serde(rename = "NO_SEASONALITY")]
        #[doc = "No seasonality"]
        NoSeasonality,
        #[serde(rename = "DAILY")]
        #[doc = "Daily period, 24 hours."]
        Daily,
        #[serde(rename = "WEEKLY")]
        #[doc = "Weekly period, 7 days."]
        Weekly,
        #[serde(rename = "MONTHLY")]
        #[doc = "Monthly period, 30 days or irregular."]
        Monthly,
        #[serde(rename = "QUARTERLY")]
        #[doc = "Quarterly period, 90 days or irregular."]
        Quarterly,
        #[serde(rename = "YEARLY")]
        #[doc = "Yearly period, 365 days or irregular."]
        Yearly,
    }
    impl ::std::default::Default for ArimaModelInfoSeasonalPeriodsEnum {
        fn default() -> Self {
            Self::SeasonalPeriodTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Arima order, can be used for both non-seasonal and seasonal parts."]
    pub struct ArimaOrder {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "d")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Order of the differencing part."]
        pub d: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "p")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Order of the autoregressive part."]
        pub p: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "q")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Order of the moving-average part."]
        pub q: ::std::option::Option<::std::string::String>,
    }
    impl ArimaOrder {
        pub fn builder() -> ArimaOrderBuilder {
            ArimaOrderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "(Auto-)arima fitting result. Wrap everything in ArimaResult for easier refactoring if we want to use model-specific iteration results."]
    pub struct ArimaResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arimaModelInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This message is repeated because there are multiple arima models fitted in auto-arima. For non-auto-arima model, its size is one."]
        pub arima_model_info:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ArimaModelInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seasonalPeriods")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Seasonal periods. Repeated because multiple periods are supported for one time series."]
        pub seasonal_periods:
            ::std::option::Option<::std::vec::Vec<ArimaResultSeasonalPeriodsEnum>>,
    }
    impl ArimaResult {
        pub fn builder() -> ArimaResultBuilder {
            ArimaResultBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum ArimaResultSeasonalPeriodsEnum {
        #[serde(rename = "SEASONAL_PERIOD_TYPE_UNSPECIFIED")]
        #[doc = ""]
        SeasonalPeriodTypeUnspecified,
        #[serde(rename = "NO_SEASONALITY")]
        #[doc = "No seasonality"]
        NoSeasonality,
        #[serde(rename = "DAILY")]
        #[doc = "Daily period, 24 hours."]
        Daily,
        #[serde(rename = "WEEKLY")]
        #[doc = "Weekly period, 7 days."]
        Weekly,
        #[serde(rename = "MONTHLY")]
        #[doc = "Monthly period, 30 days or irregular."]
        Monthly,
        #[serde(rename = "QUARTERLY")]
        #[doc = "Quarterly period, 90 days or irregular."]
        Quarterly,
        #[serde(rename = "YEARLY")]
        #[doc = "Yearly period, 365 days or irregular."]
        Yearly,
    }
    impl ::std::default::Default for ArimaResultSeasonalPeriodsEnum {
        fn default() -> Self {
            Self::SeasonalPeriodTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Model evaluation metrics for a single ARIMA forecasting model."]
    pub struct ArimaSingleModelForecastingMetrics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arimaFittingMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Arima fitting metrics."]
        pub arima_fitting_metrics: ::std::option::Option<::std::boxed::Box<ArimaFittingMetrics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasDrift")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Is arima model fitted with drift or not. It is always false when d is not 1."]
        pub has_drift: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonSeasonalOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Non-seasonal order."]
        pub non_seasonal_order: ::std::option::Option<::std::boxed::Box<ArimaOrder>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seasonalPeriods")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Seasonal periods. Repeated because multiple periods are supported for one time series."]
        pub seasonal_periods: ::std::option::Option<
            ::std::vec::Vec<ArimaSingleModelForecastingMetricsSeasonalPeriodsEnum>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSeriesId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id to indicate different time series."]
        pub time_series_id: ::std::option::Option<::std::string::String>,
    }
    impl ArimaSingleModelForecastingMetrics {
        pub fn builder() -> ArimaSingleModelForecastingMetricsBuilder {
            ArimaSingleModelForecastingMetricsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum ArimaSingleModelForecastingMetricsSeasonalPeriodsEnum {
        #[serde(rename = "SEASONAL_PERIOD_TYPE_UNSPECIFIED")]
        #[doc = ""]
        SeasonalPeriodTypeUnspecified,
        #[serde(rename = "NO_SEASONALITY")]
        #[doc = "No seasonality"]
        NoSeasonality,
        #[serde(rename = "DAILY")]
        #[doc = "Daily period, 24 hours."]
        Daily,
        #[serde(rename = "WEEKLY")]
        #[doc = "Weekly period, 7 days."]
        Weekly,
        #[serde(rename = "MONTHLY")]
        #[doc = "Monthly period, 30 days or irregular."]
        Monthly,
        #[serde(rename = "QUARTERLY")]
        #[doc = "Quarterly period, 90 days or irregular."]
        Quarterly,
        #[serde(rename = "YEARLY")]
        #[doc = "Yearly period, 365 days or irregular."]
        Yearly,
    }
    impl ::std::default::Default for ArimaSingleModelForecastingMetricsSeasonalPeriodsEnum {
        fn default() -> Self {
            Self::SeasonalPeriodTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { \"audit_configs\": [ { \"service\": \"allServices\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" }, { \"log_type\": \"ADMIN_READ\" } ] }, { \"service\": \"sampleservice.googleapis.com\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\" }, { \"log_type\": \"DATA_WRITE\", \"exempted_members\": [ \"user:aliya@example.com\" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging."]
    pub struct AuditConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "auditLogConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The configuration for logging of each type of permission."]
        pub audit_log_configs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditLogConfig>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "service")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services."]
        pub service: ::std::option::Option<::std::string::String>,
    }
    impl AuditConfig {
        pub fn builder() -> AuditConfigBuilder {
            AuditConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides the configuration for logging a type of permissions. Example: { \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging."]
    pub struct AuditLogConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exemptedMembers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members."]
        pub exempted_members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The log type that this config enables."]
        pub log_type: ::std::option::Option<AuditLogConfigLogTypeEnum>,
    }
    impl AuditLogConfig {
        pub fn builder() -> AuditLogConfigBuilder {
            AuditLogConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The log type that this config enables."]
    pub enum AuditLogConfigLogTypeEnum {
        #[serde(rename = "LOG_TYPE_UNSPECIFIED")]
        #[doc = "Default case. Should never be this."]
        LogTypeUnspecified,
        #[serde(rename = "ADMIN_READ")]
        #[doc = "Admin reads. Example: CloudIAM getIamPolicy"]
        AdminRead,
        #[serde(rename = "DATA_WRITE")]
        #[doc = "Data writes. Example: CloudSQL Users create"]
        DataWrite,
        #[serde(rename = "DATA_READ")]
        #[doc = "Data reads. Example: CloudSQL Users list"]
        DataRead,
    }
    impl ::std::default::Default for AuditLogConfigLogTypeEnum {
        fn default() -> Self {
            Self::LogTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BigQueryModelTraining {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentIteration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only, Beta] Index of current ML training iteration. Updated during create model query job to show job progress."]
        pub current_iteration: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expectedTotalIterations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only, Beta] Expected number of iterations for the create model query job specified as num_iterations in the input query. The actual total number of iterations may be less than this number due to early stop."]
        pub expected_total_iterations: ::std::option::Option<::std::string::String>,
    }
    impl BigQueryModelTraining {
        pub fn builder() -> BigQueryModelTrainingBuilder {
            BigQueryModelTrainingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BigtableColumn {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encoding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The encoding of the values when the type is not STRING. Acceptable encoding values are: TEXT - indicates values are alphanumeric text strings. BINARY - indicates values are encoded using HBase Bytes.toBytes family of functions. 'encoding' can also be set at the column family level. However, the setting at this level takes precedence if 'encoding' is set at both levels."]
        pub encoding: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] If the qualifier is not a valid BigQuery field identifier i.e. does not match [a-zA-Z][a-zA-Z0-9_]*, a valid identifier must be provided as the column field name and is used as field name in queries."]
        pub field_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "onlyReadLatest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] If this is set, only the latest version of value in this column are exposed. 'onlyReadLatest' can also be set at the column family level. However, the setting at this level takes precedence if 'onlyReadLatest' is set at both levels."]
        pub only_read_latest: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "qualifierEncoded")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] Qualifier of the column. Columns in the parent column family that has this exact qualifier are exposed as . field. If the qualifier is valid UTF-8 string, it can be specified in the qualifier_string field. Otherwise, a base-64 encoded value must be set to qualifier_encoded. The column field name is the same as the column qualifier. However, if the qualifier is not a valid BigQuery field identifier i.e. does not match [a-zA-Z][a-zA-Z0-9_]*, a valid identifier must be provided as field_name."]
        pub qualifier_encoded: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "qualifierString")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub qualifier_string: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The type to convert the value in cells of this column. The values are expected to be encoded using HBase Bytes.toBytes function when using the BINARY encoding value. Following BigQuery types are allowed (case-sensitive) - BYTES STRING INTEGER FLOAT BOOLEAN Default type is BYTES. 'type' can also be set at the column family level. However, the setting at this level takes precedence if 'type' is set at both levels."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl BigtableColumn {
        pub fn builder() -> BigtableColumnBuilder {
            BigtableColumnBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BigtableColumnFamily {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Lists of columns that should be exposed as individual fields as opposed to a list of (column name, value) pairs. All columns whose qualifier matches a qualifier in this list can be accessed as .. Other columns can be accessed as a list through .Column field."]
        pub columns: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BigtableColumn>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encoding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The encoding of the values when the type is not STRING. Acceptable encoding values are: TEXT - indicates values are alphanumeric text strings. BINARY - indicates values are encoded using HBase Bytes.toBytes family of functions. This can be overridden for a specific column by listing that column in 'columns' and specifying an encoding for it."]
        pub encoding: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "familyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier of the column family."]
        pub family_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "onlyReadLatest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] If this is set only the latest version of value are exposed for all columns in this column family. This can be overridden for a specific column by listing that column in 'columns' and specifying a different setting for that column."]
        pub only_read_latest: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The type to convert the value in cells of this column family. The values are expected to be encoded using HBase Bytes.toBytes function when using the BINARY encoding value. Following BigQuery types are allowed (case-sensitive) - BYTES STRING INTEGER FLOAT BOOLEAN Default type is BYTES. This can be overridden for a specific column by listing that column in 'columns' and specifying a type for it."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl BigtableColumnFamily {
        pub fn builder() -> BigtableColumnFamilyBuilder {
            BigtableColumnFamilyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BigtableOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnFamilies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] List of column families to expose in the table schema along with their types. This list restricts the column families that can be referenced in queries and specifies their value types. You can use this list to do type conversions - see the 'type' field for more details. If you leave this list empty, all column families are present in the table schema and their values are read as BYTES. During a query only the column families referenced in that query are read from Bigtable."]
        pub column_families:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BigtableColumnFamily>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ignoreUnspecifiedColumnFamilies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] If field is true, then the column families that are not specified in columnFamilies list are not exposed in the table schema. Otherwise, they are read with BYTES type values. The default value is false."]
        pub ignore_unspecified_column_families: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "readRowkeyAsString")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] If field is true, then the rowkey column families will be read and converted to string. Otherwise they are read with BYTES type values and users need to manually cast them with CAST if necessary. The default value is false."]
        pub read_rowkey_as_string: ::std::option::Option<::std::primitive::bool>,
    }
    impl BigtableOptions {
        pub fn builder() -> BigtableOptionsBuilder {
            BigtableOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Evaluation metrics for binary classification/classifier models."]
    pub struct BinaryClassificationMetrics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aggregateClassificationMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aggregate classification metrics."]
        pub aggregate_classification_metrics:
            ::std::option::Option<::std::boxed::Box<AggregateClassificationMetrics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "binaryConfusionMatrixList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Binary confusion matrix at multiple thresholds."]
        pub binary_confusion_matrix_list:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BinaryConfusionMatrix>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "negativeLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label representing the negative class."]
        pub negative_label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "positiveLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label representing the positive class."]
        pub positive_label: ::std::option::Option<::std::string::String>,
    }
    impl BinaryClassificationMetrics {
        pub fn builder() -> BinaryClassificationMetricsBuilder {
            BinaryClassificationMetricsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Confusion matrix for binary classification models."]
    pub struct BinaryConfusionMatrix {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accuracy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fraction of predictions given the correct label."]
        pub accuracy: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "f1Score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The equally weighted average of recall and precision."]
        pub f1_score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "falseNegatives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of false samples predicted as false."]
        pub false_negatives: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "falsePositives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of false samples predicted as true."]
        pub false_positives: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "positiveClassThreshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Threshold value used when computing each of the following metric."]
        pub positive_class_threshold: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "precision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fraction of actual positive predictions that had positive actual labels."]
        pub precision: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recall")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fraction of actual positive labels that were given a positive prediction."]
        pub recall: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trueNegatives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of true samples predicted as false."]
        pub true_negatives: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "truePositives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of true samples predicted as true."]
        pub true_positives: ::std::option::Option<::std::string::String>,
    }
    impl BinaryConfusionMatrix {
        pub fn builder() -> BinaryConfusionMatrixBuilder {
            BinaryConfusionMatrixBuilder::default()
        }
    }
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
    pub struct BqmlIterationResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "durationMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only, Beta] Time taken to run the training iteration in milliseconds."]
        pub duration_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "evalLoss")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only, Beta] Eval loss computed on the eval data at the end of the iteration. The eval loss is used for early stopping to avoid overfitting. No eval loss if eval_split_method option is specified as no_split or auto_split with input data size less than 500 rows."]
        pub eval_loss: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only, Beta] Index of the ML training iteration, starting from zero for each training run."]
        pub index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "learnRate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only, Beta] Learning rate used for this iteration, it varies for different training iterations if learn_rate_strategy option is not constant."]
        pub learn_rate: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trainingLoss")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only, Beta] Training loss computed on the training data at the end of the iteration. The training loss function is defined by model type."]
        pub training_loss: ::std::option::Option<::std::primitive::f64>,
    }
    impl BqmlIterationResult {
        pub fn builder() -> BqmlIterationResultBuilder {
            BqmlIterationResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BqmlTrainingRun {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iterationResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only, Beta] List of each iteration results."]
        pub iteration_results:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BqmlIterationResult>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only, Beta] Training run start time in milliseconds since the epoch."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only, Beta] Different state applicable for a training run. IN PROGRESS: Training run is in progress. FAILED: Training run ended due to a non-retryable failure. SUCCEEDED: Training run successfully completed. CANCELLED: Training run cancelled by the user."]
        pub state: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trainingOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only, Beta] Training options used by this training run. These options are mutable for subsequent training runs. Default values are explicitly stored for options not specified in the input query of the first training run. For subsequent training runs, any option not explicitly specified in the input query will be copied from the previous training run."]
        pub training_options: ::std::option::Option<BqmlTrainingRunTrainingOptions>,
    }
    impl BqmlTrainingRun {
        pub fn builder() -> BqmlTrainingRunBuilder {
            BqmlTrainingRunBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "[Output-only, Beta] Training options used by this training run. These options are mutable for subsequent training runs. Default values are explicitly stored for options not specified in the input query of the first training run. For subsequent training runs, any option not explicitly specified in the input query will be copied from the previous training run."]
    pub struct BqmlTrainingRunTrainingOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "earlyStop")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub early_stop: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "l1Reg")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub l1_reg: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "l2Reg")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub l2_reg: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "learnRate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub learn_rate: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "learnRateStrategy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub learn_rate_strategy: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineSearchInitLearnRate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub line_search_init_learn_rate: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxIteration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub max_iteration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minRelProgress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub min_rel_progress: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warmStart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub warm_start: ::std::option::Option<::std::primitive::bool>,
    }
    impl BqmlTrainingRunTrainingOptions {
        pub fn builder() -> BqmlTrainingRunTrainingOptionsBuilder {
            BqmlTrainingRunTrainingOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Representative value of a categorical feature."]
    pub struct CategoricalValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categoryCounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Counts of all categories for the categorical feature. If there are more than ten categories, we return top ten (by count) and return one more CategoryCount with category \"_OTHER_\" and count as aggregate counts of remaining categories."]
        pub category_counts:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CategoryCount>>>,
    }
    impl CategoricalValue {
        pub fn builder() -> CategoricalValueBuilder {
            CategoricalValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the count of a single category within the cluster."]
    pub struct CategoryCount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "category")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of category."]
        pub category: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The count of training samples matching the category within the cluster."]
        pub count: ::std::option::Option<::std::string::String>,
    }
    impl CategoryCount {
        pub fn builder() -> CategoryCountBuilder {
            CategoryCountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message containing the information about one cluster."]
    pub struct Cluster {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "centroidId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Centroid id."]
        pub centroid_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of training data rows that were assigned to this cluster."]
        pub count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "featureValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Values of highly variant features for this cluster."]
        pub feature_values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FeatureValue>>>,
    }
    impl Cluster {
        pub fn builder() -> ClusterBuilder {
            ClusterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a single cluster for clustering model."]
    pub struct ClusterInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "centroidId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Centroid id."]
        pub centroid_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterRadius")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cluster radius, the average distance from centroid to each point assigned to the cluster."]
        pub cluster_radius: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cluster size, the total number of points assigned to the cluster."]
        pub cluster_size: ::std::option::Option<::std::string::String>,
    }
    impl ClusterInfo {
        pub fn builder() -> ClusterInfoBuilder {
            ClusterInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Clustering {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Repeated] One or more fields on which data should be clustered. Only top-level, non-repeated, simple-type fields are supported. When you cluster a table using multiple columns, the order of columns you specify is important. The order of the specified columns determines the sort order of the data."]
        pub fields: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Clustering {
        pub fn builder() -> ClusteringBuilder {
            ClusteringBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Evaluation metrics for clustering models."]
    pub struct ClusteringMetrics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information for all clusters."]
        pub clusters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Cluster>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "daviesBouldinIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Davies-Bouldin index."]
        pub davies_bouldin_index: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meanSquaredDistance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mean of squared distances between each sample to its cluster centroid."]
        pub mean_squared_distance: ::std::option::Option<::std::primitive::f64>,
    }
    impl ClusteringMetrics {
        pub fn builder() -> ClusteringMetricsBuilder {
            ClusteringMetricsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Confusion matrix for multi-class classification models."]
    pub struct ConfusionMatrix {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidenceThreshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence threshold used when computing the entries of the confusion matrix."]
        pub confidence_threshold: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "One row per actual label."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Row>>>,
    }
    impl ConfusionMatrix {
        pub fn builder() -> ConfusionMatrixBuilder {
            ConfusionMatrixBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ConnectionProperty {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] Name of the connection property to set."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] Value of the connection property."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl ConnectionProperty {
        pub fn builder() -> ConnectionPropertyBuilder {
            ConnectionPropertyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CsvOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowJaggedRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Indicates if BigQuery should accept rows that are missing trailing optional columns. If true, BigQuery treats missing trailing columns as null values. If false, records with missing trailing columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false."]
        pub allow_jagged_rows: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowQuotedNewlines")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Indicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false."]
        pub allow_quoted_newlines: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encoding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The character encoding of the data. The supported values are UTF-8 or ISO-8859-1. The default value is UTF-8. BigQuery decodes the data after the raw, binary data has been split using the values of the quote and fieldDelimiter properties."]
        pub encoding: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldDelimiter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The separator for fields in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. BigQuery also supports the escape sequence \"\\t\" to specify a tab separator. The default value is a comma (',')."]
        pub field_delimiter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ csv_options_defaults :: quote () }", setter(into))]
        #[serde(rename = "quote")]
        #[serde(default = "csv_options_defaults :: quote")]
        #[doc = "[Optional] The value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. The default value is a double-quote ('\"'). If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true."]
        pub quote: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skipLeadingRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The number of rows at the top of a CSV file that BigQuery will skip when reading the data. The default value is 0. This property is useful if you have header rows in the file that should be skipped. When autodetect is on, the behavior is the following: * skipLeadingRows unspecified - Autodetect tries to detect headers in the first row. If they are not detected, the row is read as data. Otherwise data is read starting from the second row. * skipLeadingRows is 0 - Instructs autodetect that there are no headers and data should be read starting from the first row. * skipLeadingRows = N > 0 - Autodetect skips N-1 rows and tries to detect headers in row N. If headers are not detected, row N is just skipped. Otherwise row N is used to extract column names for the detected schema."]
        pub skip_leading_rows: ::std::option::Option<::std::string::String>,
    }
    impl CsvOptions {
        pub fn builder() -> CsvOptionsBuilder {
            CsvOptionsBuilder::default()
        }
    }
    mod csv_options_defaults {
        pub fn quote() -> ::std::string::String {
            serde_json::from_str(&"\"\\\"\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data split result. This contains references to the training and evaluation data tables that were used to train the model."]
    pub struct DataSplitResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "evaluationTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Table reference of the evaluation data after split."]
        pub evaluation_table: ::std::option::Option<::std::boxed::Box<TableReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trainingTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Table reference of the training data after split."]
        pub training_table: ::std::option::Option<::std::boxed::Box<TableReference>>,
    }
    impl DataSplitResult {
        pub fn builder() -> DataSplitResultBuilder {
            DataSplitResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Dataset {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "access")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] An array of objects that define dataset access for one or more entities. You can set this property when inserting or updating a dataset in order to control who is allowed to access the data. If unspecified at dataset creation time, BigQuery adds default dataset access for the following entities: access.specialGroup: projectReaders; access.role: READER; access.specialGroup: projectWriters; access.role: WRITER; access.specialGroup: projectOwners; access.role: OWNER; access.userByEmail: [dataset creator email]; access.role: OWNER;"]
        pub access: ::std::option::Option<::std::vec::Vec<DatasetAccess>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The time when this dataset was created, in milliseconds since the epoch."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datasetReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] A reference that identifies the dataset."]
        pub dataset_reference: ::std::option::Option<::std::boxed::Box<DatasetReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultEncryptionConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub default_encryption_configuration:
            ::std::option::Option<::std::boxed::Box<EncryptionConfiguration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultPartitionExpirationMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The default partition expiration for all partitioned tables in the dataset, in milliseconds. Once this property is set, all newly-created partitioned tables in the dataset will have an expirationMs property in the timePartitioning settings set to this value, and changing the value will only affect new tables, not existing ones. The storage in a partition will have an expiration time of its partition time plus this value. Setting this property overrides the use of defaultTableExpirationMs for partitioned tables: only one of defaultTableExpirationMs and defaultPartitionExpirationMs will be used for any new partitioned table. If you provide an explicit timePartitioning.expirationMs when creating or updating a partitioned table, that value takes precedence over the default partition expiration time indicated by this property."]
        pub default_partition_expiration_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultTableExpirationMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The default lifetime of all tables in the dataset, in milliseconds. The minimum value is 3600000 milliseconds (one hour). Once this property is set, all newly-created tables in the dataset will have an expirationTime property set to the creation time plus the value in this property, and changing the value will only affect new tables, not existing ones. When the expirationTime for a given table is reached, that table will be deleted automatically. If a table's expirationTime is modified or removed before the table expires, or if you provide an explicit expirationTime when creating a table, that value takes precedence over the default expiration time indicated by this property."]
        pub default_table_expiration_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] A user-friendly description of the dataset."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] A hash of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "friendlyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] A descriptive name for the dataset."]
        pub friendly_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The fully-qualified unique name of the dataset in the format projectId:datasetId. The dataset name without the project name is given in the datasetId field. When creating a new dataset, leave this field blank, and instead specify the datasetId field."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ dataset_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "dataset_defaults :: kind")]
        #[doc = "[Output-only] The resource type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The labels associated with this dataset. You can use these to organize and group your datasets. You can set this property when inserting or updating a dataset. See Creating and Updating Dataset Labels for more information."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The date when this dataset or any of its tables was last modified, in milliseconds since the epoch."]
        pub last_modified_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The geographic location where the dataset should reside. The default value is US. See details at https://cloud.google.com/bigquery/docs/locations."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "satisfiesPZS")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Reserved for future use."]
        pub satisfies_pzs: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] A URL that can be used to access the resource again. You can use this URL in Get or Update requests to the resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl Dataset {
        pub fn builder() -> DatasetBuilder {
            DatasetBuilder::default()
        }
    }
    mod dataset_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#dataset\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DatasetAccess {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Pick one] A grant authorizing all resources of a particular type in a particular dataset access to this dataset. Only views are supported for now. The role field is not required when this field is set. If that dataset is deleted and re-created, its access needs to be granted again via an update operation."]
        pub dataset: ::std::option::Option<::std::boxed::Box<DatasetAccessEntry>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Pick one] A domain to grant access to. Any users signed in with the domain specified will be granted the specified access. Example: \"example.com\". Maps to IAM policy member \"domain:DOMAIN\"."]
        pub domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupByEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Pick one] An email address of a Google Group to grant access to. Maps to IAM policy member \"group:GROUP\"."]
        pub group_by_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iamMember")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Pick one] Some other type of member that appears in the IAM Policy but isn't a user, group, domain, or special group."]
        pub iam_member: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] An IAM role ID that should be granted to the user, group, or domain specified in this access entry. The following legacy mappings will be applied: OWNER  roles/bigquery.dataOwner WRITER  roles/bigquery.dataEditor READER  roles/bigquery.dataViewer This field will accept any of the above formats, but will return only the legacy format. For example, if you set this field to \"roles/bigquery.dataOwner\", it will be returned back as \"OWNER\"."]
        pub role: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "routine")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Pick one] A routine from a different dataset to grant access to. Queries executed against that routine will have read access to views/tables/routines in this dataset. Only UDF is supported for now. The role field is not required when this field is set. If that routine is updated by any user, access to the routine needs to be granted again via an update operation."]
        pub routine: ::std::option::Option<::std::boxed::Box<RoutineReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "specialGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Pick one] A special group to grant access to. Possible values include: projectOwners: Owners of the enclosing project. projectReaders: Readers of the enclosing project. projectWriters: Writers of the enclosing project. allAuthenticatedUsers: All authenticated BigQuery users. Maps to similarly-named IAM members."]
        pub special_group: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userByEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Pick one] An email address of a user to grant access to. For example: fred@example.com. Maps to IAM policy member \"user:EMAIL\" or \"serviceAccount:EMAIL\"."]
        pub user_by_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "view")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Pick one] A view from a different dataset to grant access to. Queries executed against that view will have read access to tables in this dataset. The role field is not required when this field is set. If that view is updated by any user, access to the view needs to be granted again via an update operation."]
        pub view: ::std::option::Option<::std::boxed::Box<TableReference>>,
    }
    impl DatasetAccess {
        pub fn builder() -> DatasetAccessBuilder {
            DatasetAccessBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DatasetAccessEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The dataset this entry applies to."]
        pub dataset: ::std::option::Option<::std::boxed::Box<DatasetReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target_types")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub target_types: ::std::option::Option<::std::vec::Vec<DatasetAccessEntryTargetTypes>>,
    }
    impl DatasetAccessEntry {
        pub fn builder() -> DatasetAccessEntryBuilder {
            DatasetAccessEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DatasetAccessEntryTargetTypes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] Which resources in the dataset this entry applies to. Currently, only views are supported, but additional target types may be added in the future. Possible values: VIEWS: This entry applies to all views in the dataset."]
        pub target_type: ::std::option::Option<::std::string::String>,
    }
    impl DatasetAccessEntryTargetTypes {
        pub fn builder() -> DatasetAccessEntryTargetTypesBuilder {
            DatasetAccessEntryTargetTypesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DatasetList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datasets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An array of the dataset resources in the project. Each resource contains basic information. For full information about a particular dataset resource, use the Datasets: get method. This property is omitted when there are no datasets in the project."]
        pub datasets: ::std::option::Option<::std::vec::Vec<DatasetListDatasets>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A hash value of the results page. You can use this property to determine if the page has changed since the last request."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ dataset_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "dataset_list_defaults :: kind")]
        #[doc = "The list type. This property always returns the value \"bigquery#datasetList\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token that can be used to request the next results page. This property is omitted on the final results page."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl DatasetList {
        pub fn builder() -> DatasetListBuilder {
            DatasetListBuilder::default()
        }
    }
    mod dataset_list_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#datasetList\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DatasetListDatasets {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datasetReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dataset reference. Use this property to access specific parts of the dataset's ID, such as project ID or dataset ID."]
        pub dataset_reference: ::std::option::Option<::std::boxed::Box<DatasetReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "friendlyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A descriptive name for the dataset, if one exists."]
        pub friendly_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fully-qualified, unique, opaque ID of the dataset."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ dataset_list_datasets_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "dataset_list_datasets_defaults :: kind")]
        #[doc = "The resource type. This property always returns the value \"bigquery#dataset\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The labels associated with this dataset. You can use these to organize and group your datasets."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The geographic location where the data resides."]
        pub location: ::std::option::Option<::std::string::String>,
    }
    impl DatasetListDatasets {
        pub fn builder() -> DatasetListDatasetsBuilder {
            DatasetListDatasetsBuilder::default()
        }
    }
    mod dataset_list_datasets_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#dataset\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DatasetReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datasetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] A unique ID for this dataset, without the project name. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters."]
        pub dataset_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The ID of the project containing this dataset."]
        pub project_id: ::std::option::Option<::std::string::String>,
    }
    impl DatasetReference {
        pub fn builder() -> DatasetReferenceBuilder {
            DatasetReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DestinationTableProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The description for the destination table. This will only be used if the destination table is newly created. If the table already exists and a value different than the current description is provided, the job will fail."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "friendlyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The friendly name for the destination table. This will only be used if the destination table is newly created. If the table already exists and a value different than the current friendly name is provided, the job will fail."]
        pub friendly_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The labels associated with this table. You can use these to organize and group your tables. This will only be used if the destination table is newly created. If the table already exists and labels are different than the current labels are provided, the job will fail."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl DestinationTableProperties {
        pub fn builder() -> DestinationTablePropertiesBuilder {
            DestinationTablePropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Model evaluation metrics for dimensionality reduction models."]
    pub struct DimensionalityReductionMetrics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalExplainedVarianceRatio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total percentage of variance explained by the selected principal components."]
        pub total_explained_variance_ratio: ::std::option::Option<::std::primitive::f64>,
    }
    impl DimensionalityReductionMetrics {
        pub fn builder() -> DimensionalityReductionMetricsBuilder {
            DimensionalityReductionMetricsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct EncryptionConfiguration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmsKeyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Describes the Cloud KMS encryption key that will be used to protect destination BigQuery table. The BigQuery Service Account associated with your project requires access to this encryption key."]
        pub kms_key_name: ::std::option::Option<::std::string::String>,
    }
    impl EncryptionConfiguration {
        pub fn builder() -> EncryptionConfigurationBuilder {
            EncryptionConfigurationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single entry in the confusion matrix."]
    pub struct Entry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of items being predicted as this label."]
        pub item_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "predictedLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The predicted label. For confidence_threshold > 0, we will also add an entry indicating the number of items under the confidence threshold."]
        pub predicted_label: ::std::option::Option<::std::string::String>,
    }
    impl Entry {
        pub fn builder() -> EntryBuilder {
            EntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ErrorProto {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "debugInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Debugging information. This property is internal to Google and should not be used."]
        pub debug_info: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies where the error occurred, if present."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A human-readable description of the error."]
        pub message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short error code that summarizes the error."]
        pub reason: ::std::option::Option<::std::string::String>,
    }
    impl ErrorProto {
        pub fn builder() -> ErrorProtoBuilder {
            ErrorProtoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Evaluation metrics of a model. These are either computed on all training data or just the eval data based on whether eval data was used during training. These are not present for imported models."]
    pub struct EvaluationMetrics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arimaForecastingMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Populated for ARIMA models."]
        pub arima_forecasting_metrics:
            ::std::option::Option<::std::boxed::Box<ArimaForecastingMetrics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "binaryClassificationMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Populated for binary classification/classifier models."]
        pub binary_classification_metrics:
            ::std::option::Option<::std::boxed::Box<BinaryClassificationMetrics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusteringMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Populated for clustering models."]
        pub clustering_metrics: ::std::option::Option<::std::boxed::Box<ClusteringMetrics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionalityReductionMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Evaluation metrics when the model is a dimensionality reduction model, which currently includes PCA."]
        pub dimensionality_reduction_metrics:
            ::std::option::Option<::std::boxed::Box<DimensionalityReductionMetrics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "multiClassClassificationMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Populated for multi-class classification/classifier models."]
        pub multi_class_classification_metrics:
            ::std::option::Option<::std::boxed::Box<MultiClassClassificationMetrics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rankingMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Populated for implicit feedback type matrix factorization models."]
        pub ranking_metrics: ::std::option::Option<::std::boxed::Box<RankingMetrics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regressionMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Populated for regression models and explicit feedback type matrix factorization models."]
        pub regression_metrics: ::std::option::Option<::std::boxed::Box<RegressionMetrics>>,
    }
    impl EvaluationMetrics {
        pub fn builder() -> EvaluationMetricsBuilder {
            EvaluationMetricsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ExplainQueryStage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "completedParallelInputs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of parallel input segments completed."]
        pub completed_parallel_inputs: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "computeMsAvg")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Milliseconds the average shard spent on CPU-bound tasks."]
        pub compute_ms_avg: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "computeMsMax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Milliseconds the slowest shard spent on CPU-bound tasks."]
        pub compute_ms_max: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "computeRatioAvg")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relative amount of time the average shard spent on CPU-bound tasks."]
        pub compute_ratio_avg: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "computeRatioMax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relative amount of time the slowest shard spent on CPU-bound tasks."]
        pub compute_ratio_max: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Stage end time represented as milliseconds since epoch."]
        pub end_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique ID for stage within plan."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputStages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IDs for stages that are inputs to this stage."]
        pub input_stages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human-readable name for stage."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parallelInputs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of parallel input segments to be processed."]
        pub parallel_inputs: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "readMsAvg")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Milliseconds the average shard spent reading input."]
        pub read_ms_avg: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "readMsMax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Milliseconds the slowest shard spent reading input."]
        pub read_ms_max: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "readRatioAvg")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relative amount of time the average shard spent reading input."]
        pub read_ratio_avg: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "readRatioMax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relative amount of time the slowest shard spent reading input."]
        pub read_ratio_max: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recordsRead")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of records read into the stage."]
        pub records_read: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recordsWritten")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of records written by the stage."]
        pub records_written: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shuffleOutputBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of bytes written to shuffle."]
        pub shuffle_output_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shuffleOutputBytesSpilled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of bytes written to shuffle and spilled to disk."]
        pub shuffle_output_bytes_spilled: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slotMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Slot-milliseconds used by the stage."]
        pub slot_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Stage start time represented as milliseconds since epoch."]
        pub start_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Current status for the stage."]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "steps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of operations within the stage in dependency order (approximately chronological)."]
        pub steps: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ExplainQueryStep>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "waitMsAvg")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Milliseconds the average shard spent waiting to be scheduled."]
        pub wait_ms_avg: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "waitMsMax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Milliseconds the slowest shard spent waiting to be scheduled."]
        pub wait_ms_max: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "waitRatioAvg")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relative amount of time the average shard spent waiting to be scheduled."]
        pub wait_ratio_avg: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "waitRatioMax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relative amount of time the slowest shard spent waiting to be scheduled."]
        pub wait_ratio_max: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "writeMsAvg")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Milliseconds the average shard spent on writing output."]
        pub write_ms_avg: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "writeMsMax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Milliseconds the slowest shard spent on writing output."]
        pub write_ms_max: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "writeRatioAvg")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relative amount of time the average shard spent on writing output."]
        pub write_ratio_avg: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "writeRatioMax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relative amount of time the slowest shard spent on writing output."]
        pub write_ratio_max: ::std::option::Option<::std::primitive::f64>,
    }
    impl ExplainQueryStage {
        pub fn builder() -> ExplainQueryStageBuilder {
            ExplainQueryStageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ExplainQueryStep {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Machine-readable operation type."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "substeps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human-readable stage descriptions."]
        pub substeps: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ExplainQueryStep {
        pub fn builder() -> ExplainQueryStepBuilder {
            ExplainQueryStepBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Explanation for a single feature."]
    pub struct Explanation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attribution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Attribution of feature."]
        pub attribution: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "featureName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Full name of the feature. For non-numerical features, will be formatted like .. Overall size of feature name will always be truncated to first 120 characters."]
        pub feature_name: ::std::option::Option<::std::string::String>,
    }
    impl Explanation {
        pub fn builder() -> ExplanationBuilder {
            ExplanationBuilder::default()
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
    pub struct ExternalDataConfiguration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autodetect")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Try to detect schema and format options automatically. Any option specified explicitly will be honored."]
        pub autodetect: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bigtableOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Additional options if sourceFormat is set to BIGTABLE."]
        pub bigtable_options: ::std::option::Option<::std::boxed::Box<BigtableOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compression")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The compression type of the data source. Possible values include GZIP and NONE. The default value is NONE. This setting is ignored for Google Cloud Bigtable, Google Cloud Datastore backups and Avro formats."]
        pub compression: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "connectionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional, Trusted Tester] Connection for external data source."]
        pub connection_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "csvOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional properties to set if sourceFormat is set to CSV."]
        pub csv_options: ::std::option::Option<::std::boxed::Box<CsvOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "googleSheetsOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Additional options if sourceFormat is set to GOOGLE_SHEETS."]
        pub google_sheets_options: ::std::option::Option<::std::boxed::Box<GoogleSheetsOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hivePartitioningOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Options to configure hive partitioning support."]
        pub hive_partitioning_options:
            ::std::option::Option<::std::boxed::Box<HivePartitioningOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ignoreUnknownValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Indicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. The sourceFormat property determines what BigQuery treats as an extra value: CSV: Trailing columns JSON: Named values that don't match any column names Google Cloud Bigtable: This setting is ignored. Google Cloud Datastore backups: This setting is ignored. Avro: This setting is ignored."]
        pub ignore_unknown_values: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxBadRecords")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The maximum number of bad records that BigQuery can ignore when reading data. If the number of bad records exceeds this value, an invalid error is returned in the job result. This is only valid for CSV, JSON, and Google Sheets. The default value is 0, which requires that all records are valid. This setting is ignored for Google Cloud Bigtable, Google Cloud Datastore backups and Avro formats."]
        pub max_bad_records: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schema")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The schema for the data. Schema is required for CSV and JSON formats. Schema is disallowed for Google Cloud Bigtable, Cloud Datastore backups, and Avro formats."]
        pub schema: ::std::option::Option<::std::boxed::Box<TableSchema>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The data format. For CSV files, specify \"CSV\". For Google sheets, specify \"GOOGLE_SHEETS\". For newline-delimited JSON, specify \"NEWLINE_DELIMITED_JSON\". For Avro files, specify \"AVRO\". For Google Cloud Datastore backups, specify \"DATASTORE_BACKUP\". [Beta] For Google Cloud Bigtable, specify \"BIGTABLE\"."]
        pub source_format: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceUris")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The fully-qualified URIs that point to your data in Google Cloud. For Google Cloud Storage URIs: Each URI can contain one '*' wildcard character and it must come after the 'bucket' name. Size limits related to load jobs apply to external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be specified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table. For Google Cloud Datastore backups, exactly one URI can be specified. Also, the '*' wildcard character is not allowed."]
        pub source_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ExternalDataConfiguration {
        pub fn builder() -> ExternalDataConfigurationBuilder {
            ExternalDataConfigurationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Representative value of a single feature within the cluster."]
    pub struct FeatureValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categoricalValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The categorical feature value."]
        pub categorical_value: ::std::option::Option<::std::boxed::Box<CategoricalValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "featureColumn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The feature column name."]
        pub feature_column: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numericalValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The numerical feature value. This is the centroid value for this feature."]
        pub numerical_value: ::std::option::Option<::std::primitive::f64>,
    }
    impl FeatureValue {
        pub fn builder() -> FeatureValueBuilder {
            FeatureValueBuilder::default()
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
    pub struct GetQueryResultsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cacheHit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the query result was fetched from the query cache."]
        pub cache_hit: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The first errors or warnings encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has completed or was unsuccessful."]
        pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ErrorProto>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A hash of this response."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobComplete")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the query has completed or not. If rows or totalRows are present, this will always be true. If this is false, totalRows will not be available."]
        pub job_complete: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reference to the BigQuery Job that was created to run the query. This field will be present even if the original request timed out, in which case GetQueryResults can be used to read the results once the query has completed. Since this API only returns the first page of results, subsequent pages can be fetched via the same mechanism (GetQueryResults)."]
        pub job_reference: ::std::option::Option<::std::boxed::Box<JobReference>>,
        #[builder(
            default = "{ get_query_results_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "get_query_results_response_defaults :: kind")]
        #[doc = "The resource type of the response."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numDmlAffectedRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE."]
        pub num_dml_affected_rows: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token used for paging results."]
        pub page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An object with as many results as can be contained within the maximum permitted reply size. To get any additional rows, you can call GetQueryResults and specify the jobReference returned above. Present only when the query completes successfully."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TableRow>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schema")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The schema of the results. Present only when the query completes successfully."]
        pub schema: ::std::option::Option<::std::boxed::Box<TableSchema>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalBytesProcessed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of bytes processed for this query."]
        pub total_bytes_processed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of rows in the complete query result set, which can be more than the number of rows in this single page of results. Present only when the query completes successfully."]
        pub total_rows: ::std::option::Option<::std::string::String>,
    }
    impl GetQueryResultsResponse {
        pub fn builder() -> GetQueryResultsResponseBuilder {
            GetQueryResultsResponseBuilder::default()
        }
    }
    mod get_query_results_response_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#getQueryResultsResponse\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GetServiceAccountResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The service account email address."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ get_service_account_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "get_service_account_response_defaults :: kind")]
        #[doc = "The resource type of the response."]
        pub kind: ::std::string::String,
    }
    impl GetServiceAccountResponse {
        pub fn builder() -> GetServiceAccountResponseBuilder {
            GetServiceAccountResponseBuilder::default()
        }
    }
    mod get_service_account_response_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#getServiceAccountResponse\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Global explanations containing the top most important features after training."]
    pub struct GlobalExplanation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "classLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Class label for this set of global explanations. Will be empty/null for binary logistic and linear regression models. Sorted alphabetically in descending order."]
        pub class_label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explanations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of the top global explanations. Sorted by absolute value of attribution in descending order."]
        pub explanations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Explanation>>>,
    }
    impl GlobalExplanation {
        pub fn builder() -> GlobalExplanationBuilder {
            GlobalExplanationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleSheetsOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Range of a sheet to query from. Only used when non-empty. Typical format: sheet_name!top_left_cell_id:bottom_right_cell_id For example: sheet1!A1:B20"]
        pub range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skipLeadingRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The number of rows at the top of a sheet that BigQuery will skip when reading the data. The default value is 0. This property is useful if you have header rows that should be skipped. When autodetect is on, behavior is the following: * skipLeadingRows unspecified - Autodetect tries to detect headers in the first row. If they are not detected, the row is read as data. Otherwise data is read starting from the second row. * skipLeadingRows is 0 - Instructs autodetect that there are no headers and data should be read starting from the first row. * skipLeadingRows = N > 0 - Autodetect skips N-1 rows and tries to detect headers in row N. If headers are not detected, row N is just skipped. Otherwise row N is used to extract column names for the detected schema."]
        pub skip_leading_rows: ::std::option::Option<::std::string::String>,
    }
    impl GoogleSheetsOptions {
        pub fn builder() -> GoogleSheetsOptionsBuilder {
            GoogleSheetsOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct HivePartitioningOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] When set, what mode of hive partitioning to use when reading data. The following modes are supported. (1) AUTO: automatically infer partition key name(s) and type(s). (2) STRINGS: automatically infer partition key name(s). All types are interpreted as strings. (3) CUSTOM: partition key schema is encoded in the source URI prefix. Not all storage formats support hive partitioning. Requesting hive partitioning on an unsupported format will lead to an error. Currently supported types include: AVRO, CSV, JSON, ORC and Parquet."]
        pub mode: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requirePartitionFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] If set to true, queries over this table require a partition filter that can be used for partition elimination to be specified. Note that this field should only be true when creating a permanent external table or querying a temporary external table. Hive-partitioned loads with requirePartitionFilter explicitly set to true will fail."]
        pub require_partition_filter: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceUriPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] When hive partition detection is requested, a common prefix for all source uris should be supplied. The prefix must end immediately before the partition key encoding begins. For example, consider files following this data layout. gs://bucket/path_to_table/dt=2019-01-01/country=BR/id=7/file.avro gs://bucket/path_to_table/dt=2018-12-31/country=CA/id=3/file.avro When hive partitioning is requested with either AUTO or STRINGS detection, the common prefix can be either of gs://bucket/path_to_table or gs://bucket/path_to_table/ (trailing slash does not matter)."]
        pub source_uri_prefix: ::std::option::Option<::std::string::String>,
    }
    impl HivePartitioningOptions {
        pub fn builder() -> HivePartitioningOptionsBuilder {
            HivePartitioningOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a single iteration of the training run."]
    pub struct IterationResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arimaResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub arima_result: ::std::option::Option<::std::boxed::Box<ArimaResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterInfos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about top clusters for clustering models."]
        pub cluster_infos: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ClusterInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "durationMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time taken to run the iteration in milliseconds."]
        pub duration_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "evalLoss")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Loss computed on the eval data at the end of iteration."]
        pub eval_loss: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Index of the iteration, 0 based."]
        pub index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "learnRate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Learn rate used for this iteration."]
        pub learn_rate: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "principalComponentInfos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The information of the principal components."]
        pub principal_component_infos:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PrincipalComponentInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trainingLoss")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Loss computed on the training data at the end of iteration."]
        pub training_loss: ::std::option::Option<::std::primitive::f64>,
    }
    impl IterationResult {
        pub fn builder() -> IterationResultBuilder {
            IterationResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Job {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "configuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] Describes the job configuration."]
        pub configuration: ::std::option::Option<::std::boxed::Box<JobConfiguration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] A hash of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Opaque ID field of the job"]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Reference describing the unique-per-user name of the job."]
        pub job_reference: ::std::option::Option<::std::boxed::Box<JobReference>>,
        #[builder(default = "{ job_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "job_defaults :: kind")]
        #[doc = "[Output-only] The type of the resource."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] A URL that can be used to access this resource again."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statistics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Information about the job, including starting time and ending time of the job."]
        pub statistics: ::std::option::Option<::std::boxed::Box<JobStatistics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The status of this job. Examine this value when polling an asynchronous job to see if the job is complete."]
        pub status: ::std::option::Option<::std::boxed::Box<JobStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user_email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Email address of the user who ran the job."]
        pub user_email: ::std::option::Option<::std::string::String>,
    }
    impl Job {
        pub fn builder() -> JobBuilder {
            JobBuilder::default()
        }
    }
    mod job_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#job\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct JobCancelResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "job")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The final state of the job."]
        pub job: ::std::option::Option<::std::boxed::Box<Job>>,
        #[builder(default = "{ job_cancel_response_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "job_cancel_response_defaults :: kind")]
        #[doc = "The resource type of the response."]
        pub kind: ::std::string::String,
    }
    impl JobCancelResponse {
        pub fn builder() -> JobCancelResponseBuilder {
            JobCancelResponseBuilder::default()
        }
    }
    mod job_cancel_response_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#jobCancelResponse\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct JobConfiguration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "copy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Pick one] Copies a table."]
        pub copy: ::std::option::Option<::std::boxed::Box<JobConfigurationTableCopy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dryRun")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] If set, don't actually run this job. A valid query will return a mostly empty response with some processing statistics, while an invalid query will return the same error it would if it wasn't a dry run. Behavior of non-query jobs is undefined."]
        pub dry_run: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "extract")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Pick one] Configures an extract job."]
        pub extract: ::std::option::Option<::std::boxed::Box<JobConfigurationExtract>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobTimeoutMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Job timeout in milliseconds. If this time limit is exceeded, BigQuery may attempt to terminate the job."]
        pub job_timeout_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The type of the job. Can be QUERY, LOAD, EXTRACT, COPY or UNKNOWN."]
        pub job_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The labels associated with this job. You can use these to organize and group your jobs. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "load")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Pick one] Configures a load job."]
        pub load: ::std::option::Option<::std::boxed::Box<JobConfigurationLoad>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Pick one] Configures a query job."]
        pub query: ::std::option::Option<::std::boxed::Box<JobConfigurationQuery>>,
    }
    impl JobConfiguration {
        pub fn builder() -> JobConfigurationBuilder {
            JobConfigurationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct JobConfigurationExtract {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compression")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The compression type to use for exported files. Possible values include GZIP, DEFLATE, SNAPPY, and NONE. The default value is NONE. DEFLATE and SNAPPY are only supported for Avro. Not applicable when extracting models."]
        pub compression: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The exported file format. Possible values include CSV, NEWLINE_DELIMITED_JSON, PARQUET or AVRO for tables and ML_TF_SAVED_MODEL or ML_XGBOOST_BOOSTER for models. The default value for tables is CSV. Tables with nested or repeated fields cannot be exported as CSV. The default value for models is ML_TF_SAVED_MODEL."]
        pub destination_format: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Pick one] DEPRECATED: Use destinationUris instead, passing only one URI as necessary. The fully-qualified Google Cloud Storage URI where the extracted table should be written."]
        pub destination_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationUris")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Pick one] A list of fully-qualified Google Cloud Storage URIs where the extracted table should be written."]
        pub destination_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldDelimiter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Delimiter to use between fields in the exported data. Default is ','. Not applicable when extracting models."]
        pub field_delimiter: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ job_configuration_extract_defaults :: print_header () }",
            setter(into)
        )]
        #[serde(rename = "printHeader")]
        #[serde(default = "job_configuration_extract_defaults :: print_header")]
        #[doc = "[Optional] Whether to print out a header row in the results. Default is true. Not applicable when extracting models."]
        pub print_header: ::std::primitive::bool,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceModel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reference to the model being exported."]
        pub source_model: ::std::option::Option<::std::boxed::Box<ModelReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reference to the table being exported."]
        pub source_table: ::std::option::Option<::std::boxed::Box<TableReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useAvroLogicalTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] If destinationFormat is set to \"AVRO\", this flag indicates whether to enable extracting applicable column types (such as TIMESTAMP) to their corresponding AVRO logical types (timestamp-micros), instead of only using their raw types (avro-long). Not applicable when extracting models."]
        pub use_avro_logical_types: ::std::option::Option<::std::primitive::bool>,
    }
    impl JobConfigurationExtract {
        pub fn builder() -> JobConfigurationExtractBuilder {
            JobConfigurationExtractBuilder::default()
        }
    }
    mod job_configuration_extract_defaults {
        pub fn print_header() -> ::std::primitive::bool {
            serde_json::from_str(&"true").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct JobConfigurationLoad {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowJaggedRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Accept rows that are missing trailing optional columns. The missing values are treated as nulls. If false, records with missing trailing columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. Only applicable to CSV, ignored for other formats."]
        pub allow_jagged_rows: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowQuotedNewlines")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false."]
        pub allow_quoted_newlines: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autodetect")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Indicates if we should automatically infer the options and schema for CSV and JSON sources."]
        pub autodetect: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clustering")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Beta] Clustering specification for the destination table. Must be specified with time-based partitioning, data in the table will be first partitioned and subsequently clustered."]
        pub clustering: ::std::option::Option<::std::boxed::Box<Clustering>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createDisposition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion."]
        pub create_disposition: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "decimalTargetTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the list of possible SQL data types to which the source decimal values are converted. This list and the precision and the scale parameters of the decimal field determine the target type. In the order of NUMERIC, BIGNUMERIC ([Preview](/products/#product-launch-stages)), and STRING, a type is picked if it is in the specified list and if it supports the precision and the scale. STRING supports all precision and scale values. If none of the listed types supports the precision and the scale, the type supporting the widest range in the specified list is picked, and if a value exceeds the supported range when reading the data, an error will be thrown. Example: Suppose the value of this field is [\"NUMERIC\", \"BIGNUMERIC\"]. If (precision,scale) is: * (38,9) -> NUMERIC; * (39,9) -> BIGNUMERIC (NUMERIC cannot hold 30 integer digits); * (38,10) -> BIGNUMERIC (NUMERIC cannot hold 10 fractional digits); * (76,38) -> BIGNUMERIC; * (77,38) -> BIGNUMERIC (error if value exeeds supported range). This field cannot contain duplicate types. The order of the types in this field is ignored. For example, [\"BIGNUMERIC\", \"NUMERIC\"] is the same as [\"NUMERIC\", \"BIGNUMERIC\"] and NUMERIC always takes precedence over BIGNUMERIC. Defaults to [\"NUMERIC\", \"STRING\"] for ORC and [\"NUMERIC\"] for the other file formats."]
        pub decimal_target_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationEncryptionConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom encryption configuration (e.g., Cloud KMS keys)."]
        pub destination_encryption_configuration:
            ::std::option::Option<::std::boxed::Box<EncryptionConfiguration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The destination table to load the data into."]
        pub destination_table: ::std::option::Option<::std::boxed::Box<TableReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationTableProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Beta] [Optional] Properties with which to create the destination table if it is new."]
        pub destination_table_properties:
            ::std::option::Option<::std::boxed::Box<DestinationTableProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encoding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The character encoding of the data. The supported values are UTF-8 or ISO-8859-1. The default value is UTF-8. BigQuery decodes the data after the raw, binary data has been split using the values of the quote and fieldDelimiter properties."]
        pub encoding: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldDelimiter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The separator for fields in a CSV file. The separator can be any ISO-8859-1 single-byte character. To use a character in the range 128-255, you must encode the character as UTF8. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. BigQuery also supports the escape sequence \"\\t\" to specify a tab separator. The default value is a comma (',')."]
        pub field_delimiter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hivePartitioningOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Options to configure hive partitioning support."]
        pub hive_partitioning_options:
            ::std::option::Option<::std::boxed::Box<HivePartitioningOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ignoreUnknownValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Indicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. The sourceFormat property determines what BigQuery treats as an extra value: CSV: Trailing columns JSON: Named values that don't match any column names"]
        pub ignore_unknown_values: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jsonExtension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] If sourceFormat is set to newline-delimited JSON, indicates whether it should be processed as a JSON variant such as GeoJSON. For a sourceFormat other than JSON, omit this field. If the sourceFormat is newline-delimited JSON: - for newline-delimited GeoJSON: set to GEOJSON."]
        pub json_extension: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxBadRecords")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The maximum number of bad records that BigQuery can ignore when running the job. If the number of bad records exceeds this value, an invalid error is returned in the job result. This is only valid for CSV and JSON. The default value is 0, which requires that all records are valid."]
        pub max_bad_records: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nullMarker")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Specifies a string that represents a null value in a CSV file. For example, if you specify \"\\N\", BigQuery interprets \"\\N\" as a null value when loading a CSV file. The default value is the empty string. If you set this property to a custom value, BigQuery throws an error if an empty string is present for all data types except for STRING and BYTE. For STRING and BYTE columns, BigQuery interprets the empty string as an empty value."]
        pub null_marker: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectionFields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If sourceFormat is set to \"DATASTORE_BACKUP\", indicates which entity properties to load into BigQuery from a Cloud Datastore backup. Property names are case sensitive and must be top-level properties. If no properties are specified, BigQuery loads all properties. If any named property isn't found in the Cloud Datastore backup, an invalid error is returned in the job result."]
        pub projection_fields: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(
            default = "{ job_configuration_load_defaults :: quote () }",
            setter(into)
        )]
        #[serde(rename = "quote")]
        #[serde(default = "job_configuration_load_defaults :: quote")]
        #[doc = "[Optional] The value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. The default value is a double-quote ('\"'). If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true."]
        pub quote: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rangePartitioning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[TrustedTester] Range partitioning specification for this table. Only one of timePartitioning and rangePartitioning should be specified."]
        pub range_partitioning: ::std::option::Option<::std::boxed::Box<RangePartitioning>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schema")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The schema for the destination table. The schema can be omitted if the destination table already exists, or if you're loading data from Google Cloud Datastore."]
        pub schema: ::std::option::Option<::std::boxed::Box<TableSchema>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schemaInline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Deprecated] The inline schema. For CSV schemas, specify as \"Field1:Type1[,Field2:Type2]*\". For example, \"foo:STRING, bar:INTEGER, baz:FLOAT\"."]
        pub schema_inline: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schemaInlineFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Deprecated] The format of the schemaInline property."]
        pub schema_inline_format: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schemaUpdateOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allows the schema of the destination table to be updated as a side effect of the load job if a schema is autodetected or supplied in the job configuration. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND; when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified: ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema. ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable."]
        pub schema_update_options: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skipLeadingRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The number of rows at the top of a CSV file that BigQuery will skip when loading the data. The default value is 0. This property is useful if you have header rows in the file that should be skipped."]
        pub skip_leading_rows: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The format of the data files. For CSV files, specify \"CSV\". For datastore backups, specify \"DATASTORE_BACKUP\". For newline-delimited JSON, specify \"NEWLINE_DELIMITED_JSON\". For Avro, specify \"AVRO\". For parquet, specify \"PARQUET\". For orc, specify \"ORC\". The default value is CSV."]
        pub source_format: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceUris")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The fully-qualified URIs that point to your data in Google Cloud. For Google Cloud Storage URIs: Each URI can contain one '*' wildcard character and it must come after the 'bucket' name. Size limits related to load jobs apply to external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be specified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table. For Google Cloud Datastore backups: Exactly one URI can be specified. Also, the '*' wildcard character is not allowed."]
        pub source_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timePartitioning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-based partitioning specification for the destination table. Only one of timePartitioning and rangePartitioning should be specified."]
        pub time_partitioning: ::std::option::Option<::std::boxed::Box<TimePartitioning>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useAvroLogicalTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] If sourceFormat is set to \"AVRO\", indicates whether to enable interpreting logical types into their corresponding types (ie. TIMESTAMP), instead of only using their raw types (ie. INTEGER)."]
        pub use_avro_logical_types: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "writeDisposition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_APPEND. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion."]
        pub write_disposition: ::std::option::Option<::std::string::String>,
    }
    impl JobConfigurationLoad {
        pub fn builder() -> JobConfigurationLoadBuilder {
            JobConfigurationLoadBuilder::default()
        }
    }
    mod job_configuration_load_defaults {
        pub fn quote() -> ::std::string::String {
            serde_json::from_str(&"\"\\\"\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct JobConfigurationQuery {
        #[builder(
            default = "{ job_configuration_query_defaults :: allow_large_results () }",
            setter(into)
        )]
        #[serde(rename = "allowLargeResults")]
        #[serde(default = "job_configuration_query_defaults :: allow_large_results")]
        #[doc = "[Optional] If true and query uses legacy SQL dialect, allows the query to produce arbitrarily large result tables at a slight cost in performance. Requires destinationTable to be set. For standard SQL queries, this flag is ignored and large results are always allowed. However, you must still set destinationTable when result size exceeds the allowed maximum response size."]
        pub allow_large_results: ::std::primitive::bool,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clustering")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Beta] Clustering specification for the destination table. Must be specified with time-based partitioning, data in the table will be first partitioned and subsequently clustered."]
        pub clustering: ::std::option::Option<::std::boxed::Box<Clustering>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "connectionProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Connection properties."]
        pub connection_properties:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ConnectionProperty>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createDisposition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion."]
        pub create_disposition: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultDataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Specifies the default dataset to use for unqualified table names in the query. Note that this does not alter behavior of unqualified dataset names."]
        pub default_dataset: ::std::option::Option<::std::boxed::Box<DatasetReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationEncryptionConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom encryption configuration (e.g., Cloud KMS keys)."]
        pub destination_encryption_configuration:
            ::std::option::Option<::std::boxed::Box<EncryptionConfiguration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Describes the table where the query results should be stored. If not present, a new table will be created to store the results. This property must be set for large results that exceed the maximum response size."]
        pub destination_table: ::std::option::Option<::std::boxed::Box<TableReference>>,
        #[builder(
            default = "{ job_configuration_query_defaults :: flatten_results () }",
            setter(into)
        )]
        #[serde(rename = "flattenResults")]
        #[serde(default = "job_configuration_query_defaults :: flatten_results")]
        #[doc = "[Optional] If true and query uses legacy SQL dialect, flattens all nested and repeated fields in the query results. allowLargeResults must be true if this is set to false. For standard SQL queries, this flag is ignored and results are never flattened."]
        pub flatten_results: ::std::primitive::bool,
        #[builder(
            default = "{ job_configuration_query_defaults :: maximum_billing_tier () }",
            setter(into)
        )]
        #[serde(rename = "maximumBillingTier")]
        #[serde(default = "job_configuration_query_defaults :: maximum_billing_tier")]
        #[doc = "[Optional] Limits the billing tier for this job. Queries that have resource usage beyond this tier will fail (without incurring a charge). If unspecified, this will be set to your project default."]
        pub maximum_billing_tier: ::std::primitive::i64,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maximumBytesBilled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Limits the bytes billed for this job. Queries that will have bytes billed beyond this limit will fail (without incurring a charge). If unspecified, this will be set to your project default."]
        pub maximum_bytes_billed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameterMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Standard SQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query."]
        pub parameter_mode: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "preserveNulls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Deprecated] This property is deprecated."]
        pub preserve_nulls: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "priority")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Specifies a priority for the query. Possible values include INTERACTIVE and BATCH. The default value is INTERACTIVE."]
        pub priority: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] SQL query text to execute. The useLegacySql field can be used to indicate whether the query uses legacy SQL or standard SQL."]
        pub query: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Query parameters for standard SQL queries."]
        pub query_parameters:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<QueryParameter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rangePartitioning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[TrustedTester] Range partitioning specification for this table. Only one of timePartitioning and rangePartitioning should be specified."]
        pub range_partitioning: ::std::option::Option<::std::boxed::Box<RangePartitioning>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schemaUpdateOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allows the schema of the destination table to be updated as a side effect of the query job. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND; when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified: ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema. ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable."]
        pub schema_update_options: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableDefinitions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] If querying an external data source outside of BigQuery, describes the data format, location and other properties of the data source. By defining these properties, the data source can then be queried as if it were a standard BigQuery table."]
        pub table_definitions: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<ExternalDataConfiguration>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timePartitioning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-based partitioning specification for the destination table. Only one of timePartitioning and rangePartitioning should be specified."]
        pub time_partitioning: ::std::option::Option<::std::boxed::Box<TimePartitioning>>,
        #[builder(
            default = "{ job_configuration_query_defaults :: use_legacy_sql () }",
            setter(into)
        )]
        #[serde(rename = "useLegacySql")]
        #[serde(default = "job_configuration_query_defaults :: use_legacy_sql")]
        #[doc = "Specifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true. If set to false, the query will use BigQuery's standard SQL: https://cloud.google.com/bigquery/sql-reference/ When useLegacySql is set to false, the value of flattenResults is ignored; query will be run as if flattenResults is false."]
        pub use_legacy_sql: ::std::primitive::bool,
        #[builder(
            default = "{ job_configuration_query_defaults :: use_query_cache () }",
            setter(into)
        )]
        #[serde(rename = "useQueryCache")]
        #[serde(default = "job_configuration_query_defaults :: use_query_cache")]
        #[doc = "[Optional] Whether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever tables in the query are modified. Moreover, the query cache is only available when a query does not have a destination table specified. The default value is true."]
        pub use_query_cache: ::std::primitive::bool,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userDefinedFunctionResources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes user-defined function resources used in the query."]
        pub user_defined_function_resources:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UserDefinedFunctionResource>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "writeDisposition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_EMPTY. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion."]
        pub write_disposition: ::std::option::Option<::std::string::String>,
    }
    impl JobConfigurationQuery {
        pub fn builder() -> JobConfigurationQueryBuilder {
            JobConfigurationQueryBuilder::default()
        }
    }
    mod job_configuration_query_defaults {
        pub fn allow_large_results() -> ::std::primitive::bool {
            serde_json::from_str(&"false").unwrap()
        }
        pub fn flatten_results() -> ::std::primitive::bool {
            serde_json::from_str(&"true").unwrap()
        }
        pub fn maximum_billing_tier() -> ::std::primitive::i64 {
            serde_json::from_str(&"1").unwrap()
        }
        pub fn use_legacy_sql() -> ::std::primitive::bool {
            serde_json::from_str(&"true").unwrap()
        }
        pub fn use_query_cache() -> ::std::primitive::bool {
            serde_json::from_str(&"true").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct JobConfigurationTableCopy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createDisposition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion."]
        pub create_disposition: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationEncryptionConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom encryption configuration (e.g., Cloud KMS keys)."]
        pub destination_encryption_configuration:
            ::std::option::Option<::std::boxed::Box<EncryptionConfiguration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationExpirationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The time when the destination table expires. Expired tables will be deleted and their storage reclaimed."]
        pub destination_expiration_time: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The destination table"]
        pub destination_table: ::std::option::Option<::std::boxed::Box<TableReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Supported operation types in table copy job."]
        pub operation_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Pick one] Source table to copy."]
        pub source_table: ::std::option::Option<::std::boxed::Box<TableReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceTables")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Pick one] Source tables to copy."]
        pub source_tables:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TableReference>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "writeDisposition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_EMPTY. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion."]
        pub write_disposition: ::std::option::Option<::std::string::String>,
    }
    impl JobConfigurationTableCopy {
        pub fn builder() -> JobConfigurationTableCopyBuilder {
            JobConfigurationTableCopyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct JobList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A hash of this page of results."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of jobs that were requested."]
        pub jobs: ::std::option::Option<::std::vec::Vec<JobListJobs>>,
        #[builder(default = "{ job_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "job_list_defaults :: kind")]
        #[doc = "The resource type of the response."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to request the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl JobList {
        pub fn builder() -> JobListBuilder {
            JobListBuilder::default()
        }
    }
    mod job_list_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#jobList\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct JobListJobs {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "configuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Full-projection-only] Specifies the job configuration."]
        pub configuration: ::std::option::Option<::std::boxed::Box<JobConfiguration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A result object that will be present only if the job has failed."]
        pub error_result: ::std::option::Option<::std::boxed::Box<ErrorProto>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique opaque ID of the job."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Job reference uniquely identifying the job."]
        pub job_reference: ::std::option::Option<::std::boxed::Box<JobReference>>,
        #[builder(default = "{ job_list_jobs_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "job_list_jobs_defaults :: kind")]
        #[doc = "The resource type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Running state of the job. When the state is DONE, errorResult can be checked to determine whether the job succeeded or failed."]
        pub state: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statistics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Information about the job, including starting time and ending time of the job."]
        pub statistics: ::std::option::Option<::std::boxed::Box<JobStatistics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Full-projection-only] Describes the state of the job."]
        pub status: ::std::option::Option<::std::boxed::Box<JobStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user_email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Full-projection-only] Email address of the user who ran the job."]
        pub user_email: ::std::option::Option<::std::string::String>,
    }
    impl JobListJobs {
        pub fn builder() -> JobListJobsBuilder {
            JobListJobsBuilder::default()
        }
    }
    mod job_list_jobs_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#job\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct JobReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The ID of the job. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-). The maximum length is 1,024 characters."]
        pub job_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The geographic location of the job. See details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The ID of the project containing this job."]
        pub project_id: ::std::option::Option<::std::string::String>,
    }
    impl JobReference {
        pub fn builder() -> JobReferenceBuilder {
            JobReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct JobStatistics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "completionRatio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[TrustedTester] [Output-only] Job progress (0.0 -> 1.0) for LOAD and EXTRACT jobs."]
        pub completion_ratio: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Creation time of this job, in milliseconds since the epoch. This field will be present on all jobs."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] End time of this job, in milliseconds since the epoch. This field will be present whenever a job is in the DONE state."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "extract")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Statistics for an extract job."]
        pub extract: ::std::option::Option<::std::boxed::Box<JobStatistics4>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "load")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Statistics for a load job."]
        pub load: ::std::option::Option<::std::boxed::Box<JobStatistics3>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numChildJobs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Number of child jobs executed."]
        pub num_child_jobs: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentJobId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] If this is a child job, the id of the parent."]
        pub parent_job_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Statistics for a query job."]
        pub query: ::std::option::Option<::std::boxed::Box<JobStatistics2>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaDeferments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Quotas which delayed this job's start time."]
        pub quota_deferments: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reservationUsage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Job resource usage breakdown by reservation."]
        pub reservation_usage:
            ::std::option::Option<::std::vec::Vec<JobStatisticsReservationUsage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reservation_id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Name of the primary reservation assigned to this job. Note that this could be different than reservations reported in the reservation usage field if parent reservations were used to execute this job."]
        pub reservation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowLevelSecurityStatistics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] [Preview] Statistics for row-level security. Present only for query and extract jobs."]
        pub row_level_security_statistics:
            ::std::option::Option<::std::boxed::Box<RowLevelSecurityStatistics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scriptStatistics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Statistics for a child job of a script."]
        pub script_statistics: ::std::option::Option<::std::boxed::Box<ScriptStatistics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Start time of this job, in milliseconds since the epoch. This field will be present when the job transitions from the PENDING state to either RUNNING or DONE."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalBytesProcessed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] [Deprecated] Use the bytes processed in the query statistics instead."]
        pub total_bytes_processed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSlotMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Slot-milliseconds for the job."]
        pub total_slot_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transactionInfoTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] [Alpha] Information of the multi-statement transaction if this job is part of one."]
        pub transaction_info_template: ::std::option::Option<::std::boxed::Box<TransactionInfo>>,
    }
    impl JobStatistics {
        pub fn builder() -> JobStatisticsBuilder {
            JobStatisticsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct JobStatisticsReservationUsage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Reservation name or \"unreserved\" for on-demand resources usage."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slotMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Slot-milliseconds the job spent in the given reservation."]
        pub slot_ms: ::std::option::Option<::std::string::String>,
    }
    impl JobStatisticsReservationUsage {
        pub fn builder() -> JobStatisticsReservationUsageBuilder {
            JobStatisticsReservationUsageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct JobStatistics2 {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "billingTier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Billing tier for the job."]
        pub billing_tier: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cacheHit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Whether the query result was fetched from the query cache."]
        pub cache_hit: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ddlAffectedRowAccessPolicyCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] [Preview] The number of row access policies affected by a DDL statement. Present only for DROP ALL ROW ACCESS POLICIES queries."]
        pub ddl_affected_row_access_policy_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ddlOperationPerformed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The DDL operation performed, possibly dependent on the pre-existence of the DDL target. Possible values (new values might be added in the future): \"CREATE\": The query created the DDL target. \"SKIP\": No-op. Example cases: the query is CREATE TABLE IF NOT EXISTS while the table already exists, or the query is DROP TABLE IF EXISTS while the table does not exist. \"REPLACE\": The query replaced the DDL target. Example case: the query is CREATE OR REPLACE TABLE, and the table already exists. \"DROP\": The query deleted the DDL target."]
        pub ddl_operation_performed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ddlTargetDataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The DDL target dataset. Present only for CREATE/ALTER/DROP SCHEMA queries."]
        pub ddl_target_dataset: ::std::option::Option<::std::boxed::Box<DatasetReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ddlTargetRoutine")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The DDL target routine. Present only for CREATE/DROP FUNCTION/PROCEDURE queries."]
        pub ddl_target_routine: ::std::option::Option<::std::boxed::Box<RoutineReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ddlTargetRowAccessPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] [Preview] The DDL target row access policy. Present only for CREATE/DROP ROW ACCESS POLICY queries."]
        pub ddl_target_row_access_policy:
            ::std::option::Option<::std::boxed::Box<RowAccessPolicyReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ddlTargetTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The DDL target table. Present only for CREATE/DROP TABLE/VIEW and DROP ALL ROW ACCESS POLICIES queries."]
        pub ddl_target_table: ::std::option::Option<::std::boxed::Box<TableReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "estimatedBytesProcessed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The original estimate of bytes processed for the job."]
        pub estimated_bytes_processed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modelTraining")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only, Beta] Information about create model query job progress."]
        pub model_training: ::std::option::Option<::std::boxed::Box<BigQueryModelTraining>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modelTrainingCurrentIteration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only, Beta] Deprecated; do not use."]
        pub model_training_current_iteration: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modelTrainingExpectedTotalIteration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only, Beta] Deprecated; do not use."]
        pub model_training_expected_total_iteration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numDmlAffectedRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE."]
        pub num_dml_affected_rows: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryPlan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Describes execution plan for the query."]
        pub query_plan:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ExplainQueryStage>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referencedRoutines")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Referenced routines (persistent user-defined functions and stored procedures) for the job."]
        pub referenced_routines:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RoutineReference>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referencedTables")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Referenced tables for the job. Queries that reference more than 50 tables will not have a complete list."]
        pub referenced_tables:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TableReference>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reservationUsage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Job resource usage breakdown by reservation."]
        pub reservation_usage:
            ::std::option::Option<::std::vec::Vec<JobStatistics2ReservationUsage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schema")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The schema of the results. Present only for successful dry run of non-legacy SQL queries."]
        pub schema: ::std::option::Option<::std::boxed::Box<TableSchema>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statementType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of query statement, if valid. Possible values (new values might be added in the future): \"SELECT\": SELECT query. \"INSERT\": INSERT query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"UPDATE\": UPDATE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"DELETE\": DELETE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"MERGE\": MERGE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"ALTER_TABLE\": ALTER TABLE query. \"ALTER_VIEW\": ALTER VIEW query. \"ASSERT\": ASSERT condition AS 'description'. \"CREATE_FUNCTION\": CREATE FUNCTION query. \"CREATE_MODEL\": CREATE [OR REPLACE] MODEL ... AS SELECT ... . \"CREATE_PROCEDURE\": CREATE PROCEDURE query. \"CREATE_TABLE\": CREATE [OR REPLACE] TABLE without AS SELECT. \"CREATE_TABLE_AS_SELECT\": CREATE [OR REPLACE] TABLE ... AS SELECT ... . \"CREATE_VIEW\": CREATE [OR REPLACE] VIEW ... AS SELECT ... . \"DROP_FUNCTION\" : DROP FUNCTION query. \"DROP_PROCEDURE\": DROP PROCEDURE query. \"DROP_TABLE\": DROP TABLE query. \"DROP_VIEW\": DROP VIEW query."]
        pub statement_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] [Beta] Describes a timeline of job execution."]
        pub timeline:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<QueryTimelineSample>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalBytesBilled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Total bytes billed for the job."]
        pub total_bytes_billed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalBytesProcessed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Total bytes processed for the job."]
        pub total_bytes_processed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalBytesProcessedAccuracy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] For dry-run jobs, totalBytesProcessed is an estimate and this field specifies the accuracy of the estimate. Possible values can be: UNKNOWN: accuracy of the estimate is unknown. PRECISE: estimate is precise. LOWER_BOUND: estimate is lower bound of what the query would cost. UPPER_BOUND: estimate is upper bound of what the query would cost."]
        pub total_bytes_processed_accuracy: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalPartitionsProcessed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Total number of partitions processed from all partitioned tables referenced in the job."]
        pub total_partitions_processed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSlotMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Slot-milliseconds for the job."]
        pub total_slot_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "undeclaredQueryParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Standard SQL only: list of undeclared query parameters detected during a dry run validation."]
        pub undeclared_query_parameters:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<QueryParameter>>>,
    }
    impl JobStatistics2 {
        pub fn builder() -> JobStatistics2Builder {
            JobStatistics2Builder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct JobStatistics2ReservationUsage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Reservation name or \"unreserved\" for on-demand resources usage."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slotMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Slot-milliseconds the job spent in the given reservation."]
        pub slot_ms: ::std::option::Option<::std::string::String>,
    }
    impl JobStatistics2ReservationUsage {
        pub fn builder() -> JobStatistics2ReservationUsageBuilder {
            JobStatistics2ReservationUsageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct JobStatistics3 {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "badRecords")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The number of bad records encountered. Note that if the job has failed because of more bad records encountered than the maximum allowed in the load job configuration, then this number can be less than the total number of bad records present in the input data."]
        pub bad_records: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputFileBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Number of bytes of source data in a load job."]
        pub input_file_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputFiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Number of source files in a load job."]
        pub input_files: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Size of the loaded data in bytes. Note that while a load job is in the running state, this value may change."]
        pub output_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Number of rows imported in a load job. Note that while an import job is in the running state, this value may change."]
        pub output_rows: ::std::option::Option<::std::string::String>,
    }
    impl JobStatistics3 {
        pub fn builder() -> JobStatistics3Builder {
            JobStatistics3Builder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct JobStatistics4 {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationUriFileCounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Number of files per destination URI or URI pattern specified in the extract configuration. These values will be in the same order as the URIs specified in the 'destinationUris' field."]
        pub destination_uri_file_counts:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Number of user bytes extracted into the result. This is the byte count as computed by BigQuery for billing purposes."]
        pub input_bytes: ::std::option::Option<::std::string::String>,
    }
    impl JobStatistics4 {
        pub fn builder() -> JobStatistics4Builder {
            JobStatistics4Builder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct JobStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Final error result of the job. If present, indicates that the job has completed and was unsuccessful."]
        pub error_result: ::std::option::Option<::std::boxed::Box<ErrorProto>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The first errors encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has completed or was unsuccessful."]
        pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ErrorProto>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Running state of the job."]
        pub state: ::std::option::Option<::std::string::String>,
    }
    impl JobStatus {
        pub fn builder() -> JobStatusBuilder {
            JobStatusBuilder::default()
        }
    }
    #[doc = "Represents a single JSON object."]
    pub type JsonObject = ::std::collections::BTreeMap<String, ::std::boxed::Box<JsonValue>>;
    pub type JsonValue = ::serde_json::Value;
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListModelsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "models")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Models in the requested dataset. Only the following fields are populated: model_reference, model_type, creation_time, last_modified_time and labels."]
        pub models: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Model>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to request the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListModelsResponse {
        pub fn builder() -> ListModelsResponseBuilder {
            ListModelsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListRoutinesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to request the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "routines")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Routines in the requested dataset. Unless read_mask is set in the request, only the following fields are populated: etag, project_id, dataset_id, routine_id, routine_type, creation_time, last_modified_time, and language."]
        pub routines: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Routine>>>,
    }
    impl ListRoutinesResponse {
        pub fn builder() -> ListRoutinesResponseBuilder {
            ListRoutinesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for the ListRowAccessPolicies method."]
    pub struct ListRowAccessPoliciesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to request the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowAccessPolicies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Row access policies on the requested table."]
        pub row_access_policies:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RowAccessPolicy>>>,
    }
    impl ListRowAccessPoliciesResponse {
        pub fn builder() -> ListRowAccessPoliciesResponseBuilder {
            ListRowAccessPoliciesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "BigQuery-specific metadata about a location. This will be set on google.cloud.location.Location.metadata in Cloud Location API responses."]
    pub struct LocationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "legacyLocationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The legacy BigQuery location ID, e.g. “EU” for the “europe” location. This is for any API consumers that need the legacy “US” and “EU” locations."]
        pub legacy_location_id: ::std::option::Option<::std::string::String>,
    }
    impl LocationMetadata {
        pub fn builder() -> LocationMetadataBuilder {
            LocationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct MaterializedViewDefinition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableRefresh")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] [TrustedTester] Enable automatic refresh of the materialized view when the base table is updated. The default value is \"true\"."]
        pub enable_refresh: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastRefreshTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] [TrustedTester] The time when this materialized view was last modified, in milliseconds since the epoch."]
        pub last_refresh_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] A query whose result is persisted."]
        pub query: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "refreshIntervalMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] [TrustedTester] The maximum frequency at which this materialized view will be refreshed. The default value is \"1800000\" (30 minutes)."]
        pub refresh_interval_ms: ::std::option::Option<::std::string::String>,
    }
    impl MaterializedViewDefinition {
        pub fn builder() -> MaterializedViewDefinitionBuilder {
            MaterializedViewDefinitionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Model {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time when this model was created, in millisecs since the epoch."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A user-friendly description of this model."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encryptionConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom encryption configuration (e.g., Cloud KMS keys). This shows the encryption configuration of the model data while stored in BigQuery storage. This field can be used with PatchModel to update encryption key for an already encrypted model."]
        pub encryption_configuration:
            ::std::option::Option<::std::boxed::Box<EncryptionConfiguration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A hash of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expirationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The time when this model expires, in milliseconds since the epoch. If not present, the model will persist indefinitely. Expired models will be deleted and their storage reclaimed. The defaultTableExpirationMs property of the encapsulating dataset can be used to set a default expirationTime on newly created models."]
        pub expiration_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "featureColumns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Input feature columns that were used to train this model."]
        pub feature_columns:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StandardSqlField>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "friendlyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A descriptive name for this model."]
        pub friendly_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelColumns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Label columns that were used to train this model. The output of the model will have a \"predicted_\" prefix to these columns."]
        pub label_columns:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StandardSqlField>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The labels associated with this model. You can use these to organize and group your models. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time when this model was last modified, in millisecs since the epoch."]
        pub last_modified_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The geographic location where the model resides. This value is inherited from the dataset."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modelReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Unique identifier for this model."]
        pub model_reference: ::std::option::Option<::std::boxed::Box<ModelReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modelType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Type of the model resource."]
        pub model_type: ::std::option::Option<ModelModelTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trainingRuns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Information for all training runs in increasing order of start_time."]
        pub training_runs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TrainingRun>>>,
    }
    impl Model {
        pub fn builder() -> ModelBuilder {
            ModelBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Type of the model resource."]
    pub enum ModelModelTypeEnum {
        #[serde(rename = "MODEL_TYPE_UNSPECIFIED")]
        #[doc = ""]
        ModelTypeUnspecified,
        #[serde(rename = "LINEAR_REGRESSION")]
        #[doc = "Linear regression model."]
        LinearRegression,
        #[serde(rename = "LOGISTIC_REGRESSION")]
        #[doc = "Logistic regression based classification model."]
        LogisticRegression,
        #[serde(rename = "KMEANS")]
        #[doc = "K-means clustering model."]
        Kmeans,
        #[serde(rename = "MATRIX_FACTORIZATION")]
        #[doc = "Matrix factorization model."]
        MatrixFactorization,
        #[serde(rename = "DNN_CLASSIFIER")]
        #[doc = "DNN classifier model."]
        DnnClassifier,
        #[serde(rename = "TENSORFLOW")]
        #[doc = "An imported TensorFlow model."]
        Tensorflow,
        #[serde(rename = "DNN_REGRESSOR")]
        #[doc = "DNN regressor model."]
        DnnRegressor,
        #[serde(rename = "BOOSTED_TREE_REGRESSOR")]
        #[doc = "Boosted tree regressor model."]
        BoostedTreeRegressor,
        #[serde(rename = "BOOSTED_TREE_CLASSIFIER")]
        #[doc = "Boosted tree classifier model."]
        BoostedTreeClassifier,
        #[serde(rename = "ARIMA")]
        #[doc = "ARIMA model."]
        Arima,
        #[serde(rename = "AUTOML_REGRESSOR")]
        #[doc = "[Beta] AutoML Tables regression model."]
        AutomlRegressor,
        #[serde(rename = "AUTOML_CLASSIFIER")]
        #[doc = "[Beta] AutoML Tables classification model."]
        AutomlClassifier,
    }
    impl ::std::default::Default for ModelModelTypeEnum {
        fn default() -> Self {
            Self::ModelTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ModelDefinition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modelOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only, Beta] Model options used for the first training run. These options are immutable for subsequent training runs. Default values are used for any options not specified in the input query."]
        pub model_options: ::std::option::Option<ModelDefinitionModelOptions>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trainingRuns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only, Beta] Information about ml training runs, each training run comprises of multiple iterations and there may be multiple training runs for the model if warm start is used or if a user decides to continue a previously cancelled query."]
        pub training_runs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BqmlTrainingRun>>>,
    }
    impl ModelDefinition {
        pub fn builder() -> ModelDefinitionBuilder {
            ModelDefinitionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "[Output-only, Beta] Model options used for the first training run. These options are immutable for subsequent training runs. Default values are used for any options not specified in the input query."]
    pub struct ModelDefinitionModelOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub labels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lossType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub loss_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modelType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub model_type: ::std::option::Option<::std::string::String>,
    }
    impl ModelDefinitionModelOptions {
        pub fn builder() -> ModelDefinitionModelOptionsBuilder {
            ModelDefinitionModelOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ModelReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datasetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The ID of the dataset containing this model."]
        pub dataset_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The ID of the model. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters."]
        pub model_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The ID of the project containing this model."]
        pub project_id: ::std::option::Option<::std::string::String>,
    }
    impl ModelReference {
        pub fn builder() -> ModelReferenceBuilder {
            ModelReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Evaluation metrics for multi-class classification/classifier models."]
    pub struct MultiClassClassificationMetrics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aggregateClassificationMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aggregate classification metrics."]
        pub aggregate_classification_metrics:
            ::std::option::Option<::std::boxed::Box<AggregateClassificationMetrics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confusionMatrixList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confusion matrix at different thresholds."]
        pub confusion_matrix_list:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ConfusionMatrix>>>,
    }
    impl MultiClassClassificationMetrics {
        pub fn builder() -> MultiClassClassificationMetricsBuilder {
            MultiClassClassificationMetricsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/)."]
    pub struct Policy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "auditConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies cloud audit logging configuration for this policy."]
        pub audit_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditConfig>>>,
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
    #[doc = "Principal component infos, used only for eigen decomposition based models, e.g., PCA. Ordered by explained_variance in the descending order."]
    pub struct PrincipalComponentInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cumulativeExplainedVarianceRatio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explained_variance is pre-ordered in the descending order to compute the cumulative explained variance ratio."]
        pub cumulative_explained_variance_ratio: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explainedVariance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Explained variance by this principal component, which is simply the eigenvalue."]
        pub explained_variance: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explainedVarianceRatio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Explained_variance over the total explained variance."]
        pub explained_variance_ratio: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "principalComponentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Id of the principal component."]
        pub principal_component_id: ::std::option::Option<::std::string::String>,
    }
    impl PrincipalComponentInfo {
        pub fn builder() -> PrincipalComponentInfoBuilder {
            PrincipalComponentInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProjectList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A hash of the page of results"]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ project_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "project_list_defaults :: kind")]
        #[doc = "The type of list."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to request the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projects")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Projects to which you have at least READ access."]
        pub projects: ::std::option::Option<::std::vec::Vec<ProjectListProjects>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of projects in the list."]
        pub total_items: ::std::option::Option<::std::primitive::i64>,
    }
    impl ProjectList {
        pub fn builder() -> ProjectListBuilder {
            ProjectListBuilder::default()
        }
    }
    mod project_list_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#projectList\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProjectListProjects {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "friendlyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A descriptive name for this project."]
        pub friendly_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An opaque ID of this project."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ project_list_projects_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "project_list_projects_defaults :: kind")]
        #[doc = "The resource type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numericId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The numeric ID of this project."]
        pub numeric_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique reference to this project."]
        pub project_reference: ::std::option::Option<::std::boxed::Box<ProjectReference>>,
    }
    impl ProjectListProjects {
        pub fn builder() -> ProjectListProjectsBuilder {
            ProjectListProjectsBuilder::default()
        }
    }
    mod project_list_projects_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#project\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProjectReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] ID of the project. Can be either the numeric ID or the assigned ID of the project."]
        pub project_id: ::std::option::Option<::std::string::String>,
    }
    impl ProjectReference {
        pub fn builder() -> ProjectReferenceBuilder {
            ProjectReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct QueryParameter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] If unset, this is a positional parameter. Otherwise, should be unique within a query."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameterType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The type of this parameter."]
        pub parameter_type: ::std::option::Option<::std::boxed::Box<QueryParameterType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameterValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The value of this parameter."]
        pub parameter_value: ::std::option::Option<::std::boxed::Box<QueryParameterValue>>,
    }
    impl QueryParameter {
        pub fn builder() -> QueryParameterBuilder {
            QueryParameterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct QueryParameterType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arrayType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The type of the array's elements, if this is an array."]
        pub array_type: ::std::option::Option<::std::boxed::Box<QueryParameterType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "structTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The types of the fields of this struct, in order, if this is a struct."]
        pub struct_types: ::std::option::Option<::std::vec::Vec<QueryParameterTypeStructTypes>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The top level type of this field."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl QueryParameterType {
        pub fn builder() -> QueryParameterTypeBuilder {
            QueryParameterTypeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct QueryParameterTypeStructTypes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Human-oriented description of the field."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The name of this field."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The type of this field."]
        pub _type: ::std::option::Option<::std::boxed::Box<QueryParameterType>>,
    }
    impl QueryParameterTypeStructTypes {
        pub fn builder() -> QueryParameterTypeStructTypesBuilder {
            QueryParameterTypeStructTypesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct QueryParameterValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arrayValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The array values, if this is an array type."]
        pub array_values:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<QueryParameterValue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "structValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The struct field values, in order of the struct type's declaration."]
        pub struct_values: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<QueryParameterValue>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The value of this value, if a simple scalar type."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl QueryParameterValue {
        pub fn builder() -> QueryParameterValueBuilder {
            QueryParameterValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct QueryRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "connectionProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Connection properties."]
        pub connection_properties:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ConnectionProperty>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultDataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Specifies the default datasetId and projectId to assume for any unqualified table names in the query. If not set, all table names in the query string must be qualified in the format 'datasetId.tableId'."]
        pub default_dataset: ::std::option::Option<::std::boxed::Box<DatasetReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dryRun")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] If set to true, BigQuery doesn't run the job. Instead, if the query is valid, BigQuery returns statistics about the job such as how many bytes would be processed. If the query is invalid, an error returns. The default value is false."]
        pub dry_run: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ query_request_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "query_request_defaults :: kind")]
        #[doc = "The resource type of the request."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The labels associated with this job. You can use these to organize and group your jobs. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The geographic location where the job should run. See details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The maximum number of rows of data to return per page of results. Setting this flag to a small value such as 1000 and then paging through results might improve reliability when the query result set is large. In addition to this limit, responses are also limited to 10 MB. By default, there is no maximum row count, and only the byte limit applies."]
        pub max_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maximumBytesBilled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Limits the bytes billed for this job. Queries that will have bytes billed beyond this limit will fail (without incurring a charge). If unspecified, this will be set to your project default."]
        pub maximum_bytes_billed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameterMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Standard SQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query."]
        pub parameter_mode: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "preserveNulls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Deprecated] This property is deprecated."]
        pub preserve_nulls: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] A query string, following the BigQuery query syntax, of the query to execute. Example: \"SELECT count(f1) FROM [myProjectId:myDatasetId.myTableId]\"."]
        pub query: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Query parameters for Standard SQL queries."]
        pub query_parameters:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<QueryParameter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique user provided identifier to ensure idempotent behavior for queries. Note that this is different from the job_id. It has the following properties: 1. It is case-sensitive, limited to up to 36 ASCII characters. A UUID is recommended. 2. Read only queries can ignore this token since they are nullipotent by definition. 3. For the purposes of idempotency ensured by the request_id, a request is considered duplicate of another only if they have the same request_id and are actually duplicates. When determining whether a request is a duplicate of the previous request, all parameters in the request that may affect the behavior are considered. For example, query, connection_properties, query_parameters, use_legacy_sql are parameters that affect the result and are considered when determining whether a request is a duplicate, but properties like timeout_ms don't affect the result and are thus not considered. Dry run query requests are never considered duplicate of another request. 4. When a duplicate mutating query request is detected, it returns: a. the results of the mutation if it completes successfully within the timeout. b. the running operation if it is still in progress at the end of the timeout. 5. Its lifetime is limited to 15 minutes. In other words, if two requests are sent with the same request_id, but more than 15 minutes apart, idempotency is not guaranteed."]
        pub request_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeoutMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] How long to wait for the query to complete, in milliseconds, before the request times out and returns. Note that this is only a timeout for the request, not the query. If the query takes longer to run than the timeout value, the call returns without any results and with the 'jobComplete' flag set to false. You can call GetQueryResults() to wait for the query to complete and read the results. The default value is 10000 milliseconds (10 seconds)."]
        pub timeout_ms: ::std::option::Option<::std::primitive::i64>,
        #[builder(
            default = "{ query_request_defaults :: use_legacy_sql () }",
            setter(into)
        )]
        #[serde(rename = "useLegacySql")]
        #[serde(default = "query_request_defaults :: use_legacy_sql")]
        #[doc = "Specifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true. If set to false, the query will use BigQuery's standard SQL: https://cloud.google.com/bigquery/sql-reference/ When useLegacySql is set to false, the value of flattenResults is ignored; query will be run as if flattenResults is false."]
        pub use_legacy_sql: ::std::primitive::bool,
        #[builder(
            default = "{ query_request_defaults :: use_query_cache () }",
            setter(into)
        )]
        #[serde(rename = "useQueryCache")]
        #[serde(default = "query_request_defaults :: use_query_cache")]
        #[doc = "[Optional] Whether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever tables in the query are modified. The default value is true."]
        pub use_query_cache: ::std::primitive::bool,
    }
    impl QueryRequest {
        pub fn builder() -> QueryRequestBuilder {
            QueryRequestBuilder::default()
        }
    }
    mod query_request_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#queryRequest\"").unwrap()
        }
        pub fn use_legacy_sql() -> ::std::primitive::bool {
            serde_json::from_str(&"true").unwrap()
        }
        pub fn use_query_cache() -> ::std::primitive::bool {
            serde_json::from_str(&"true").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct QueryResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cacheHit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the query result was fetched from the query cache."]
        pub cache_hit: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The first errors or warnings encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has completed or was unsuccessful."]
        pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ErrorProto>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobComplete")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the query has completed or not. If rows or totalRows are present, this will always be true. If this is false, totalRows will not be available."]
        pub job_complete: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reference to the Job that was created to run the query. This field will be present even if the original request timed out, in which case GetQueryResults can be used to read the results once the query has completed. Since this API only returns the first page of results, subsequent pages can be fetched via the same mechanism (GetQueryResults)."]
        pub job_reference: ::std::option::Option<::std::boxed::Box<JobReference>>,
        #[builder(default = "{ query_response_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "query_response_defaults :: kind")]
        #[doc = "The resource type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numDmlAffectedRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE."]
        pub num_dml_affected_rows: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token used for paging results."]
        pub page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An object with as many results as can be contained within the maximum permitted reply size. To get any additional rows, you can call GetQueryResults and specify the jobReference returned above."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TableRow>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schema")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The schema of the results. Present only when the query completes successfully."]
        pub schema: ::std::option::Option<::std::boxed::Box<TableSchema>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalBytesProcessed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of bytes processed for this query. If this query was a dry run, this is the number of bytes that would be processed if the query were run."]
        pub total_bytes_processed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of rows in the complete query result set, which can be more than the number of rows in this single page of results."]
        pub total_rows: ::std::option::Option<::std::string::String>,
    }
    impl QueryResponse {
        pub fn builder() -> QueryResponseBuilder {
            QueryResponseBuilder::default()
        }
    }
    mod query_response_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#queryResponse\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct QueryTimelineSample {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activeUnits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of units currently being processed by workers. This does not correspond directly to slot usage. This is the largest value observed since the last sample."]
        pub active_units: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "completedUnits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total parallel units of work completed by this query."]
        pub completed_units: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "elapsedMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Milliseconds elapsed since the start of query execution."]
        pub elapsed_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pendingUnits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total parallel units of work remaining for the active stages."]
        pub pending_units: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSlotMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cumulative slot-ms consumed by the query."]
        pub total_slot_ms: ::std::option::Option<::std::string::String>,
    }
    impl QueryTimelineSample {
        pub fn builder() -> QueryTimelineSampleBuilder {
            QueryTimelineSampleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct RangePartitioning {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[TrustedTester] [Required] The table is partitioned by this field. The field must be a top-level NULLABLE/REQUIRED field. The only supported type is INTEGER/INT64."]
        pub field: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[TrustedTester] [Required] Defines the ranges for range partitioning."]
        pub range: ::std::option::Option<RangePartitioningRange>,
    }
    impl RangePartitioning {
        pub fn builder() -> RangePartitioningBuilder {
            RangePartitioningBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "[TrustedTester] [Required] Defines the ranges for range partitioning."]
    pub struct RangePartitioningRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "end")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[TrustedTester] [Required] The end of range partitioning, exclusive."]
        pub end: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "interval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[TrustedTester] [Required] The width of each interval."]
        pub interval: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "start")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[TrustedTester] [Required] The start of range partitioning, inclusive."]
        pub start: ::std::option::Option<::std::string::String>,
    }
    impl RangePartitioningRange {
        pub fn builder() -> RangePartitioningRangeBuilder {
            RangePartitioningRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Evaluation metrics used by weighted-ALS models specified by feedback_type=implicit."]
    pub struct RankingMetrics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "averageRank")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines the goodness of a ranking by computing the percentile rank from the predicted confidence and dividing it by the original rank."]
        pub average_rank: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meanAveragePrecision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Calculates a precision per user for all the items by ranking them and then averages all the precisions across all the users."]
        pub mean_average_precision: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meanSquaredError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Similar to the mean squared error computed in regression and explicit recommendation models except instead of computing the rating directly, the output from evaluate is computed against a preference which is 1 or 0 depending on if the rating exists or not."]
        pub mean_squared_error: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedDiscountedCumulativeGain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A metric to determine the goodness of a ranking calculated from the predicted confidence by comparing it to an ideal rank measured by the original ratings."]
        pub normalized_discounted_cumulative_gain: ::std::option::Option<::std::primitive::f64>,
    }
    impl RankingMetrics {
        pub fn builder() -> RankingMetricsBuilder {
            RankingMetricsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Evaluation metrics for regression and explicit feedback type matrix factorization models."]
    pub struct RegressionMetrics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meanAbsoluteError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mean absolute error."]
        pub mean_absolute_error: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meanSquaredError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mean squared error."]
        pub mean_squared_error: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meanSquaredLogError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mean squared log error."]
        pub mean_squared_log_error: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "medianAbsoluteError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Median absolute error."]
        pub median_absolute_error: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rSquared")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "R^2 score. This corresponds to r2_score in ML.EVALUATE."]
        pub r_squared: ::std::option::Option<::std::primitive::f64>,
    }
    impl RegressionMetrics {
        pub fn builder() -> RegressionMetricsBuilder {
            RegressionMetricsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A user-defined function or a stored procedure."]
    pub struct Routine {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arguments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional."]
        pub arguments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Argument>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time when this routine was created, in milliseconds since the epoch."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "definitionBody")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The body of the routine. For functions, this is the expression in the AS clause. If language=SQL, it is the substring inside (but excluding) the parentheses. For example, for the function created with the following statement: `CREATE FUNCTION JoinLines(x string, y string) as (concat(x, \"\\n\", y))` The definition_body is `concat(x, \"\\n\", y)` (\\n is not replaced with linebreak). If language=JAVASCRIPT, it is the evaluated string in the AS clause. For example, for the function created with the following statement: `CREATE FUNCTION f() RETURNS STRING LANGUAGE js AS 'return \"\\n\";\\n'` The definition_body is `return \"\\n\";\\n` Note that both \\n are replaced with linebreaks."]
        pub definition_body: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. [Experimental] The description of the routine if defined."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "determinismLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. [Experimental] The determinism level of the JavaScript UDF if defined."]
        pub determinism_level: ::std::option::Option<RoutineDeterminismLevelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A hash of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importedLibraries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If language = \"JAVASCRIPT\", this field stores the path of the imported JAVASCRIPT libraries."]
        pub imported_libraries: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Defaults to \"SQL\"."]
        pub language: ::std::option::Option<RoutineLanguageEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time when this routine was last modified, in milliseconds since the epoch."]
        pub last_modified_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional if language = \"SQL\"; required otherwise. If absent, the return type is inferred from definition_body at query time in each query that references this routine. If present, then the evaluated result will be cast to the specified returned type at query time. For example, for the functions created with the following statements: * `CREATE FUNCTION Add(x FLOAT64, y FLOAT64) RETURNS FLOAT64 AS (x + y);` * `CREATE FUNCTION Increment(x FLOAT64) AS (Add(x, 1));` * `CREATE FUNCTION Decrement(x FLOAT64) RETURNS FLOAT64 AS (Add(x, -1));` The return_type is `{type_kind: \"FLOAT64\"}` for `Add` and `Decrement`, and is absent for `Increment` (inferred as FLOAT64 at query time). Suppose the function `Add` is replaced by `CREATE OR REPLACE FUNCTION Add(x INT64, y INT64) AS (x + y);` Then the inferred return type of `Increment` is automatically changed to INT64 at query time, while the return type of `Decrement` remains FLOAT64."]
        pub return_type: ::std::option::Option<::std::boxed::Box<StandardSqlDataType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "routineReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Reference describing the ID of this routine."]
        pub routine_reference: ::std::option::Option<::std::boxed::Box<RoutineReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "routineType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The type of routine."]
        pub routine_type: ::std::option::Option<RoutineRoutineTypeEnum>,
    }
    impl Routine {
        pub fn builder() -> RoutineBuilder {
            RoutineBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. [Experimental] The determinism level of the JavaScript UDF if defined."]
    pub enum RoutineDeterminismLevelEnum {
        #[serde(rename = "DETERMINISM_LEVEL_UNSPECIFIED")]
        #[doc = "The determinism of the UDF is unspecified."]
        DeterminismLevelUnspecified,
        #[serde(rename = "DETERMINISTIC")]
        #[doc = "The UDF is deterministic, meaning that 2 function calls with the same inputs always produce the same result, even across 2 query runs."]
        Deterministic,
        #[serde(rename = "NOT_DETERMINISTIC")]
        #[doc = "The UDF is not deterministic."]
        NotDeterministic,
    }
    impl ::std::default::Default for RoutineDeterminismLevelEnum {
        fn default() -> Self {
            Self::DeterminismLevelUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Defaults to \"SQL\"."]
    pub enum RoutineLanguageEnum {
        #[serde(rename = "LANGUAGE_UNSPECIFIED")]
        #[doc = ""]
        LanguageUnspecified,
        #[serde(rename = "SQL")]
        #[doc = "SQL language."]
        Sql,
        #[serde(rename = "JAVASCRIPT")]
        #[doc = "JavaScript language."]
        Javascript,
    }
    impl ::std::default::Default for RoutineLanguageEnum {
        fn default() -> Self {
            Self::LanguageUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The type of routine."]
    pub enum RoutineRoutineTypeEnum {
        #[serde(rename = "ROUTINE_TYPE_UNSPECIFIED")]
        #[doc = ""]
        RoutineTypeUnspecified,
        #[serde(rename = "SCALAR_FUNCTION")]
        #[doc = "Non-builtin permanent scalar function."]
        ScalarFunction,
        #[serde(rename = "PROCEDURE")]
        #[doc = "Stored procedure."]
        Procedure,
    }
    impl ::std::default::Default for RoutineRoutineTypeEnum {
        fn default() -> Self {
            Self::RoutineTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct RoutineReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datasetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The ID of the dataset containing this routine."]
        pub dataset_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The ID of the project containing this routine."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "routineId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The ID of the routine. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 256 characters."]
        pub routine_id: ::std::option::Option<::std::string::String>,
    }
    impl RoutineReference {
        pub fn builder() -> RoutineReferenceBuilder {
            RoutineReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single row in the confusion matrix."]
    pub struct Row {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actualLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The original label of this row."]
        pub actual_label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Info describing predicted label distribution."]
        pub entries: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Entry>>>,
    }
    impl Row {
        pub fn builder() -> RowBuilder {
            RowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents access on a subset of rows on the specified table, defined by its filter predicate. Access to the subset of rows is controlled by its IAM policy."]
    pub struct RowAccessPolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time when this row access policy was created, in milliseconds since the epoch."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A hash of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filterPredicate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A SQL boolean expression that represents the rows defined by this row access policy, similar to the boolean expression in a WHERE clause of a SELECT query on a table. References to other tables, routines, and temporary functions are not supported. Examples: region=\"EU\" date_field = CAST('2019-9-27' as DATE) nullable_field is not NULL numeric_field BETWEEN 1.0 AND 5.0"]
        pub filter_predicate: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time when this row access policy was last modified, in milliseconds since the epoch."]
        pub last_modified_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowAccessPolicyReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Reference describing the ID of this row access policy."]
        pub row_access_policy_reference:
            ::std::option::Option<::std::boxed::Box<RowAccessPolicyReference>>,
    }
    impl RowAccessPolicy {
        pub fn builder() -> RowAccessPolicyBuilder {
            RowAccessPolicyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct RowAccessPolicyReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datasetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The ID of the dataset containing this row access policy."]
        pub dataset_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The ID of the row access policy. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 256 characters."]
        pub policy_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The ID of the project containing this row access policy."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The ID of the table containing this row access policy."]
        pub table_id: ::std::option::Option<::std::string::String>,
    }
    impl RowAccessPolicyReference {
        pub fn builder() -> RowAccessPolicyReferenceBuilder {
            RowAccessPolicyReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct RowLevelSecurityStatistics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowLevelSecurityApplied")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] [Preview] Whether any accessed data was protected by row access policies."]
        pub row_level_security_applied: ::std::option::Option<::std::primitive::bool>,
    }
    impl RowLevelSecurityStatistics {
        pub fn builder() -> RowLevelSecurityStatisticsBuilder {
            RowLevelSecurityStatisticsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ScriptStackFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endColumn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] One-based end column."]
        pub end_column: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endLine")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] One-based end line."]
        pub end_line: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "procedureId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Name of the active procedure, empty if in a top-level script."]
        pub procedure_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startColumn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] One-based start column."]
        pub start_column: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startLine")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] One-based start line."]
        pub start_line: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Text of the current statement/expression."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl ScriptStackFrame {
        pub fn builder() -> ScriptStackFrameBuilder {
            ScriptStackFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ScriptStatistics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "evaluationKind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Whether this child job was a statement or expression."]
        pub evaluation_kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stackFrames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Stack trace showing the line/column/procedure name of each frame on the stack at the point where the current evaluation happened. The leaf frame is first, the primary script is last. Never empty."]
        pub stack_frames:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ScriptStackFrame>>>,
    }
    impl ScriptStatistics {
        pub fn builder() -> ScriptStatisticsBuilder {
            ScriptStatisticsBuilder::default()
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
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: \"bindings, etag\"`"]
        pub update_mask: ::std::option::Option<::std::string::String>,
    }
    impl SetIamPolicyRequest {
        pub fn builder() -> SetIamPolicyRequestBuilder {
            SetIamPolicyRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SnapshotDefinition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "baseTableReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] Reference describing the ID of the table that is snapshotted."]
        pub base_table_reference: ::std::option::Option<::std::boxed::Box<TableReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snapshotTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The time at which the base table was snapshot."]
        pub snapshot_time: ::std::option::Option<::std::string::String>,
    }
    impl SnapshotDefinition {
        pub fn builder() -> SnapshotDefinitionBuilder {
            SnapshotDefinitionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The type of a variable, e.g., a function argument. Examples: INT64: {type_kind=\"INT64\"} ARRAY: {type_kind=\"ARRAY\", array_element_type=\"STRING\"} STRUCT>: {type_kind=\"STRUCT\", struct_type={fields=[ {name=\"x\", type={type_kind=\"STRING\"}}, {name=\"y\", type={type_kind=\"ARRAY\", array_element_type=\"DATE\"}} ]}}"]
    pub struct StandardSqlDataType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arrayElementType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the array's elements, if type_kind = \"ARRAY\"."]
        pub array_element_type: ::std::option::Option<::std::boxed::Box<StandardSqlDataType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "structType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields of this struct, in order, if type_kind = \"STRUCT\"."]
        pub struct_type: ::std::option::Option<::std::boxed::Box<StandardSqlStructType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "typeKind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The top level type of this field. Can be any standard SQL data type (e.g., \"INT64\", \"DATE\", \"ARRAY\")."]
        pub type_kind: ::std::option::Option<StandardSqlDataTypeTypeKindEnum>,
    }
    impl StandardSqlDataType {
        pub fn builder() -> StandardSqlDataTypeBuilder {
            StandardSqlDataTypeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The top level type of this field. Can be any standard SQL data type (e.g., \"INT64\", \"DATE\", \"ARRAY\")."]
    pub enum StandardSqlDataTypeTypeKindEnum {
        #[serde(rename = "TYPE_KIND_UNSPECIFIED")]
        #[doc = "Invalid type."]
        TypeKindUnspecified,
        #[serde(rename = "INT64")]
        #[doc = "Encoded as a string in decimal format."]
        Int64,
        #[serde(rename = "BOOL")]
        #[doc = "Encoded as a boolean \"false\" or \"true\"."]
        Bool,
        #[serde(rename = "FLOAT64")]
        #[doc = "Encoded as a number, or string \"NaN\", \"Infinity\" or \"-Infinity\"."]
        Float64,
        #[serde(rename = "STRING")]
        #[doc = "Encoded as a string value."]
        String,
        #[serde(rename = "BYTES")]
        #[doc = "Encoded as a base64 string per RFC 4648, section 4."]
        Bytes,
        #[serde(rename = "TIMESTAMP")]
        #[doc = "Encoded as an RFC 3339 timestamp with mandatory \"Z\" time zone string: 1985-04-12T23:20:50.52Z"]
        Timestamp,
        #[serde(rename = "DATE")]
        #[doc = "Encoded as RFC 3339 full-date format string: 1985-04-12"]
        Date,
        #[serde(rename = "TIME")]
        #[doc = "Encoded as RFC 3339 partial-time format string: 23:20:50.52"]
        Time,
        #[serde(rename = "DATETIME")]
        #[doc = "Encoded as RFC 3339 full-date \"T\" partial-time: 1985-04-12T23:20:50.52"]
        Datetime,
        #[serde(rename = "GEOGRAPHY")]
        #[doc = "Encoded as WKT"]
        Geography,
        #[serde(rename = "NUMERIC")]
        #[doc = "Encoded as a decimal string."]
        Numeric,
        #[serde(rename = "BIGNUMERIC")]
        #[doc = "Encoded as a decimal string."]
        Bignumeric,
        #[serde(rename = "ARRAY")]
        #[doc = "Encoded as a list with types matching Type.array_type."]
        Array,
        #[serde(rename = "STRUCT")]
        #[doc = "Encoded as a list with fields of type Type.struct_type[i]. List is used because a JSON object cannot have duplicate field names."]
        Struct,
    }
    impl ::std::default::Default for StandardSqlDataTypeTypeKindEnum {
        fn default() -> Self {
            Self::TypeKindUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A field or a column."]
    pub struct StandardSqlField {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The name of this field. Can be absent for struct fields."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The type of this parameter. Absent if not explicitly specified (e.g., CREATE FUNCTION statement can omit the return type; in this case the output parameter does not have this \"type\" field)."]
        pub _type: ::std::option::Option<::std::boxed::Box<StandardSqlDataType>>,
    }
    impl StandardSqlField {
        pub fn builder() -> StandardSqlFieldBuilder {
            StandardSqlFieldBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct StandardSqlStructType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub fields: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StandardSqlField>>>,
    }
    impl StandardSqlStructType {
        pub fn builder() -> StandardSqlStructTypeBuilder {
            StandardSqlStructTypeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Streamingbuffer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "estimatedBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] A lower-bound estimate of the number of bytes currently in the streaming buffer."]
        pub estimated_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "estimatedRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] A lower-bound estimate of the number of rows currently in the streaming buffer."]
        pub estimated_rows: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oldestEntryTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Contains the timestamp of the oldest entry in the streaming buffer, in milliseconds since the epoch, if the streaming buffer is available."]
        pub oldest_entry_time: ::std::option::Option<::std::string::String>,
    }
    impl Streamingbuffer {
        pub fn builder() -> StreamingbufferBuilder {
            StreamingbufferBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Table {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clustering")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Beta] Clustering specification for the table. Must be specified with partitioning, data in the table will be first partitioned and subsequently clustered."]
        pub clustering: ::std::option::Option<::std::boxed::Box<Clustering>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The time when this table was created, in milliseconds since the epoch."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] A user-friendly description of this table."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encryptionConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom encryption configuration (e.g., Cloud KMS keys)."]
        pub encryption_configuration:
            ::std::option::Option<::std::boxed::Box<EncryptionConfiguration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] A hash of the table metadata. Used to ensure there were no concurrent modifications to the resource when attempting an update. Not guaranteed to change when the table contents or the fields numRows, numBytes, numLongTermBytes or lastModifiedTime change."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expirationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed. The defaultTableExpirationMs property of the encapsulating dataset can be used to set a default expirationTime on newly created tables."]
        pub expiration_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "externalDataConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Describes the data format, location, and other properties of a table stored outside of BigQuery. By defining these properties, the data source can then be queried as if it were a standard BigQuery table."]
        pub external_data_configuration:
            ::std::option::Option<::std::boxed::Box<ExternalDataConfiguration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "friendlyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] A descriptive name for this table."]
        pub friendly_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] An opaque ID uniquely identifying the table."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ table_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "table_defaults :: kind")]
        #[doc = "[Output-only] The type of the resource."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The labels associated with this table. You can use these to organize and group your tables. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The time when this table was last modified, in milliseconds since the epoch."]
        pub last_modified_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The geographic location where the table resides. This value is inherited from the dataset."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "materializedView")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Materialized view definition."]
        pub materialized_view: ::std::option::Option<::std::boxed::Box<MaterializedViewDefinition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only, Beta] Present iff this table represents a ML model. Describes the training information for the model, and it is required to run 'PREDICT' queries."]
        pub model: ::std::option::Option<::std::boxed::Box<ModelDefinition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The size of this table in bytes, excluding any data in the streaming buffer."]
        pub num_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numLongTermBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The number of bytes in the table that are considered \"long-term storage\"."]
        pub num_long_term_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numPhysicalBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] [TrustedTester] The physical size of this table in bytes, excluding any data in the streaming buffer. This includes compression and storage used for time travel."]
        pub num_physical_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] The number of rows of data in this table, excluding any data in the streaming buffer."]
        pub num_rows: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rangePartitioning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[TrustedTester] Range partitioning specification for this table. Only one of timePartitioning and rangePartitioning should be specified."]
        pub range_partitioning: ::std::option::Option<::std::boxed::Box<RangePartitioning>>,
        #[builder(
            default = "{ table_defaults :: require_partition_filter () }",
            setter(into)
        )]
        #[serde(rename = "requirePartitionFilter")]
        #[serde(default = "table_defaults :: require_partition_filter")]
        #[doc = "[Optional] If set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
        pub require_partition_filter: ::std::primitive::bool,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schema")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Describes the schema of this table."]
        pub schema: ::std::option::Option<::std::boxed::Box<TableSchema>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] A URL that can be used to access this resource again."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snapshotDefinition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Snapshot definition."]
        pub snapshot_definition: ::std::option::Option<::std::boxed::Box<SnapshotDefinition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "streamingBuffer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Contains information regarding this table's streaming buffer, if one is present. This field will be absent if the table is not being streamed to or if there is no data in the streaming buffer."]
        pub streaming_buffer: ::std::option::Option<::std::boxed::Box<Streamingbuffer>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] Reference describing the ID of this table."]
        pub table_reference: ::std::option::Option<::std::boxed::Box<TableReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timePartitioning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-based partitioning specification for this table. Only one of timePartitioning and rangePartitioning should be specified."]
        pub time_partitioning: ::std::option::Option<::std::boxed::Box<TimePartitioning>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] Describes the table type. The following values are supported: TABLE: A normal BigQuery table. VIEW: A virtual table defined by a SQL query. SNAPSHOT: An immutable, read-only table that is a copy of another table. [TrustedTester] MATERIALIZED_VIEW: SQL query whose result is persisted. EXTERNAL: A table that references data stored in an external storage system, such as Google Cloud Storage. The default value is TABLE."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "view")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The view definition."]
        pub view: ::std::option::Option<::std::boxed::Box<ViewDefinition>>,
    }
    impl Table {
        pub fn builder() -> TableBuilder {
            TableBuilder::default()
        }
    }
    mod table_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#table\"").unwrap()
        }
        pub fn require_partition_filter() -> ::std::primitive::bool {
            serde_json::from_str(&"false").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TableCell {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "v")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub v: ::std::option::Option<::serde_json::Value>,
    }
    impl TableCell {
        pub fn builder() -> TableCellBuilder {
            TableCellBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TableDataInsertAllRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ignoreUnknownValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Accept rows that contain values that do not match the schema. The unknown values are ignored. Default is false, which treats unknown values as errors."]
        pub ignore_unknown_values: ::std::option::Option<::std::primitive::bool>,
        #[builder(
            default = "{ table_data_insert_all_request_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "table_data_insert_all_request_defaults :: kind")]
        #[doc = "The resource type of the response."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rows to insert."]
        pub rows: ::std::option::Option<::std::vec::Vec<TableDataInsertAllRequestRows>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skipInvalidRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Insert all valid rows of a request, even if invalid rows exist. The default value is false, which causes the entire request to fail if any invalid rows exist."]
        pub skip_invalid_rows: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "templateSuffix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, treats the destination table as a base template, and inserts the rows into an instance table named \"{destination}{templateSuffix}\". BigQuery will manage creation of the instance table, using the schema of the base template table. See https://cloud.google.com/bigquery/streaming-data-into-bigquery#template-tables for considerations when working with templates tables."]
        pub template_suffix: ::std::option::Option<::std::string::String>,
    }
    impl TableDataInsertAllRequest {
        pub fn builder() -> TableDataInsertAllRequestBuilder {
            TableDataInsertAllRequestBuilder::default()
        }
    }
    mod table_data_insert_all_request_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#tableDataInsertAllRequest\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TableDataInsertAllRequestRows {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] A unique ID for each row. BigQuery uses this property to detect duplicate insertion requests on a best-effort basis."]
        pub insert_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "json")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] A JSON object that contains a row of data. The object's properties and values must match the destination table's schema."]
        pub json: ::std::option::Option<::std::boxed::Box<JsonObject>>,
    }
    impl TableDataInsertAllRequestRows {
        pub fn builder() -> TableDataInsertAllRequestRowsBuilder {
            TableDataInsertAllRequestRowsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TableDataInsertAllResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An array of errors for rows that were not inserted."]
        pub insert_errors:
            ::std::option::Option<::std::vec::Vec<TableDataInsertAllResponseInsertErrors>>,
        #[builder(
            default = "{ table_data_insert_all_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "table_data_insert_all_response_defaults :: kind")]
        #[doc = "The resource type of the response."]
        pub kind: ::std::string::String,
    }
    impl TableDataInsertAllResponse {
        pub fn builder() -> TableDataInsertAllResponseBuilder {
            TableDataInsertAllResponseBuilder::default()
        }
    }
    mod table_data_insert_all_response_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#tableDataInsertAllResponse\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TableDataInsertAllResponseInsertErrors {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error information for the row indicated by the index property."]
        pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ErrorProto>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the row that error applies to."]
        pub index: ::std::option::Option<::std::primitive::i64>,
    }
    impl TableDataInsertAllResponseInsertErrors {
        pub fn builder() -> TableDataInsertAllResponseInsertErrorsBuilder {
            TableDataInsertAllResponseInsertErrorsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TableDataList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A hash of this page of results."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ table_data_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "table_data_list_defaults :: kind")]
        #[doc = "The resource type of the response."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token used for paging results. Providing this token instead of the startIndex parameter can help you retrieve stable results when an underlying table is changing."]
        pub page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rows of results."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TableRow>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of rows in the complete table."]
        pub total_rows: ::std::option::Option<::std::string::String>,
    }
    impl TableDataList {
        pub fn builder() -> TableDataListBuilder {
            TableDataListBuilder::default()
        }
    }
    mod table_data_list_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#tableDataList\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TableFieldSchema {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The categories attached to this field, used for field-level access control."]
        pub categories: ::std::option::Option<TableFieldSchemaCategories>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The field description. The maximum length is 1,024 characters."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Describes the nested schema fields if the type property is set to RECORD."]
        pub fields: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TableFieldSchema>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The field mode. Possible values include NULLABLE, REQUIRED and REPEATED. The default value is NULLABLE."]
        pub mode: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The field name. The name must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_), and must start with a letter or underscore. The maximum length is 128 characters."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policyTags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub policy_tags: ::std::option::Option<TableFieldSchemaPolicyTags>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The field data type. Possible values include STRING, BYTES, INTEGER, INT64 (same as INTEGER), FLOAT, FLOAT64 (same as FLOAT), NUMERIC, BIGNUMERIC, BOOLEAN, BOOL (same as BOOLEAN), TIMESTAMP, DATE, TIME, DATETIME, RECORD (where RECORD indicates that the field contains a nested schema) or STRUCT (same as RECORD)."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl TableFieldSchema {
        pub fn builder() -> TableFieldSchemaBuilder {
            TableFieldSchemaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "[Optional] The categories attached to this field, used for field-level access control."]
    pub struct TableFieldSchemaCategories {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "names")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of category resource names. For example, \"projects/1/taxonomies/2/categories/3\". At most 5 categories are allowed."]
        pub names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl TableFieldSchemaCategories {
        pub fn builder() -> TableFieldSchemaCategoriesBuilder {
            TableFieldSchemaCategoriesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TableFieldSchemaPolicyTags {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "names")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of category resource names. For example, \"projects/1/location/eu/taxonomies/2/policyTags/3\". At most 1 policy tag is allowed."]
        pub names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl TableFieldSchemaPolicyTags {
        pub fn builder() -> TableFieldSchemaPolicyTagsBuilder {
            TableFieldSchemaPolicyTagsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TableList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A hash of this page of results."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ table_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "table_list_defaults :: kind")]
        #[doc = "The type of list."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to request the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tables")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tables in the requested dataset."]
        pub tables: ::std::option::Option<::std::vec::Vec<TableListTables>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of tables in the dataset."]
        pub total_items: ::std::option::Option<::std::primitive::i64>,
    }
    impl TableList {
        pub fn builder() -> TableListBuilder {
            TableListBuilder::default()
        }
    }
    mod table_list_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#tableList\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TableListTables {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clustering")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Beta] Clustering specification for this table, if configured."]
        pub clustering: ::std::option::Option<::std::boxed::Box<Clustering>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when this table was created, in milliseconds since the epoch."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expirationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed."]
        pub expiration_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "friendlyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user-friendly name for this table."]
        pub friendly_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An opaque ID of the table"]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ table_list_tables_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "table_list_tables_defaults :: kind")]
        #[doc = "The resource type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The labels associated with this table. You can use these to organize and group your tables."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rangePartitioning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range partitioning specification for this table, if configured."]
        pub range_partitioning: ::std::option::Option<::std::boxed::Box<RangePartitioning>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reference uniquely identifying the table."]
        pub table_reference: ::std::option::Option<::std::boxed::Box<TableReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timePartitioning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time-based partitioning specification for this table, if configured."]
        pub time_partitioning: ::std::option::Option<::std::boxed::Box<TimePartitioning>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of table. Possible values are: TABLE, VIEW."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "view")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional details for a view."]
        pub view: ::std::option::Option<TableListTablesView>,
    }
    impl TableListTables {
        pub fn builder() -> TableListTablesBuilder {
            TableListTablesBuilder::default()
        }
    }
    mod table_list_tables_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"bigquery#table\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional details for a view."]
    pub struct TableListTablesView {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useLegacySql")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if view is defined in legacy SQL dialect, false if in standard SQL."]
        pub use_legacy_sql: ::std::option::Option<::std::primitive::bool>,
    }
    impl TableListTablesView {
        pub fn builder() -> TableListTablesViewBuilder {
            TableListTablesViewBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TableReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datasetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The ID of the dataset containing this table."]
        pub dataset_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The ID of the project containing this table."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The ID of the table. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters."]
        pub table_id: ::std::option::Option<::std::string::String>,
    }
    impl TableReference {
        pub fn builder() -> TableReferenceBuilder {
            TableReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TableRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "f")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a single row in the result set, consisting of one or more fields."]
        pub f: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TableCell>>>,
    }
    impl TableRow {
        pub fn builder() -> TableRowBuilder {
            TableRowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TableSchema {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes the fields in a table."]
        pub fields: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TableFieldSchema>>>,
    }
    impl TableSchema {
        pub fn builder() -> TableSchemaBuilder {
            TableSchemaBuilder::default()
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
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TimePartitioning {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expirationMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] Number of milliseconds for which to keep the storage for partitions in the table. The storage in a partition will have an expiration time of its partition time plus this value."]
        pub expiration_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Beta] [Optional] If not set, the table is partitioned by pseudo column, referenced via either '_PARTITIONTIME' as TIMESTAMP type, or '_PARTITIONDATE' as DATE type. If field is specified, the table is instead partitioned by this field. The field must be a top-level TIMESTAMP or DATE field. Its mode must be NULLABLE or REQUIRED."]
        pub field: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requirePartitionFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub require_partition_filter: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] The supported types are DAY, HOUR, MONTH, and YEAR, which will generate one partition per day, hour, month, and year, respectively. When the type is not specified, the default behavior is DAY."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl TimePartitioning {
        pub fn builder() -> TimePartitioningBuilder {
            TimePartitioningBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options used in model training."]
    pub struct TrainingOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoArima")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to enable auto ARIMA or not."]
        pub auto_arima: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoArimaMaxOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The max value of non-seasonal p and q."]
        pub auto_arima_max_order: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Batch size for dnn models."]
        pub batch_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataFrequency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data frequency of a time series."]
        pub data_frequency: ::std::option::Option<TrainingOptionsDataFrequencyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSplitColumn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The column to split data with. This column won't be used as a feature. 1. When data_split_method is CUSTOM, the corresponding column should be boolean. The rows with true value tag are eval data, and the false are training data. 2. When data_split_method is SEQ, the first DATA_SPLIT_EVAL_FRACTION rows (from smallest to largest) in the corresponding column are used as training data, and the rest are eval data. It respects the order in Orderable data types: https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#data-type-properties"]
        pub data_split_column: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSplitEvalFraction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fraction of evaluation data over the whole input data. The rest of data will be used as training data. The format should be double. Accurate to two decimal places. Default value is 0.2."]
        pub data_split_eval_fraction: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSplitMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data split type for training and evaluation, e.g. RANDOM."]
        pub data_split_method: ::std::option::Option<TrainingOptionsDataSplitMethodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "distanceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Distance type for clustering models."]
        pub distance_type: ::std::option::Option<TrainingOptionsDistanceTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dropout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dropout probability for dnn models."]
        pub dropout: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "earlyStop")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to stop early when the loss doesn't improve significantly any more (compared to min_relative_progress). Used only for iterative training algorithms."]
        pub early_stop: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feedbackType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feedback type that specifies which algorithm to run for matrix factorization."]
        pub feedback_type: ::std::option::Option<TrainingOptionsFeedbackTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hiddenUnits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hidden units for dnn models."]
        pub hidden_units: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "holidayRegion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The geographical region based on which the holidays are considered in time series modeling. If a valid value is specified, then holiday effects modeling is enabled."]
        pub holiday_region: ::std::option::Option<TrainingOptionsHolidayRegionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "horizon")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of periods ahead that need to be forecasted."]
        pub horizon: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeDrift")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Include drift when fitting an ARIMA model."]
        pub include_drift: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "initialLearnRate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the initial learning rate for the line search learn rate strategy."]
        pub initial_learn_rate: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputLabelColumns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of input label columns in training data."]
        pub input_label_columns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemColumn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Item column specified for matrix factorization models."]
        pub item_column: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmeansInitializationColumn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The column used to provide the initial centroids for kmeans algorithm when kmeans_initialization_method is CUSTOM."]
        pub kmeans_initialization_column: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmeansInitializationMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The method used to initialize the centroids for kmeans algorithm."]
        pub kmeans_initialization_method:
            ::std::option::Option<TrainingOptionsKmeansInitializationMethodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "l1Regularization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "L1 regularization coefficient."]
        pub l1_regularization: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "l2Regularization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "L2 regularization coefficient."]
        pub l2_regularization: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelClassWeights")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Weights associated with each label class, for rebalancing the training data. Only applicable for classification models."]
        pub label_class_weights:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::primitive::f64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "learnRate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Learning rate in training. Used only for iterative training algorithms."]
        pub learn_rate: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "learnRateStrategy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The strategy to determine learn rate for the current iteration."]
        pub learn_rate_strategy: ::std::option::Option<TrainingOptionsLearnRateStrategyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lossType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of loss function used during training run."]
        pub loss_type: ::std::option::Option<TrainingOptionsLossTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxIterations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of iterations in training. Used only for iterative training algorithms."]
        pub max_iterations: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxTreeDepth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum depth of a tree for boosted tree models."]
        pub max_tree_depth: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minRelativeProgress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When early_stop is true, stops training when accuracy improvement is less than 'min_relative_progress'. Used only for iterative training algorithms."]
        pub min_relative_progress: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minSplitLoss")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum split loss for boosted tree models."]
        pub min_split_loss: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modelUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage URI from which the model was imported. Only applicable for imported models."]
        pub model_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonSeasonalOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A specification of the non-seasonal part of the ARIMA model: the three components (p, d, q) are the AR order, the degree of differencing, and the MA order."]
        pub non_seasonal_order: ::std::option::Option<::std::boxed::Box<ArimaOrder>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numClusters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of clusters for clustering models."]
        pub num_clusters: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numFactors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Num factors specified for matrix factorization models."]
        pub num_factors: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "optimizationStrategy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optimization strategy for training linear regression models."]
        pub optimization_strategy: ::std::option::Option<TrainingOptionsOptimizationStrategyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "preserveInputStructs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to preserve the input structs in output feature names. Suppose there is a struct A with field b. When false (default), the output feature name is A_b. When true, the output feature name is A.b."]
        pub preserve_input_structs: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subsample")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subsample fraction of the training data to grow tree to prevent overfitting for boosted tree models."]
        pub subsample: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSeriesDataColumn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Column to be designated as time series data for ARIMA model."]
        pub time_series_data_column: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSeriesIdColumn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id column that will be used to indicate different time series to forecast in parallel."]
        pub time_series_id_column: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSeriesTimestampColumn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Column to be designated as time series timestamp for ARIMA model."]
        pub time_series_timestamp_column: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userColumn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User column specified for matrix factorization models."]
        pub user_column: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "walsAlpha")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hyperparameter for matrix factoration when implicit feedback type is specified."]
        pub wals_alpha: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warmStart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to train a model from the last checkpoint."]
        pub warm_start: ::std::option::Option<::std::primitive::bool>,
    }
    impl TrainingOptions {
        pub fn builder() -> TrainingOptionsBuilder {
            TrainingOptionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The data frequency of a time series."]
    pub enum TrainingOptionsDataFrequencyEnum {
        #[serde(rename = "DATA_FREQUENCY_UNSPECIFIED")]
        #[doc = ""]
        DataFrequencyUnspecified,
        #[serde(rename = "AUTO_FREQUENCY")]
        #[doc = "Automatically inferred from timestamps."]
        AutoFrequency,
        #[serde(rename = "YEARLY")]
        #[doc = "Yearly data."]
        Yearly,
        #[serde(rename = "QUARTERLY")]
        #[doc = "Quarterly data."]
        Quarterly,
        #[serde(rename = "MONTHLY")]
        #[doc = "Monthly data."]
        Monthly,
        #[serde(rename = "WEEKLY")]
        #[doc = "Weekly data."]
        Weekly,
        #[serde(rename = "DAILY")]
        #[doc = "Daily data."]
        Daily,
        #[serde(rename = "HOURLY")]
        #[doc = "Hourly data."]
        Hourly,
        #[serde(rename = "PER_MINUTE")]
        #[doc = "Per-minute data."]
        PerMinute,
    }
    impl ::std::default::Default for TrainingOptionsDataFrequencyEnum {
        fn default() -> Self {
            Self::DataFrequencyUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The data split type for training and evaluation, e.g. RANDOM."]
    pub enum TrainingOptionsDataSplitMethodEnum {
        #[serde(rename = "DATA_SPLIT_METHOD_UNSPECIFIED")]
        #[doc = ""]
        DataSplitMethodUnspecified,
        #[serde(rename = "RANDOM")]
        #[doc = "Splits data randomly."]
        Random,
        #[serde(rename = "CUSTOM")]
        #[doc = "Splits data with the user provided tags."]
        Custom,
        #[serde(rename = "SEQUENTIAL")]
        #[doc = "Splits data sequentially."]
        Sequential,
        #[serde(rename = "NO_SPLIT")]
        #[doc = "Data split will be skipped."]
        NoSplit,
        #[serde(rename = "AUTO_SPLIT")]
        #[doc = "Splits data automatically: Uses NO_SPLIT if the data size is small. Otherwise uses RANDOM."]
        AutoSplit,
    }
    impl ::std::default::Default for TrainingOptionsDataSplitMethodEnum {
        fn default() -> Self {
            Self::DataSplitMethodUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Distance type for clustering models."]
    pub enum TrainingOptionsDistanceTypeEnum {
        #[serde(rename = "DISTANCE_TYPE_UNSPECIFIED")]
        #[doc = ""]
        DistanceTypeUnspecified,
        #[serde(rename = "EUCLIDEAN")]
        #[doc = "Eculidean distance."]
        Euclidean,
        #[serde(rename = "COSINE")]
        #[doc = "Cosine distance."]
        Cosine,
    }
    impl ::std::default::Default for TrainingOptionsDistanceTypeEnum {
        fn default() -> Self {
            Self::DistanceTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Feedback type that specifies which algorithm to run for matrix factorization."]
    pub enum TrainingOptionsFeedbackTypeEnum {
        #[serde(rename = "FEEDBACK_TYPE_UNSPECIFIED")]
        #[doc = ""]
        FeedbackTypeUnspecified,
        #[serde(rename = "IMPLICIT")]
        #[doc = "Use weighted-als for implicit feedback problems."]
        Implicit,
        #[serde(rename = "EXPLICIT")]
        #[doc = "Use nonweighted-als for explicit feedback problems."]
        Explicit,
    }
    impl ::std::default::Default for TrainingOptionsFeedbackTypeEnum {
        fn default() -> Self {
            Self::FeedbackTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The geographical region based on which the holidays are considered in time series modeling. If a valid value is specified, then holiday effects modeling is enabled."]
    pub enum TrainingOptionsHolidayRegionEnum {
        #[serde(rename = "HOLIDAY_REGION_UNSPECIFIED")]
        #[doc = "Holiday region unspecified."]
        HolidayRegionUnspecified,
        #[serde(rename = "GLOBAL")]
        #[doc = "Global."]
        Global,
        #[serde(rename = "NA")]
        #[doc = "North America."]
        Na,
        #[serde(rename = "JAPAC")]
        #[doc = "Japan and Asia Pacific: Korea, Greater China, India, Australia, and New Zealand."]
        Japac,
        #[serde(rename = "EMEA")]
        #[doc = "Europe, the Middle East and Africa."]
        Emea,
        #[serde(rename = "LAC")]
        #[doc = "Latin America and the Caribbean."]
        Lac,
        #[serde(rename = "AE")]
        #[doc = "United Arab Emirates"]
        Ae,
        #[serde(rename = "AR")]
        #[doc = "Argentina"]
        Ar,
        #[serde(rename = "AT")]
        #[doc = "Austria"]
        At,
        #[serde(rename = "AU")]
        #[doc = "Australia"]
        Au,
        #[serde(rename = "BE")]
        #[doc = "Belgium"]
        Be,
        #[serde(rename = "BR")]
        #[doc = "Brazil"]
        Br,
        #[serde(rename = "CA")]
        #[doc = "Canada"]
        Ca,
        #[serde(rename = "CH")]
        #[doc = "Switzerland"]
        Ch,
        #[serde(rename = "CL")]
        #[doc = "Chile"]
        Cl,
        #[serde(rename = "CN")]
        #[doc = "China"]
        Cn,
        #[serde(rename = "CO")]
        #[doc = "Colombia"]
        Co,
        #[serde(rename = "CS")]
        #[doc = "Czechoslovakia"]
        Cs,
        #[serde(rename = "CZ")]
        #[doc = "Czech Republic"]
        Cz,
        #[serde(rename = "DE")]
        #[doc = "Germany"]
        De,
        #[serde(rename = "DK")]
        #[doc = "Denmark"]
        Dk,
        #[serde(rename = "DZ")]
        #[doc = "Algeria"]
        Dz,
        #[serde(rename = "EC")]
        #[doc = "Ecuador"]
        Ec,
        #[serde(rename = "EE")]
        #[doc = "Estonia"]
        Ee,
        #[serde(rename = "EG")]
        #[doc = "Egypt"]
        Eg,
        #[serde(rename = "ES")]
        #[doc = "Spain"]
        Es,
        #[serde(rename = "FI")]
        #[doc = "Finland"]
        Fi,
        #[serde(rename = "FR")]
        #[doc = "France"]
        Fr,
        #[serde(rename = "GB")]
        #[doc = "Great Britain (United Kingdom)"]
        Gb,
        #[serde(rename = "GR")]
        #[doc = "Greece"]
        Gr,
        #[serde(rename = "HK")]
        #[doc = "Hong Kong"]
        Hk,
        #[serde(rename = "HU")]
        #[doc = "Hungary"]
        Hu,
        #[serde(rename = "ID")]
        #[doc = "Indonesia"]
        Id,
        #[serde(rename = "IE")]
        #[doc = "Ireland"]
        Ie,
        #[serde(rename = "IL")]
        #[doc = "Israel"]
        Il,
        #[serde(rename = "IN")]
        #[doc = "India"]
        In,
        #[serde(rename = "IR")]
        #[doc = "Iran"]
        Ir,
        #[serde(rename = "IT")]
        #[doc = "Italy"]
        It,
        #[serde(rename = "JP")]
        #[doc = "Japan"]
        Jp,
        #[serde(rename = "KR")]
        #[doc = "Korea (South)"]
        Kr,
        #[serde(rename = "LV")]
        #[doc = "Latvia"]
        Lv,
        #[serde(rename = "MA")]
        #[doc = "Morocco"]
        Ma,
        #[serde(rename = "MX")]
        #[doc = "Mexico"]
        Mx,
        #[serde(rename = "MY")]
        #[doc = "Malaysia"]
        My,
        #[serde(rename = "NG")]
        #[doc = "Nigeria"]
        Ng,
        #[serde(rename = "NL")]
        #[doc = "Netherlands"]
        Nl,
        #[serde(rename = "NO")]
        #[doc = "Norway"]
        No,
        #[serde(rename = "NZ")]
        #[doc = "New Zealand"]
        Nz,
        #[serde(rename = "PE")]
        #[doc = "Peru"]
        Pe,
        #[serde(rename = "PH")]
        #[doc = "Philippines"]
        Ph,
        #[serde(rename = "PK")]
        #[doc = "Pakistan"]
        Pk,
        #[serde(rename = "PL")]
        #[doc = "Poland"]
        Pl,
        #[serde(rename = "PT")]
        #[doc = "Portugal"]
        Pt,
        #[serde(rename = "RO")]
        #[doc = "Romania"]
        Ro,
        #[serde(rename = "RS")]
        #[doc = "Serbia"]
        Rs,
        #[serde(rename = "RU")]
        #[doc = "Russian Federation"]
        Ru,
        #[serde(rename = "SA")]
        #[doc = "Saudi Arabia"]
        Sa,
        #[serde(rename = "SE")]
        #[doc = "Sweden"]
        Se,
        #[serde(rename = "SG")]
        #[doc = "Singapore"]
        Sg,
        #[serde(rename = "SI")]
        #[doc = "Slovenia"]
        Si,
        #[serde(rename = "SK")]
        #[doc = "Slovakia"]
        Sk,
        #[serde(rename = "TH")]
        #[doc = "Thailand"]
        Th,
        #[serde(rename = "TR")]
        #[doc = "Turkey"]
        Tr,
        #[serde(rename = "TW")]
        #[doc = "Taiwan"]
        Tw,
        #[serde(rename = "UA")]
        #[doc = "Ukraine"]
        Ua,
        #[serde(rename = "US")]
        #[doc = "United States"]
        Us,
        #[serde(rename = "VE")]
        #[doc = "Venezuela"]
        Ve,
        #[serde(rename = "VN")]
        #[doc = "Viet Nam"]
        Vn,
        #[serde(rename = "ZA")]
        #[doc = "South Africa"]
        Za,
    }
    impl ::std::default::Default for TrainingOptionsHolidayRegionEnum {
        fn default() -> Self {
            Self::HolidayRegionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The method used to initialize the centroids for kmeans algorithm."]
    pub enum TrainingOptionsKmeansInitializationMethodEnum {
        #[serde(rename = "KMEANS_INITIALIZATION_METHOD_UNSPECIFIED")]
        #[doc = "Unspecified initialization method."]
        KmeansInitializationMethodUnspecified,
        #[serde(rename = "RANDOM")]
        #[doc = "Initializes the centroids randomly."]
        Random,
        #[serde(rename = "CUSTOM")]
        #[doc = "Initializes the centroids using data specified in kmeans_initialization_column."]
        Custom,
        #[serde(rename = "KMEANS_PLUS_PLUS")]
        #[doc = "Initializes with kmeans++."]
        KmeansPlusPlus,
    }
    impl ::std::default::Default for TrainingOptionsKmeansInitializationMethodEnum {
        fn default() -> Self {
            Self::KmeansInitializationMethodUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The strategy to determine learn rate for the current iteration."]
    pub enum TrainingOptionsLearnRateStrategyEnum {
        #[serde(rename = "LEARN_RATE_STRATEGY_UNSPECIFIED")]
        #[doc = ""]
        LearnRateStrategyUnspecified,
        #[serde(rename = "LINE_SEARCH")]
        #[doc = "Use line search to determine learning rate."]
        LineSearch,
        #[serde(rename = "CONSTANT")]
        #[doc = "Use a constant learning rate."]
        Constant,
    }
    impl ::std::default::Default for TrainingOptionsLearnRateStrategyEnum {
        fn default() -> Self {
            Self::LearnRateStrategyUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of loss function used during training run."]
    pub enum TrainingOptionsLossTypeEnum {
        #[serde(rename = "LOSS_TYPE_UNSPECIFIED")]
        #[doc = ""]
        LossTypeUnspecified,
        #[serde(rename = "MEAN_SQUARED_LOSS")]
        #[doc = "Mean squared loss, used for linear regression."]
        MeanSquaredLoss,
        #[serde(rename = "MEAN_LOG_LOSS")]
        #[doc = "Mean log loss, used for logistic regression."]
        MeanLogLoss,
    }
    impl ::std::default::Default for TrainingOptionsLossTypeEnum {
        fn default() -> Self {
            Self::LossTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optimization strategy for training linear regression models."]
    pub enum TrainingOptionsOptimizationStrategyEnum {
        #[serde(rename = "OPTIMIZATION_STRATEGY_UNSPECIFIED")]
        #[doc = ""]
        OptimizationStrategyUnspecified,
        #[serde(rename = "BATCH_GRADIENT_DESCENT")]
        #[doc = "Uses an iterative batch gradient descent algorithm."]
        BatchGradientDescent,
        #[serde(rename = "NORMAL_EQUATION")]
        #[doc = "Uses a normal equation to solve linear regression problem."]
        NormalEquation,
    }
    impl ::std::default::Default for TrainingOptionsOptimizationStrategyEnum {
        fn default() -> Self {
            Self::OptimizationStrategyUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a single training query run for the model."]
    pub struct TrainingRun {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSplitResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data split result of the training run. Only set when the input data is actually split."]
        pub data_split_result: ::std::option::Option<::std::boxed::Box<DataSplitResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "evaluationMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The evaluation metrics over training/eval data that were computed at the end of training."]
        pub evaluation_metrics: ::std::option::Option<::std::boxed::Box<EvaluationMetrics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "globalExplanations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Global explanations for important features of the model. For multi-class models, there is one entry for each label class. For other models, there is only one entry in the list."]
        pub global_explanations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GlobalExplanation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output of each iteration run, results.size() <= max_iterations."]
        pub results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IterationResult>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start time of this training run."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trainingOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Options that were used for this training run, includes user specified and default options that were used."]
        pub training_options: ::std::option::Option<::std::boxed::Box<TrainingOptions>>,
    }
    impl TrainingRun {
        pub fn builder() -> TrainingRunBuilder {
            TrainingRunBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TransactionInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transactionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output-only] // [Alpha] Id of the transaction."]
        pub transaction_id: ::std::option::Option<::std::string::String>,
    }
    impl TransactionInfo {
        pub fn builder() -> TransactionInfoBuilder {
            TransactionInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This is used for defining User Defined Function (UDF) resources only when using legacy SQL. Users of Standard SQL should leverage either DDL (e.g. CREATE [TEMPORARY] FUNCTION ... ) or the Routines API to define UDF resources. For additional information on migrating, see: https://cloud.google.com/bigquery/docs/reference/standard-sql/migrating-from-legacy-sql#differences_in_user-defined_javascript_functions"]
    pub struct UserDefinedFunctionResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inlineCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Pick one] An inline resource that contains code for a user-defined function (UDF). Providing a inline code resource is equivalent to providing a URI for a file containing the same code."]
        pub inline_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Pick one] A code resource to load from a Google Cloud Storage URI (gs://bucket/path)."]
        pub resource_uri: ::std::option::Option<::std::string::String>,
    }
    impl UserDefinedFunctionResource {
        pub fn builder() -> UserDefinedFunctionResourceBuilder {
            UserDefinedFunctionResourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ViewDefinition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Required] A query that BigQuery executes when the view is referenced."]
        pub query: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useLegacySql")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies whether to use BigQuery's legacy SQL for this view. The default value is true. If set to false, the view will use BigQuery's standard SQL: https://cloud.google.com/bigquery/sql-reference/ Queries and views that reference this view must use the same flag value."]
        pub use_legacy_sql: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userDefinedFunctionResources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes user-defined function resources used in the query."]
        pub user_defined_function_resources:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UserDefinedFunctionResource>>>,
    }
    impl ViewDefinition {
        pub fn builder() -> ViewDefinitionBuilder {
            ViewDefinitionBuilder::default()
        }
    }
}
