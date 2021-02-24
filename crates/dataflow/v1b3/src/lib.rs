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
        pub mod methods {
            pub mod delete_snapshots {
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
                    #[doc = "The location that contains this snapshot."]
                    pub location: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "snapshotId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The ID of the snapshot."]
                    pub snapshot_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
        pub mod resources {
            pub mod jobs {
                pub mod methods {
                    pub mod aggregated {
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
                            #[doc = "The kind of filter to use."]
                            pub filter: ::std::option::Option<QueryParametersFilterEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "location")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains this job."]
                            pub location: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If there are many jobs, limit response to at most this many. The actual number of jobs returned will be the lesser of max_responses and an unspecified server-defined limit."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Set this to the 'next_page_token' field of a previous response to request additional results in a long list."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Deprecated. ListJobs always returns summaries now. Use GetJob for other JobViews."]
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
                        #[doc = "The kind of filter to use."]
                        pub enum QueryParametersFilterEnum {
                            #[serde(rename = "UNKNOWN")]
                            #[doc = "The filter isn't specified, or is unknown. This returns all jobs ordered on descending `JobUuid`."]
                            Unknown,
                            #[serde(rename = "ALL")]
                            #[doc = "Returns all running jobs first ordered on creation timestamp, then returns all terminated jobs ordered on the termination timestamp."]
                            All,
                            #[serde(rename = "TERMINATED")]
                            #[doc = "Filters the jobs that have a terminated state, ordered on the termination timestamp. Example terminated states: `JOB_STATE_STOPPED`, `JOB_STATE_UPDATED`, `JOB_STATE_DRAINED`, etc."]
                            Terminated,
                            #[serde(rename = "ACTIVE")]
                            #[doc = "Filters the jobs that are running ordered on the creation timestamp."]
                            Active,
                        }
                        impl ::std::default::Default for QueryParametersFilterEnum {
                            fn default() -> Self {
                                Self::Unknown
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Deprecated. ListJobs always returns summaries now. Use GetJob for other JobViews."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "JOB_VIEW_UNKNOWN")]
                            #[doc = "The job view to return isn't specified, or is unknown. Responses will contain at least the `JOB_VIEW_SUMMARY` information, and may contain additional information."]
                            JobViewUnknown,
                            #[serde(rename = "JOB_VIEW_SUMMARY")]
                            #[doc = "Request summary information only: Project ID, Job ID, job name, job type, job status, start/end time, and Cloud SDK version details."]
                            JobViewSummary,
                            #[serde(rename = "JOB_VIEW_ALL")]
                            #[doc = "Request all information available for this job."]
                            JobViewAll,
                            #[serde(rename = "JOB_VIEW_DESCRIPTION")]
                            #[doc = "Request summary info and limited job description data for steps, labels and environment."]
                            JobViewDescription,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::JobViewUnknown
                            }
                        }
                    }
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
                            #[serde(rename = "location")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains this job."]
                            pub location: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "replaceJobId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Deprecated. This field is now in the Job message."]
                            pub replace_job_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The level of information requested in response."]
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
                        #[doc = "The level of information requested in response."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "JOB_VIEW_UNKNOWN")]
                            #[doc = "The job view to return isn't specified, or is unknown. Responses will contain at least the `JOB_VIEW_SUMMARY` information, and may contain additional information."]
                            JobViewUnknown,
                            #[serde(rename = "JOB_VIEW_SUMMARY")]
                            #[doc = "Request summary information only: Project ID, Job ID, job name, job type, job status, start/end time, and Cloud SDK version details."]
                            JobViewSummary,
                            #[serde(rename = "JOB_VIEW_ALL")]
                            #[doc = "Request all information available for this job."]
                            JobViewAll,
                            #[serde(rename = "JOB_VIEW_DESCRIPTION")]
                            #[doc = "Request summary info and limited job description data for steps, labels and environment."]
                            JobViewDescription,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::JobViewUnknown
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
                            #[serde(rename = "location")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains this job."]
                            pub location: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The level of information requested in response."]
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
                        #[doc = "The level of information requested in response."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "JOB_VIEW_UNKNOWN")]
                            #[doc = "The job view to return isn't specified, or is unknown. Responses will contain at least the `JOB_VIEW_SUMMARY` information, and may contain additional information."]
                            JobViewUnknown,
                            #[serde(rename = "JOB_VIEW_SUMMARY")]
                            #[doc = "Request summary information only: Project ID, Job ID, job name, job type, job status, start/end time, and Cloud SDK version details."]
                            JobViewSummary,
                            #[serde(rename = "JOB_VIEW_ALL")]
                            #[doc = "Request all information available for this job."]
                            JobViewAll,
                            #[serde(rename = "JOB_VIEW_DESCRIPTION")]
                            #[doc = "Request summary info and limited job description data for steps, labels and environment."]
                            JobViewDescription,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::JobViewUnknown
                            }
                        }
                    }
                    pub mod get_metrics {
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
                            #[serde(rename = "location")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains the job specified by job_id."]
                            pub location: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startTime")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Return only metric data that has changed since this time. Default is to return all information about all metrics for the job."]
                            pub start_time: ::std::option::Option<::std::string::String>,
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
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The kind of filter to use."]
                            pub filter: ::std::option::Option<QueryParametersFilterEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "location")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains this job."]
                            pub location: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If there are many jobs, limit response to at most this many. The actual number of jobs returned will be the lesser of max_responses and an unspecified server-defined limit."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Set this to the 'next_page_token' field of a previous response to request additional results in a long list."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Deprecated. ListJobs always returns summaries now. Use GetJob for other JobViews."]
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
                        #[doc = "The kind of filter to use."]
                        pub enum QueryParametersFilterEnum {
                            #[serde(rename = "UNKNOWN")]
                            #[doc = "The filter isn't specified, or is unknown. This returns all jobs ordered on descending `JobUuid`."]
                            Unknown,
                            #[serde(rename = "ALL")]
                            #[doc = "Returns all running jobs first ordered on creation timestamp, then returns all terminated jobs ordered on the termination timestamp."]
                            All,
                            #[serde(rename = "TERMINATED")]
                            #[doc = "Filters the jobs that have a terminated state, ordered on the termination timestamp. Example terminated states: `JOB_STATE_STOPPED`, `JOB_STATE_UPDATED`, `JOB_STATE_DRAINED`, etc."]
                            Terminated,
                            #[serde(rename = "ACTIVE")]
                            #[doc = "Filters the jobs that are running ordered on the creation timestamp."]
                            Active,
                        }
                        impl ::std::default::Default for QueryParametersFilterEnum {
                            fn default() -> Self {
                                Self::Unknown
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Deprecated. ListJobs always returns summaries now. Use GetJob for other JobViews."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "JOB_VIEW_UNKNOWN")]
                            #[doc = "The job view to return isn't specified, or is unknown. Responses will contain at least the `JOB_VIEW_SUMMARY` information, and may contain additional information."]
                            JobViewUnknown,
                            #[serde(rename = "JOB_VIEW_SUMMARY")]
                            #[doc = "Request summary information only: Project ID, Job ID, job name, job type, job status, start/end time, and Cloud SDK version details."]
                            JobViewSummary,
                            #[serde(rename = "JOB_VIEW_ALL")]
                            #[doc = "Request all information available for this job."]
                            JobViewAll,
                            #[serde(rename = "JOB_VIEW_DESCRIPTION")]
                            #[doc = "Request summary info and limited job description data for steps, labels and environment."]
                            JobViewDescription,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::JobViewUnknown
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
                            #[serde(rename = "location")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains this job."]
                            pub location: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod messages {
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
                                    #[serde(rename = "endTime")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Return only messages with timestamps < end_time. The default is now (i.e. return up to the latest messages available)."]
                                    pub end_time: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "location")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains the job specified by job_id."]
                                    pub location: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "minimumImportance")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Filter to only get messages with importance >= level"]
                                    pub minimum_importance:
                                        ::std::option::Option<QueryParametersMinimumImportanceEnum>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "If specified, determines the maximum number of messages to return. If unspecified, the service may choose an appropriate default, or may return an arbitrarily large number of results."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "If supplied, this should be the value of next_page_token returned by an earlier call. This will cause the next page of results to be returned."]
                                    pub page_token: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "startTime")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "If specified, return only messages with timestamps >= start_time. The default is the job creation time (i.e. beginning of messages)."]
                                    pub start_time: ::std::option::Option<::std::string::String>,
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
                                #[doc = "Filter to only get messages with importance >= level"]
                                pub enum QueryParametersMinimumImportanceEnum {
                                    #[serde(rename = "JOB_MESSAGE_IMPORTANCE_UNKNOWN")]
                                    #[doc = "The message importance isn't specified, or is unknown."]
                                    JobMessageImportanceUnknown,
                                    #[serde(rename = "JOB_MESSAGE_DEBUG")]
                                    #[doc = "The message is at the 'debug' level: typically only useful for software engineers working on the code the job is running. Typically, Dataflow pipeline runners do not display log messages at this level by default."]
                                    JobMessageDebug,
                                    #[serde(rename = "JOB_MESSAGE_DETAILED")]
                                    #[doc = "The message is at the 'detailed' level: somewhat verbose, but potentially useful to users. Typically, Dataflow pipeline runners do not display log messages at this level by default. These messages are displayed by default in the Dataflow monitoring UI."]
                                    JobMessageDetailed,
                                    #[serde(rename = "JOB_MESSAGE_BASIC")]
                                    #[doc = "The message is at the 'basic' level: useful for keeping track of the execution of a Dataflow pipeline. Typically, Dataflow pipeline runners display log messages at this level by default, and these messages are displayed by default in the Dataflow monitoring UI."]
                                    JobMessageBasic,
                                    #[serde(rename = "JOB_MESSAGE_WARNING")]
                                    #[doc = "The message is at the 'warning' level: indicating a condition pertaining to a job which may require human intervention. Typically, Dataflow pipeline runners display log messages at this level by default, and these messages are displayed by default in the Dataflow monitoring UI."]
                                    JobMessageWarning,
                                    #[serde(rename = "JOB_MESSAGE_ERROR")]
                                    #[doc = "The message is at the 'error' level: indicating a condition preventing a job from succeeding. Typically, Dataflow pipeline runners display log messages at this level by default, and these messages are displayed by default in the Dataflow monitoring UI."]
                                    JobMessageError,
                                }
                                impl ::std::default::Default for QueryParametersMinimumImportanceEnum {
                                    fn default() -> Self {
                                        Self::JobMessageImportanceUnknown
                                    }
                                }
                            }
                        }
                    }
                }
            }
            pub mod locations {
                pub mod resources {
                    pub mod jobs {
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
                                    #[serde(rename = "replaceJobId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. This field is now in the Job message."]
                                    pub replace_job_id:
                                        ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "view")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The level of information requested in response."]
                                    pub view: ::std::option::Option<QueryParametersViewEnum>,
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
                                #[doc = "The level of information requested in response."]
                                pub enum QueryParametersViewEnum {
                                    #[serde(rename = "JOB_VIEW_UNKNOWN")]
                                    #[doc = "The job view to return isn't specified, or is unknown. Responses will contain at least the `JOB_VIEW_SUMMARY` information, and may contain additional information."]
                                    JobViewUnknown,
                                    #[serde(rename = "JOB_VIEW_SUMMARY")]
                                    #[doc = "Request summary information only: Project ID, Job ID, job name, job type, job status, start/end time, and Cloud SDK version details."]
                                    JobViewSummary,
                                    #[serde(rename = "JOB_VIEW_ALL")]
                                    #[doc = "Request all information available for this job."]
                                    JobViewAll,
                                    #[serde(rename = "JOB_VIEW_DESCRIPTION")]
                                    #[doc = "Request summary info and limited job description data for steps, labels and environment."]
                                    JobViewDescription,
                                }
                                impl ::std::default::Default for QueryParametersViewEnum {
                                    fn default() -> Self {
                                        Self::JobViewUnknown
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
                                    #[serde(rename = "view")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The level of information requested in response."]
                                    pub view: ::std::option::Option<QueryParametersViewEnum>,
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
                                #[doc = "The level of information requested in response."]
                                pub enum QueryParametersViewEnum {
                                    #[serde(rename = "JOB_VIEW_UNKNOWN")]
                                    #[doc = "The job view to return isn't specified, or is unknown. Responses will contain at least the `JOB_VIEW_SUMMARY` information, and may contain additional information."]
                                    JobViewUnknown,
                                    #[serde(rename = "JOB_VIEW_SUMMARY")]
                                    #[doc = "Request summary information only: Project ID, Job ID, job name, job type, job status, start/end time, and Cloud SDK version details."]
                                    JobViewSummary,
                                    #[serde(rename = "JOB_VIEW_ALL")]
                                    #[doc = "Request all information available for this job."]
                                    JobViewAll,
                                    #[serde(rename = "JOB_VIEW_DESCRIPTION")]
                                    #[doc = "Request summary info and limited job description data for steps, labels and environment."]
                                    JobViewDescription,
                                }
                                impl ::std::default::Default for QueryParametersViewEnum {
                                    fn default() -> Self {
                                        Self::JobViewUnknown
                                    }
                                }
                            }
                            pub mod get_execution_details {
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
                                    #[doc = "If specified, determines the maximum number of stages to return. If unspecified, the service may choose an appropriate default, or may return an arbitrarily large number of results."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "If supplied, this should be the value of next_page_token returned by an earlier call. This will cause the next page of results to be returned."]
                                    pub page_token: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                            pub mod get_metrics {
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
                                    #[serde(rename = "startTime")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Return only metric data that has changed since this time. Default is to return all information about all metrics for the job."]
                                    pub start_time: ::std::option::Option<::std::string::String>,
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
                                    #[doc = "The kind of filter to use."]
                                    pub filter: ::std::option::Option<QueryParametersFilterEnum>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "If there are many jobs, limit response to at most this many. The actual number of jobs returned will be the lesser of max_responses and an unspecified server-defined limit."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Set this to the 'next_page_token' field of a previous response to request additional results in a long list."]
                                    pub page_token: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "view")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. ListJobs always returns summaries now. Use GetJob for other JobViews."]
                                    pub view: ::std::option::Option<QueryParametersViewEnum>,
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
                                #[doc = "The kind of filter to use."]
                                pub enum QueryParametersFilterEnum {
                                    #[serde(rename = "UNKNOWN")]
                                    #[doc = "The filter isn't specified, or is unknown. This returns all jobs ordered on descending `JobUuid`."]
                                    Unknown,
                                    #[serde(rename = "ALL")]
                                    #[doc = "Returns all running jobs first ordered on creation timestamp, then returns all terminated jobs ordered on the termination timestamp."]
                                    All,
                                    #[serde(rename = "TERMINATED")]
                                    #[doc = "Filters the jobs that have a terminated state, ordered on the termination timestamp. Example terminated states: `JOB_STATE_STOPPED`, `JOB_STATE_UPDATED`, `JOB_STATE_DRAINED`, etc."]
                                    Terminated,
                                    #[serde(rename = "ACTIVE")]
                                    #[doc = "Filters the jobs that are running ordered on the creation timestamp."]
                                    Active,
                                }
                                impl ::std::default::Default for QueryParametersFilterEnum {
                                    fn default() -> Self {
                                        Self::Unknown
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
                                #[doc = "Deprecated. ListJobs always returns summaries now. Use GetJob for other JobViews."]
                                pub enum QueryParametersViewEnum {
                                    #[serde(rename = "JOB_VIEW_UNKNOWN")]
                                    #[doc = "The job view to return isn't specified, or is unknown. Responses will contain at least the `JOB_VIEW_SUMMARY` information, and may contain additional information."]
                                    JobViewUnknown,
                                    #[serde(rename = "JOB_VIEW_SUMMARY")]
                                    #[doc = "Request summary information only: Project ID, Job ID, job name, job type, job status, start/end time, and Cloud SDK version details."]
                                    JobViewSummary,
                                    #[serde(rename = "JOB_VIEW_ALL")]
                                    #[doc = "Request all information available for this job."]
                                    JobViewAll,
                                    #[serde(rename = "JOB_VIEW_DESCRIPTION")]
                                    #[doc = "Request summary info and limited job description data for steps, labels and environment."]
                                    JobViewDescription,
                                }
                                impl ::std::default::Default for QueryParametersViewEnum {
                                    fn default() -> Self {
                                        Self::JobViewUnknown
                                    }
                                }
                            }
                        }
                        pub mod resources {
                            pub mod messages {
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
                                            #[serde(rename = "endTime")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Return only messages with timestamps < end_time. The default is now (i.e. return up to the latest messages available)."]
                                            pub end_time:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "minimumImportance")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Filter to only get messages with importance >= level"]
                                            pub minimum_importance: ::std::option::Option<
                                                QueryParametersMinimumImportanceEnum,
                                            >,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "If specified, determines the maximum number of messages to return. If unspecified, the service may choose an appropriate default, or may return an arbitrarily large number of results."]
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
                                            #[doc = "If supplied, this should be the value of next_page_token returned by an earlier call. This will cause the next page of results to be returned."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "startTime")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "If specified, return only messages with timestamps >= start_time. The default is the job creation time (i.e. beginning of messages)."]
                                            pub start_time:
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
                                        #[doc = "Filter to only get messages with importance >= level"]
                                        pub enum QueryParametersMinimumImportanceEnum {
                                            #[serde(rename = "JOB_MESSAGE_IMPORTANCE_UNKNOWN")]
                                            #[doc = "The message importance isn't specified, or is unknown."]
                                            JobMessageImportanceUnknown,
                                            #[serde(rename = "JOB_MESSAGE_DEBUG")]
                                            #[doc = "The message is at the 'debug' level: typically only useful for software engineers working on the code the job is running. Typically, Dataflow pipeline runners do not display log messages at this level by default."]
                                            JobMessageDebug,
                                            #[serde(rename = "JOB_MESSAGE_DETAILED")]
                                            #[doc = "The message is at the 'detailed' level: somewhat verbose, but potentially useful to users. Typically, Dataflow pipeline runners do not display log messages at this level by default. These messages are displayed by default in the Dataflow monitoring UI."]
                                            JobMessageDetailed,
                                            #[serde(rename = "JOB_MESSAGE_BASIC")]
                                            #[doc = "The message is at the 'basic' level: useful for keeping track of the execution of a Dataflow pipeline. Typically, Dataflow pipeline runners display log messages at this level by default, and these messages are displayed by default in the Dataflow monitoring UI."]
                                            JobMessageBasic,
                                            #[serde(rename = "JOB_MESSAGE_WARNING")]
                                            #[doc = "The message is at the 'warning' level: indicating a condition pertaining to a job which may require human intervention. Typically, Dataflow pipeline runners display log messages at this level by default, and these messages are displayed by default in the Dataflow monitoring UI."]
                                            JobMessageWarning,
                                            #[serde(rename = "JOB_MESSAGE_ERROR")]
                                            #[doc = "The message is at the 'error' level: indicating a condition preventing a job from succeeding. Typically, Dataflow pipeline runners display log messages at this level by default, and these messages are displayed by default in the Dataflow monitoring UI."]
                                            JobMessageError,
                                        }
                                        impl ::std::default::Default for QueryParametersMinimumImportanceEnum {
                                            fn default() -> Self {
                                                Self::JobMessageImportanceUnknown
                                            }
                                        }
                                    }
                                }
                            }
                            pub mod stages {
                                pub mod methods {
                                    pub mod get_execution_details {
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
                                            #[serde(rename = "endTime")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Upper time bound of work items to include, by start time."]
                                            pub end_time:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "If specified, determines the maximum number of work items to return. If unspecified, the service may choose an appropriate default, or may return an arbitrarily large number of results."]
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
                                            #[doc = "If supplied, this should be the value of next_page_token returned by an earlier call. This will cause the next page of results to be returned."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "startTime")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Lower time bound of work items to include, by start time."]
                                            pub start_time:
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
                    pub mod snapshots {
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
                                    #[serde(rename = "jobId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "If specified, list snapshots created from this job."]
                                    pub job_id: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                    }
                    pub mod sql {
                        pub mod methods {
                            pub mod validate {
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
                                    #[serde(rename = "query")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The sql query to validate."]
                                    pub query: ::std::option::Option<::std::string::String>,
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
                                    #[serde(rename = "gcsPath")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Required. A Cloud Storage path to the template from which to create the job. Must be valid Cloud Storage URL, beginning with 'gs://'."]
                                    pub gcs_path: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "view")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The view to retrieve. Defaults to METADATA_ONLY."]
                                    pub view: ::std::option::Option<QueryParametersViewEnum>,
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
                                #[doc = "The view to retrieve. Defaults to METADATA_ONLY."]
                                pub enum QueryParametersViewEnum {
                                    #[serde(rename = "METADATA_ONLY")]
                                    #[doc = "Template view that retrieves only the metadata associated with the template."]
                                    MetadataOnly,
                                }
                                impl ::std::default::Default for QueryParametersViewEnum {
                                    fn default() -> Self {
                                        Self::MetadataOnly
                                    }
                                }
                            }
                            pub mod launch {
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
                                    #[serde(rename = "dynamicTemplate.gcsPath")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Path to dynamic template spec file on GCS. The file must be a Json serialized DynamicTemplateFieSpec object."]
                                    pub dynamic_template_gcs_path:
                                        ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "dynamicTemplate.stagingLocation")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Cloud Storage path for staging dependencies. Must be a valid Cloud Storage URL, beginning with `gs://`."]
                                    pub dynamic_template_staging_location:
                                        ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "gcsPath")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A Cloud Storage path to the template from which to create the job. Must be valid Cloud Storage URL, beginning with 'gs://'."]
                                    pub gcs_path: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "validateOnly")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "If true, the request is validated but not actually executed. Defaults to false."]
                                    pub validate_only:
                                        ::std::option::Option<::std::primitive::bool>,
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "location")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The location that contains this snapshot."]
                            pub location: ::std::option::Option<::std::string::String>,
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
                            #[serde(rename = "jobId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If specified, list snapshots created from this job."]
                            pub job_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "location")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The location to list snapshots in."]
                            pub location: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod template_versions {
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
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of TemplateVersions to return per page."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The page token, received from a previous ListTemplateVersions call. Provide this to retrieve the subsequent page."]
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
            pub mod templates {
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
                            #[serde(rename = "gcsPath")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. A Cloud Storage path to the template from which to create the job. Must be valid Cloud Storage URL, beginning with 'gs://'."]
                            pub gcs_path: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "location")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) to which to direct the request."]
                            pub location: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The view to retrieve. Defaults to METADATA_ONLY."]
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
                        #[doc = "The view to retrieve. Defaults to METADATA_ONLY."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "METADATA_ONLY")]
                            #[doc = "Template view that retrieves only the metadata associated with the template."]
                            MetadataOnly,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::MetadataOnly
                            }
                        }
                    }
                    pub mod launch {
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
                            #[serde(rename = "dynamicTemplate.gcsPath")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Path to dynamic template spec file on GCS. The file must be a Json serialized DynamicTemplateFieSpec object."]
                            pub dynamic_template_gcs_path:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "dynamicTemplate.stagingLocation")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Cloud Storage path for staging dependencies. Must be a valid Cloud Storage URL, beginning with `gs://`."]
                            pub dynamic_template_staging_location:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "gcsPath")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A Cloud Storage path to the template from which to create the job. Must be valid Cloud Storage URL, beginning with 'gs://'."]
                            pub gcs_path: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "location")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) to which to direct the request."]
                            pub location: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "validateOnly")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If true, the request is validated but not actually executed. Defaults to false."]
                            pub validate_only: ::std::option::Option<::std::primitive::bool>,
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
    #[doc = "Obsolete in favor of ApproximateReportedProgress and ApproximateSplitRequest."]
    pub struct ApproximateProgress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "percentComplete")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Obsolete."]
        pub percent_complete: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Obsolete."]
        pub position: ::std::option::Option<::std::boxed::Box<Position>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "remainingTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Obsolete."]
        pub remaining_time: ::std::option::Option<::std::string::String>,
    }
    impl ApproximateProgress {
        pub fn builder() -> ApproximateProgressBuilder {
            ApproximateProgressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A progress measurement of a WorkItem by a worker."]
    pub struct ApproximateReportedProgress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumedParallelism")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total amount of parallelism in the portion of input of this task that has already been consumed and is no longer active. In the first two examples above (see remaining_parallelism), the value should be 29 or 2 respectively. The sum of remaining_parallelism and consumed_parallelism should equal the total amount of parallelism in this work item. If specified, must be finite."]
        pub consumed_parallelism: ::std::option::Option<::std::boxed::Box<ReportedParallelism>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fractionConsumed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Completion as fraction of the input consumed, from 0.0 (beginning, nothing consumed), to 1.0 (end of the input, entire input consumed)."]
        pub fraction_consumed: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Position within the work to represent a progress."]
        pub position: ::std::option::Option<::std::boxed::Box<Position>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "remainingParallelism")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total amount of parallelism in the input of this task that remains, (i.e. can be delegated to this task and any new tasks via dynamic splitting). Always at least 1 for non-finished work items and 0 for finished. \"Amount of parallelism\" refers to how many non-empty parts of the input can be read in parallel. This does not necessarily equal number of records. An input that can be read in parallel down to the individual records is called \"perfectly splittable\". An example of non-perfectly parallelizable input is a block-compressed file format where a block of records has to be read as a whole, but different blocks can be read in parallel. Examples: * If we are processing record #30 (starting at 1) out of 50 in a perfectly splittable 50-record input, this value should be 21 (20 remaining + 1 current). * If we are reading through block 3 in a block-compressed file consisting of 5 blocks, this value should be 3 (since blocks 4 and 5 can be processed in parallel by new tasks via dynamic splitting and the current task remains processing block 3). * If we are reading through the last block in a block-compressed file, or reading or processing the last record in a perfectly splittable input, this value should be 1, because apart from the current task, no additional remainder can be split off."]
        pub remaining_parallelism: ::std::option::Option<::std::boxed::Box<ReportedParallelism>>,
    }
    impl ApproximateReportedProgress {
        pub fn builder() -> ApproximateReportedProgressBuilder {
            ApproximateReportedProgressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A suggestion by the service to the worker to dynamically split the WorkItem."]
    pub struct ApproximateSplitRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fractionConsumed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A fraction at which to split the work item, from 0.0 (beginning of the input) to 1.0 (end of the input)."]
        pub fraction_consumed: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fractionOfRemainder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fraction of the remainder of work to split the work item at, from 0.0 (split at the current position) to 1.0 (end of the input)."]
        pub fraction_of_remainder: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Position at which to split the work item."]
        pub position: ::std::option::Option<::std::boxed::Box<Position>>,
    }
    impl ApproximateSplitRequest {
        pub fn builder() -> ApproximateSplitRequestBuilder {
            ApproximateSplitRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Job information for templates."]
    pub struct Artifact {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Container image path set for flex Template."]
        pub container_spec: ::std::option::Option<::std::boxed::Box<ContainerSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobGraphGcsPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "job_graph_gcs_path set for legacy Template."]
        pub job_graph_gcs_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata set for legacy Template."]
        pub metadata: ::std::option::Option<::std::boxed::Box<TemplateMetadata>>,
    }
    impl Artifact {
        pub fn builder() -> ArtifactBuilder {
            ArtifactBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A structured message reporting an autoscaling decision made by the Dataflow service."]
    pub struct AutoscalingEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentNumWorkers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current number of workers the job has."]
        pub current_num_workers: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message describing why the system decided to adjust the current number of workers, why it failed, or why the system decided to not make any changes to the number of workers."]
        pub description: ::std::option::Option<::std::boxed::Box<StructuredMessage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of autoscaling event to report."]
        pub event_type: ::std::option::Option<AutoscalingEventEventTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetNumWorkers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target number of workers the worker pool wants to resize to use."]
        pub target_num_workers: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "time")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time this event was emitted to indicate a new target or current num_workers value."]
        pub time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerPool")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short and friendly name for the worker pool this event refers to, populated from the value of PoolStageRelation::user_pool_name."]
        pub worker_pool: ::std::option::Option<::std::string::String>,
    }
    impl AutoscalingEvent {
        pub fn builder() -> AutoscalingEventBuilder {
            AutoscalingEventBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of autoscaling event to report."]
    pub enum AutoscalingEventEventTypeEnum {
        #[serde(rename = "TYPE_UNKNOWN")]
        #[doc = "Default type for the enum. Value should never be returned."]
        TypeUnknown,
        #[serde(rename = "TARGET_NUM_WORKERS_CHANGED")]
        #[doc = "The TARGET_NUM_WORKERS_CHANGED type should be used when the target worker pool size has changed at the start of an actuation. An event should always be specified as TARGET_NUM_WORKERS_CHANGED if it reflects a change in the target_num_workers."]
        TargetNumWorkersChanged,
        #[serde(rename = "CURRENT_NUM_WORKERS_CHANGED")]
        #[doc = "The CURRENT_NUM_WORKERS_CHANGED type should be used when actual worker pool size has been changed, but the target_num_workers has not changed."]
        CurrentNumWorkersChanged,
        #[serde(rename = "ACTUATION_FAILURE")]
        #[doc = "The ACTUATION_FAILURE type should be used when we want to report an error to the user indicating why the current number of workers in the pool could not be changed. Displayed in the current status and history widgets."]
        ActuationFailure,
        #[serde(rename = "NO_CHANGE")]
        #[doc = "Used when we want to report to the user a reason why we are not currently adjusting the number of workers. Should specify both target_num_workers, current_num_workers and a decision_message."]
        NoChange,
    }
    impl ::std::default::Default for AutoscalingEventEventTypeEnum {
        fn default() -> Self {
            Self::TypeUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings for WorkerPool autoscaling."]
    pub struct AutoscalingSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "algorithm")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The algorithm to use for autoscaling."]
        pub algorithm: ::std::option::Option<AutoscalingSettingsAlgorithmEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxNumWorkers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of workers to cap scaling at."]
        pub max_num_workers: ::std::option::Option<::std::primitive::i64>,
    }
    impl AutoscalingSettings {
        pub fn builder() -> AutoscalingSettingsBuilder {
            AutoscalingSettingsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The algorithm to use for autoscaling."]
    pub enum AutoscalingSettingsAlgorithmEnum {
        #[serde(rename = "AUTOSCALING_ALGORITHM_UNKNOWN")]
        #[doc = "The algorithm is unknown, or unspecified."]
        AutoscalingAlgorithmUnknown,
        #[serde(rename = "AUTOSCALING_ALGORITHM_NONE")]
        #[doc = "Disable autoscaling."]
        AutoscalingAlgorithmNone,
        #[serde(rename = "AUTOSCALING_ALGORITHM_BASIC")]
        #[doc = "Increase worker count over time to reduce job execution time."]
        AutoscalingAlgorithmBasic,
    }
    impl ::std::default::Default for AutoscalingSettingsAlgorithmEnum {
        fn default() -> Self {
            Self::AutoscalingAlgorithmUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for a BigQuery connector used by the job."]
    pub struct BigQueryIoDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dataset accessed in the connection."]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Project accessed in the connection."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Query used to access data in the connection."]
        pub query: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "table")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Table accessed in the connection."]
        pub table: ::std::option::Option<::std::string::String>,
    }
    impl BigQueryIoDetails {
        pub fn builder() -> BigQueryIoDetailsBuilder {
            BigQueryIoDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for a BigTable connector used by the job."]
    pub struct BigTableIoDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "InstanceId accessed in the connection."]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ProjectId accessed in the connection."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "TableId accessed in the connection."]
        pub table_id: ::std::option::Option<::std::string::String>,
    }
    impl BigTableIoDetails {
        pub fn builder() -> BigTableIoDetailsBuilder {
            BigTableIoDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Modeled after information exposed by /proc/stat."]
    pub struct CpuTime {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Average CPU utilization rate (% non-idle cpu / second) since previous sample."]
        pub rate: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of the measurement."]
        pub timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total active CPU time across all cores (ie., non-idle) in milliseconds since start-up."]
        pub total_ms: ::std::option::Option<::std::string::String>,
    }
    impl CpuTime {
        pub fn builder() -> CpuTimeBuilder {
            CpuTimeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Commit will add a new TemplateVersion to an existing template."]
    pub struct CommitTemplateVersionRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "templateVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "TemplateVersion obejct to create."]
        pub template_version: ::std::option::Option<::std::boxed::Box<TemplateVersion>>,
    }
    impl CommitTemplateVersionRequest {
        pub fn builder() -> CommitTemplateVersionRequestBuilder {
            CommitTemplateVersionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Description of an interstitial value between transforms in an execution stage."]
    pub struct ComponentSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dataflow service generated name for this source."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalTransformOrCollection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User name for the original user transform or collection with which this source is most closely associated."]
        pub original_transform_or_collection: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human-readable name for this transform; may be user or system generated."]
        pub user_name: ::std::option::Option<::std::string::String>,
    }
    impl ComponentSource {
        pub fn builder() -> ComponentSourceBuilder {
            ComponentSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Description of a transform executed as part of an execution stage."]
    pub struct ComponentTransform {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dataflow service generated name for this source."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalTransform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User name for the original user transform with which this transform is most closely associated."]
        pub original_transform: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human-readable name for this transform; may be user or system generated."]
        pub user_name: ::std::option::Option<::std::string::String>,
    }
    impl ComponentTransform {
        pub fn builder() -> ComponentTransformBuilder {
            ComponentTransformBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "All configuration data for a particular Computation."]
    pub struct ComputationTopology {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "computationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the computation."]
        pub computation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The inputs to the computation."]
        pub inputs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StreamLocation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key ranges processed by the computation."]
        pub key_ranges: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<KeyRangeLocation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The outputs from the computation."]
        pub outputs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StreamLocation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateFamilies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state family values."]
        pub state_families:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StateFamilyConfig>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemStageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The system stage name."]
        pub system_stage_name: ::std::option::Option<::std::string::String>,
    }
    impl ComputationTopology {
        pub fn builder() -> ComputationTopologyBuilder {
            ComputationTopologyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A position that encapsulates an inner position and an index for the inner position. A ConcatPosition can be used by a reader of a source that encapsulates a set of other sources."]
    pub struct ConcatPosition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Index of the inner source."]
        pub index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Position within the inner source."]
        pub position: ::std::option::Option<::std::boxed::Box<Position>>,
    }
    impl ConcatPosition {
        pub fn builder() -> ConcatPositionBuilder {
            ConcatPositionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Container Spec."]
    pub struct ContainerSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultEnvironment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Default runtime environment for the job."]
        pub default_environment:
            ::std::option::Option<::std::boxed::Box<FlexTemplateRuntimeEnvironment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the docker container image. E.g., gcr.io/project/some-image"]
        pub image: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata describing a template including description and validation rules."]
        pub metadata: ::std::option::Option<::std::boxed::Box<TemplateMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sdkInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. SDK info of the Flex Template."]
        pub sdk_info: ::std::option::Option<::std::boxed::Box<SdkInfo>>,
    }
    impl ContainerSpec {
        pub fn builder() -> ContainerSpecBuilder {
            ContainerSpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "CounterMetadata includes all static non-name non-value counter attributes."]
    pub struct CounterMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human-readable description of the counter semantics."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Counter aggregation kind."]
        pub kind: ::std::option::Option<CounterMetadataKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "otherUnits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A string referring to the unit type."]
        pub other_units: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "standardUnits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System defined Units, see above enum."]
        pub standard_units: ::std::option::Option<CounterMetadataStandardUnitsEnum>,
    }
    impl CounterMetadata {
        pub fn builder() -> CounterMetadataBuilder {
            CounterMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Counter aggregation kind."]
    pub enum CounterMetadataKindEnum {
        #[serde(rename = "INVALID")]
        #[doc = "Counter aggregation kind was not set."]
        Invalid,
        #[serde(rename = "SUM")]
        #[doc = "Aggregated value is the sum of all contributed values."]
        Sum,
        #[serde(rename = "MAX")]
        #[doc = "Aggregated value is the max of all contributed values."]
        Max,
        #[serde(rename = "MIN")]
        #[doc = "Aggregated value is the min of all contributed values."]
        Min,
        #[serde(rename = "MEAN")]
        #[doc = "Aggregated value is the mean of all contributed values."]
        Mean,
        #[serde(rename = "OR")]
        #[doc = "Aggregated value represents the logical 'or' of all contributed values."]
        Or,
        #[serde(rename = "AND")]
        #[doc = "Aggregated value represents the logical 'and' of all contributed values."]
        And,
        #[serde(rename = "SET")]
        #[doc = "Aggregated value is a set of unique contributed values."]
        Set,
        #[serde(rename = "DISTRIBUTION")]
        #[doc = "Aggregated value captures statistics about a distribution."]
        Distribution,
        #[serde(rename = "LATEST_VALUE")]
        #[doc = "Aggregated value tracks the latest value of a variable."]
        LatestValue,
    }
    impl ::std::default::Default for CounterMetadataKindEnum {
        fn default() -> Self {
            Self::Invalid
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "System defined Units, see above enum."]
    pub enum CounterMetadataStandardUnitsEnum {
        #[serde(rename = "BYTES")]
        #[doc = "Counter returns a value in bytes."]
        Bytes,
        #[serde(rename = "BYTES_PER_SEC")]
        #[doc = "Counter returns a value in bytes per second."]
        BytesPerSec,
        #[serde(rename = "MILLISECONDS")]
        #[doc = "Counter returns a value in milliseconds."]
        Milliseconds,
        #[serde(rename = "MICROSECONDS")]
        #[doc = "Counter returns a value in microseconds."]
        Microseconds,
        #[serde(rename = "NANOSECONDS")]
        #[doc = "Counter returns a value in nanoseconds."]
        Nanoseconds,
        #[serde(rename = "TIMESTAMP_MSEC")]
        #[doc = "Counter returns a timestamp in milliseconds."]
        TimestampMsec,
        #[serde(rename = "TIMESTAMP_USEC")]
        #[doc = "Counter returns a timestamp in microseconds."]
        TimestampUsec,
        #[serde(rename = "TIMESTAMP_NSEC")]
        #[doc = "Counter returns a timestamp in nanoseconds."]
        TimestampNsec,
    }
    impl ::std::default::Default for CounterMetadataStandardUnitsEnum {
        fn default() -> Self {
            Self::Bytes
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identifies a counter within a per-job namespace. Counters whose structured names are the same get merged into a single value for the job."]
    pub struct CounterStructuredName {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "componentStepName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the optimized step being executed by the workers."]
        pub component_step_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStepName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the stage. An execution step contains multiple component steps."]
        pub execution_step_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Index of an input collection that's being read from/written to as a side input. The index identifies a step's side inputs starting by 1 (e.g. the first side input has input_index 1, the third has input_index 3). Side inputs are identified by a pair of (original_step_name, input_index). This field helps uniquely identify them."]
        pub input_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Counter name. Not necessarily globally-unique, but unique within the context of the other fields. Required."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "origin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "One of the standard Origins defined above."]
        pub origin: ::std::option::Option<CounterStructuredNameOriginEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originNamespace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A string containing a more specific namespace of the counter's origin."]
        pub origin_namespace: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalRequestingStepName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The step name requesting an operation, such as GBK. I.e. the ParDo causing a read/write from shuffle to occur, or a read from side inputs."]
        pub original_requesting_step_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalStepName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System generated name of the original step in the user's graph, before optimization."]
        pub original_step_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "portion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Portion of this counter, either key or value."]
        pub portion: ::std::option::Option<CounterStructuredNamePortionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of a particular worker."]
        pub worker_id: ::std::option::Option<::std::string::String>,
    }
    impl CounterStructuredName {
        pub fn builder() -> CounterStructuredNameBuilder {
            CounterStructuredNameBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "One of the standard Origins defined above."]
    pub enum CounterStructuredNameOriginEnum {
        #[serde(rename = "SYSTEM")]
        #[doc = "Counter was created by the Dataflow system."]
        System,
        #[serde(rename = "USER")]
        #[doc = "Counter was created by the user."]
        User,
    }
    impl ::std::default::Default for CounterStructuredNameOriginEnum {
        fn default() -> Self {
            Self::System
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Portion of this counter, either key or value."]
    pub enum CounterStructuredNamePortionEnum {
        #[serde(rename = "ALL")]
        #[doc = "Counter portion has not been set."]
        All,
        #[serde(rename = "KEY")]
        #[doc = "Counter reports a key."]
        Key,
        #[serde(rename = "VALUE")]
        #[doc = "Counter reports a value."]
        Value,
    }
    impl ::std::default::Default for CounterStructuredNamePortionEnum {
        fn default() -> Self {
            Self::All
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single message which encapsulates structured name and metadata for a given counter."]
    pub struct CounterStructuredNameAndMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata associated with a counter"]
        pub metadata: ::std::option::Option<::std::boxed::Box<CounterMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Structured name of the counter."]
        pub name: ::std::option::Option<::std::boxed::Box<CounterStructuredName>>,
    }
    impl CounterStructuredNameAndMetadata {
        pub fn builder() -> CounterStructuredNameAndMetadataBuilder {
            CounterStructuredNameAndMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An update to a Counter sent from a worker."]
    pub struct CounterUpdate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boolean")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boolean value for And, Or."]
        pub boolean: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cumulative")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if this counter is reported as the total cumulative aggregate value accumulated since the worker started working on this WorkItem. By default this is false, indicating that this counter is reported as a delta."]
        pub cumulative: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "distribution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Distribution data"]
        pub distribution: ::std::option::Option<::std::boxed::Box<DistributionUpdate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "floatingPoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Floating point value for Sum, Max, Min."]
        pub floating_point: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "floatingPointList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of floating point numbers, for Set."]
        pub floating_point_list: ::std::option::Option<::std::boxed::Box<FloatingPointList>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "floatingPointMean")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Floating point mean aggregation value for Mean."]
        pub floating_point_mean: ::std::option::Option<::std::boxed::Box<FloatingPointMean>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "integer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Integer value for Sum, Max, Min."]
        pub integer: ::std::option::Option<::std::boxed::Box<SplitInt64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "integerGauge")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Gauge data"]
        pub integer_gauge: ::std::option::Option<::std::boxed::Box<IntegerGauge>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "integerList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of integers, for Set."]
        pub integer_list: ::std::option::Option<::std::boxed::Box<IntegerList>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "integerMean")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Integer mean aggregation value for Mean."]
        pub integer_mean: ::std::option::Option<::std::boxed::Box<IntegerMean>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "internal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value for internally-defined counters used by the Dataflow service."]
        pub internal: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nameAndKind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Counter name and aggregation type."]
        pub name_and_kind: ::std::option::Option<::std::boxed::Box<NameAndKind>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shortId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The service-generated short identifier for this counter. The short_id -> (name, metadata) mapping is constant for the lifetime of a job."]
        pub short_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of strings, for Set."]
        pub string_list: ::std::option::Option<::std::boxed::Box<StringList>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "structuredNameAndMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Counter structured name and metadata."]
        pub structured_name_and_metadata:
            ::std::option::Option<::std::boxed::Box<CounterStructuredNameAndMetadata>>,
    }
    impl CounterUpdate {
        pub fn builder() -> CounterUpdateBuilder {
            CounterUpdateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to create a Cloud Dataflow job from a template."]
    pub struct CreateJobFromTemplateRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The runtime environment for the job."]
        pub environment: ::std::option::Option<::std::boxed::Box<RuntimeEnvironment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A Cloud Storage path to the template from which to create the job. Must be a valid Cloud Storage URL, beginning with `gs://`."]
        pub gcs_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The job name to use for the created job."]
        pub job_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) to which to direct the request."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The runtime parameters to pass to the job."]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl CreateJobFromTemplateRequest {
        pub fn builder() -> CreateJobFromTemplateRequestBuilder {
            CreateJobFromTemplateRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Creates a new Template with TemplateVersions."]
    pub struct CreateTemplateVersionRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "templateVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The TemplateVersion object to create."]
        pub template_version: ::std::option::Option<::std::boxed::Box<TemplateVersion>>,
    }
    impl CreateTemplateVersionRequest {
        pub fn builder() -> CreateTemplateVersionRequestBuilder {
            CreateTemplateVersionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identifies the location of a custom souce."]
    pub struct CustomSourceLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateful")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this source is stateful."]
        pub stateful: ::std::option::Option<::std::primitive::bool>,
    }
    impl CustomSourceLocation {
        pub fn builder() -> CustomSourceLocationBuilder {
            CustomSourceLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data disk assignment for a given VM instance."]
    pub struct DataDiskAssignment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataDisks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mounted data disks. The order is important a data disk's 0-based index in this list defines which persistent directory the disk is mounted to, for example the list of { \"myproject-1014-104817-4c2-harness-0-disk-0\" }, { \"myproject-1014-104817-4c2-harness-0-disk-1\" }."]
        pub data_disks: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vmInstance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "VM instance name the data disks mounted to, for example \"myproject-1014-104817-4c2-harness-0\"."]
        pub vm_instance: ::std::option::Option<::std::string::String>,
    }
    impl DataDiskAssignment {
        pub fn builder() -> DataDiskAssignmentBuilder {
            DataDiskAssignmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for a Datastore connector used by the job."]
    pub struct DatastoreIoDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namespace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Namespace used in the connection."]
        pub namespace: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ProjectId accessed in the connection."]
        pub project_id: ::std::option::Option<::std::string::String>,
    }
    impl DatastoreIoDetails {
        pub fn builder() -> DatastoreIoDetailsBuilder {
            DatastoreIoDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response from deleting a snapshot."]
    pub struct DeleteSnapshotResponse {}
    impl DeleteSnapshotResponse {
        pub fn builder() -> DeleteSnapshotResponseBuilder {
            DeleteSnapshotResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specification of one of the bundles produced as a result of splitting a Source (e.g. when executing a SourceSplitRequest, or when splitting an active task using WorkItemStatus.dynamic_source_split), relative to the source being split."]
    pub struct DerivedSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "derivationMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "What source to base the produced source on (if any)."]
        pub derivation_mode: ::std::option::Option<DerivedSourceDerivationModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specification of the source."]
        pub source: ::std::option::Option<::std::boxed::Box<Source>>,
    }
    impl DerivedSource {
        pub fn builder() -> DerivedSourceBuilder {
            DerivedSourceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "What source to base the produced source on (if any)."]
    pub enum DerivedSourceDerivationModeEnum {
        #[serde(rename = "SOURCE_DERIVATION_MODE_UNKNOWN")]
        #[doc = "The source derivation is unknown, or unspecified."]
        SourceDerivationModeUnknown,
        #[serde(rename = "SOURCE_DERIVATION_MODE_INDEPENDENT")]
        #[doc = "Produce a completely independent Source with no base."]
        SourceDerivationModeIndependent,
        #[serde(rename = "SOURCE_DERIVATION_MODE_CHILD_OF_CURRENT")]
        #[doc = "Produce a Source based on the Source being split."]
        SourceDerivationModeChildOfCurrent,
        #[serde(rename = "SOURCE_DERIVATION_MODE_SIBLING_OF_CURRENT")]
        #[doc = "Produce a Source based on the base of the Source being split."]
        SourceDerivationModeSiblingOfCurrent,
    }
    impl ::std::default::Default for DerivedSourceDerivationModeEnum {
        fn default() -> Self {
            Self::SourceDerivationModeUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes the data disk used by a workflow job."]
    pub struct Disk {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Disk storage type, as defined by Google Compute Engine. This must be a disk type appropriate to the project and zone in which the workers will run. If unknown or unspecified, the service will attempt to choose a reasonable default. For example, the standard persistent disk type is a resource name typically ending in \"pd-standard\". If SSD persistent disks are available, the resource name typically ends with \"pd-ssd\". The actual valid values are defined the Google Compute Engine API, not by the Cloud Dataflow API; consult the Google Compute Engine documentation for more information about determining the set of available disk types for a particular project and zone. Google Compute Engine Disk types are local to a particular project in a particular zone, and so the resource name will typically look something like this: compute.googleapis.com/projects/project-id/zones/zone/diskTypes/pd-standard"]
        pub disk_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mountPoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Directory in a VM where disk is mounted."]
        pub mount_point: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sizeGb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Size of disk in GB. If zero or unspecified, the service will attempt to choose a reasonable default."]
        pub size_gb: ::std::option::Option<::std::primitive::i64>,
    }
    impl Disk {
        pub fn builder() -> DiskBuilder {
            DiskBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data provided with a pipeline or transform to provide descriptive info."]
    pub struct DisplayData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boolValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains value if the data is of a boolean type."]
        pub bool_value: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "durationValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains value if the data is of duration type."]
        pub duration_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "floatValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains value if the data is of float type."]
        pub float_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "int64Value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains value if the data is of int64 type."]
        pub int64_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "javaClassValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains value if the data is of java class type."]
        pub java_class_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key identifying the display data. This is intended to be used as a label for the display data when viewed in a dax monitoring system."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional label to display in a dax UI for the element."]
        pub label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namespace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The namespace for the key. This is usually a class name or programming language namespace (i.e. python module) which defines the display data. This allows a dax monitoring system to specially handle the data and perform custom rendering."]
        pub namespace: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shortStrValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A possible additional shorter value to display. For example a java_class_name_value of com.mypackage.MyDoFn will be stored with MyDoFn as the short_str_value and com.mypackage.MyDoFn as the java_class_name value. short_str_value can be displayed and java_class_name_value will be displayed as a tooltip."]
        pub short_str_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "strValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains value if the data is of string type."]
        pub str_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestampValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains value if the data is of timestamp type."]
        pub timestamp_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional full URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl DisplayData {
        pub fn builder() -> DisplayDataBuilder {
            DisplayDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A metric value representing a distribution."]
    pub struct DistributionUpdate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The count of the number of elements present in the distribution."]
        pub count: ::std::option::Option<::std::boxed::Box<SplitInt64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "histogram")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(Optional) Histogram of value counts for the distribution."]
        pub histogram: ::std::option::Option<::std::boxed::Box<Histogram>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "max")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum value present in the distribution."]
        pub max: ::std::option::Option<::std::boxed::Box<SplitInt64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "min")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum value present in the distribution."]
        pub min: ::std::option::Option<::std::boxed::Box<SplitInt64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Use an int64 since we'd prefer the added precision. If overflow is a common problem we can detect it and use an additional int64 or a double."]
        pub sum: ::std::option::Option<::std::boxed::Box<SplitInt64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sumOfSquares")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Use a double since the sum of squares is likely to overflow int64."]
        pub sum_of_squares: ::std::option::Option<::std::primitive::f64>,
    }
    impl DistributionUpdate {
        pub fn builder() -> DistributionUpdateBuilder {
            DistributionUpdateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "When a task splits using WorkItemStatus.dynamic_source_split, this message describes the two parts of the split relative to the description of the current task's input."]
    pub struct DynamicSourceSplit {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Primary part (continued to be processed by worker). Specified relative to the previously-current source. Becomes current."]
        pub primary: ::std::option::Option<::std::boxed::Box<DerivedSource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "residual")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Residual part (returned to the pool of work). Specified relative to the previously-current source."]
        pub residual: ::std::option::Option<::std::boxed::Box<DerivedSource>>,
    }
    impl DynamicSourceSplit {
        pub fn builder() -> DynamicSourceSplitBuilder {
            DynamicSourceSplitBuilder::default()
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
    #[doc = "Describes the environment in which a Dataflow Job runs."]
    pub struct Environment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterManagerApiService")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of cluster manager API to use. If unknown or unspecified, the service will attempt to choose a reasonable default. This should be in the form of the API service name, e.g. \"compute.googleapis.com\"."]
        pub cluster_manager_api_service: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dataset for the current project where various workflow related tables are stored. The supported resource type is: Google BigQuery: bigquery.googleapis.com/{dataset}"]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "experiments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of experiments to enable."]
        pub experiments: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "flexResourceSchedulingGoal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which Flexible Resource Scheduling mode to run in."]
        pub flex_resource_scheduling_goal:
            ::std::option::Option<EnvironmentFlexResourceSchedulingGoalEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "internalExperiments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Experimental settings."]
        pub internal_experiments:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sdkPipelineOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud Dataflow SDK pipeline options specified by the user. These options are passed through the service and are used to recreate the SDK pipeline options on the worker in a language agnostic and platform independent way."]
        pub sdk_pipeline_options:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceAccountEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identity to run virtual machines as. Defaults to the default account."]
        pub service_account_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceKmsKeyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, contains the Cloud KMS key identifier used to encrypt data at rest, AKA a Customer Managed Encryption Key (CMEK). Format: projects/PROJECT_ID/locations/LOCATION/keyRings/KEY_RING/cryptoKeys/KEY"]
        pub service_kms_key_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shuffleMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The shuffle mode used for the job."]
        pub shuffle_mode: ::std::option::Option<EnvironmentShuffleModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tempStoragePrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The prefix of the resources the system should use for temporary storage. The system will append the suffix \"/temp-{JOBNAME} to this resource prefix, where {JOBNAME} is the value of the job_name field. The resulting bucket and object prefix is used as the prefix of the resources used to store temporary data needed during the job execution. NOTE: This will override the value in taskrunner_settings. The supported resource type is: Google Cloud Storage: storage.googleapis.com/{bucket}/{object} bucket.storage.googleapis.com/{object}"]
        pub temp_storage_prefix: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userAgent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of the process that generated the request."]
        pub user_agent:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A structure describing which components and their versions of the service are required in order to run the job."]
        pub version:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerPools")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The worker pools. At least one \"harness\" worker pool must be specified in order for the job to have workers."]
        pub worker_pools: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WorkerPool>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerRegion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Compute Engine region (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1\". Mutually exclusive with worker_zone. If neither worker_region nor worker_zone is specified, default to the control plane's region."]
        pub worker_region: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Compute Engine zone (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1-a\". Mutually exclusive with worker_region. If neither worker_region nor worker_zone is specified, a zone in the control plane's region is chosen based on available capacity."]
        pub worker_zone: ::std::option::Option<::std::string::String>,
    }
    impl Environment {
        pub fn builder() -> EnvironmentBuilder {
            EnvironmentBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Which Flexible Resource Scheduling mode to run in."]
    pub enum EnvironmentFlexResourceSchedulingGoalEnum {
        #[serde(rename = "FLEXRS_UNSPECIFIED")]
        #[doc = "Run in the default mode."]
        FlexrsUnspecified,
        #[serde(rename = "FLEXRS_SPEED_OPTIMIZED")]
        #[doc = "Optimize for lower execution time."]
        FlexrsSpeedOptimized,
        #[serde(rename = "FLEXRS_COST_OPTIMIZED")]
        #[doc = "Optimize for lower cost."]
        FlexrsCostOptimized,
    }
    impl ::std::default::Default for EnvironmentFlexResourceSchedulingGoalEnum {
        fn default() -> Self {
            Self::FlexrsUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The shuffle mode used for the job."]
    pub enum EnvironmentShuffleModeEnum {
        #[serde(rename = "SHUFFLE_MODE_UNSPECIFIED")]
        #[doc = "Shuffle mode information is not available."]
        ShuffleModeUnspecified,
        #[serde(rename = "VM_BASED")]
        #[doc = "Shuffle is done on the worker VMs."]
        VmBased,
        #[serde(rename = "SERVICE_BASED")]
        #[doc = "Shuffle is done on the service side."]
        ServiceBased,
    }
    impl ::std::default::Default for EnvironmentShuffleModeEnum {
        fn default() -> Self {
            Self::ShuffleModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A message describing the state of a particular execution stage."]
    pub struct ExecutionStageState {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentStateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the stage transitioned to this state."]
        pub current_state_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the execution stage."]
        pub execution_stage_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStageState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Executions stage states allow the same set of values as JobState."]
        pub execution_stage_state:
            ::std::option::Option<ExecutionStageStateExecutionStageStateEnum>,
    }
    impl ExecutionStageState {
        pub fn builder() -> ExecutionStageStateBuilder {
            ExecutionStageStateBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Executions stage states allow the same set of values as JobState."]
    pub enum ExecutionStageStateExecutionStageStateEnum {
        #[serde(rename = "JOB_STATE_UNKNOWN")]
        #[doc = "The job's run state isn't specified."]
        JobStateUnknown,
        #[serde(rename = "JOB_STATE_STOPPED")]
        #[doc = "`JOB_STATE_STOPPED` indicates that the job has not yet started to run."]
        JobStateStopped,
        #[serde(rename = "JOB_STATE_RUNNING")]
        #[doc = "`JOB_STATE_RUNNING` indicates that the job is currently running."]
        JobStateRunning,
        #[serde(rename = "JOB_STATE_DONE")]
        #[doc = "`JOB_STATE_DONE` indicates that the job has successfully completed. This is a terminal job state. This state may be set by the Cloud Dataflow service, as a transition from `JOB_STATE_RUNNING`. It may also be set via a Cloud Dataflow `UpdateJob` call, if the job has not yet reached a terminal state."]
        JobStateDone,
        #[serde(rename = "JOB_STATE_FAILED")]
        #[doc = "`JOB_STATE_FAILED` indicates that the job has failed. This is a terminal job state. This state may only be set by the Cloud Dataflow service, and only as a transition from `JOB_STATE_RUNNING`."]
        JobStateFailed,
        #[serde(rename = "JOB_STATE_CANCELLED")]
        #[doc = "`JOB_STATE_CANCELLED` indicates that the job has been explicitly cancelled. This is a terminal job state. This state may only be set via a Cloud Dataflow `UpdateJob` call, and only if the job has not yet reached another terminal state."]
        JobStateCancelled,
        #[serde(rename = "JOB_STATE_UPDATED")]
        #[doc = "`JOB_STATE_UPDATED` indicates that the job was successfully updated, meaning that this job was stopped and another job was started, inheriting state from this one. This is a terminal job state. This state may only be set by the Cloud Dataflow service, and only as a transition from `JOB_STATE_RUNNING`."]
        JobStateUpdated,
        #[serde(rename = "JOB_STATE_DRAINING")]
        #[doc = "`JOB_STATE_DRAINING` indicates that the job is in the process of draining. A draining job has stopped pulling from its input sources and is processing any data that remains in-flight. This state may be set via a Cloud Dataflow `UpdateJob` call, but only as a transition from `JOB_STATE_RUNNING`. Jobs that are draining may only transition to `JOB_STATE_DRAINED`, `JOB_STATE_CANCELLED`, or `JOB_STATE_FAILED`."]
        JobStateDraining,
        #[serde(rename = "JOB_STATE_DRAINED")]
        #[doc = "`JOB_STATE_DRAINED` indicates that the job has been drained. A drained job terminated by stopping pulling from its input sources and processing any data that remained in-flight when draining was requested. This state is a terminal state, may only be set by the Cloud Dataflow service, and only as a transition from `JOB_STATE_DRAINING`."]
        JobStateDrained,
        #[serde(rename = "JOB_STATE_PENDING")]
        #[doc = "`JOB_STATE_PENDING` indicates that the job has been created but is not yet running. Jobs that are pending may only transition to `JOB_STATE_RUNNING`, or `JOB_STATE_FAILED`."]
        JobStatePending,
        #[serde(rename = "JOB_STATE_CANCELLING")]
        #[doc = "`JOB_STATE_CANCELLING` indicates that the job has been explicitly cancelled and is in the process of stopping. Jobs that are cancelling may only transition to `JOB_STATE_CANCELLED` or `JOB_STATE_FAILED`."]
        JobStateCancelling,
        #[serde(rename = "JOB_STATE_QUEUED")]
        #[doc = "`JOB_STATE_QUEUED` indicates that the job has been created but is being delayed until launch. Jobs that are queued may only transition to `JOB_STATE_PENDING` or `JOB_STATE_CANCELLED`."]
        JobStateQueued,
        #[serde(rename = "JOB_STATE_RESOURCE_CLEANING_UP")]
        #[doc = "`JOB_STATE_RESOURCE_CLEANING_UP` indicates that the batch job's associated resources are currently being cleaned up after a successful run. Currently, this is an opt-in feature, please reach out to Cloud support team if you are intersted."]
        JobStateResourceCleaningUp,
    }
    impl ::std::default::Default for ExecutionStageStateExecutionStageStateEnum {
        fn default() -> Self {
            Self::JobStateUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Description of the composing transforms, names/ids, and input/outputs of a stage of execution. Some composing transforms and sources may have been generated by the Dataflow service during execution planning."]
    pub struct ExecutionStageSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "componentSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Collections produced and consumed by component transforms of this stage."]
        pub component_source:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ComponentSource>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "componentTransform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transforms that comprise this execution stage."]
        pub component_transform:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ComponentTransform>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dataflow service generated id for this stage."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input sources for this stage."]
        pub input_source: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StageSource>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of tranform this stage is executing."]
        pub kind: ::std::option::Option<ExecutionStageSummaryKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dataflow service generated name for this stage."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output sources for this stage."]
        pub output_source: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StageSource>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prerequisiteStage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Other stages that must complete before this stage can run."]
        pub prerequisite_stage: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ExecutionStageSummary {
        pub fn builder() -> ExecutionStageSummaryBuilder {
            ExecutionStageSummaryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of tranform this stage is executing."]
    pub enum ExecutionStageSummaryKindEnum {
        #[serde(rename = "UNKNOWN_KIND")]
        #[doc = "Unrecognized transform type."]
        UnknownKind,
        #[serde(rename = "PAR_DO_KIND")]
        #[doc = "ParDo transform."]
        ParDoKind,
        #[serde(rename = "GROUP_BY_KEY_KIND")]
        #[doc = "Group By Key transform."]
        GroupByKeyKind,
        #[serde(rename = "FLATTEN_KIND")]
        #[doc = "Flatten transform."]
        FlattenKind,
        #[serde(rename = "READ_KIND")]
        #[doc = "Read transform."]
        ReadKind,
        #[serde(rename = "WRITE_KIND")]
        #[doc = "Write transform."]
        WriteKind,
        #[serde(rename = "CONSTANT_KIND")]
        #[doc = "Constructs from a constant value, such as with Create.of."]
        ConstantKind,
        #[serde(rename = "SINGLETON_KIND")]
        #[doc = "Creates a Singleton view of a collection."]
        SingletonKind,
        #[serde(rename = "SHUFFLE_KIND")]
        #[doc = "Opening or closing a shuffle session, often as part of a GroupByKey."]
        ShuffleKind,
    }
    impl ::std::default::Default for ExecutionStageSummaryKindEnum {
        fn default() -> Self {
            Self::UnknownKind
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Indicates which [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) failed to respond to a request for data."]
    pub struct FailedLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that failed to respond."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl FailedLocation {
        pub fn builder() -> FailedLocationBuilder {
            FailedLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for a File connector used by the job."]
    pub struct FileIoDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filePattern")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "File Pattern used to access files by the connector."]
        pub file_pattern: ::std::option::Option<::std::string::String>,
    }
    impl FileIoDetails {
        pub fn builder() -> FileIoDetailsBuilder {
            FileIoDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An instruction that copies its inputs (zero or more) to its (single) output."]
    pub struct FlattenInstruction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes the inputs to the flatten instruction."]
        pub inputs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InstructionInput>>>,
    }
    impl FlattenInstruction {
        pub fn builder() -> FlattenInstructionBuilder {
            FlattenInstructionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The environment values to be set at runtime for flex template."]
    pub struct FlexTemplateRuntimeEnvironment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalExperiments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional experiment flags for the job."]
        pub additional_experiments: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalUserLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional user labels to be specified for the job. Keys and values must follow the restrictions specified in the [labeling restrictions](https://cloud.google.com/compute/docs/labeling-resources#restrictions) page. An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1kg\", \"count\": \"3\" }."]
        pub additional_user_labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableStreamingEngine")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to enable Streaming Engine for the job."]
        pub enable_streaming_engine: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "flexrsGoal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set FlexRS goal for the job. https://cloud.google.com/dataflow/docs/guides/flexrs"]
        pub flexrs_goal: ::std::option::Option<FlexTemplateRuntimeEnvironmentFlexrsGoalEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for VM IPs."]
        pub ip_configuration:
            ::std::option::Option<FlexTemplateRuntimeEnvironmentIpConfigurationEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmsKeyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name for the Cloud KMS key for the job. Key format is: projects//locations//keyRings//cryptoKeys/"]
        pub kms_key_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "machineType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The machine type to use for the job. Defaults to the value from the template if not specified."]
        pub machine_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxWorkers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of Google Compute Engine instances to be made available to your pipeline during execution, from 1 to 1000."]
        pub max_workers: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Network to which VMs will be assigned. If empty or unspecified, the service will use the network \"default\"."]
        pub network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numWorkers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The initial number of Google Compute Engine instances for the job."]
        pub num_workers: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceAccountEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of the service account to run the job as."]
        pub service_account_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subnetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subnetwork to which VMs will be assigned, if desired. You can specify a subnetwork using either a complete URL or an abbreviated path. Expected to be of the form \"https://www.googleapis.com/compute/v1/projects/HOST_PROJECT_ID/regions/REGION/subnetworks/SUBNETWORK\" or \"regions/REGION/subnetworks/SUBNETWORK\". If the subnetwork is located in a Shared VPC network, you must use the complete URL."]
        pub subnetwork: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tempLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud Storage path to use for temporary files. Must be a valid Cloud Storage URL, beginning with `gs://`."]
        pub temp_location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerRegion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Compute Engine region (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1\". Mutually exclusive with worker_zone. If neither worker_region nor worker_zone is specified, default to the control plane's region."]
        pub worker_region: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Compute Engine zone (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1-a\". Mutually exclusive with worker_region. If neither worker_region nor worker_zone is specified, a zone in the control plane's region is chosen based on available capacity. If both `worker_zone` and `zone` are set, `worker_zone` takes precedence."]
        pub worker_zone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Compute Engine [availability zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones) for launching worker instances to run your pipeline. In the future, worker_zone will take precedence."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl FlexTemplateRuntimeEnvironment {
        pub fn builder() -> FlexTemplateRuntimeEnvironmentBuilder {
            FlexTemplateRuntimeEnvironmentBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Set FlexRS goal for the job. https://cloud.google.com/dataflow/docs/guides/flexrs"]
    pub enum FlexTemplateRuntimeEnvironmentFlexrsGoalEnum {
        #[serde(rename = "FLEXRS_UNSPECIFIED")]
        #[doc = "Run in the default mode."]
        FlexrsUnspecified,
        #[serde(rename = "FLEXRS_SPEED_OPTIMIZED")]
        #[doc = "Optimize for lower execution time."]
        FlexrsSpeedOptimized,
        #[serde(rename = "FLEXRS_COST_OPTIMIZED")]
        #[doc = "Optimize for lower cost."]
        FlexrsCostOptimized,
    }
    impl ::std::default::Default for FlexTemplateRuntimeEnvironmentFlexrsGoalEnum {
        fn default() -> Self {
            Self::FlexrsUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Configuration for VM IPs."]
    pub enum FlexTemplateRuntimeEnvironmentIpConfigurationEnum {
        #[serde(rename = "WORKER_IP_UNSPECIFIED")]
        #[doc = "The configuration is unknown, or unspecified."]
        WorkerIpUnspecified,
        #[serde(rename = "WORKER_IP_PUBLIC")]
        #[doc = "Workers should have public IP addresses."]
        WorkerIpPublic,
        #[serde(rename = "WORKER_IP_PRIVATE")]
        #[doc = "Workers should have private IP addresses."]
        WorkerIpPrivate,
    }
    impl ::std::default::Default for FlexTemplateRuntimeEnvironmentIpConfigurationEnum {
        fn default() -> Self {
            Self::WorkerIpUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A metric value representing a list of floating point numbers."]
    pub struct FloatingPointList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "elements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Elements of the list."]
        pub elements: ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
    }
    impl FloatingPointList {
        pub fn builder() -> FloatingPointListBuilder {
            FloatingPointListBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A representation of a floating point mean metric contribution."]
    pub struct FloatingPointMean {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of values being aggregated."]
        pub count: ::std::option::Option<::std::boxed::Box<SplitInt64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sum of all values being aggregated."]
        pub sum: ::std::option::Option<::std::primitive::f64>,
    }
    impl FloatingPointMean {
        pub fn builder() -> FloatingPointMeanBuilder {
            FloatingPointMeanBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to get updated debug configuration for component."]
    pub struct GetDebugConfigRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "componentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The internal component id for which debug configuration is requested."]
        pub component_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains the job specified by job_id."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The worker id, i.e., VM hostname."]
        pub worker_id: ::std::option::Option<::std::string::String>,
    }
    impl GetDebugConfigRequest {
        pub fn builder() -> GetDebugConfigRequestBuilder {
            GetDebugConfigRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to a get debug configuration request."]
    pub struct GetDebugConfigResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "config")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The encoded debug configuration for the requested component."]
        pub config: ::std::option::Option<::std::string::String>,
    }
    impl GetDebugConfigResponse {
        pub fn builder() -> GetDebugConfigResponseBuilder {
            GetDebugConfigResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response to a GetTemplate request."]
    pub struct GetTemplateResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The template metadata describing the template name, available parameters, etc."]
        pub metadata: ::std::option::Option<::std::boxed::Box<TemplateMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "runtimeMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes the runtime metadata with SDKInfo and available parameters."]
        pub runtime_metadata: ::std::option::Option<::std::boxed::Box<RuntimeMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the get template request. Any problems with the request will be indicated in the error_details."]
        pub status: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "templateType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Template Type."]
        pub template_type: ::std::option::Option<GetTemplateResponseTemplateTypeEnum>,
    }
    impl GetTemplateResponse {
        pub fn builder() -> GetTemplateResponseBuilder {
            GetTemplateResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Template Type."]
    pub enum GetTemplateResponseTemplateTypeEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown Template Type."]
        Unknown,
        #[serde(rename = "LEGACY")]
        #[doc = "Legacy Template."]
        Legacy,
        #[serde(rename = "FLEX")]
        #[doc = "Flex Template."]
        Flex,
    }
    impl ::std::default::Default for GetTemplateResponseTemplateTypeEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Histogram of value counts for a distribution. Buckets have an inclusive lower bound and exclusive upper bound and use \"1,2,5 bucketing\": The first bucket range is from [0,1) and all subsequent bucket boundaries are powers of ten multiplied by 1, 2, or 5. Thus, bucket boundaries are 0, 1, 2, 5, 10, 20, 50, 100, 200, 500, 1000, ... Negative values are not supported."]
    pub struct Histogram {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketCounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Counts of values in each bucket. For efficiency, prefix and trailing buckets with count = 0 are elided. Buckets can store the full range of values of an unsigned long, with ULLONG_MAX falling into the 59th bucket with range [1e19, 2e19)."]
        pub bucket_counts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstBucketOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Starting index of first stored bucket. The non-inclusive upper-bound of the ith bucket is given by: pow(10,(i-first_bucket_offset)/3) * (1,2,5)[(i-first_bucket_offset)%3]"]
        pub first_bucket_offset: ::std::option::Option<::std::primitive::i64>,
    }
    impl Histogram {
        pub fn builder() -> HistogramBuilder {
            HistogramBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Proto describing a hot key detected on a given WorkItem."]
    pub struct HotKeyDetection {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hotKeyAge")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The age of the hot key measured from when it was first detected."]
        pub hot_key_age: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System-defined name of the step containing this hot key. Unique across the workflow."]
        pub system_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userStepName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-provided name of the step that contains this hot key."]
        pub user_step_name: ::std::option::Option<::std::string::String>,
    }
    impl HotKeyDetection {
        pub fn builder() -> HotKeyDetectionBuilder {
            HotKeyDetectionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An input of an instruction, as a reference to an output of a producer instruction."]
    pub struct InstructionInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputNum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output index (origin zero) within the producer."]
        pub output_num: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "producerInstructionIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index (origin zero) of the parallel instruction that produces the output to be consumed by this input. This index is relative to the list of instructions in this input's instruction's containing MapTask."]
        pub producer_instruction_index: ::std::option::Option<::std::primitive::i64>,
    }
    impl InstructionInput {
        pub fn builder() -> InstructionInputBuilder {
            InstructionInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An output of an instruction."]
    pub struct InstructionOutput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "codec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The codec to use to encode data being written via this output."]
        pub codec: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user-provided name of this output."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "onlyCountKeyBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For system-generated byte and mean byte metrics, certain instructions should only report the key size."]
        pub only_count_key_bytes: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "onlyCountValueBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For system-generated byte and mean byte metrics, certain instructions should only report the value size."]
        pub only_count_value_bytes: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System-defined name for this output in the original workflow graph. Outputs that do not contribute to an original instruction do not set this."]
        pub original_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System-defined name of this output. Unique across the workflow."]
        pub system_name: ::std::option::Option<::std::string::String>,
    }
    impl InstructionOutput {
        pub fn builder() -> InstructionOutputBuilder {
            InstructionOutputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A metric value representing temporal values of a variable."]
    pub struct IntegerGauge {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which this value was measured. Measured as msecs from epoch."]
        pub timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of the variable represented by this gauge."]
        pub value: ::std::option::Option<::std::boxed::Box<SplitInt64>>,
    }
    impl IntegerGauge {
        pub fn builder() -> IntegerGaugeBuilder {
            IntegerGaugeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A metric value representing a list of integers."]
    pub struct IntegerList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "elements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Elements of the list."]
        pub elements: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SplitInt64>>>,
    }
    impl IntegerList {
        pub fn builder() -> IntegerListBuilder {
            IntegerListBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A representation of an integer mean metric contribution."]
    pub struct IntegerMean {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of values being aggregated."]
        pub count: ::std::option::Option<::std::boxed::Box<SplitInt64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sum of all values being aggregated."]
        pub sum: ::std::option::Option<::std::boxed::Box<SplitInt64>>,
    }
    impl IntegerMean {
        pub fn builder() -> IntegerMeanBuilder {
            IntegerMeanBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines a job to be run by the Cloud Dataflow service. nextID: 26"]
    pub struct Job {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientRequestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The client's unique identifier of the job, re-used across retried attempts. If this field is set, the service will ensure its uniqueness. The request to create a job will fail if the service has knowledge of a previously submitted job with the same client's ID and job name. The caller may use this field to ensure idempotence of job creation across retried attempts to create a job. By default, the field is empty and, in that case, the service ignores it."]
        pub client_request_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the job was initially created. Immutable and set by the Cloud Dataflow service."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdFromSnapshotId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this is specified, the job's initial state is populated from the given snapshot."]
        pub created_from_snapshot_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current state of the job. Jobs are created in the `JOB_STATE_STOPPED` state unless otherwise specified. A job in the `JOB_STATE_RUNNING` state may asynchronously enter a terminal state. After a job has reached a terminal state, no further state updates may be made. This field may be mutated by the Cloud Dataflow service; callers cannot mutate it."]
        pub current_state: ::std::option::Option<JobCurrentStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentStateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp associated with the current state."]
        pub current_state_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The environment for the job."]
        pub environment: ::std::option::Option<::std::boxed::Box<Environment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated."]
        pub execution_info: ::std::option::Option<::std::boxed::Box<JobExecutionInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID of this job. This field is set by the Cloud Dataflow service when the Job is created, and is immutable for the life of the job."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is populated by the Dataflow service to support filtering jobs by the metadata values provided here. Populated for ListJobs and all GetJob views SUMMARY and higher."]
        pub job_metadata: ::std::option::Option<::std::boxed::Box<JobMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-defined labels for this job. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: * Keys must conform to regexp: \\p{Ll}\\p{Lo}{0,62} * Values must conform to regexp: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63} * Both keys and values are additionally constrained to be <= 128 bytes in size."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains this job."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user-specified Cloud Dataflow job name. Only one Job with a given name may exist in a project at any given time. If a caller attempts to create a Job with the same name as an already-existing Job, the attempt returns the existing Job. The name must match the regular expression `[a-z]([-a-z0-9]{0,38}[a-z0-9])?`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pipelineDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Preliminary field: The format of this data may change at any time. A description of the user pipeline and stages through which it is executed. Created by Cloud Dataflow service. Only retrieved with JOB_VIEW_DESCRIPTION or JOB_VIEW_ALL."]
        pub pipeline_description: ::std::option::Option<::std::boxed::Box<PipelineDescription>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the Cloud Platform project that the job belongs to."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replaceJobId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this job is an update of an existing job, this field is the job ID of the job it replaced. When sending a `CreateJobRequest`, you can update a job by specifying it here. The job named here is stopped, and its intermediate state is transferred to this job."]
        pub replace_job_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replacedByJobId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If another job is an update of this job (and thus, this job is in `JOB_STATE_UPDATED`), this field contains the ID of that job."]
        pub replaced_by_job_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestedState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The job's requested state. `UpdateJob` may be used to switch between the `JOB_STATE_STOPPED` and `JOB_STATE_RUNNING` states, by setting requested_state. `UpdateJob` may also be used to directly set a job's requested state to `JOB_STATE_CANCELLED` or `JOB_STATE_DONE`, irrevocably terminating the job if it has not already reached a terminal state."]
        pub requested_state: ::std::option::Option<JobRequestedStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "satisfiesPzs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reserved for future use. This field is set only in responses from the server; it is ignored if it is set in any requests."]
        pub satisfies_pzs: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stageStates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field may be mutated by the Cloud Dataflow service; callers cannot mutate it."]
        pub stage_states:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ExecutionStageState>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the job was started (transitioned to JOB_STATE_PENDING). Flexible resource scheduling jobs are started with some delay after job creation, so start_time is unset before start and is updated when the job is started by the Cloud Dataflow service. For other jobs, start_time always equals to create_time and is immutable and set by the Cloud Dataflow service."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "steps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Exactly one of step or steps_location should be specified. The top-level steps that constitute the entire job. Only retrieved with JOB_VIEW_ALL."]
        pub steps: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Step>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stepsLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GCS location where the steps are stored."]
        pub steps_location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tempFiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A set of files the system should be aware of that are used for temporary storage. These temporary files will be removed on job completion. No duplicates are allowed. No file patterns are supported. The supported files are: Google Cloud Storage: storage.googleapis.com/{bucket}/{object} bucket.storage.googleapis.com/{object}"]
        pub temp_files: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transformNameMapping")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The map of transform name prefixes of the job to be replaced to the corresponding name prefixes of the new job."]
        pub transform_name_mapping:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of Cloud Dataflow job."]
        pub _type: ::std::option::Option<JobTypeEnum>,
    }
    impl Job {
        pub fn builder() -> JobBuilder {
            JobBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The current state of the job. Jobs are created in the `JOB_STATE_STOPPED` state unless otherwise specified. A job in the `JOB_STATE_RUNNING` state may asynchronously enter a terminal state. After a job has reached a terminal state, no further state updates may be made. This field may be mutated by the Cloud Dataflow service; callers cannot mutate it."]
    pub enum JobCurrentStateEnum {
        #[serde(rename = "JOB_STATE_UNKNOWN")]
        #[doc = "The job's run state isn't specified."]
        JobStateUnknown,
        #[serde(rename = "JOB_STATE_STOPPED")]
        #[doc = "`JOB_STATE_STOPPED` indicates that the job has not yet started to run."]
        JobStateStopped,
        #[serde(rename = "JOB_STATE_RUNNING")]
        #[doc = "`JOB_STATE_RUNNING` indicates that the job is currently running."]
        JobStateRunning,
        #[serde(rename = "JOB_STATE_DONE")]
        #[doc = "`JOB_STATE_DONE` indicates that the job has successfully completed. This is a terminal job state. This state may be set by the Cloud Dataflow service, as a transition from `JOB_STATE_RUNNING`. It may also be set via a Cloud Dataflow `UpdateJob` call, if the job has not yet reached a terminal state."]
        JobStateDone,
        #[serde(rename = "JOB_STATE_FAILED")]
        #[doc = "`JOB_STATE_FAILED` indicates that the job has failed. This is a terminal job state. This state may only be set by the Cloud Dataflow service, and only as a transition from `JOB_STATE_RUNNING`."]
        JobStateFailed,
        #[serde(rename = "JOB_STATE_CANCELLED")]
        #[doc = "`JOB_STATE_CANCELLED` indicates that the job has been explicitly cancelled. This is a terminal job state. This state may only be set via a Cloud Dataflow `UpdateJob` call, and only if the job has not yet reached another terminal state."]
        JobStateCancelled,
        #[serde(rename = "JOB_STATE_UPDATED")]
        #[doc = "`JOB_STATE_UPDATED` indicates that the job was successfully updated, meaning that this job was stopped and another job was started, inheriting state from this one. This is a terminal job state. This state may only be set by the Cloud Dataflow service, and only as a transition from `JOB_STATE_RUNNING`."]
        JobStateUpdated,
        #[serde(rename = "JOB_STATE_DRAINING")]
        #[doc = "`JOB_STATE_DRAINING` indicates that the job is in the process of draining. A draining job has stopped pulling from its input sources and is processing any data that remains in-flight. This state may be set via a Cloud Dataflow `UpdateJob` call, but only as a transition from `JOB_STATE_RUNNING`. Jobs that are draining may only transition to `JOB_STATE_DRAINED`, `JOB_STATE_CANCELLED`, or `JOB_STATE_FAILED`."]
        JobStateDraining,
        #[serde(rename = "JOB_STATE_DRAINED")]
        #[doc = "`JOB_STATE_DRAINED` indicates that the job has been drained. A drained job terminated by stopping pulling from its input sources and processing any data that remained in-flight when draining was requested. This state is a terminal state, may only be set by the Cloud Dataflow service, and only as a transition from `JOB_STATE_DRAINING`."]
        JobStateDrained,
        #[serde(rename = "JOB_STATE_PENDING")]
        #[doc = "`JOB_STATE_PENDING` indicates that the job has been created but is not yet running. Jobs that are pending may only transition to `JOB_STATE_RUNNING`, or `JOB_STATE_FAILED`."]
        JobStatePending,
        #[serde(rename = "JOB_STATE_CANCELLING")]
        #[doc = "`JOB_STATE_CANCELLING` indicates that the job has been explicitly cancelled and is in the process of stopping. Jobs that are cancelling may only transition to `JOB_STATE_CANCELLED` or `JOB_STATE_FAILED`."]
        JobStateCancelling,
        #[serde(rename = "JOB_STATE_QUEUED")]
        #[doc = "`JOB_STATE_QUEUED` indicates that the job has been created but is being delayed until launch. Jobs that are queued may only transition to `JOB_STATE_PENDING` or `JOB_STATE_CANCELLED`."]
        JobStateQueued,
        #[serde(rename = "JOB_STATE_RESOURCE_CLEANING_UP")]
        #[doc = "`JOB_STATE_RESOURCE_CLEANING_UP` indicates that the batch job's associated resources are currently being cleaned up after a successful run. Currently, this is an opt-in feature, please reach out to Cloud support team if you are intersted."]
        JobStateResourceCleaningUp,
    }
    impl ::std::default::Default for JobCurrentStateEnum {
        fn default() -> Self {
            Self::JobStateUnknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The job's requested state. `UpdateJob` may be used to switch between the `JOB_STATE_STOPPED` and `JOB_STATE_RUNNING` states, by setting requested_state. `UpdateJob` may also be used to directly set a job's requested state to `JOB_STATE_CANCELLED` or `JOB_STATE_DONE`, irrevocably terminating the job if it has not already reached a terminal state."]
    pub enum JobRequestedStateEnum {
        #[serde(rename = "JOB_STATE_UNKNOWN")]
        #[doc = "The job's run state isn't specified."]
        JobStateUnknown,
        #[serde(rename = "JOB_STATE_STOPPED")]
        #[doc = "`JOB_STATE_STOPPED` indicates that the job has not yet started to run."]
        JobStateStopped,
        #[serde(rename = "JOB_STATE_RUNNING")]
        #[doc = "`JOB_STATE_RUNNING` indicates that the job is currently running."]
        JobStateRunning,
        #[serde(rename = "JOB_STATE_DONE")]
        #[doc = "`JOB_STATE_DONE` indicates that the job has successfully completed. This is a terminal job state. This state may be set by the Cloud Dataflow service, as a transition from `JOB_STATE_RUNNING`. It may also be set via a Cloud Dataflow `UpdateJob` call, if the job has not yet reached a terminal state."]
        JobStateDone,
        #[serde(rename = "JOB_STATE_FAILED")]
        #[doc = "`JOB_STATE_FAILED` indicates that the job has failed. This is a terminal job state. This state may only be set by the Cloud Dataflow service, and only as a transition from `JOB_STATE_RUNNING`."]
        JobStateFailed,
        #[serde(rename = "JOB_STATE_CANCELLED")]
        #[doc = "`JOB_STATE_CANCELLED` indicates that the job has been explicitly cancelled. This is a terminal job state. This state may only be set via a Cloud Dataflow `UpdateJob` call, and only if the job has not yet reached another terminal state."]
        JobStateCancelled,
        #[serde(rename = "JOB_STATE_UPDATED")]
        #[doc = "`JOB_STATE_UPDATED` indicates that the job was successfully updated, meaning that this job was stopped and another job was started, inheriting state from this one. This is a terminal job state. This state may only be set by the Cloud Dataflow service, and only as a transition from `JOB_STATE_RUNNING`."]
        JobStateUpdated,
        #[serde(rename = "JOB_STATE_DRAINING")]
        #[doc = "`JOB_STATE_DRAINING` indicates that the job is in the process of draining. A draining job has stopped pulling from its input sources and is processing any data that remains in-flight. This state may be set via a Cloud Dataflow `UpdateJob` call, but only as a transition from `JOB_STATE_RUNNING`. Jobs that are draining may only transition to `JOB_STATE_DRAINED`, `JOB_STATE_CANCELLED`, or `JOB_STATE_FAILED`."]
        JobStateDraining,
        #[serde(rename = "JOB_STATE_DRAINED")]
        #[doc = "`JOB_STATE_DRAINED` indicates that the job has been drained. A drained job terminated by stopping pulling from its input sources and processing any data that remained in-flight when draining was requested. This state is a terminal state, may only be set by the Cloud Dataflow service, and only as a transition from `JOB_STATE_DRAINING`."]
        JobStateDrained,
        #[serde(rename = "JOB_STATE_PENDING")]
        #[doc = "`JOB_STATE_PENDING` indicates that the job has been created but is not yet running. Jobs that are pending may only transition to `JOB_STATE_RUNNING`, or `JOB_STATE_FAILED`."]
        JobStatePending,
        #[serde(rename = "JOB_STATE_CANCELLING")]
        #[doc = "`JOB_STATE_CANCELLING` indicates that the job has been explicitly cancelled and is in the process of stopping. Jobs that are cancelling may only transition to `JOB_STATE_CANCELLED` or `JOB_STATE_FAILED`."]
        JobStateCancelling,
        #[serde(rename = "JOB_STATE_QUEUED")]
        #[doc = "`JOB_STATE_QUEUED` indicates that the job has been created but is being delayed until launch. Jobs that are queued may only transition to `JOB_STATE_PENDING` or `JOB_STATE_CANCELLED`."]
        JobStateQueued,
        #[serde(rename = "JOB_STATE_RESOURCE_CLEANING_UP")]
        #[doc = "`JOB_STATE_RESOURCE_CLEANING_UP` indicates that the batch job's associated resources are currently being cleaned up after a successful run. Currently, this is an opt-in feature, please reach out to Cloud support team if you are intersted."]
        JobStateResourceCleaningUp,
    }
    impl ::std::default::Default for JobRequestedStateEnum {
        fn default() -> Self {
            Self::JobStateUnknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of Cloud Dataflow job."]
    pub enum JobTypeEnum {
        #[serde(rename = "JOB_TYPE_UNKNOWN")]
        #[doc = "The type of the job is unspecified, or unknown."]
        JobTypeUnknown,
        #[serde(rename = "JOB_TYPE_BATCH")]
        #[doc = "A batch job with a well-defined end point: data is read, data is processed, data is written, and the job is done."]
        JobTypeBatch,
        #[serde(rename = "JOB_TYPE_STREAMING")]
        #[doc = "A continuously streaming job with no end: data is read, processed, and written continuously."]
        JobTypeStreaming,
    }
    impl ::std::default::Default for JobTypeEnum {
        fn default() -> Self {
            Self::JobTypeUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the execution of a job."]
    pub struct JobExecutionDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, this response does not contain all requested tasks. To obtain the next page of results, repeat the request with page_token set to this value."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stages of the job execution."]
        pub stages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StageSummary>>>,
    }
    impl JobExecutionDetails {
        pub fn builder() -> JobExecutionDetailsBuilder {
            JobExecutionDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional information about how a Cloud Dataflow job will be executed that isn't contained in the submitted job."]
    pub struct JobExecutionInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A mapping from each stage to the information about that stage."]
        pub stages: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<JobExecutionStageInfo>>,
        >,
    }
    impl JobExecutionInfo {
        pub fn builder() -> JobExecutionInfoBuilder {
            JobExecutionInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains information about how a particular google.dataflow.v1beta3.Step will be executed."]
    pub struct JobExecutionStageInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stepName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The steps associated with the execution stage. Note that stages may have several steps, and that a given step might be run by more than one stage."]
        pub step_name: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl JobExecutionStageInfo {
        pub fn builder() -> JobExecutionStageInfoBuilder {
            JobExecutionStageInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A particular message pertaining to a Dataflow job."]
    pub struct JobMessage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageImportance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Importance level of the message."]
        pub message_importance: ::std::option::Option<JobMessageMessageImportanceEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text of the message."]
        pub message_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "time")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp of the message."]
        pub time: ::std::option::Option<::std::string::String>,
    }
    impl JobMessage {
        pub fn builder() -> JobMessageBuilder {
            JobMessageBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Importance level of the message."]
    pub enum JobMessageMessageImportanceEnum {
        #[serde(rename = "JOB_MESSAGE_IMPORTANCE_UNKNOWN")]
        #[doc = "The message importance isn't specified, or is unknown."]
        JobMessageImportanceUnknown,
        #[serde(rename = "JOB_MESSAGE_DEBUG")]
        #[doc = "The message is at the 'debug' level: typically only useful for software engineers working on the code the job is running. Typically, Dataflow pipeline runners do not display log messages at this level by default."]
        JobMessageDebug,
        #[serde(rename = "JOB_MESSAGE_DETAILED")]
        #[doc = "The message is at the 'detailed' level: somewhat verbose, but potentially useful to users. Typically, Dataflow pipeline runners do not display log messages at this level by default. These messages are displayed by default in the Dataflow monitoring UI."]
        JobMessageDetailed,
        #[serde(rename = "JOB_MESSAGE_BASIC")]
        #[doc = "The message is at the 'basic' level: useful for keeping track of the execution of a Dataflow pipeline. Typically, Dataflow pipeline runners display log messages at this level by default, and these messages are displayed by default in the Dataflow monitoring UI."]
        JobMessageBasic,
        #[serde(rename = "JOB_MESSAGE_WARNING")]
        #[doc = "The message is at the 'warning' level: indicating a condition pertaining to a job which may require human intervention. Typically, Dataflow pipeline runners display log messages at this level by default, and these messages are displayed by default in the Dataflow monitoring UI."]
        JobMessageWarning,
        #[serde(rename = "JOB_MESSAGE_ERROR")]
        #[doc = "The message is at the 'error' level: indicating a condition preventing a job from succeeding. Typically, Dataflow pipeline runners display log messages at this level by default, and these messages are displayed by default in the Dataflow monitoring UI."]
        JobMessageError,
    }
    impl ::std::default::Default for JobMessageMessageImportanceEnum {
        fn default() -> Self {
            Self::JobMessageImportanceUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata available primarily for filtering jobs. Will be included in the ListJob response and Job SUMMARY view."]
    pub struct JobMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bigTableDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identification of a BigTable source used in the Dataflow job."]
        pub big_table_details:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BigTableIoDetails>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bigqueryDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identification of a BigQuery source used in the Dataflow job."]
        pub bigquery_details:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BigQueryIoDetails>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datastoreDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identification of a Datastore source used in the Dataflow job."]
        pub datastore_details:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DatastoreIoDetails>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identification of a File source used in the Dataflow job."]
        pub file_details: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FileIoDetails>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pubsubDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identification of a PubSub source used in the Dataflow job."]
        pub pubsub_details:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PubSubIoDetails>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sdkVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The SDK version used to run the job."]
        pub sdk_version: ::std::option::Option<::std::boxed::Box<SdkVersion>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spannerDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identification of a Spanner source used in the Dataflow job."]
        pub spanner_details:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SpannerIoDetails>>>,
    }
    impl JobMetadata {
        pub fn builder() -> JobMetadataBuilder {
            JobMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JobMetrics contains a collection of metrics describing the detailed progress of a Dataflow job. Metrics correspond to user-defined and system-defined metrics in the job. This resource captures only the most recent values of each metric; time-series data can be queried for them (under the same metric names) from Cloud Monitoring."]
    pub struct JobMetrics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp as of which metric values are current."]
        pub metric_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All metrics for this job."]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricUpdate>>>,
    }
    impl JobMetrics {
        pub fn builder() -> JobMetricsBuilder {
            JobMetricsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data disk assignment information for a specific key-range of a sharded computation. Currently we only support UTF-8 character splits to simplify encoding into JSON."]
    pub struct KeyRangeDataDiskAssignment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataDisk")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the data disk where data for this range is stored. This name is local to the Google Cloud Platform project and uniquely identifies the disk within that project, for example \"myproject-1014-104817-4c2-harness-0-disk-1\"."]
        pub data_disk: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "end")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end (exclusive) of the key range."]
        pub end: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "start")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start (inclusive) of the key range."]
        pub start: ::std::option::Option<::std::string::String>,
    }
    impl KeyRangeDataDiskAssignment {
        pub fn builder() -> KeyRangeDataDiskAssignmentBuilder {
            KeyRangeDataDiskAssignmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Location information for a specific key-range of a sharded computation. Currently we only support UTF-8 character splits to simplify encoding into JSON."]
    pub struct KeyRangeLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataDisk")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the data disk where data for this range is stored. This name is local to the Google Cloud Platform project and uniquely identifies the disk within that project, for example \"myproject-1014-104817-4c2-harness-0-disk-1\"."]
        pub data_disk: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deliveryEndpoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The physical location of this range assignment to be used for streaming computation cross-worker message delivery."]
        pub delivery_endpoint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deprecatedPersistentDirectory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED. The location of the persistent state for this range, as a persistent directory in the worker local filesystem."]
        pub deprecated_persistent_directory: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "end")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end (exclusive) of the key range."]
        pub end: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "start")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start (inclusive) of the key range."]
        pub start: ::std::option::Option<::std::string::String>,
    }
    impl KeyRangeLocation {
        pub fn builder() -> KeyRangeLocationBuilder {
            KeyRangeLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Launch FlexTemplate Parameter."]
    pub struct LaunchFlexTemplateParameter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Spec about the container image to launch."]
        pub container_spec: ::std::option::Option<::std::boxed::Box<ContainerSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerSpecGcsPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Gcs path to a file with json serialized ContainerSpec as content."]
        pub container_spec_gcs_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The runtime environment for the FlexTemplate job"]
        pub environment: ::std::option::Option<::std::boxed::Box<FlexTemplateRuntimeEnvironment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The job name to use for the created job. For update job request, job name should be same as the existing running job."]
        pub job_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "launchOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Launch options for this flex template job. This is a common set of options across languages and templates. This should not be used to pass job parameters."]
        pub launch_options:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The parameters for FlexTemplate. Ex. {\"num_workers\":\"5\"}"]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transformNameMappings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Use this to pass transform_name_mappings for streaming update jobs. Ex:{\"oldTransformName\":\"newTransformName\",...}'"]
        pub transform_name_mappings:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "update")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set this to true if you are sending a request to update a running streaming job. When set, the job name should be the same as the running job."]
        pub update: ::std::option::Option<::std::primitive::bool>,
    }
    impl LaunchFlexTemplateParameter {
        pub fn builder() -> LaunchFlexTemplateParameterBuilder {
            LaunchFlexTemplateParameterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to launch a Cloud Dataflow job from a FlexTemplate."]
    pub struct LaunchFlexTemplateRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "launchParameter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Parameter to launch a job form Flex Template."]
        pub launch_parameter: ::std::option::Option<::std::boxed::Box<LaunchFlexTemplateParameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validateOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, the request is validated but not actually executed. Defaults to false."]
        pub validate_only: ::std::option::Option<::std::primitive::bool>,
    }
    impl LaunchFlexTemplateRequest {
        pub fn builder() -> LaunchFlexTemplateRequestBuilder {
            LaunchFlexTemplateRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to the request to launch a job from Flex Template."]
    pub struct LaunchFlexTemplateResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "job")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The job that was launched, if the request was not a dry run and the job was successfully launched."]
        pub job: ::std::option::Option<::std::boxed::Box<Job>>,
    }
    impl LaunchFlexTemplateResponse {
        pub fn builder() -> LaunchFlexTemplateResponseBuilder {
            LaunchFlexTemplateResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters to provide to the template being launched."]
    pub struct LaunchTemplateParameters {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The runtime environment for the job."]
        pub environment: ::std::option::Option<::std::boxed::Box<RuntimeEnvironment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The job name to use for the created job."]
        pub job_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The runtime parameters to pass to the job."]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transformNameMapping")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only applicable when updating a pipeline. Map of transform name prefixes of the job to be replaced to the corresponding name prefixes of the new job."]
        pub transform_name_mapping:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "update")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, replace the existing pipeline with the name specified by jobName with this pipeline, preserving state."]
        pub update: ::std::option::Option<::std::primitive::bool>,
    }
    impl LaunchTemplateParameters {
        pub fn builder() -> LaunchTemplateParametersBuilder {
            LaunchTemplateParametersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to the request to launch a template."]
    pub struct LaunchTemplateResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "job")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The job that was launched, if the request was not a dry run and the job was successfully launched."]
        pub job: ::std::option::Option<::std::boxed::Box<Job>>,
    }
    impl LaunchTemplateResponse {
        pub fn builder() -> LaunchTemplateResponseBuilder {
            LaunchTemplateResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to lease WorkItems."]
    pub struct LeaseWorkItemRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentWorkerTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current timestamp at the worker."]
        pub current_worker_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains the WorkItem's job."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestedLeaseDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The initial lease period."]
        pub requested_lease_duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unifiedWorkerRequest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Untranslated bag-of-bytes WorkRequest from UnifiedWorker."]
        pub unified_worker_request:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workItemTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Filter for WorkItem type."]
        pub work_item_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerCapabilities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Worker capabilities. WorkItems might be limited to workers with specific capabilities."]
        pub worker_capabilities: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies the worker leasing work -- typically the ID of the virtual machine running the worker."]
        pub worker_id: ::std::option::Option<::std::string::String>,
    }
    impl LeaseWorkItemRequest {
        pub fn builder() -> LeaseWorkItemRequestBuilder {
            LeaseWorkItemRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to a request to lease WorkItems."]
    pub struct LeaseWorkItemResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unifiedWorkerResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Untranslated bag-of-bytes WorkResponse for UnifiedWorker."]
        pub unified_worker_response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of the leased WorkItems."]
        pub work_items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WorkItem>>>,
    }
    impl LeaseWorkItemResponse {
        pub fn builder() -> LeaseWorkItemResponseBuilder {
            LeaseWorkItemResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to a request to list job messages."]
    pub struct ListJobMessagesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoscalingEvents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Autoscaling events in ascending timestamp order."]
        pub autoscaling_events:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AutoscalingEvent>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobMessages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Messages in ascending timestamp order."]
        pub job_messages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<JobMessage>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token to obtain the next page of results if there are more."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListJobMessagesResponse {
        pub fn builder() -> ListJobMessagesResponseBuilder {
            ListJobMessagesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to a request to list Cloud Dataflow jobs in a project. This might be a partial response, depending on the page size in the ListJobsRequest. However, if the project does not have any jobs, an instance of ListJobsResponse is not returned and the requests's response body is empty {}."]
    pub struct ListJobsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failedLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Zero or more messages describing the [regional endpoints] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that failed to respond."]
        pub failed_location:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FailedLocation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A subset of the requested job information."]
        pub jobs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Job>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set if there may be more results than fit in this response."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListJobsResponse {
        pub fn builder() -> ListJobsResponseBuilder {
            ListJobsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "List of snapshots."]
    pub struct ListSnapshotsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snapshots")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Returned snapshots."]
        pub snapshots: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Snapshot>>>,
    }
    impl ListSnapshotsResponse {
        pub fn builder() -> ListSnapshotsResponseBuilder {
            ListSnapshotsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Respond a list of TemplateVersions."]
    pub struct ListTemplateVersionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "templateVersions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of TemplateVersions."]
        pub template_versions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TemplateVersion>>>,
    }
    impl ListTemplateVersionsResponse {
        pub fn builder() -> ListTemplateVersionsResponseBuilder {
            ListTemplateVersionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "MapTask consists of an ordered set of instructions, each of which describes one particular low-level operation for the worker to perform in order to accomplish the MapTask's WorkItem. Each instruction must appear in the list before any instructions which depends on its output."]
    pub struct MapTask {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "counterPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Counter prefix that can be used to prefix counters. Not currently used in Dataflow."]
        pub counter_prefix: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instructions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The instructions in the MapTask."]
        pub instructions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ParallelInstruction>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System-defined name of the stage containing this MapTask. Unique across the workflow."]
        pub stage_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System-defined name of this MapTask. Unique across the workflow."]
        pub system_name: ::std::option::Option<::std::string::String>,
    }
    impl MapTask {
        pub fn builder() -> MapTaskBuilder {
            MapTaskBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the memory usage of a worker or a container within a worker."]
    pub struct MemInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentLimitBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instantenous memory limit in bytes."]
        pub current_limit_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentRssBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instantenous memory (RSS) size in bytes."]
        pub current_rss_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of the measurement."]
        pub timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalGbMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total memory (RSS) usage since start up in GB * ms."]
        pub total_gb_ms: ::std::option::Option<::std::string::String>,
    }
    impl MemInfo {
        pub fn builder() -> MemInfoBuilder {
            MemInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The metric short id is returned to the user alongside an offset into ReportWorkItemStatusRequest"]
    pub struct MetricShortId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the corresponding metric in the ReportWorkItemStatusRequest. Required."]
        pub metric_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shortId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The service-generated short identifier for the metric."]
        pub short_id: ::std::option::Option<::std::string::String>,
    }
    impl MetricShortId {
        pub fn builder() -> MetricShortIdBuilder {
            MetricShortIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identifies a metric, by describing the source which generated the metric."]
    pub struct MetricStructuredName {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Zero or more labeled fields which identify the part of the job this metric is associated with, such as the name of a step or collection. For example, built-in counters associated with steps will have context['step'] = . Counters associated with PCollections in the SDK will have context['pcollection'] = ."]
        pub context:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Worker-defined metric name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "origin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Origin (namespace) of metric name. May be blank for user-define metrics; will be \"dataflow\" for metrics defined by the Dataflow service or SDK."]
        pub origin: ::std::option::Option<::std::string::String>,
    }
    impl MetricStructuredName {
        pub fn builder() -> MetricStructuredNameBuilder {
            MetricStructuredNameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes the state of a metric."]
    pub struct MetricUpdate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cumulative")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if this metric is reported as the total cumulative aggregate value accumulated since the worker started working on this WorkItem. By default this is false, indicating that this metric is reported as a delta that is not associated with any WorkItem."]
        pub cumulative: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "distribution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A struct value describing properties of a distribution of numeric values."]
        pub distribution: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gauge")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A struct value describing properties of a Gauge. Metrics of gauge type show the value of a metric across time, and is aggregated based on the newest value."]
        pub gauge: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "internal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Worker-computed aggregate value for internal use by the Dataflow service."]
        pub internal: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metric aggregation kind. The possible metric aggregation kinds are \"Sum\", \"Max\", \"Min\", \"Mean\", \"Set\", \"And\", \"Or\", and \"Distribution\". The specified aggregation kind is case-insensitive. If omitted, this is not an aggregated value but instead a single metric sample value."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meanCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Worker-computed aggregate value for the \"Mean\" aggregation kind. This holds the count of the aggregated values and is used in combination with mean_sum above to obtain the actual mean aggregate value. The only possible value type is Long."]
        pub mean_count: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meanSum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Worker-computed aggregate value for the \"Mean\" aggregation kind. This holds the sum of the aggregated values and is used in combination with mean_count below to obtain the actual mean aggregate value. The only possible value types are Long and Double."]
        pub mean_sum: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the metric."]
        pub name: ::std::option::Option<::std::boxed::Box<MetricStructuredName>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scalar")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Worker-computed aggregate value for aggregation kinds \"Sum\", \"Max\", \"Min\", \"And\", and \"Or\". The possible value types are Long, Double, and Boolean."]
        pub scalar: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "set")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Worker-computed aggregate value for the \"Set\" aggregation kind. The only possible value type is a list of Values whose type can be Long, Double, or String, according to the metric's type. All Values in the list must be of the same type."]
        pub set: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp associated with the metric value. Optional when workers are reporting work progress; it will be filled in responses from the metrics API."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl MetricUpdate {
        pub fn builder() -> MetricUpdateBuilder {
            MetricUpdateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Either add the label to TemplateVersion or remove it from the TemplateVersion."]
    pub struct ModifyTemplateVersionLabelRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The label key for update."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "op")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Requests for add label to TemplateVersion or remove label from TemplateVersion."]
        pub op: ::std::option::Option<ModifyTemplateVersionLabelRequestOpEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The label value for update."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl ModifyTemplateVersionLabelRequest {
        pub fn builder() -> ModifyTemplateVersionLabelRequestBuilder {
            ModifyTemplateVersionLabelRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Requests for add label to TemplateVersion or remove label from TemplateVersion."]
    pub enum ModifyTemplateVersionLabelRequestOpEnum {
        #[serde(rename = "OPERATION_UNSPECIFIED")]
        #[doc = "Default value."]
        OperationUnspecified,
        #[serde(rename = "ADD")]
        #[doc = "Add the label to the TemplateVersion object."]
        Add,
        #[serde(rename = "REMOVE")]
        #[doc = "Remove the label from the TemplateVersion object."]
        Remove,
    }
    impl ::std::default::Default for ModifyTemplateVersionLabelRequestOpEnum {
        fn default() -> Self {
            Self::OperationUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Respond the labels in the TemplateVersion."]
    pub struct ModifyTemplateVersionLabelResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All the label in the TemplateVersion."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl ModifyTemplateVersionLabelResponse {
        pub fn builder() -> ModifyTemplateVersionLabelResponseBuilder {
            ModifyTemplateVersionLabelResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Add a tag to the current TemplateVersion. If tag exist in another TemplateVersion in the Template, remove the tag before add it to the current TemplateVersion. If remove_only set, remove the tag from the current TemplateVersion."]
    pub struct ModifyTemplateVersionTagRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "removeOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The flag that indicates if the request is only for remove tag from TemplateVersion."]
        pub remove_only: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tag for update."]
        pub tag: ::std::option::Option<::std::string::String>,
    }
    impl ModifyTemplateVersionTagRequest {
        pub fn builder() -> ModifyTemplateVersionTagRequestBuilder {
            ModifyTemplateVersionTagRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Respond the current tags in the TemplateVersion."]
    pub struct ModifyTemplateVersionTagResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All the tags in the TemplateVersion."]
        pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ModifyTemplateVersionTagResponse {
        pub fn builder() -> ModifyTemplateVersionTagResponseBuilder {
            ModifyTemplateVersionTagResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes mounted data disk."]
    pub struct MountedDataDisk {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataDisk")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the data disk. This name is local to the Google Cloud Platform project and uniquely identifies the disk within that project, for example \"myproject-1014-104817-4c2-harness-0-disk-1\"."]
        pub data_disk: ::std::option::Option<::std::string::String>,
    }
    impl MountedDataDisk {
        pub fn builder() -> MountedDataDiskBuilder {
            MountedDataDiskBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about an output of a multi-output DoFn."]
    pub struct MultiOutputInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id of the tag the user code will emit to this output by; this should correspond to the tag of some SideInputInfo."]
        pub tag: ::std::option::Option<::std::string::String>,
    }
    impl MultiOutputInfo {
        pub fn builder() -> MultiOutputInfoBuilder {
            MultiOutputInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic metadata about a counter."]
    pub struct NameAndKind {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Counter aggregation kind."]
        pub kind: ::std::option::Option<NameAndKindKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the counter."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl NameAndKind {
        pub fn builder() -> NameAndKindBuilder {
            NameAndKindBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Counter aggregation kind."]
    pub enum NameAndKindKindEnum {
        #[serde(rename = "INVALID")]
        #[doc = "Counter aggregation kind was not set."]
        Invalid,
        #[serde(rename = "SUM")]
        #[doc = "Aggregated value is the sum of all contributed values."]
        Sum,
        #[serde(rename = "MAX")]
        #[doc = "Aggregated value is the max of all contributed values."]
        Max,
        #[serde(rename = "MIN")]
        #[doc = "Aggregated value is the min of all contributed values."]
        Min,
        #[serde(rename = "MEAN")]
        #[doc = "Aggregated value is the mean of all contributed values."]
        Mean,
        #[serde(rename = "OR")]
        #[doc = "Aggregated value represents the logical 'or' of all contributed values."]
        Or,
        #[serde(rename = "AND")]
        #[doc = "Aggregated value represents the logical 'and' of all contributed values."]
        And,
        #[serde(rename = "SET")]
        #[doc = "Aggregated value is a set of unique contributed values."]
        Set,
        #[serde(rename = "DISTRIBUTION")]
        #[doc = "Aggregated value captures statistics about a distribution."]
        Distribution,
        #[serde(rename = "LATEST_VALUE")]
        #[doc = "Aggregated value tracks the latest value of a variable."]
        LatestValue,
    }
    impl ::std::default::Default for NameAndKindKindEnum {
        fn default() -> Self {
            Self::Invalid
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The packages that must be installed in order for a worker to run the steps of the Cloud Dataflow job that will be assigned to its worker pool. This is the mechanism by which the Cloud Dataflow SDK causes code to be loaded onto the workers. For example, the Cloud Dataflow Java SDK might use this to install jars containing the user's code and all of the various dependencies (libraries, data files, etc.) required in order for that code to run."]
    pub struct Package {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource to read the package from. The supported resource type is: Google Cloud Storage: storage.googleapis.com/{bucket} bucket.storage.googleapis.com/"]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the package."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Package {
        pub fn builder() -> PackageBuilder {
            PackageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An instruction that does a ParDo operation. Takes one main input and zero or more side inputs, and produces zero or more outputs. Runs user code."]
    pub struct ParDoInstruction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "input")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The input."]
        pub input: ::std::option::Option<::std::boxed::Box<InstructionInput>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "multiOutputInfos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about each of the outputs, if user_fn is a MultiDoFn."]
        pub multi_output_infos:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MultiOutputInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numOutputs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of outputs."]
        pub num_outputs: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sideInputs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Zero or more side inputs."]
        pub side_inputs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SideInputInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userFn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user function to invoke."]
        pub user_fn:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ParDoInstruction {
        pub fn builder() -> ParDoInstructionBuilder {
            ParDoInstructionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a particular operation comprising a MapTask."]
    pub struct ParallelInstruction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "flatten")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information for Flatten instructions."]
        pub flatten: ::std::option::Option<::std::boxed::Box<FlattenInstruction>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-provided name of this operation."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System-defined name for the operation in the original workflow graph."]
        pub original_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes the outputs of the instruction."]
        pub outputs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InstructionOutput>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parDo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information for ParDo instructions."]
        pub par_do: ::std::option::Option<::std::boxed::Box<ParDoInstruction>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialGroupByKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information for PartialGroupByKey instructions."]
        pub partial_group_by_key:
            ::std::option::Option<::std::boxed::Box<PartialGroupByKeyInstruction>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "read")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information for Read instructions."]
        pub read: ::std::option::Option<::std::boxed::Box<ReadInstruction>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System-defined name of this operation. Unique across the workflow."]
        pub system_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "write")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information for Write instructions."]
        pub write: ::std::option::Option<::std::boxed::Box<WriteInstruction>>,
    }
    impl ParallelInstruction {
        pub fn builder() -> ParallelInstructionBuilder {
            ParallelInstructionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Structured data associated with this message."]
    pub struct Parameter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key or name for this parameter."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value for this parameter."]
        pub value: ::std::option::Option<::serde_json::Value>,
    }
    impl Parameter {
        pub fn builder() -> ParameterBuilder {
            ParameterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for a specific parameter."]
    pub struct ParameterMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "helpText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The help text to display for the parameter."]
        pub help_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isOptional")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Whether the parameter is optional. Defaults to false."]
        pub is_optional: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The label to display for the parameter."]
        pub label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the parameter."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paramType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The type of the parameter. Used for selecting input picker."]
        pub param_type: ::std::option::Option<ParameterMetadataParamTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regexes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Regexes that the parameter must match."]
        pub regexes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ParameterMetadata {
        pub fn builder() -> ParameterMetadataBuilder {
            ParameterMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The type of the parameter. Used for selecting input picker."]
    pub enum ParameterMetadataParamTypeEnum {
        #[serde(rename = "DEFAULT")]
        #[doc = "Default input type."]
        Default,
        #[serde(rename = "TEXT")]
        #[doc = "The parameter specifies generic text input."]
        Text,
        #[serde(rename = "GCS_READ_BUCKET")]
        #[doc = "The parameter specifies a GCS Bucket to read from."]
        GcsReadBucket,
        #[serde(rename = "GCS_WRITE_BUCKET")]
        #[doc = "The parameter specifies a GCS Bucket to write to."]
        GcsWriteBucket,
        #[serde(rename = "GCS_READ_FILE")]
        #[doc = "The parameter specifies a GCS file path to read from."]
        GcsReadFile,
        #[serde(rename = "GCS_WRITE_FILE")]
        #[doc = "The parameter specifies a GCS file path to write to."]
        GcsWriteFile,
        #[serde(rename = "GCS_READ_FOLDER")]
        #[doc = "The parameter specifies a GCS folder path to read from."]
        GcsReadFolder,
        #[serde(rename = "GCS_WRITE_FOLDER")]
        #[doc = "The parameter specifies a GCS folder to write to."]
        GcsWriteFolder,
        #[serde(rename = "PUBSUB_TOPIC")]
        #[doc = "The parameter specifies a Pub/Sub Topic."]
        PubsubTopic,
        #[serde(rename = "PUBSUB_SUBSCRIPTION")]
        #[doc = "The parameter specifies a Pub/Sub Subscription."]
        PubsubSubscription,
    }
    impl ::std::default::Default for ParameterMetadataParamTypeEnum {
        fn default() -> Self {
            Self::Default
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An instruction that does a partial group-by-key. One input and one output."]
    pub struct PartialGroupByKeyInstruction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "input")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes the input to the partial group-by-key instruction."]
        pub input: ::std::option::Option<::std::boxed::Box<InstructionInput>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputElementCodec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The codec to use for interpreting an element in the input PTable."]
        pub input_element_codec:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalCombineValuesInputStoreName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this instruction includes a combining function this is the name of the intermediate store between the GBK and the CombineValues."]
        pub original_combine_values_input_store_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalCombineValuesStepName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this instruction includes a combining function, this is the name of the CombineValues instruction lifted into this instruction."]
        pub original_combine_values_step_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sideInputs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Zero or more side inputs."]
        pub side_inputs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SideInputInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueCombiningFn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value combining function to invoke."]
        pub value_combining_fn:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl PartialGroupByKeyInstruction {
        pub fn builder() -> PartialGroupByKeyInstructionBuilder {
            PartialGroupByKeyInstructionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A descriptive representation of submitted pipeline as well as the executed form. This data is provided by the Dataflow service for ease of visualizing the pipeline and interpreting Dataflow provided metrics."]
    pub struct PipelineDescription {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pipeline level display data."]
        pub display_data: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DisplayData>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionPipelineStage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of each stage of execution of the pipeline."]
        pub execution_pipeline_stage:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ExecutionStageSummary>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalPipelineTransform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of each transform in the pipeline and collections between them."]
        pub original_pipeline_transform:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TransformSummary>>>,
    }
    impl PipelineDescription {
        pub fn builder() -> PipelineDescriptionBuilder {
            PipelineDescriptionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A point in the timeseries."]
    pub struct Point {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "time")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp of the point."]
        pub time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of the point."]
        pub value: ::std::option::Option<::std::primitive::f64>,
    }
    impl Point {
        pub fn builder() -> PointBuilder {
            PointBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Position defines a position within a collection of data. The value can be either the end position, a key (used with ordered collections), a byte offset, or a record index."]
    pub struct Position {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "byteOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Position is a byte offset."]
        pub byte_offset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "concatPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "CloudPosition is a concat position."]
        pub concat_position: ::std::option::Option<::std::boxed::Box<ConcatPosition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "end")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Position is past all other positions. Also useful for the end position of an unbounded range."]
        pub end: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Position is a string key, ordered lexicographically."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recordIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Position is a record index."]
        pub record_index: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shufflePosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "CloudPosition is a base64 encoded BatchShufflePosition (with FIXED sharding)."]
        pub shuffle_position: ::std::option::Option<::std::string::String>,
    }
    impl Position {
        pub fn builder() -> PositionBuilder {
            PositionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the progress of some component of job execution."]
    pub struct ProgressTimeseries {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentProgress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current progress of the component, in the range [0,1]."]
        pub current_progress: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataPoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "History of progress for the component. Points are sorted by time."]
        pub data_points: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Point>>>,
    }
    impl ProgressTimeseries {
        pub fn builder() -> ProgressTimeseriesBuilder {
            ProgressTimeseriesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for a PubSub connector used by the job."]
    pub struct PubSubIoDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subscription used in the connection."]
        pub subscription: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Topic accessed in the connection."]
        pub topic: ::std::option::Option<::std::string::String>,
    }
    impl PubSubIoDetails {
        pub fn builder() -> PubSubIoDetailsBuilder {
            PubSubIoDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identifies a pubsub location to use for transferring data into or out of a streaming Dataflow job."]
    pub struct PubsubLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dropLateData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the pipeline allows late-arriving data."]
        pub drop_late_data: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, contains a pubsub label from which to extract record ids. If left empty, record deduplication will be strictly best effort."]
        pub id_label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A pubsub subscription, in the form of \"pubsub.googleapis.com/subscriptions//\""]
        pub subscription: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestampLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, contains a pubsub label from which to extract record timestamps. If left empty, record timestamps will be generated upon arrival."]
        pub timestamp_label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A pubsub topic, in the form of \"pubsub.googleapis.com/topics//\""]
        pub topic: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackingSubscription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, specifies the pubsub subscription that will be used for tracking custom time timestamps for watermark estimation."]
        pub tracking_subscription: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "withAttributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, then the client has requested to get pubsub attributes."]
        pub with_attributes: ::std::option::Option<::std::primitive::bool>,
    }
    impl PubsubLocation {
        pub fn builder() -> PubsubLocationBuilder {
            PubsubLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Pubsub snapshot."]
    pub struct PubsubSnapshotMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The expire time of the Pubsub snapshot."]
        pub expire_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snapshotName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the Pubsub snapshot."]
        pub snapshot_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the Pubsub topic."]
        pub topic_name: ::std::option::Option<::std::string::String>,
    }
    impl PubsubSnapshotMetadata {
        pub fn builder() -> PubsubSnapshotMetadataBuilder {
            PubsubSnapshotMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a validated query."]
    pub struct QueryInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryProperty")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Includes an entry for each satisfied QueryProperty."]
        pub query_property: ::std::option::Option<::std::vec::Vec<QueryInfoQueryPropertyEnum>>,
    }
    impl QueryInfo {
        pub fn builder() -> QueryInfoBuilder {
            QueryInfoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum QueryInfoQueryPropertyEnum {
        #[serde(rename = "QUERY_PROPERTY_UNSPECIFIED")]
        #[doc = "The query property is unknown or unspecified."]
        QueryPropertyUnspecified,
        #[serde(rename = "HAS_UNBOUNDED_SOURCE")]
        #[doc = "Indicates this query reads from >= 1 unbounded source."]
        HasUnboundedSource,
    }
    impl ::std::default::Default for QueryInfoQueryPropertyEnum {
        fn default() -> Self {
            Self::QueryPropertyUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An instruction that reads records. Takes no inputs, produces one output."]
    pub struct ReadInstruction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source to read from."]
        pub source: ::std::option::Option<::std::boxed::Box<Source>>,
    }
    impl ReadInstruction {
        pub fn builder() -> ReadInstructionBuilder {
            ReadInstructionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to report the status of WorkItems."]
    pub struct ReportWorkItemStatusRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentWorkerTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current timestamp at the worker."]
        pub current_worker_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains the WorkItem's job."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unifiedWorkerRequest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Untranslated bag-of-bytes WorkProgressUpdateRequest from UnifiedWorker."]
        pub unified_worker_request:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workItemStatuses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The order is unimportant, except that the order of the WorkItemServiceState messages in the ReportWorkItemStatusResponse corresponds to the order of WorkItemStatus messages here."]
        pub work_item_statuses:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WorkItemStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the worker reporting the WorkItem status. If this does not match the ID of the worker which the Dataflow service believes currently has the lease on the WorkItem, the report will be dropped (with an error response)."]
        pub worker_id: ::std::option::Option<::std::string::String>,
    }
    impl ReportWorkItemStatusRequest {
        pub fn builder() -> ReportWorkItemStatusRequestBuilder {
            ReportWorkItemStatusRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response from a request to report the status of WorkItems."]
    pub struct ReportWorkItemStatusResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unifiedWorkerResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Untranslated bag-of-bytes WorkProgressUpdateResponse for UnifiedWorker."]
        pub unified_worker_response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workItemServiceStates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A set of messages indicating the service-side state for each WorkItem whose status was reported, in the same order as the WorkItemStatus messages in the ReportWorkItemStatusRequest which resulting in this response."]
        pub work_item_service_states:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WorkItemServiceState>>>,
    }
    impl ReportWorkItemStatusResponse {
        pub fn builder() -> ReportWorkItemStatusResponseBuilder {
            ReportWorkItemStatusResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the level of parallelism in a WorkItem's input, reported by the worker."]
    pub struct ReportedParallelism {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isInfinite")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies whether the parallelism is infinite. If true, \"value\" is ignored. Infinite parallelism means the service will assume that the work item can always be split into more non-empty work items by dynamic splitting. This is a work-around for lack of support for infinity by the current JSON-based Java RPC stack."]
        pub is_infinite: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the level of parallelism in case it is finite."]
        pub value: ::std::option::Option<::std::primitive::f64>,
    }
    impl ReportedParallelism {
        pub fn builder() -> ReportedParallelismBuilder {
            ReportedParallelismBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Worker metrics exported from workers. This contains resource utilization metrics accumulated from a variety of sources. For more information, see go/df-resource-signals."]
    pub struct ResourceUtilizationReport {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Per container information. Key: container name."]
        pub containers: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<ResourceUtilizationReport>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "CPU utilization samples."]
        pub cpu_time: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CpuTime>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memoryInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Memory utilization samples."]
        pub memory_info: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MemInfo>>>,
    }
    impl ResourceUtilizationReport {
        pub fn builder() -> ResourceUtilizationReportBuilder {
            ResourceUtilizationReportBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Service-side response to WorkerMessage reporting resource utilization."]
    pub struct ResourceUtilizationReportResponse {}
    impl ResourceUtilizationReportResponse {
        pub fn builder() -> ResourceUtilizationReportResponseBuilder {
            ResourceUtilizationReportResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The environment values to set at runtime."]
    pub struct RuntimeEnvironment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalExperiments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional experiment flags for the job."]
        pub additional_experiments: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalUserLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional user labels to be specified for the job. Keys and values should follow the restrictions specified in the [labeling restrictions](https://cloud.google.com/compute/docs/labeling-resources#restrictions) page. An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1kg\", \"count\": \"3\" }."]
        pub additional_user_labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bypassTempDirValidation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to bypass the safety checks for the job's temporary directory. Use with caution."]
        pub bypass_temp_dir_validation: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableStreamingEngine")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to enable Streaming Engine for the job."]
        pub enable_streaming_engine: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for VM IPs."]
        pub ip_configuration: ::std::option::Option<RuntimeEnvironmentIpConfigurationEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmsKeyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name for the Cloud KMS key for the job. Key format is: projects//locations//keyRings//cryptoKeys/"]
        pub kms_key_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "machineType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The machine type to use for the job. Defaults to the value from the template if not specified."]
        pub machine_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxWorkers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of Google Compute Engine instances to be made available to your pipeline during execution, from 1 to 1000."]
        pub max_workers: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Network to which VMs will be assigned. If empty or unspecified, the service will use the network \"default\"."]
        pub network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numWorkers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The initial number of Google Compute Engine instnaces for the job."]
        pub num_workers: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceAccountEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of the service account to run the job as."]
        pub service_account_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subnetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subnetwork to which VMs will be assigned, if desired. You can specify a subnetwork using either a complete URL or an abbreviated path. Expected to be of the form \"https://www.googleapis.com/compute/v1/projects/HOST_PROJECT_ID/regions/REGION/subnetworks/SUBNETWORK\" or \"regions/REGION/subnetworks/SUBNETWORK\". If the subnetwork is located in a Shared VPC network, you must use the complete URL."]
        pub subnetwork: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tempLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud Storage path to use for temporary files. Must be a valid Cloud Storage URL, beginning with `gs://`."]
        pub temp_location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerRegion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Compute Engine region (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1\". Mutually exclusive with worker_zone. If neither worker_region nor worker_zone is specified, default to the control plane's region."]
        pub worker_region: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Compute Engine zone (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1-a\". Mutually exclusive with worker_region. If neither worker_region nor worker_zone is specified, a zone in the control plane's region is chosen based on available capacity. If both `worker_zone` and `zone` are set, `worker_zone` takes precedence."]
        pub worker_zone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Compute Engine [availability zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones) for launching worker instances to run your pipeline. In the future, worker_zone will take precedence."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl RuntimeEnvironment {
        pub fn builder() -> RuntimeEnvironmentBuilder {
            RuntimeEnvironmentBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Configuration for VM IPs."]
    pub enum RuntimeEnvironmentIpConfigurationEnum {
        #[serde(rename = "WORKER_IP_UNSPECIFIED")]
        #[doc = "The configuration is unknown, or unspecified."]
        WorkerIpUnspecified,
        #[serde(rename = "WORKER_IP_PUBLIC")]
        #[doc = "Workers should have public IP addresses."]
        WorkerIpPublic,
        #[serde(rename = "WORKER_IP_PRIVATE")]
        #[doc = "Workers should have private IP addresses."]
        WorkerIpPrivate,
    }
    impl ::std::default::Default for RuntimeEnvironmentIpConfigurationEnum {
        fn default() -> Self {
            Self::WorkerIpUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "RuntimeMetadata describing a runtime environment."]
    pub struct RuntimeMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The parameters for the template."]
        pub parameters:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ParameterMetadata>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sdkInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "SDK Info for the template."]
        pub sdk_info: ::std::option::Option<::std::boxed::Box<SdkInfo>>,
    }
    impl RuntimeMetadata {
        pub fn builder() -> RuntimeMetadataBuilder {
            RuntimeMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SDK Information."]
    pub struct SdkInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The SDK Language."]
        pub language: ::std::option::Option<SdkInfoLanguageEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The SDK version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl SdkInfo {
        pub fn builder() -> SdkInfoBuilder {
            SdkInfoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The SDK Language."]
    pub enum SdkInfoLanguageEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "UNKNOWN Language."]
        Unknown,
        #[serde(rename = "JAVA")]
        #[doc = "Java."]
        Java,
        #[serde(rename = "PYTHON")]
        #[doc = "Python."]
        Python,
    }
    impl ::std::default::Default for SdkInfoLanguageEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines a SDK harness container for executing Dataflow pipelines."]
    pub struct SdkHarnessContainerImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A docker container image that resides in Google Container Registry."]
        pub container_image: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environmentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Environment ID for the Beam runner API proto Environment that corresponds to the current SDK Harness."]
        pub environment_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useSingleCorePerContainer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, recommends the Dataflow service to use only one core per SDK container instance with this image. If false (or unset) recommends using more than one core per SDK container instance with this image for efficiency. Note that Dataflow service may choose to override this property if needed."]
        pub use_single_core_per_container: ::std::option::Option<::std::primitive::bool>,
    }
    impl SdkHarnessContainerImage {
        pub fn builder() -> SdkHarnessContainerImageBuilder {
            SdkHarnessContainerImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The version of the SDK used to run the job."]
    pub struct SdkVersion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sdkSupportStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The support status for this SDK version."]
        pub sdk_support_status: ::std::option::Option<SdkVersionSdkSupportStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the SDK used to run the job."]
        pub version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionDisplayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A readable string describing the version of the SDK."]
        pub version_display_name: ::std::option::Option<::std::string::String>,
    }
    impl SdkVersion {
        pub fn builder() -> SdkVersionBuilder {
            SdkVersionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The support status for this SDK version."]
    pub enum SdkVersionSdkSupportStatusEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Cloud Dataflow is unaware of this version."]
        Unknown,
        #[serde(rename = "SUPPORTED")]
        #[doc = "This is a known version of an SDK, and is supported."]
        Supported,
        #[serde(rename = "STALE")]
        #[doc = "A newer version of the SDK family exists, and an update is recommended."]
        Stale,
        #[serde(rename = "DEPRECATED")]
        #[doc = "This version of the SDK is deprecated and will eventually be no longer supported."]
        Deprecated,
        #[serde(rename = "UNSUPPORTED")]
        #[doc = "Support for this SDK version has ended and it should no longer be used."]
        Unsupported,
    }
    impl ::std::default::Default for SdkVersionSdkSupportStatusEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to send encoded debug information."]
    pub struct SendDebugCaptureRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "componentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The internal component id for which debug information is sent."]
        pub component_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The encoded debug information."]
        pub data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains the job specified by job_id."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The worker id, i.e., VM hostname."]
        pub worker_id: ::std::option::Option<::std::string::String>,
    }
    impl SendDebugCaptureRequest {
        pub fn builder() -> SendDebugCaptureRequestBuilder {
            SendDebugCaptureRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to a send capture request. nothing"]
    pub struct SendDebugCaptureResponse {}
    impl SendDebugCaptureResponse {
        pub fn builder() -> SendDebugCaptureResponseBuilder {
            SendDebugCaptureResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request for sending worker messages to the service."]
    pub struct SendWorkerMessagesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains the job."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerMessages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The WorkerMessages to send."]
        pub worker_messages:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WorkerMessage>>>,
    }
    impl SendWorkerMessagesRequest {
        pub fn builder() -> SendWorkerMessagesRequestBuilder {
            SendWorkerMessagesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response to the worker messages."]
    pub struct SendWorkerMessagesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerMessageResponses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The servers response to the worker messages."]
        pub worker_message_responses:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WorkerMessageResponse>>>,
    }
    impl SendWorkerMessagesResponse {
        pub fn builder() -> SendWorkerMessagesResponseBuilder {
            SendWorkerMessagesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a particular function to invoke."]
    pub struct SeqMapTask {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about each of the inputs."]
        pub inputs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SideInputInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user-provided name of the SeqDo operation."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputInfos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about each of the outputs."]
        pub output_infos:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SeqMapTaskOutputInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System-defined name of the stage containing the SeqDo operation. Unique across the workflow."]
        pub stage_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System-defined name of the SeqDo operation. Unique across the workflow."]
        pub system_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userFn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user function to invoke."]
        pub user_fn:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl SeqMapTask {
        pub fn builder() -> SeqMapTaskBuilder {
            SeqMapTaskBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about an output of a SeqMapTask."]
    pub struct SeqMapTaskOutputInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sink to write the output value to."]
        pub sink: ::std::option::Option<::std::boxed::Box<Sink>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id of the TupleTag the user code will tag the output value by."]
        pub tag: ::std::option::Option<::std::string::String>,
    }
    impl SeqMapTaskOutputInfo {
        pub fn builder() -> SeqMapTaskOutputInfoBuilder {
            SeqMapTaskOutputInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A task which consists of a shell command for the worker to execute."]
    pub struct ShellTask {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "command")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The shell command to run."]
        pub command: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exitCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Exit code for the task."]
        pub exit_code: ::std::option::Option<::std::primitive::i64>,
    }
    impl ShellTask {
        pub fn builder() -> ShellTaskBuilder {
            ShellTaskBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a side input of a DoFn or an input of a SeqDoFn."]
    pub struct SideInputInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How to interpret the source element(s) as a side input value."]
        pub kind: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source(s) to read element(s) from to get the value of this side input. If more than one source, then the elements are taken from the sources, in the specified order if order matters. At least one source is required."]
        pub sources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Source>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id of the tag the user code will access this side input by; this should correspond to the tag of some MultiOutputInfo."]
        pub tag: ::std::option::Option<::std::string::String>,
    }
    impl SideInputInfo {
        pub fn builder() -> SideInputInfoBuilder {
            SideInputInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A sink that records can be encoded and written to."]
    pub struct Sink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "codec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The codec to use to encode data written to the sink."]
        pub codec: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sink to write to, plus its parameters."]
        pub spec: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl Sink {
        pub fn builder() -> SinkBuilder {
            SinkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a snapshot of a job."]
    pub struct Snapshot {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time this snapshot was created."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User specified description of the snapshot. Maybe empty."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskSizeBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The disk byte size of the snapshot. Only available for snapshots in READY state."]
        pub disk_size_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID of this snapshot."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The project this snapshot belongs to."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pubsubMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "PubSub snapshot metadata."]
        pub pubsub_metadata:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PubsubSnapshotMetadata>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceJobId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The job this snapshot was created from."]
        pub source_job_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "State of the snapshot."]
        pub state: ::std::option::Option<SnapshotStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ttl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time after which this snapshot will be automatically deleted."]
        pub ttl: ::std::option::Option<::std::string::String>,
    }
    impl Snapshot {
        pub fn builder() -> SnapshotBuilder {
            SnapshotBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "State of the snapshot."]
    pub enum SnapshotStateEnum {
        #[serde(rename = "UNKNOWN_SNAPSHOT_STATE")]
        #[doc = "Unknown state."]
        UnknownSnapshotState,
        #[serde(rename = "PENDING")]
        #[doc = "Snapshot intent to create has been persisted, snapshotting of state has not yet started."]
        Pending,
        #[serde(rename = "RUNNING")]
        #[doc = "Snapshotting is being performed."]
        Running,
        #[serde(rename = "READY")]
        #[doc = "Snapshot has been created and is ready to be used."]
        Ready,
        #[serde(rename = "FAILED")]
        #[doc = "Snapshot failed to be created."]
        Failed,
        #[serde(rename = "DELETED")]
        #[doc = "Snapshot has been deleted."]
        Deleted,
    }
    impl ::std::default::Default for SnapshotStateEnum {
        fn default() -> Self {
            Self::UnknownSnapshotState
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to create a snapshot of a job."]
    pub struct SnapshotJobRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User specified description of the snapshot. Maybe empty."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location that contains this job."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snapshotSources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, perform snapshots for sources which support this."]
        pub snapshot_sources: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ttl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "TTL for the snapshot."]
        pub ttl: ::std::option::Option<::std::string::String>,
    }
    impl SnapshotJobRequest {
        pub fn builder() -> SnapshotJobRequestBuilder {
            SnapshotJobRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A source that records can be read and decoded from."]
    pub struct Source {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "baseSpecs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "While splitting, sources may specify the produced bundles as differences against another source, in order to save backend-side memory and allow bigger jobs. For details, see SourceSplitRequest. To support this use case, the full set of parameters of the source is logically obtained by taking the latest explicitly specified value of each parameter in the order: base_specs (later items win), spec (overrides anything in base_specs)."]
        pub base_specs: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "codec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The codec to use to decode data read from the source."]
        pub codec: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "doesNotNeedSplitting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Setting this value to true hints to the framework that the source doesn't need splitting, and using SourceSplitRequest on it would yield SOURCE_SPLIT_OUTCOME_USE_CURRENT. E.g. a file splitter may set this to true when splitting a single file into a set of byte ranges of appropriate size, and set this to false when splitting a filepattern into individual files. However, for efficiency, a file splitter may decide to produce file subranges directly from the filepattern to avoid a splitting round-trip. See SourceSplitRequest for an overview of the splitting process. This field is meaningful only in the Source objects populated by the user (e.g. when filling in a DerivedSource). Source objects supplied by the framework to the user don't have this field populated."]
        pub does_not_need_splitting: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optionally, metadata for this source can be supplied right away, avoiding a SourceGetMetadataOperation roundtrip (see SourceOperationRequest). This field is meaningful only in the Source objects populated by the user (e.g. when filling in a DerivedSource). Source objects supplied by the framework to the user don't have this field populated."]
        pub metadata: ::std::option::Option<::std::boxed::Box<SourceMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source to read from, plus its parameters."]
        pub spec: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl Source {
        pub fn builder() -> SourceBuilder {
            SourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "DEPRECATED in favor of DynamicSourceSplit."]
    pub struct SourceFork {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED"]
        pub primary: ::std::option::Option<::std::boxed::Box<SourceSplitShard>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primarySource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED"]
        pub primary_source: ::std::option::Option<::std::boxed::Box<DerivedSource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "residual")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED"]
        pub residual: ::std::option::Option<::std::boxed::Box<SourceSplitShard>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "residualSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED"]
        pub residual_source: ::std::option::Option<::std::boxed::Box<DerivedSource>>,
    }
    impl SourceFork {
        pub fn builder() -> SourceForkBuilder {
            SourceForkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to compute the SourceMetadata of a Source."]
    pub struct SourceGetMetadataRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specification of the source whose metadata should be computed."]
        pub source: ::std::option::Option<::std::boxed::Box<Source>>,
    }
    impl SourceGetMetadataRequest {
        pub fn builder() -> SourceGetMetadataRequestBuilder {
            SourceGetMetadataRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of a SourceGetMetadataOperation."]
    pub struct SourceGetMetadataResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The computed metadata."]
        pub metadata: ::std::option::Option<::std::boxed::Box<SourceMetadata>>,
    }
    impl SourceGetMetadataResponse {
        pub fn builder() -> SourceGetMetadataResponseBuilder {
            SourceGetMetadataResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata about a Source useful for automatically optimizing and tuning the pipeline, etc."]
    pub struct SourceMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "estimatedSizeBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An estimate of the total size (in bytes) of the data that would be read from this source. This estimate is in terms of external storage size, before any decompression or other processing done by the reader."]
        pub estimated_size_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infinite")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies that the size of this source is known to be infinite (this is a streaming source)."]
        pub infinite: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "producesSortedKeys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this source is known to produce key/value pairs with the (encoded) keys in lexicographically sorted order."]
        pub produces_sorted_keys: ::std::option::Option<::std::primitive::bool>,
    }
    impl SourceMetadata {
        pub fn builder() -> SourceMetadataBuilder {
            SourceMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A work item that represents the different operations that can be performed on a user-defined Source specification."]
    pub struct SourceOperationRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "getMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about a request to get metadata about a source."]
        pub get_metadata: ::std::option::Option<::std::boxed::Box<SourceGetMetadataRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-provided name of the Read instruction for this source."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System-defined name for the Read instruction for this source in the original workflow graph."]
        pub original_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "split")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about a request to split a source."]
        pub split: ::std::option::Option<::std::boxed::Box<SourceSplitRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System-defined name of the stage containing the source operation. Unique across the workflow."]
        pub stage_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System-defined name of the Read instruction for this source. Unique across the workflow."]
        pub system_name: ::std::option::Option<::std::string::String>,
    }
    impl SourceOperationRequest {
        pub fn builder() -> SourceOperationRequestBuilder {
            SourceOperationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of a SourceOperationRequest, specified in ReportWorkItemStatusRequest.source_operation when the work item is completed."]
    pub struct SourceOperationResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "getMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A response to a request to get metadata about a source."]
        pub get_metadata: ::std::option::Option<::std::boxed::Box<SourceGetMetadataResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "split")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A response to a request to split a source."]
        pub split: ::std::option::Option<::std::boxed::Box<SourceSplitResponse>>,
    }
    impl SourceOperationResponse {
        pub fn builder() -> SourceOperationResponseBuilder {
            SourceOperationResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Hints for splitting a Source into bundles (parts for parallel processing) using SourceSplitRequest."]
    pub struct SourceSplitOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredBundleSizeBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source should be split into a set of bundles where the estimated size of each is approximately this many bytes."]
        pub desired_bundle_size_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredShardSizeBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED in favor of desired_bundle_size_bytes."]
        pub desired_shard_size_bytes: ::std::option::Option<::std::string::String>,
    }
    impl SourceSplitOptions {
        pub fn builder() -> SourceSplitOptionsBuilder {
            SourceSplitOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the operation to split a high-level Source specification into bundles (parts for parallel processing). At a high level, splitting of a source into bundles happens as follows: SourceSplitRequest is applied to the source. If it returns SOURCE_SPLIT_OUTCOME_USE_CURRENT, no further splitting happens and the source is used \"as is\". Otherwise, splitting is applied recursively to each produced DerivedSource. As an optimization, for any Source, if its does_not_need_splitting is true, the framework assumes that splitting this source would return SOURCE_SPLIT_OUTCOME_USE_CURRENT, and doesn't initiate a SourceSplitRequest. This applies both to the initial source being split and to bundles produced from it."]
    pub struct SourceSplitRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "options")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hints for tuning the splitting process."]
        pub options: ::std::option::Option<::std::boxed::Box<SourceSplitOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specification of the source to be split."]
        pub source: ::std::option::Option<::std::boxed::Box<Source>>,
    }
    impl SourceSplitRequest {
        pub fn builder() -> SourceSplitRequestBuilder {
            SourceSplitRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response to a SourceSplitRequest."]
    pub struct SourceSplitResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bundles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If outcome is SPLITTING_HAPPENED, then this is a list of bundles into which the source was split. Otherwise this field is ignored. This list can be empty, which means the source represents an empty input."]
        pub bundles: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DerivedSource>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outcome")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether splitting happened and produced a list of bundles. If this is USE_CURRENT_SOURCE_AS_IS, the current source should be processed \"as is\" without splitting. \"bundles\" is ignored in this case. If this is SPLITTING_HAPPENED, then \"bundles\" contains a list of bundles into which the source was split."]
        pub outcome: ::std::option::Option<SourceSplitResponseOutcomeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shards")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED in favor of bundles."]
        pub shards: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SourceSplitShard>>>,
    }
    impl SourceSplitResponse {
        pub fn builder() -> SourceSplitResponseBuilder {
            SourceSplitResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates whether splitting happened and produced a list of bundles. If this is USE_CURRENT_SOURCE_AS_IS, the current source should be processed \"as is\" without splitting. \"bundles\" is ignored in this case. If this is SPLITTING_HAPPENED, then \"bundles\" contains a list of bundles into which the source was split."]
    pub enum SourceSplitResponseOutcomeEnum {
        #[serde(rename = "SOURCE_SPLIT_OUTCOME_UNKNOWN")]
        #[doc = "The source split outcome is unknown, or unspecified."]
        SourceSplitOutcomeUnknown,
        #[serde(rename = "SOURCE_SPLIT_OUTCOME_USE_CURRENT")]
        #[doc = "The current source should be processed \"as is\" without splitting."]
        SourceSplitOutcomeUseCurrent,
        #[serde(rename = "SOURCE_SPLIT_OUTCOME_SPLITTING_HAPPENED")]
        #[doc = "Splitting produced a list of bundles."]
        SourceSplitOutcomeSplittingHappened,
    }
    impl ::std::default::Default for SourceSplitResponseOutcomeEnum {
        fn default() -> Self {
            Self::SourceSplitOutcomeUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "DEPRECATED in favor of DerivedSource."]
    pub struct SourceSplitShard {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "derivationMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED"]
        pub derivation_mode: ::std::option::Option<SourceSplitShardDerivationModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED"]
        pub source: ::std::option::Option<::std::boxed::Box<Source>>,
    }
    impl SourceSplitShard {
        pub fn builder() -> SourceSplitShardBuilder {
            SourceSplitShardBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "DEPRECATED"]
    pub enum SourceSplitShardDerivationModeEnum {
        #[serde(rename = "SOURCE_DERIVATION_MODE_UNKNOWN")]
        #[doc = "The source derivation is unknown, or unspecified."]
        SourceDerivationModeUnknown,
        #[serde(rename = "SOURCE_DERIVATION_MODE_INDEPENDENT")]
        #[doc = "Produce a completely independent Source with no base."]
        SourceDerivationModeIndependent,
        #[serde(rename = "SOURCE_DERIVATION_MODE_CHILD_OF_CURRENT")]
        #[doc = "Produce a Source based on the Source being split."]
        SourceDerivationModeChildOfCurrent,
        #[serde(rename = "SOURCE_DERIVATION_MODE_SIBLING_OF_CURRENT")]
        #[doc = "Produce a Source based on the base of the Source being split."]
        SourceDerivationModeSiblingOfCurrent,
    }
    impl ::std::default::Default for SourceSplitShardDerivationModeEnum {
        fn default() -> Self {
            Self::SourceDerivationModeUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for a Spanner connector used by the job."]
    pub struct SpannerIoDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "databaseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DatabaseId accessed in the connection."]
        pub database_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "InstanceId accessed in the connection."]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ProjectId accessed in the connection."]
        pub project_id: ::std::option::Option<::std::string::String>,
    }
    impl SpannerIoDetails {
        pub fn builder() -> SpannerIoDetailsBuilder {
            SpannerIoDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A representation of an int64, n, that is immune to precision loss when encoded in JSON."]
    pub struct SplitInt64 {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "highBits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The high order bits, including the sign: n >> 32."]
        pub high_bits: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lowBits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The low order bits: n & 0xffffffff."]
        pub low_bits: ::std::option::Option<::std::primitive::i64>,
    }
    impl SplitInt64 {
        pub fn builder() -> SplitInt64Builder {
            SplitInt64Builder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the workers and work items within a stage."]
    pub struct StageExecutionDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, this response does not contain all requested tasks. To obtain the next page of results, repeat the request with page_token set to this value."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Workers that have done work on the stage."]
        pub workers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WorkerDetails>>>,
    }
    impl StageExecutionDetails {
        pub fn builder() -> StageExecutionDetailsBuilder {
            StageExecutionDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Description of an input or output of an execution stage."]
    pub struct StageSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dataflow service generated name for this source."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalTransformOrCollection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User name for the original user transform or collection with which this source is most closely associated."]
        pub original_transform_or_collection: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sizeBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Size of the source, if measurable."]
        pub size_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human-readable name for this source; may be user or system generated."]
        pub user_name: ::std::option::Option<::std::string::String>,
    }
    impl StageSource {
        pub fn builder() -> StageSourceBuilder {
            StageSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a particular execution stage of a job."]
    pub struct StageSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End time of this stage. If the work item is completed, this is the actual end time of the stage. Otherwise, it is the predicted end time."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metrics for this stage."]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricUpdate>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Progress for this stage. Only applicable to Batch jobs."]
        pub progress: ::std::option::Option<::std::boxed::Box<ProgressTimeseries>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stageId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of this stage"]
        pub stage_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start time of this stage."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "State of this stage."]
        pub state: ::std::option::Option<StageSummaryStateEnum>,
    }
    impl StageSummary {
        pub fn builder() -> StageSummaryBuilder {
            StageSummaryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "State of this stage."]
    pub enum StageSummaryStateEnum {
        #[serde(rename = "EXECUTION_STATE_UNKNOWN")]
        #[doc = "The component state is unknown or unspecified."]
        ExecutionStateUnknown,
        #[serde(rename = "EXECUTION_STATE_NOT_STARTED")]
        #[doc = "The component is not yet running."]
        ExecutionStateNotStarted,
        #[serde(rename = "EXECUTION_STATE_RUNNING")]
        #[doc = "The component is currently running."]
        ExecutionStateRunning,
        #[serde(rename = "EXECUTION_STATE_SUCCEEDED")]
        #[doc = "The component succeeded."]
        ExecutionStateSucceeded,
        #[serde(rename = "EXECUTION_STATE_FAILED")]
        #[doc = "The component failed."]
        ExecutionStateFailed,
        #[serde(rename = "EXECUTION_STATE_CANCELLED")]
        #[doc = "Execution of the component was cancelled."]
        ExecutionStateCancelled,
    }
    impl ::std::default::Default for StageSummaryStateEnum {
        fn default() -> Self {
            Self::ExecutionStateUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "State family configuration."]
    pub struct StateFamilyConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isRead")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, this family corresponds to a read operation."]
        pub is_read: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateFamily")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state family value."]
        pub state_family: ::std::option::Option<::std::string::String>,
    }
    impl StateFamilyConfig {
        pub fn builder() -> StateFamilyConfigBuilder {
            StateFamilyConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub struct Status {
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
    impl Status {
        pub fn builder() -> StatusBuilder {
            StatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines a particular step within a Cloud Dataflow job. A job consists of multiple steps, each of which performs some specific operation as part of the overall job. Data is typically passed from one step to another as part of the job. Here's an example of a sequence of steps which together implement a Map-Reduce job: * Read a collection of data from some source, parsing the collection's elements. * Validate the elements. * Apply a user-defined function to map each element to some value and extract an element-specific key value. * Group elements with the same key into a single element with that key, transforming a multiply-keyed collection into a uniquely-keyed collection. * Write the elements out to some data sink. Note that the Cloud Dataflow service may be used to run many different types of jobs, not just Map-Reduce."]
    pub struct Step {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of step in the Cloud Dataflow job."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name that identifies the step. This must be unique for each step with respect to all other steps in the Cloud Dataflow job."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Named properties associated with the step. Each kind of predefined step has its own required set of properties. Must be provided on Create. Only retrieved with JOB_VIEW_ALL."]
        pub properties:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl Step {
        pub fn builder() -> StepBuilder {
            StepBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a stream of data, either as input to be processed or as output of a streaming Dataflow job."]
    pub struct StreamLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customSourceLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stream is a custom source."]
        pub custom_source_location: ::std::option::Option<::std::boxed::Box<CustomSourceLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pubsubLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stream is a pubsub stream."]
        pub pubsub_location: ::std::option::Option<::std::boxed::Box<PubsubLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sideInputLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stream is a streaming side input."]
        pub side_input_location:
            ::std::option::Option<::std::boxed::Box<StreamingSideInputLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "streamingStageLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stream is part of another computation within the current streaming Dataflow job."]
        pub streaming_stage_location:
            ::std::option::Option<::std::boxed::Box<StreamingStageLocation>>,
    }
    impl StreamLocation {
        pub fn builder() -> StreamLocationBuilder {
            StreamLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Streaming appliance snapshot configuration."]
    pub struct StreamingApplianceSnapshotConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importStateEndpoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates which endpoint is used to import appliance state."]
        pub import_state_endpoint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snapshotId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, indicates the snapshot id for the snapshot being performed."]
        pub snapshot_id: ::std::option::Option<::std::string::String>,
    }
    impl StreamingApplianceSnapshotConfig {
        pub fn builder() -> StreamingApplianceSnapshotConfigBuilder {
            StreamingApplianceSnapshotConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration information for a single streaming computation."]
    pub struct StreamingComputationConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "computationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier for this computation."]
        pub computation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instructions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instructions that comprise the computation."]
        pub instructions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ParallelInstruction>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Stage name of this computation."]
        pub stage_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System defined name for this computation."]
        pub system_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transformUserNameToStateFamily")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map from user name of stateful transforms in this stage to their state family."]
        pub transform_user_name_to_state_family:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl StreamingComputationConfig {
        pub fn builder() -> StreamingComputationConfigBuilder {
            StreamingComputationConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes full or partial data disk assignment information of the computation ranges."]
    pub struct StreamingComputationRanges {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "computationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the computation."]
        pub computation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rangeAssignments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data disk assignments for ranges from this computation."]
        pub range_assignments:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<KeyRangeDataDiskAssignment>>>,
    }
    impl StreamingComputationRanges {
        pub fn builder() -> StreamingComputationRangesBuilder {
            StreamingComputationRangesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A task which describes what action should be performed for the specified streaming computation ranges."]
    pub struct StreamingComputationTask {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "computationRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains ranges of a streaming computation this task should apply to."]
        pub computation_ranges:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StreamingComputationRanges>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataDisks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes the set of data disks this task should apply to."]
        pub data_disks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MountedDataDisk>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taskType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A type of streaming computation task."]
        pub task_type: ::std::option::Option<StreamingComputationTaskTaskTypeEnum>,
    }
    impl StreamingComputationTask {
        pub fn builder() -> StreamingComputationTaskBuilder {
            StreamingComputationTaskBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "A type of streaming computation task."]
    pub enum StreamingComputationTaskTaskTypeEnum {
        #[serde(rename = "STREAMING_COMPUTATION_TASK_UNKNOWN")]
        #[doc = "The streaming computation task is unknown, or unspecified."]
        StreamingComputationTaskUnknown,
        #[serde(rename = "STREAMING_COMPUTATION_TASK_STOP")]
        #[doc = "Stop processing specified streaming computation range(s)."]
        StreamingComputationTaskStop,
        #[serde(rename = "STREAMING_COMPUTATION_TASK_START")]
        #[doc = "Start processing specified streaming computation range(s)."]
        StreamingComputationTaskStart,
    }
    impl ::std::default::Default for StreamingComputationTaskTaskTypeEnum {
        fn default() -> Self {
            Self::StreamingComputationTaskUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A task that carries configuration information for streaming computations."]
    pub struct StreamingConfigTask {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commitStreamChunkSizeBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Chunk size for commit streams from the harness to windmill."]
        pub commit_stream_chunk_size_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "getDataStreamChunkSizeBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Chunk size for get data streams from the harness to windmill."]
        pub get_data_stream_chunk_size_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxWorkItemCommitBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum size for work item commit supported windmill storage layer."]
        pub max_work_item_commit_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "streamingComputationConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of computation configuration information."]
        pub streaming_computation_configs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StreamingComputationConfig>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userStepToStateFamilyNameMap")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map from user step names to state families."]
        pub user_step_to_state_family_name_map:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "windmillServiceEndpoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, the worker must use this endpoint to communicate with Windmill Service dispatchers, otherwise the worker must continue to use whatever endpoint it had been using."]
        pub windmill_service_endpoint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "windmillServicePort")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, the worker must use this port to communicate with Windmill Service dispatchers. Only applicable when windmill_service_endpoint is specified."]
        pub windmill_service_port: ::std::option::Option<::std::string::String>,
    }
    impl StreamingConfigTask {
        pub fn builder() -> StreamingConfigTaskBuilder {
            StreamingConfigTaskBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A task which initializes part of a streaming Dataflow job."]
    pub struct StreamingSetupTask {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "drain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user has requested drain."]
        pub drain: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "receiveWorkPort")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The TCP port on which the worker should listen for messages from other streaming computation workers."]
        pub receive_work_port: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snapshotConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configures streaming appliance snapshot."]
        pub snapshot_config:
            ::std::option::Option<::std::boxed::Box<StreamingApplianceSnapshotConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "streamingComputationTopology")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The global topology of the streaming Dataflow job."]
        pub streaming_computation_topology:
            ::std::option::Option<::std::boxed::Box<TopologyConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerHarnessPort")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The TCP port used by the worker to communicate with the Dataflow worker harness."]
        pub worker_harness_port: ::std::option::Option<::std::primitive::i64>,
    }
    impl StreamingSetupTask {
        pub fn builder() -> StreamingSetupTaskBuilder {
            StreamingSetupTaskBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identifies the location of a streaming side input."]
    pub struct StreamingSideInputLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateFamily")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies the state family where this side input is stored."]
        pub state_family: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies the particular side input within the streaming Dataflow job."]
        pub tag: ::std::option::Option<::std::string::String>,
    }
    impl StreamingSideInputLocation {
        pub fn builder() -> StreamingSideInputLocationBuilder {
            StreamingSideInputLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identifies the location of a streaming computation stage, for stage-to-stage communication."]
    pub struct StreamingStageLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "streamId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies the particular stream within the streaming Dataflow job."]
        pub stream_id: ::std::option::Option<::std::string::String>,
    }
    impl StreamingStageLocation {
        pub fn builder() -> StreamingStageLocationBuilder {
            StreamingStageLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A metric value representing a list of strings."]
    pub struct StringList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "elements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Elements of the list."]
        pub elements: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl StringList {
        pub fn builder() -> StringListBuilder {
            StringListBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A rich message format, including a human readable string, a key for identifying the message, and structured data associated with the message for programmatic consumption."]
    pub struct StructuredMessage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier for this message type. Used by external systems to internationalize or personalize message."]
        pub message_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human-readable version of message."]
        pub message_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The structured data associated with this message."]
        pub parameters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Parameter>>>,
    }
    impl StructuredMessage {
        pub fn builder() -> StructuredMessageBuilder {
            StructuredMessageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Taskrunner configuration settings."]
    pub struct TaskRunnerSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alsologtostderr")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to also send taskrunner log info to stderr."]
        pub alsologtostderr: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "baseTaskDir")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location on the worker for task-specific subdirectories."]
        pub base_task_dir: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "baseUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The base URL for the taskrunner to use when accessing Google Cloud APIs. When workers access Google Cloud APIs, they logically do so via relative URLs. If this field is specified, it supplies the base URL to use for resolving these relative URLs. The normative algorithm used is defined by RFC 1808, \"Relative Uniform Resource Locators\". If not specified, the default value is \"http://www.googleapis.com/\""]
        pub base_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commandlinesFileName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The file to store preprocessing commands in."]
        pub commandlines_file_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "continueOnException")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to continue taskrunner if an exception is hit."]
        pub continue_on_exception: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataflowApiVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The API version of endpoint, e.g. \"v1b3\""]
        pub dataflow_api_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "harnessCommand")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The command to launch the worker harness."]
        pub harness_command: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageHint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The suggested backend language."]
        pub language_hint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logDir")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The directory on the VM to store logs."]
        pub log_dir: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logToSerialconsole")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to send taskrunner log info to Google Compute Engine VM serial console."]
        pub log_to_serialconsole: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logUploadLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates where to put logs. If this is not specified, the logs will not be uploaded. The supported resource type is: Google Cloud Storage: storage.googleapis.com/{bucket}/{object} bucket.storage.googleapis.com/{object}"]
        pub log_upload_location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauthScopes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The OAuth2 scopes to be requested by the taskrunner in order to access the Cloud Dataflow API."]
        pub oauth_scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parallelWorkerSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The settings to pass to the parallel worker harness."]
        pub parallel_worker_settings: ::std::option::Option<::std::boxed::Box<WorkerSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "streamingWorkerMainClass")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The streaming worker main class name."]
        pub streaming_worker_main_class: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taskGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The UNIX group ID on the worker VM to use for tasks launched by taskrunner; e.g. \"wheel\"."]
        pub task_group: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taskUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The UNIX user ID on the worker VM to use for tasks launched by taskrunner; e.g. \"root\"."]
        pub task_user: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tempStoragePrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The prefix of the resources the taskrunner should use for temporary storage. The supported resource type is: Google Cloud Storage: storage.googleapis.com/{bucket}/{object} bucket.storage.googleapis.com/{object}"]
        pub temp_storage_prefix: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vmId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID string of the VM."]
        pub vm_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workflowFileName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The file to store the workflow in."]
        pub workflow_file_name: ::std::option::Option<::std::string::String>,
    }
    impl TaskRunnerSettings {
        pub fn builder() -> TaskRunnerSettingsBuilder {
            TaskRunnerSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata describing a template."]
    pub struct TemplateMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A description of the template."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the template."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The parameters for the template."]
        pub parameters:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ParameterMetadata>>>,
    }
    impl TemplateMetadata {
        pub fn builder() -> TemplateMetadataBuilder {
            TemplateMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "///////////////////////////////////////////////////////////////////////////// //// Template Catalog is used to organize user TemplateVersions. //// TemplateVersions that have the same project_id and display_name are //// belong to the same Template. //// Templates with the same project_id belong to the same Project. //// TemplateVersion may have labels and multiple labels are allowed. //// Duplicated labels in the same `TemplateVersion` are not allowed. //// TemplateVersion may have tags and multiple tags are allowed. Duplicated //// tags in the same `Template` are not allowed!"]
    pub struct TemplateVersion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "artifact")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Job graph and metadata if it is a legacy Template. Container image path and metadata if it is flex Template."]
        pub artifact: ::std::option::Option<::std::boxed::Box<Artifact>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creation time of this TemplateVersion."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Template description from the user."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A customized name for Template. Multiple TemplateVersions per Template."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Labels for the Template Version. Labels can be duplicate within Template."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique project_id. Multiple Templates per Project."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Alias for version_id, helps locate a TemplateVersion."]
        pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Either LEGACY or FLEX. This should match with the type of artifact."]
        pub _type: ::std::option::Option<TemplateVersionTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An auto generated version_id for TemplateVersion."]
        pub version_id: ::std::option::Option<::std::string::String>,
    }
    impl TemplateVersion {
        pub fn builder() -> TemplateVersionBuilder {
            TemplateVersionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Either LEGACY or FLEX. This should match with the type of artifact."]
    pub enum TemplateVersionTypeEnum {
        #[serde(rename = "TEMPLATE_TYPE_UNSPECIFIED")]
        #[doc = "Default value. Not a useful zero case."]
        TemplateTypeUnspecified,
        #[serde(rename = "LEGACY")]
        #[doc = "Legacy Template."]
        Legacy,
        #[serde(rename = "FLEX")]
        #[doc = "Flex Template."]
        Flex,
    }
    impl ::std::default::Default for TemplateVersionTypeEnum {
        fn default() -> Self {
            Self::TemplateTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Global topology of the streaming Dataflow job, including all computations and their sharded locations."]
    pub struct TopologyConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "computations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The computations associated with a streaming Dataflow job."]
        pub computations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ComputationTopology>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataDiskAssignments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The disks assigned to a streaming Dataflow job."]
        pub data_disk_assignments:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataDiskAssignment>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "forwardingKeyBits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size (in bits) of keys that will be assigned to source messages."]
        pub forwarding_key_bits: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "persistentStateVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Version number for persistent state."]
        pub persistent_state_version: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userStageToComputationNameMap")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maps user stage names to stable computation names."]
        pub user_stage_to_computation_name_map:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl TopologyConfig {
        pub fn builder() -> TopologyConfigBuilder {
            TopologyConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Description of the type, names/ids, and input/outputs for a transform."]
    pub struct TransformSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transform-specific display data."]
        pub display_data: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DisplayData>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "SDK generated id of this transform instance."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputCollectionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User names for all collection inputs to this transform."]
        pub input_collection_name: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of transform."]
        pub kind: ::std::option::Option<TransformSummaryKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User provided name for this transform instance."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputCollectionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User names for all collection outputs to this transform."]
        pub output_collection_name: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl TransformSummary {
        pub fn builder() -> TransformSummaryBuilder {
            TransformSummaryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of transform."]
    pub enum TransformSummaryKindEnum {
        #[serde(rename = "UNKNOWN_KIND")]
        #[doc = "Unrecognized transform type."]
        UnknownKind,
        #[serde(rename = "PAR_DO_KIND")]
        #[doc = "ParDo transform."]
        ParDoKind,
        #[serde(rename = "GROUP_BY_KEY_KIND")]
        #[doc = "Group By Key transform."]
        GroupByKeyKind,
        #[serde(rename = "FLATTEN_KIND")]
        #[doc = "Flatten transform."]
        FlattenKind,
        #[serde(rename = "READ_KIND")]
        #[doc = "Read transform."]
        ReadKind,
        #[serde(rename = "WRITE_KIND")]
        #[doc = "Write transform."]
        WriteKind,
        #[serde(rename = "CONSTANT_KIND")]
        #[doc = "Constructs from a constant value, such as with Create.of."]
        ConstantKind,
        #[serde(rename = "SINGLETON_KIND")]
        #[doc = "Creates a Singleton view of a collection."]
        SingletonKind,
        #[serde(rename = "SHUFFLE_KIND")]
        #[doc = "Opening or closing a shuffle session, often as part of a GroupByKey."]
        ShuffleKind,
    }
    impl ::std::default::Default for TransformSummaryKindEnum {
        fn default() -> Self {
            Self::UnknownKind
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to the validation request."]
    pub struct ValidateResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Will be empty if validation succeeds."]
        pub error_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the validated query. Not defined if validation fails."]
        pub query_info: ::std::option::Option<::std::boxed::Box<QueryInfo>>,
    }
    impl ValidateResponse {
        pub fn builder() -> ValidateResponseBuilder {
            ValidateResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "WorkItem represents basic information about a WorkItem to be executed in the cloud."]
    pub struct WorkItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "configuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Work item-specific configuration as an opaque blob."]
        pub configuration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies this WorkItem."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "initialReportIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The initial index to use when reporting the status of the WorkItem."]
        pub initial_report_index: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies the workflow job this WorkItem belongs to."]
        pub job_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "leaseExpireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time when the lease on this Work will expire."]
        pub lease_expire_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mapTask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information for MapTask WorkItems."]
        pub map_task: ::std::option::Option<::std::boxed::Box<MapTask>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Any required packages that need to be fetched in order to execute this WorkItem."]
        pub packages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Package>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies the cloud project this WorkItem belongs to."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportStatusInterval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Recommended reporting interval."]
        pub report_status_interval: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seqMapTask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information for SeqMapTask WorkItems."]
        pub seq_map_task: ::std::option::Option<::std::boxed::Box<SeqMapTask>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shellTask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information for ShellTask WorkItems."]
        pub shell_task: ::std::option::Option<::std::boxed::Box<ShellTask>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceOperationTask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information for source operation WorkItems."]
        pub source_operation_task: ::std::option::Option<::std::boxed::Box<SourceOperationRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "streamingComputationTask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information for StreamingComputationTask WorkItems."]
        pub streaming_computation_task:
            ::std::option::Option<::std::boxed::Box<StreamingComputationTask>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "streamingConfigTask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information for StreamingConfigTask WorkItems."]
        pub streaming_config_task: ::std::option::Option<::std::boxed::Box<StreamingConfigTask>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "streamingSetupTask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information for StreamingSetupTask WorkItems."]
        pub streaming_setup_task: ::std::option::Option<::std::boxed::Box<StreamingSetupTask>>,
    }
    impl WorkItem {
        pub fn builder() -> WorkItemBuilder {
            WorkItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about an individual work item execution."]
    pub struct WorkItemDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attemptId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Attempt ID of this work item"]
        pub attempt_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End time of this work item attempt. If the work item is completed, this is the actual end time of the work item. Otherwise, it is the predicted end time."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metrics for this work item."]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricUpdate>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Progress of this work item."]
        pub progress: ::std::option::Option<::std::boxed::Box<ProgressTimeseries>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start time of this work item attempt."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "State of this work item."]
        pub state: ::std::option::Option<WorkItemDetailsStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taskId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of this work item."]
        pub task_id: ::std::option::Option<::std::string::String>,
    }
    impl WorkItemDetails {
        pub fn builder() -> WorkItemDetailsBuilder {
            WorkItemDetailsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "State of this work item."]
    pub enum WorkItemDetailsStateEnum {
        #[serde(rename = "EXECUTION_STATE_UNKNOWN")]
        #[doc = "The component state is unknown or unspecified."]
        ExecutionStateUnknown,
        #[serde(rename = "EXECUTION_STATE_NOT_STARTED")]
        #[doc = "The component is not yet running."]
        ExecutionStateNotStarted,
        #[serde(rename = "EXECUTION_STATE_RUNNING")]
        #[doc = "The component is currently running."]
        ExecutionStateRunning,
        #[serde(rename = "EXECUTION_STATE_SUCCEEDED")]
        #[doc = "The component succeeded."]
        ExecutionStateSucceeded,
        #[serde(rename = "EXECUTION_STATE_FAILED")]
        #[doc = "The component failed."]
        ExecutionStateFailed,
        #[serde(rename = "EXECUTION_STATE_CANCELLED")]
        #[doc = "Execution of the component was cancelled."]
        ExecutionStateCancelled,
    }
    impl ::std::default::Default for WorkItemDetailsStateEnum {
        fn default() -> Self {
            Self::ExecutionStateUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Dataflow service's idea of the current state of a WorkItem being processed by a worker."]
    pub struct WorkItemServiceState {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "completeWorkStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, a request to complete the work item with the given status. This will not be set to OK, unless supported by the specific kind of WorkItem. It can be used for the backend to indicate a WorkItem must terminate, e.g., for aborting work."]
        pub complete_work_status: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "harnessData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Other data returned by the service, specific to the particular worker harness."]
        pub harness_data:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hotKeyDetection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A hot key is a symptom of poor data distribution in which there are enough elements mapped to a single key to impact pipeline performance. When present, this field includes metadata associated with any hot key."]
        pub hot_key_detection: ::std::option::Option<::std::boxed::Box<HotKeyDetection>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "leaseExpireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time at which the current lease will expire."]
        pub lease_expire_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricShortId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The short ids that workers should use in subsequent metric updates. Workers should strive to use short ids whenever possible, but it is ok to request the short_id again if a worker lost track of it (e.g. if the worker is recovering from a crash). NOTE: it is possible that the response may have short ids for a subset of the metrics."]
        pub metric_short_id:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricShortId>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextReportIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index value to use for the next report sent by the worker. Note: If the report call fails for whatever reason, the worker should reuse this index for subsequent report attempts."]
        pub next_report_index: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportStatusInterval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "New recommended reporting interval."]
        pub report_status_interval: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "splitRequest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The progress point in the WorkItem where the Dataflow service suggests that the worker truncate the task."]
        pub split_request: ::std::option::Option<::std::boxed::Box<ApproximateSplitRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suggestedStopPoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED in favor of split_request."]
        pub suggested_stop_point: ::std::option::Option<::std::boxed::Box<ApproximateProgress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suggestedStopPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Obsolete, always empty."]
        pub suggested_stop_position: ::std::option::Option<::std::boxed::Box<Position>>,
    }
    impl WorkItemServiceState {
        pub fn builder() -> WorkItemServiceStateBuilder {
            WorkItemServiceStateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Conveys a worker's progress through the work described by a WorkItem."]
    pub struct WorkItemStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "completed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the WorkItem was completed (successfully or unsuccessfully)."]
        pub completed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "counterUpdates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Worker output counters for this WorkItem."]
        pub counter_updates:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CounterUpdate>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dynamicSourceSplit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "See documentation of stop_position."]
        pub dynamic_source_split: ::std::option::Option<::std::boxed::Box<DynamicSourceSplit>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies errors which occurred during processing. If errors are provided, and completed = true, then the WorkItem is considered to have failed."]
        pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Status>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricUpdates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED in favor of counter_updates."]
        pub metric_updates: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricUpdate>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED in favor of reported_progress."]
        pub progress: ::std::option::Option<::std::boxed::Box<ApproximateProgress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The report index. When a WorkItem is leased, the lease will contain an initial report index. When a WorkItem's status is reported to the system, the report should be sent with that report index, and the response will contain the index the worker should use for the next report. Reports received with unexpected index values will be rejected by the service. In order to preserve idempotency, the worker should not alter the contents of a report, even if the worker must submit the same report multiple times before getting back a response. The worker should not submit a subsequent report until the response for the previous report had been received from the service."]
        pub report_index: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportedProgress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The worker's progress through this WorkItem."]
        pub reported_progress:
            ::std::option::Option<::std::boxed::Box<ApproximateReportedProgress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestedLeaseDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Amount of time the worker requests for its lease."]
        pub requested_lease_duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceFork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED in favor of dynamic_source_split."]
        pub source_fork: ::std::option::Option<::std::boxed::Box<SourceFork>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceOperationResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the work item represented a SourceOperationRequest, and the work is completed, contains the result of the operation."]
        pub source_operation_response:
            ::std::option::Option<::std::boxed::Box<SourceOperationResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stopPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A worker may split an active map task in two parts, \"primary\" and \"residual\", continuing to process the primary part and returning the residual part into the pool of available work. This event is called a \"dynamic split\" and is critical to the dynamic work rebalancing feature. The two obtained sub-tasks are called \"parts\" of the split. The parts, if concatenated, must represent the same input as would be read by the current task if the split did not happen. The exact way in which the original task is decomposed into the two parts is specified either as a position demarcating them (stop_position), or explicitly as two DerivedSources, if this task consumes a user-defined source type (dynamic_source_split). The \"current\" task is adjusted as a result of the split: after a task with range [A, B) sends a stop_position update at C, its range is considered to be [A, C), e.g.: * Progress should be interpreted relative to the new range, e.g. \"75% completed\" means \"75% of [A, C) completed\" * The worker should interpret proposed_stop_position relative to the new range, e.g. \"split at 68%\" should be interpreted as \"split at 68% of [A, C)\". * If the worker chooses to split again using stop_position, only stop_positions in [A, C) will be accepted. * Etc. dynamic_source_split has similar semantics: e.g., if a task with source S splits using dynamic_source_split into {P, R} (where P and R must be together equivalent to S), then subsequent progress and proposed_stop_position should be interpreted relative to P, and in a potential subsequent dynamic_source_split into {P', R'}, P' and R' must be together equivalent to P, etc."]
        pub stop_position: ::std::option::Option<::std::boxed::Box<Position>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalThrottlerWaitTimeSeconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total time the worker spent being throttled by external systems."]
        pub total_throttler_wait_time_seconds: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies the WorkItem."]
        pub work_item_id: ::std::option::Option<::std::string::String>,
    }
    impl WorkItemStatus {
        pub fn builder() -> WorkItemStatusBuilder {
            WorkItemStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a worker"]
    pub struct WorkerDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Work items processed by this worker, sorted by time."]
        pub work_items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WorkItemDetails>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of this worker"]
        pub worker_name: ::std::option::Option<::std::string::String>,
    }
    impl WorkerDetails {
        pub fn builder() -> WorkerDetailsBuilder {
            WorkerDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "WorkerHealthReport contains information about the health of a worker. The VM should be identified by the labels attached to the WorkerMessage that this health ping belongs to."]
    pub struct WorkerHealthReport {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "msg")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message describing any unusual health reports."]
        pub msg: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pods")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pods running on the worker. See: http://kubernetes.io/v1.1/docs/api-reference/v1/definitions.html#_v1_pod This field is used by the worker to send the status of the indvidual containers running on each worker."]
        pub pods: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportInterval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The interval at which the worker is sending health reports. The default value of 0 should be interpreted as the field is not being explicitly set by the worker."]
        pub report_interval: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vmIsBroken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the VM is in a permanently broken state. Broken VMs should be abandoned or deleted ASAP to avoid assigning or completing any work."]
        pub vm_is_broken: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vmIsHealthy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the VM is currently healthy."]
        pub vm_is_healthy: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vmStartupTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time the VM was booted."]
        pub vm_startup_time: ::std::option::Option<::std::string::String>,
    }
    impl WorkerHealthReport {
        pub fn builder() -> WorkerHealthReportBuilder {
            WorkerHealthReportBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "WorkerHealthReportResponse contains information returned to the worker in response to a health ping."]
    pub struct WorkerHealthReportResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportInterval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A positive value indicates the worker should change its reporting interval to the specified value. The default value of zero means no change in report rate is requested by the server."]
        pub report_interval: ::std::option::Option<::std::string::String>,
    }
    impl WorkerHealthReportResponse {
        pub fn builder() -> WorkerHealthReportResponseBuilder {
            WorkerHealthReportResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A report of an event in a worker's lifecycle. The proto contains one event, because the worker is expected to asynchronously send each message immediately after the event. Due to this asynchrony, messages may arrive out of order (or missing), and it is up to the consumer to interpret. The timestamp of the event is in the enclosing WorkerMessage proto."]
    pub struct WorkerLifecycleEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start time of this container. All events will report this so that events can be grouped together across container/VM restarts."]
        pub container_start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "event")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The event being reported."]
        pub event: ::std::option::Option<WorkerLifecycleEventEventEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Other stats that can accompany an event. E.g. { \"downloaded_bytes\" : \"123456\" }"]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl WorkerLifecycleEvent {
        pub fn builder() -> WorkerLifecycleEventBuilder {
            WorkerLifecycleEventBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The event being reported."]
    pub enum WorkerLifecycleEventEventEnum {
        #[serde(rename = "UNKNOWN_EVENT")]
        #[doc = "Invalid event."]
        UnknownEvent,
        #[serde(rename = "OS_START")]
        #[doc = "The time the VM started."]
        OsStart,
        #[serde(rename = "CONTAINER_START")]
        #[doc = "Our container code starts running. Multiple containers could be distinguished with WorkerMessage.labels if desired."]
        ContainerStart,
        #[serde(rename = "NETWORK_UP")]
        #[doc = "The worker has a functional external network connection."]
        NetworkUp,
        #[serde(rename = "STAGING_FILES_DOWNLOAD_START")]
        #[doc = "Started downloading staging files."]
        StagingFilesDownloadStart,
        #[serde(rename = "STAGING_FILES_DOWNLOAD_FINISH")]
        #[doc = "Finished downloading all staging files."]
        StagingFilesDownloadFinish,
        #[serde(rename = "SDK_INSTALL_START")]
        #[doc = "For applicable SDKs, started installation of SDK and worker packages."]
        SdkInstallStart,
        #[serde(rename = "SDK_INSTALL_FINISH")]
        #[doc = "Finished installing SDK."]
        SdkInstallFinish,
    }
    impl ::std::default::Default for WorkerLifecycleEventEventEnum {
        fn default() -> Self {
            Self::UnknownEvent
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "WorkerMessage provides information to the backend about a worker."]
    pub struct WorkerMessage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Labels are used to group WorkerMessages. For example, a worker_message about a particular container might have the labels: { \"JOB_ID\": \"2015-04-22\", \"WORKER_ID\": \"wordcount-vm-2015…\" \"CONTAINER_TYPE\": \"worker\", \"CONTAINER_ID\": \"ac1234def\"} Label tags typically correspond to Label enum values. However, for ease of development other strings can be used as tags. LABEL_UNSPECIFIED should not be used here."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "time")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp of the worker_message."]
        pub time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerHealthReport")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The health of a worker."]
        pub worker_health_report: ::std::option::Option<::std::boxed::Box<WorkerHealthReport>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerLifecycleEvent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Record of worker lifecycle events."]
        pub worker_lifecycle_event: ::std::option::Option<::std::boxed::Box<WorkerLifecycleEvent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerMessageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A worker message code."]
        pub worker_message_code: ::std::option::Option<::std::boxed::Box<WorkerMessageCode>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource metrics reported by workers."]
        pub worker_metrics: ::std::option::Option<::std::boxed::Box<ResourceUtilizationReport>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerShutdownNotice")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shutdown notice by workers."]
        pub worker_shutdown_notice: ::std::option::Option<::std::boxed::Box<WorkerShutdownNotice>>,
    }
    impl WorkerMessage {
        pub fn builder() -> WorkerMessageBuilder {
            WorkerMessageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A message code is used to report status and error messages to the service. The message codes are intended to be machine readable. The service will take care of translating these into user understandable messages if necessary. Example use cases: 1. Worker processes reporting successful startup. 2. Worker processes reporting specific errors (e.g. package staging failure)."]
    pub struct WorkerMessageCode {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The code is a string intended for consumption by a machine that identifies the type of message being sent. Examples: 1. \"HARNESS_STARTED\" might be used to indicate the worker harness has started. 2. \"GCS_DOWNLOAD_ERROR\" might be used to indicate an error downloading a GCS file as part of the boot process of one of the worker containers. This is a string and not an enum to make it easy to add new codes without waiting for an API change."]
        pub code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters contains specific information about the code. This is a struct to allow parameters of different types. Examples: 1. For a \"HARNESS_STARTED\" message parameters might provide the name of the worker and additional data like timing information. 2. For a \"GCS_DOWNLOAD_ERROR\" parameters might contain fields listing the GCS objects being downloaded and fields containing errors. In general complex data structures should be avoided. If a worker needs to send a specific and complicated data structure then please consider defining a new proto and adding it to the data oneof in WorkerMessageResponse. Conventions: Parameters should only be used for information that isn't typically passed as a label. hostname and other worker identifiers should almost always be passed as labels since they will be included on most messages."]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl WorkerMessageCode {
        pub fn builder() -> WorkerMessageCodeBuilder {
            WorkerMessageCodeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A worker_message response allows the server to pass information to the sender."]
    pub struct WorkerMessageResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerHealthReportResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The service's response to a worker's health report."]
        pub worker_health_report_response:
            ::std::option::Option<::std::boxed::Box<WorkerHealthReportResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerMetricsResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Service's response to reporting worker metrics (currently empty)."]
        pub worker_metrics_response:
            ::std::option::Option<::std::boxed::Box<ResourceUtilizationReportResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerShutdownNoticeResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Service's response to shutdown notice (currently empty)."]
        pub worker_shutdown_notice_response:
            ::std::option::Option<::std::boxed::Box<WorkerShutdownNoticeResponse>>,
    }
    impl WorkerMessageResponse {
        pub fn builder() -> WorkerMessageResponseBuilder {
            WorkerMessageResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes one particular pool of Cloud Dataflow workers to be instantiated by the Cloud Dataflow service in order to perform the computations required by a job. Note that a workflow job may use multiple pools, in order to match the various computational requirements of the various stages of the job."]
    pub struct WorkerPool {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoscalingSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Settings for autoscaling of this WorkerPool."]
        pub autoscaling_settings: ::std::option::Option<::std::boxed::Box<AutoscalingSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataDisks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data disks that are used by a VM in this workflow."]
        pub data_disks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Disk>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultPackageSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The default package set to install. This allows the service to select a default set of packages which are useful to worker harnesses written in a particular language."]
        pub default_package_set: ::std::option::Option<WorkerPoolDefaultPackageSetEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskSizeGb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Size of root disk for VMs, in GB. If zero or unspecified, the service will attempt to choose a reasonable default."]
        pub disk_size_gb: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskSourceImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fully qualified source image for disks."]
        pub disk_source_image: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of root disk for VMs. If empty or unspecified, the service will attempt to choose a reasonable default."]
        pub disk_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipConfiguration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for VM IPs."]
        pub ip_configuration: ::std::option::Option<WorkerPoolIpConfigurationEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of the worker pool; currently only `harness` and `shuffle` are supported."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "machineType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Machine type (e.g. \"n1-standard-1\"). If empty or unspecified, the service will attempt to choose a reasonable default."]
        pub machine_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata to set on the Google Compute Engine VMs."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Network to which VMs will be assigned. If empty or unspecified, the service will use the network \"default\"."]
        pub network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numThreadsPerWorker")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of threads per worker harness. If empty or unspecified, the service will choose a number of threads (according to the number of cores on the selected machine type for batch, or 1 by convention for streaming)."]
        pub num_threads_per_worker: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numWorkers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of Google Compute Engine workers in this pool needed to execute the job. If zero or unspecified, the service will attempt to choose a reasonable default."]
        pub num_workers: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "onHostMaintenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The action to take on host maintenance, as defined by the Google Compute Engine API."]
        pub on_host_maintenance: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Packages to be installed on workers."]
        pub packages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Package>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "poolArgs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Extra arguments for this worker pool."]
        pub pool_args:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sdkHarnessContainerImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of SDK harness containers needed to execute this pipeline. This will only be set in the Fn API path. For non-cross-language pipelines this should have only one entry. Cross-language pipelines will have two or more entries."]
        pub sdk_harness_container_images:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SdkHarnessContainerImage>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subnetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subnetwork to which VMs will be assigned, if desired. Expected to be of the form \"regions/REGION/subnetworks/SUBNETWORK\"."]
        pub subnetwork: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taskrunnerSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Settings passed through to Google Compute Engine workers when using the standard Dataflow task runner. Users should ignore this field."]
        pub taskrunner_settings: ::std::option::Option<::std::boxed::Box<TaskRunnerSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teardownPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sets the policy for determining when to turndown worker pool. Allowed values are: `TEARDOWN_ALWAYS`, `TEARDOWN_ON_SUCCESS`, and `TEARDOWN_NEVER`. `TEARDOWN_ALWAYS` means workers are always torn down regardless of whether the job succeeds. `TEARDOWN_ON_SUCCESS` means workers are torn down if the job succeeds. `TEARDOWN_NEVER` means the workers are never torn down. If the workers are not torn down by the service, they will continue to run and use Google Compute Engine VM resources in the user's project until they are explicitly terminated by the user. Because of this, Google recommends using the `TEARDOWN_ALWAYS` policy except for small, manually supervised test jobs. If unknown or unspecified, the service will attempt to choose a reasonable default."]
        pub teardown_policy: ::std::option::Option<WorkerPoolTeardownPolicyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerHarnessContainerImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Docker container image that executes the Cloud Dataflow worker harness, residing in Google Container Registry. Deprecated for the Fn API path. Use sdk_harness_container_images instead."]
        pub worker_harness_container_image: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Zone to run the worker pools in. If empty or unspecified, the service will attempt to choose a reasonable default."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl WorkerPool {
        pub fn builder() -> WorkerPoolBuilder {
            WorkerPoolBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The default package set to install. This allows the service to select a default set of packages which are useful to worker harnesses written in a particular language."]
    pub enum WorkerPoolDefaultPackageSetEnum {
        #[serde(rename = "DEFAULT_PACKAGE_SET_UNKNOWN")]
        #[doc = "The default set of packages to stage is unknown, or unspecified."]
        DefaultPackageSetUnknown,
        #[serde(rename = "DEFAULT_PACKAGE_SET_NONE")]
        #[doc = "Indicates that no packages should be staged at the worker unless explicitly specified by the job."]
        DefaultPackageSetNone,
        #[serde(rename = "DEFAULT_PACKAGE_SET_JAVA")]
        #[doc = "Stage packages typically useful to workers written in Java."]
        DefaultPackageSetJava,
        #[serde(rename = "DEFAULT_PACKAGE_SET_PYTHON")]
        #[doc = "Stage pacakges typically useful to workers written in Python."]
        DefaultPackageSetPython,
    }
    impl ::std::default::Default for WorkerPoolDefaultPackageSetEnum {
        fn default() -> Self {
            Self::DefaultPackageSetUnknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Configuration for VM IPs."]
    pub enum WorkerPoolIpConfigurationEnum {
        #[serde(rename = "WORKER_IP_UNSPECIFIED")]
        #[doc = "The configuration is unknown, or unspecified."]
        WorkerIpUnspecified,
        #[serde(rename = "WORKER_IP_PUBLIC")]
        #[doc = "Workers should have public IP addresses."]
        WorkerIpPublic,
        #[serde(rename = "WORKER_IP_PRIVATE")]
        #[doc = "Workers should have private IP addresses."]
        WorkerIpPrivate,
    }
    impl ::std::default::Default for WorkerPoolIpConfigurationEnum {
        fn default() -> Self {
            Self::WorkerIpUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Sets the policy for determining when to turndown worker pool. Allowed values are: `TEARDOWN_ALWAYS`, `TEARDOWN_ON_SUCCESS`, and `TEARDOWN_NEVER`. `TEARDOWN_ALWAYS` means workers are always torn down regardless of whether the job succeeds. `TEARDOWN_ON_SUCCESS` means workers are torn down if the job succeeds. `TEARDOWN_NEVER` means the workers are never torn down. If the workers are not torn down by the service, they will continue to run and use Google Compute Engine VM resources in the user's project until they are explicitly terminated by the user. Because of this, Google recommends using the `TEARDOWN_ALWAYS` policy except for small, manually supervised test jobs. If unknown or unspecified, the service will attempt to choose a reasonable default."]
    pub enum WorkerPoolTeardownPolicyEnum {
        #[serde(rename = "TEARDOWN_POLICY_UNKNOWN")]
        #[doc = "The teardown policy isn't specified, or is unknown."]
        TeardownPolicyUnknown,
        #[serde(rename = "TEARDOWN_ALWAYS")]
        #[doc = "Always teardown the resource."]
        TeardownAlways,
        #[serde(rename = "TEARDOWN_ON_SUCCESS")]
        #[doc = "Teardown the resource on success. This is useful for debugging failures."]
        TeardownOnSuccess,
        #[serde(rename = "TEARDOWN_NEVER")]
        #[doc = "Never teardown the resource. This is useful for debugging and development."]
        TeardownNever,
    }
    impl ::std::default::Default for WorkerPoolTeardownPolicyEnum {
        fn default() -> Self {
            Self::TeardownPolicyUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides data to pass through to the worker harness."]
    pub struct WorkerSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "baseUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The base URL for accessing Google Cloud APIs. When workers access Google Cloud APIs, they logically do so via relative URLs. If this field is specified, it supplies the base URL to use for resolving these relative URLs. The normative algorithm used is defined by RFC 1808, \"Relative Uniform Resource Locators\". If not specified, the default value is \"http://www.googleapis.com/\""]
        pub base_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportingEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to send work progress updates to the service."]
        pub reporting_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "servicePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud Dataflow service path relative to the root URL, for example, \"dataflow/v1b3/projects\"."]
        pub service_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shuffleServicePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Shuffle service path relative to the root URL, for example, \"shuffle/v1beta1\"."]
        pub shuffle_service_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tempStoragePrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The prefix of the resources the system should use for temporary storage. The supported resource type is: Google Cloud Storage: storage.googleapis.com/{bucket}/{object} bucket.storage.googleapis.com/{object}"]
        pub temp_storage_prefix: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the worker running this pipeline."]
        pub worker_id: ::std::option::Option<::std::string::String>,
    }
    impl WorkerSettings {
        pub fn builder() -> WorkerSettingsBuilder {
            WorkerSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Shutdown notification from workers. This is to be sent by the shutdown script of the worker VM so that the backend knows that the VM is being shut down."]
    pub struct WorkerShutdownNotice {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the worker shutdown. Current possible values are: \"UNKNOWN\": shutdown reason is unknown. \"PREEMPTION\": shutdown reason is preemption. Other possible reasons may be added in the future."]
        pub reason: ::std::option::Option<::std::string::String>,
    }
    impl WorkerShutdownNotice {
        pub fn builder() -> WorkerShutdownNoticeBuilder {
            WorkerShutdownNoticeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Service-side response to WorkerMessage issuing shutdown notice."]
    pub struct WorkerShutdownNoticeResponse {}
    impl WorkerShutdownNoticeResponse {
        pub fn builder() -> WorkerShutdownNoticeResponseBuilder {
            WorkerShutdownNoticeResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An instruction that writes records. Takes one input, produces no outputs."]
    pub struct WriteInstruction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "input")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The input."]
        pub input: ::std::option::Option<::std::boxed::Box<InstructionInput>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sink to write to."]
        pub sink: ::std::option::Option<::std::boxed::Box<Sink>>,
    }
    impl WriteInstruction {
        pub fn builder() -> WriteInstructionBuilder {
            WriteInstructionBuilder::default()
        }
    }
}
