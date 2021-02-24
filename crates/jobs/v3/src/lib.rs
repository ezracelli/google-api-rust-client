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
    pub mod projects {
        pub mod methods {
            pub mod complete {
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
                    #[serde(rename = "companyName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. If provided, restricts completion to specified company. The format is \"projects/{project_id}/companies/{company_id}\", for example, \"projects/api-test-project/companies/foo\"."]
                    pub company_name: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "languageCode")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated. Use language_codes instead. Optional. The language of the query. This is the BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47). For CompletionType.JOB_TITLE type, only open jobs with the same language_code are returned. For CompletionType.COMPANY_NAME type, only companies having open jobs with the same language_code are returned. For CompletionType.COMBINED type, only open jobs with the same language_code or companies having open jobs with the same language_code are returned. The maximum number of allowed characters is 255."]
                    pub language_code: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "languageCodes")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. The list of languages of the query. This is the BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47). For CompletionType.JOB_TITLE type, only open jobs with the same language_codes are returned. For CompletionType.COMPANY_NAME type, only companies having open jobs with the same language_codes are returned. For CompletionType.COMBINED type, only open jobs with the same language_codes or companies having open jobs with the same language_codes are returned. The maximum number of allowed characters is 255."]
                    pub language_codes: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. Completion result count. The maximum allowed page size is 10."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "query")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. The query used to generate suggestions. The maximum number of allowed characters is 255."]
                    pub query: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "scope")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. The scope of the completion. The defaults is CompletionScope.PUBLIC."]
                    pub scope: ::std::option::Option<QueryParametersScopeEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "type")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. The completion topic. The default is CompletionType.COMBINED."]
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
                #[doc = "Optional. The scope of the completion. The defaults is CompletionScope.PUBLIC."]
                pub enum QueryParametersScopeEnum {
                    #[serde(rename = "COMPLETION_SCOPE_UNSPECIFIED")]
                    #[doc = "Default value."]
                    CompletionScopeUnspecified,
                    #[serde(rename = "TENANT")]
                    #[doc = "Suggestions are based only on the data provided by the client."]
                    Tenant,
                    #[serde(rename = "PUBLIC")]
                    #[doc = "Suggestions are based on all jobs data in the system that's visible to the client"]
                    Public,
                }
                impl ::std::default::Default for QueryParametersScopeEnum {
                    fn default() -> Self {
                        Self::CompletionScopeUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Optional. The completion topic. The default is CompletionType.COMBINED."]
                pub enum QueryParametersTypeEnum {
                    #[serde(rename = "COMPLETION_TYPE_UNSPECIFIED")]
                    #[doc = "Default value."]
                    CompletionTypeUnspecified,
                    #[serde(rename = "JOB_TITLE")]
                    #[doc = "Only suggest job titles."]
                    JobTitle,
                    #[serde(rename = "COMPANY_NAME")]
                    #[doc = "Only suggest company names."]
                    CompanyName,
                    #[serde(rename = "COMBINED")]
                    #[doc = "Suggest both job titles and company names."]
                    Combined,
                }
                impl ::std::default::Default for QueryParametersTypeEnum {
                    fn default() -> Self {
                        Self::CompletionTypeUnspecified
                    }
                }
            }
        }
        pub mod resources {
            pub mod companies {
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
                            #[doc = "Optional. The maximum number of companies to be returned, at most 100. Default is 100 if a non-positive number is provided."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The starting indicator from which to return results."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "requireOpenJobs")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Set to true if the companies requested must have open jobs. Defaults to false. If true, at most page_size of companies are fetched, among which only those with open jobs are returned."]
                            pub require_open_jobs: ::std::option::Option<::std::primitive::bool>,
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
                            #[doc = "Required. The filter string specifies the jobs to be enumerated. Supported operator: =, AND The fields eligible for filtering are: * `companyName` (Required) * `requisitionId` (Optional) Sample Query: * companyName = \"projects/api-test-project/companies/123\" * companyName = \"projects/api-test-project/companies/123\" AND requisitionId = \"req-1\""]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "jobView")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The desired job attributes returned for jobs in the search response. Defaults to JobView.JOB_VIEW_FULL if no value is specified."]
                            pub job_view: ::std::option::Option<QueryParametersJobViewEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The maximum number of jobs to be returned per page of results. If job_view is set to JobView.JOB_VIEW_ID_ONLY, the maximum allowed page size is 1000. Otherwise, the maximum allowed page size is 100. Default is 100 if empty or a number < 1 is specified."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The starting point of a query result."]
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
                        #[doc = "Optional. The desired job attributes returned for jobs in the search response. Defaults to JobView.JOB_VIEW_FULL if no value is specified."]
                        pub enum QueryParametersJobViewEnum {
                            #[serde(rename = "JOB_VIEW_UNSPECIFIED")]
                            #[doc = "Default value."]
                            JobViewUnspecified,
                            #[serde(rename = "JOB_VIEW_ID_ONLY")]
                            #[doc = "A ID only view of job, with following attributes: Job.name, Job.requisition_id, Job.language_code."]
                            JobViewIdOnly,
                            #[serde(rename = "JOB_VIEW_MINIMAL")]
                            #[doc = "A minimal view of the job, with the following attributes: Job.name, Job.requisition_id, Job.title, Job.company_name, Job.DerivedInfo.locations, Job.language_code."]
                            JobViewMinimal,
                            #[serde(rename = "JOB_VIEW_SMALL")]
                            #[doc = "A small view of the job, with the following attributes in the search results: Job.name, Job.requisition_id, Job.title, Job.company_name, Job.DerivedInfo.locations, Job.visibility, Job.language_code, Job.description."]
                            JobViewSmall,
                            #[serde(rename = "JOB_VIEW_FULL")]
                            #[doc = "All available attributes are included in the search results."]
                            JobViewFull,
                        }
                        impl ::std::default::Default for QueryParametersJobViewEnum {
                            fn default() -> Self {
                                Self::JobViewUnspecified
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
    #[doc = "Application related details of a job posting."]
    pub struct ApplicationInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional but at least one of uris, emails or instruction must be specified. Use this field to specify email address(es) to which resumes or applications can be sent. The maximum number of allowed characters for each entry is 255."]
        pub emails: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instruction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional but at least one of uris, emails or instruction must be specified. Use this field to provide instructions, such as \"Mail your application to ...\", that a candidate can follow to apply for the job. This field accepts and sanitizes HTML input, and also accepts bold, italic, ordered list, and unordered list markup tags. The maximum number of allowed characters is 3,000."]
        pub instruction: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uris")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional but at least one of uris, emails or instruction must be specified. Use this URI field to direct an applicant to a website, for example to link to an online application form. The maximum number of allowed characters for each entry is 2,000."]
        pub uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ApplicationInfo {
        pub fn builder() -> ApplicationInfoBuilder {
            ApplicationInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Input only. Batch delete jobs request."]
    pub struct BatchDeleteJobsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The filter string specifies the jobs to be deleted. Supported operator: =, AND The fields eligible for filtering are: * `companyName` (Required) * `requisitionId` (Required) Sample Query: companyName = \"projects/api-test-project/companies/123\" AND requisitionId = \"req-1\""]
        pub filter: ::std::option::Option<::std::string::String>,
    }
    impl BatchDeleteJobsRequest {
        pub fn builder() -> BatchDeleteJobsRequestBuilder {
            BatchDeleteJobsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents starting and ending value of a range in double."]
    pub struct BucketRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "from")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Starting value of the bucket range."]
        pub from: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "to")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ending value of the bucket range."]
        pub to: ::std::option::Option<::std::primitive::f64>,
    }
    impl BucketRange {
        pub fn builder() -> BucketRangeBuilder {
            BucketRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents count of jobs within one bucket."]
    pub struct BucketizedCount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of jobs whose numeric field value fall into `range`."]
        pub count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bucket range on which histogram was performed for the numeric field, that is, the count represents number of jobs in this range."]
        pub range: ::std::option::Option<::std::boxed::Box<BucketRange>>,
    }
    impl BucketizedCount {
        pub fn builder() -> BucketizedCountBuilder {
            BucketizedCountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event issued when an end user interacts with the application that implements Cloud Talent Solution. Providing this information improves the quality of search and recommendation for the API clients, enabling the service to perform optimally. The number of events sent must be consistent with other calls, such as job searches, issued to the service by the client."]
    pub struct ClientEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The timestamp of the event."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A unique identifier, generated by the client application. This `event_id` is used to establish the relationship between different events (see parent_event_id)."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "extraInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Extra information about this event. Used for storing information with no matching field in event payload, for example, user application specific context or details. At most 20 keys are supported. The maximum total size of all keys and values is 2 KB."]
        pub extra_info:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobEvent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A event issued when a job seeker interacts with the application that implements Cloud Talent Solution."]
        pub job_event: ::std::option::Option<::std::boxed::Box<JobEvent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentEventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The event_id of an event that resulted in the current event. For example, a Job view event usually follows a parent impression event: A job seeker first does a search where a list of jobs appears (impression). The job seeker then selects a result and views the description of a particular job (Job view)."]
        pub parent_event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A unique ID generated in the API responses. It can be found in ResponseMetadata.request_id."]
        pub request_id: ::std::option::Option<::std::string::String>,
    }
    impl ClientEvent {
        pub fn builder() -> ClientEventBuilder {
            ClientEventBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Input only. Parameters needed for commute search."]
    pub struct CommuteFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowImpreciseAddresses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If true, jobs without \"precise\" addresses (street level addresses or GPS coordinates) might also be returned. For city and coarser level addresses, text matching is used. If this field is set to false or is not specified, only jobs that include precise addresses are returned by Commute Search. Note: If `allow_imprecise_addresses` is set to true, Commute Search is not able to calculate accurate commute times to jobs with city level and coarser address information. Jobs with imprecise addresses will return a `travel_duration` time of 0 regardless of distance from the job seeker."]
        pub allow_imprecise_addresses: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commuteMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The method of transportation for which to calculate the commute time."]
        pub commute_method: ::std::option::Option<CommuteFilterCommuteMethodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "departureTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The departure time used to calculate traffic impact, represented as google.type.TimeOfDay in local time zone. Currently traffic model is restricted to hour level resolution."]
        pub departure_time: ::std::option::Option<::std::boxed::Box<TimeOfDay>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "roadTraffic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Specifies the traffic density to use when calculating commute time."]
        pub road_traffic: ::std::option::Option<CommuteFilterRoadTrafficEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startCoordinates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The latitude and longitude of the location from which to calculate the commute time."]
        pub start_coordinates: ::std::option::Option<::std::boxed::Box<LatLng>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "travelDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The maximum travel time in seconds. The maximum allowed value is `3600s` (one hour). Format is `123s`."]
        pub travel_duration: ::std::option::Option<::std::string::String>,
    }
    impl CommuteFilter {
        pub fn builder() -> CommuteFilterBuilder {
            CommuteFilterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The method of transportation for which to calculate the commute time."]
    pub enum CommuteFilterCommuteMethodEnum {
        #[serde(rename = "COMMUTE_METHOD_UNSPECIFIED")]
        #[doc = "Commute method is not specified."]
        CommuteMethodUnspecified,
        #[serde(rename = "DRIVING")]
        #[doc = "Commute time is calculated based on driving time."]
        Driving,
        #[serde(rename = "TRANSIT")]
        #[doc = "Commute time is calculated based on public transit including bus, metro, subway, etc."]
        Transit,
    }
    impl ::std::default::Default for CommuteFilterCommuteMethodEnum {
        fn default() -> Self {
            Self::CommuteMethodUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Specifies the traffic density to use when calculating commute time."]
    pub enum CommuteFilterRoadTrafficEnum {
        #[serde(rename = "ROAD_TRAFFIC_UNSPECIFIED")]
        #[doc = "Road traffic situation is not specified."]
        RoadTrafficUnspecified,
        #[serde(rename = "TRAFFIC_FREE")]
        #[doc = "Optimal commute time without considering any traffic impact."]
        TrafficFree,
        #[serde(rename = "BUSY_HOUR")]
        #[doc = "Commute time calculation takes in account the peak traffic impact."]
        BusyHour,
    }
    impl ::std::default::Default for CommuteFilterRoadTrafficEnum {
        fn default() -> Self {
            Self::RoadTrafficUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. Commute details related to this job."]
    pub struct CommuteInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location used as the destination in the commute calculation."]
        pub job_location: ::std::option::Option<::std::boxed::Box<Location>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "travelDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of seconds required to travel to the job location from the query location. A duration of 0 seconds indicates that the job is not reachable within the requested duration, but was returned as part of an expanded query."]
        pub travel_duration: ::std::option::Option<::std::string::String>,
    }
    impl CommuteInfo {
        pub fn builder() -> CommuteInfoBuilder {
            CommuteInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Company resource represents a company in the service. A company is the entity that owns job postings, that is, the hiring entity responsible for employing applicants for the job position."]
    pub struct Company {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "careerSiteUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The URI to employer's career site or careers page on the employer's web site, for example, \"https://careers.google.com\"."]
        pub career_site_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "derivedInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Derived details about the company."]
        pub derived_info: ::std::option::Option<::std::boxed::Box<CompanyDerivedInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The display name of the company, for example, \"Google LLC\"."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eeoText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Equal Employment Opportunity legal disclaimer text to be associated with all jobs, and typically to be displayed in all roles. The maximum number of allowed characters is 500."]
        pub eeo_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "externalId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Client side company identifier, used to uniquely identify the company. The maximum number of allowed characters is 255."]
        pub external_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headquartersAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The street address of the company's main headquarters, which may be different from the job location. The service attempts to geolocate the provided address, and populates a more specific location wherever possible in DerivedInfo.headquarters_location."]
        pub headquarters_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hiringAgency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Set to true if it is the hiring agency that post jobs for other employers. Defaults to false if not provided."]
        pub hiring_agency: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A URI that hosts the employer's company logo."]
        pub image_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keywordSearchableJobCustomAttributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A list of keys of filterable Job.custom_attributes, whose corresponding `string_values` are used in keyword search. Jobs with `string_values` under these specified field keys are returned if any of the values matches the search keyword. Custom field values with parenthesis, brackets and special symbols won't be properly searchable, and those keyword queries need to be surrounded by quotes."]
        pub keyword_searchable_job_custom_attributes:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required during company update. The resource name for a company. This is generated by the service when a company is created. The format is \"projects/{project_id}/companies/{company_id}\", for example, \"projects/api-test-project/companies/foo\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The employer's company size."]
        pub size: ::std::option::Option<CompanySizeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suspended")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Indicates whether a company is flagged to be suspended from public availability by the service when job content appears suspicious, abusive, or spammy."]
        pub suspended: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "websiteUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The URI representing the company's primary web site or home page, for example, \"https://www.google.com\". The maximum number of allowed characters is 255."]
        pub website_uri: ::std::option::Option<::std::string::String>,
    }
    impl Company {
        pub fn builder() -> CompanyBuilder {
            CompanyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The employer's company size."]
    pub enum CompanySizeEnum {
        #[serde(rename = "COMPANY_SIZE_UNSPECIFIED")]
        #[doc = "Default value if the size is not specified."]
        CompanySizeUnspecified,
        #[serde(rename = "MINI")]
        #[doc = "The company has less than 50 employees."]
        Mini,
        #[serde(rename = "SMALL")]
        #[doc = "The company has between 50 and 99 employees."]
        Small,
        #[serde(rename = "SMEDIUM")]
        #[doc = "The company has between 100 and 499 employees."]
        Smedium,
        #[serde(rename = "MEDIUM")]
        #[doc = "The company has between 500 and 999 employees."]
        Medium,
        #[serde(rename = "BIG")]
        #[doc = "The company has between 1,000 and 4,999 employees."]
        Big,
        #[serde(rename = "BIGGER")]
        #[doc = "The company has between 5,000 and 9,999 employees."]
        Bigger,
        #[serde(rename = "GIANT")]
        #[doc = "The company has 10,000 or more employees."]
        Giant,
    }
    impl ::std::default::Default for CompanySizeEnum {
        fn default() -> Self {
            Self::CompanySizeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Derived details about the company."]
    pub struct CompanyDerivedInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headquartersLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A structured headquarters location of the company, resolved from Company.hq_location if provided."]
        pub headquarters_location: ::std::option::Option<::std::boxed::Box<Location>>,
    }
    impl CompanyDerivedInfo {
        pub fn builder() -> CompanyDerivedInfoBuilder {
            CompanyDerivedInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A compensation entry that represents one component of compensation, such as base pay, bonus, or other compensation type. Annualization: One compensation entry can be annualized if - it contains valid amount or range. - and its expected_units_per_year is set or can be derived. Its annualized range is determined as (amount or range) times expected_units_per_year."]
    pub struct CompensationEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Compensation amount."]
        pub amount: ::std::option::Option<::std::boxed::Box<Money>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Compensation description. For example, could indicate equity terms or provide additional context to an estimated bonus."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expectedUnitsPerYear")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Expected number of units paid each year. If not specified, when Job.employment_types is FULLTIME, a default value is inferred based on unit. Default values: - HOURLY: 2080 - DAILY: 260 - WEEKLY: 52 - MONTHLY: 12 - ANNUAL: 1"]
        pub expected_units_per_year: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Compensation range."]
        pub range: ::std::option::Option<::std::boxed::Box<CompensationRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Compensation type. Default is CompensationUnit.COMPENSATION_TYPE_UNSPECIFIED."]
        pub _type: ::std::option::Option<CompensationEntryTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Frequency of the specified amount. Default is CompensationUnit.COMPENSATION_UNIT_UNSPECIFIED."]
        pub unit: ::std::option::Option<CompensationEntryUnitEnum>,
    }
    impl CompensationEntry {
        pub fn builder() -> CompensationEntryBuilder {
            CompensationEntryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Compensation type. Default is CompensationUnit.COMPENSATION_TYPE_UNSPECIFIED."]
    pub enum CompensationEntryTypeEnum {
        #[serde(rename = "COMPENSATION_TYPE_UNSPECIFIED")]
        #[doc = "Default value."]
        CompensationTypeUnspecified,
        #[serde(rename = "BASE")]
        #[doc = "Base compensation: Refers to the fixed amount of money paid to an employee by an employer in return for work performed. Base compensation does not include benefits, bonuses or any other potential compensation from an employer."]
        Base,
        #[serde(rename = "BONUS")]
        #[doc = "Bonus."]
        Bonus,
        #[serde(rename = "SIGNING_BONUS")]
        #[doc = "Signing bonus."]
        SigningBonus,
        #[serde(rename = "EQUITY")]
        #[doc = "Equity."]
        Equity,
        #[serde(rename = "PROFIT_SHARING")]
        #[doc = "Profit sharing."]
        ProfitSharing,
        #[serde(rename = "COMMISSIONS")]
        #[doc = "Commission."]
        Commissions,
        #[serde(rename = "TIPS")]
        #[doc = "Tips."]
        Tips,
        #[serde(rename = "OTHER_COMPENSATION_TYPE")]
        #[doc = "Other compensation type."]
        OtherCompensationType,
    }
    impl ::std::default::Default for CompensationEntryTypeEnum {
        fn default() -> Self {
            Self::CompensationTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Frequency of the specified amount. Default is CompensationUnit.COMPENSATION_UNIT_UNSPECIFIED."]
    pub enum CompensationEntryUnitEnum {
        #[serde(rename = "COMPENSATION_UNIT_UNSPECIFIED")]
        #[doc = "Default value."]
        CompensationUnitUnspecified,
        #[serde(rename = "HOURLY")]
        #[doc = "Hourly."]
        Hourly,
        #[serde(rename = "DAILY")]
        #[doc = "Daily."]
        Daily,
        #[serde(rename = "WEEKLY")]
        #[doc = "Weekly"]
        Weekly,
        #[serde(rename = "MONTHLY")]
        #[doc = "Monthly."]
        Monthly,
        #[serde(rename = "YEARLY")]
        #[doc = "Yearly."]
        Yearly,
        #[serde(rename = "ONE_TIME")]
        #[doc = "One time."]
        OneTime,
        #[serde(rename = "OTHER_COMPENSATION_UNIT")]
        #[doc = "Other compensation units."]
        OtherCompensationUnit,
    }
    impl ::std::default::Default for CompensationEntryUnitEnum {
        fn default() -> Self {
            Self::CompensationUnitUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Input only. Filter on job compensation type and amount."]
    pub struct CompensationFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeJobsWithUnspecifiedCompensationRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If set to true, jobs with unspecified compensation range fields are included."]
        pub include_jobs_with_unspecified_compensation_range:
            ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Compensation range."]
        pub range: ::std::option::Option<::std::boxed::Box<CompensationRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Type of filter."]
        pub _type: ::std::option::Option<CompensationFilterTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "units")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Specify desired `base compensation entry's` CompensationInfo.CompensationUnit."]
        pub units: ::std::option::Option<::std::vec::Vec<CompensationFilterUnitsEnum>>,
    }
    impl CompensationFilter {
        pub fn builder() -> CompensationFilterBuilder {
            CompensationFilterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Type of filter."]
    pub enum CompensationFilterTypeEnum {
        #[serde(rename = "FILTER_TYPE_UNSPECIFIED")]
        #[doc = "Filter type unspecified. Position holder, INVALID, should never be used."]
        FilterTypeUnspecified,
        #[serde(rename = "UNIT_ONLY")]
        #[doc = "Filter by `base compensation entry's` unit. A job is a match if and only if the job contains a base CompensationEntry and the base CompensationEntry's unit matches provided units. Populate one or more units. See CompensationInfo.CompensationEntry for definition of base compensation entry."]
        UnitOnly,
        #[serde(rename = "UNIT_AND_AMOUNT")]
        #[doc = "Filter by `base compensation entry's` unit and amount / range. A job is a match if and only if the job contains a base CompensationEntry, and the base entry's unit matches provided compensation_units and amount or range overlaps with provided compensation_range. See CompensationInfo.CompensationEntry for definition of base compensation entry. Set exactly one units and populate range."]
        UnitAndAmount,
        #[serde(rename = "ANNUALIZED_BASE_AMOUNT")]
        #[doc = "Filter by annualized base compensation amount and `base compensation entry's` unit. Populate range and zero or more units."]
        AnnualizedBaseAmount,
        #[serde(rename = "ANNUALIZED_TOTAL_AMOUNT")]
        #[doc = "Filter by annualized total compensation amount and `base compensation entry's` unit . Populate range and zero or more units."]
        AnnualizedTotalAmount,
    }
    impl ::std::default::Default for CompensationFilterTypeEnum {
        fn default() -> Self {
            Self::FilterTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum CompensationFilterUnitsEnum {
        #[serde(rename = "COMPENSATION_UNIT_UNSPECIFIED")]
        #[doc = "Default value."]
        CompensationUnitUnspecified,
        #[serde(rename = "HOURLY")]
        #[doc = "Hourly."]
        Hourly,
        #[serde(rename = "DAILY")]
        #[doc = "Daily."]
        Daily,
        #[serde(rename = "WEEKLY")]
        #[doc = "Weekly"]
        Weekly,
        #[serde(rename = "MONTHLY")]
        #[doc = "Monthly."]
        Monthly,
        #[serde(rename = "YEARLY")]
        #[doc = "Yearly."]
        Yearly,
        #[serde(rename = "ONE_TIME")]
        #[doc = "One time."]
        OneTime,
        #[serde(rename = "OTHER_COMPENSATION_UNIT")]
        #[doc = "Other compensation units."]
        OtherCompensationUnit,
    }
    impl ::std::default::Default for CompensationFilterUnitsEnum {
        fn default() -> Self {
            Self::CompensationUnitUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Input only. Compensation based histogram request."]
    pub struct CompensationHistogramRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketingOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Numeric histogram options, like buckets, whether include min or max value."]
        pub bucketing_option: ::std::option::Option<::std::boxed::Box<NumericBucketingOption>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Type of the request, representing which field the histogramming should be performed over. A single request can only specify one histogram of each `CompensationHistogramRequestType`."]
        pub _type: ::std::option::Option<CompensationHistogramRequestTypeEnum>,
    }
    impl CompensationHistogramRequest {
        pub fn builder() -> CompensationHistogramRequestBuilder {
            CompensationHistogramRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Type of the request, representing which field the histogramming should be performed over. A single request can only specify one histogram of each `CompensationHistogramRequestType`."]
    pub enum CompensationHistogramRequestTypeEnum {
        #[serde(rename = "COMPENSATION_HISTOGRAM_REQUEST_TYPE_UNSPECIFIED")]
        #[doc = "Default value. Invalid."]
        CompensationHistogramRequestTypeUnspecified,
        #[serde(rename = "BASE")]
        #[doc = "Histogram by job's base compensation. See CompensationEntry for definition of base compensation."]
        Base,
        #[serde(rename = "ANNUALIZED_BASE")]
        #[doc = "Histogram by job's annualized base compensation. See CompensationEntry for definition of annualized base compensation."]
        AnnualizedBase,
        #[serde(rename = "ANNUALIZED_TOTAL")]
        #[doc = "Histogram by job's annualized total compensation. See CompensationEntry for definition of annualized total compensation."]
        AnnualizedTotal,
    }
    impl ::std::default::Default for CompensationHistogramRequestTypeEnum {
        fn default() -> Self {
            Self::CompensationHistogramRequestTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. Compensation based histogram result."]
    pub struct CompensationHistogramResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "result")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Histogram result."]
        pub result: ::std::option::Option<::std::boxed::Box<NumericBucketingResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the request, corresponding to CompensationHistogramRequest.type."]
        pub _type: ::std::option::Option<CompensationHistogramResultTypeEnum>,
    }
    impl CompensationHistogramResult {
        pub fn builder() -> CompensationHistogramResultBuilder {
            CompensationHistogramResultBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of the request, corresponding to CompensationHistogramRequest.type."]
    pub enum CompensationHistogramResultTypeEnum {
        #[serde(rename = "COMPENSATION_HISTOGRAM_REQUEST_TYPE_UNSPECIFIED")]
        #[doc = "Default value. Invalid."]
        CompensationHistogramRequestTypeUnspecified,
        #[serde(rename = "BASE")]
        #[doc = "Histogram by job's base compensation. See CompensationEntry for definition of base compensation."]
        Base,
        #[serde(rename = "ANNUALIZED_BASE")]
        #[doc = "Histogram by job's annualized base compensation. See CompensationEntry for definition of annualized base compensation."]
        AnnualizedBase,
        #[serde(rename = "ANNUALIZED_TOTAL")]
        #[doc = "Histogram by job's annualized total compensation. See CompensationEntry for definition of annualized total compensation."]
        AnnualizedTotal,
    }
    impl ::std::default::Default for CompensationHistogramResultTypeEnum {
        fn default() -> Self {
            Self::CompensationHistogramRequestTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Job compensation details."]
    pub struct CompensationInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annualizedBaseCompensationRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Annualized base compensation range. Computed as base compensation entry's CompensationEntry.compensation times CompensationEntry.expected_units_per_year. See CompensationEntry for explanation on compensation annualization."]
        pub annualized_base_compensation_range:
            ::std::option::Option<::std::boxed::Box<CompensationRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annualizedTotalCompensationRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Annualized total compensation range. Computed as all compensation entries' CompensationEntry.compensation times CompensationEntry.expected_units_per_year. See CompensationEntry for explanation on compensation annualization."]
        pub annualized_total_compensation_range:
            ::std::option::Option<::std::boxed::Box<CompensationRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Job compensation information. At most one entry can be of type CompensationInfo.CompensationType.BASE, which is referred as ** base compensation entry ** for the job."]
        pub entries: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CompensationEntry>>>,
    }
    impl CompensationInfo {
        pub fn builder() -> CompensationInfoBuilder {
            CompensationInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Compensation range."]
    pub struct CompensationRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxCompensation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The maximum amount of compensation. If left empty, the value is set to a maximal compensation value and the currency code is set to match the currency code of min_compensation."]
        pub max_compensation: ::std::option::Option<::std::boxed::Box<Money>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minCompensation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The minimum amount of compensation. If left empty, the value is set to zero and the currency code is set to match the currency code of max_compensation."]
        pub min_compensation: ::std::option::Option<::std::boxed::Box<Money>>,
    }
    impl CompensationRange {
        pub fn builder() -> CompensationRangeBuilder {
            CompensationRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. Response of auto-complete query."]
    pub struct CompleteQueryResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "completionResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Results of the matching job/company candidates."]
        pub completion_results:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CompletionResult>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information for the API invocation, such as the request tracking id."]
        pub metadata: ::std::option::Option<::std::boxed::Box<ResponseMetadata>>,
    }
    impl CompleteQueryResponse {
        pub fn builder() -> CompleteQueryResponseBuilder {
            CompleteQueryResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. Resource that represents completion results."]
    pub struct CompletionResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of the company image for CompletionType.COMPANY_NAME."]
        pub image_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suggestion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The suggestion for the query."]
        pub suggestion: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The completion topic."]
        pub _type: ::std::option::Option<CompletionResultTypeEnum>,
    }
    impl CompletionResult {
        pub fn builder() -> CompletionResultBuilder {
            CompletionResultBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The completion topic."]
    pub enum CompletionResultTypeEnum {
        #[serde(rename = "COMPLETION_TYPE_UNSPECIFIED")]
        #[doc = "Default value."]
        CompletionTypeUnspecified,
        #[serde(rename = "JOB_TITLE")]
        #[doc = "Only suggest job titles."]
        JobTitle,
        #[serde(rename = "COMPANY_NAME")]
        #[doc = "Only suggest company names."]
        CompanyName,
        #[serde(rename = "COMBINED")]
        #[doc = "Suggest both job titles and company names."]
        Combined,
    }
    impl ::std::default::Default for CompletionResultTypeEnum {
        fn default() -> Self {
            Self::CompletionTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The report event request."]
    pub struct CreateClientEventRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientEvent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Events issued when end user interacts with customer's application that uses Cloud Talent Solution."]
        pub client_event: ::std::option::Option<::std::boxed::Box<ClientEvent>>,
    }
    impl CreateClientEventRequest {
        pub fn builder() -> CreateClientEventRequestBuilder {
            CreateClientEventRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Input only. The Request of the CreateCompany method."]
    pub struct CreateCompanyRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "company")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The company to be created."]
        pub company: ::std::option::Option<::std::boxed::Box<Company>>,
    }
    impl CreateCompanyRequest {
        pub fn builder() -> CreateCompanyRequestBuilder {
            CreateCompanyRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Input only. Create job request."]
    pub struct CreateJobRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "job")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The Job to be created."]
        pub job: ::std::option::Option<::std::boxed::Box<Job>>,
    }
    impl CreateJobRequest {
        pub fn builder() -> CreateJobRequestBuilder {
            CreateJobRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Custom attribute values that are either filterable or non-filterable."]
    pub struct CustomAttribute {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filterable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If the `filterable` flag is true, the custom field values may be used for custom attribute filters JobQuery.custom_attribute_filter. If false, these values may not be used for custom attribute filters. Default is false."]
        pub filterable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional but exactly one of string_values or long_values must be specified. This field is used to perform number range search. (`EQ`, `GT`, `GE`, `LE`, `LT`) over filterable `long_value`. Currently at most 1 long_values is supported."]
        pub long_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional but exactly one of string_values or long_values must be specified. This field is used to perform a string match (`CASE_SENSITIVE_MATCH` or `CASE_INSENSITIVE_MATCH`) search. For filterable `string_value`s, a maximum total number of 200 values is allowed, with each `string_value` has a byte size of no more than 500B. For unfilterable `string_values`, the maximum total byte size of unfilterable `string_values` is 50KB. Empty string is not allowed."]
        pub string_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl CustomAttribute {
        pub fn builder() -> CustomAttributeBuilder {
            CustomAttributeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Custom attributes histogram request. An error is thrown if neither string_value_histogram or long_value_histogram_bucketing_option has been defined."]
    pub struct CustomAttributeHistogramRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Specifies the custom field key to perform a histogram on. If specified without `long_value_histogram_bucketing_option`, histogram on string values of the given `key` is triggered, otherwise histogram is performed on long values."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longValueHistogramBucketingOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Specifies buckets used to perform a range histogram on Job's filterable long custom field values, or min/max value requirements."]
        pub long_value_histogram_bucketing_option:
            ::std::option::Option<::std::boxed::Box<NumericBucketingOption>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValueHistogram")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If set to true, the response includes the histogram value for each key as a string."]
        pub string_value_histogram: ::std::option::Option<::std::primitive::bool>,
    }
    impl CustomAttributeHistogramRequest {
        pub fn builder() -> CustomAttributeHistogramRequestBuilder {
            CustomAttributeHistogramRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. Custom attribute histogram result."]
    pub struct CustomAttributeHistogramResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Stores the key of custom attribute the histogram is performed on."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longValueHistogramResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Stores bucketed histogram counting result or min/max values for custom attribute long values associated with `key`."]
        pub long_value_histogram_result:
            ::std::option::Option<::std::boxed::Box<NumericBucketingResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValueHistogramResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Stores a map from the values of string custom field associated with `key` to the number of jobs with that value in this histogram result."]
        pub string_value_histogram_result:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::primitive::i64>>,
    }
    impl CustomAttributeHistogramResult {
        pub fn builder() -> CustomAttributeHistogramResultBuilder {
            CustomAttributeHistogramResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Device information collected from the job seeker, candidate, or other entity conducting the job search. Providing this information improves the quality of the search results across devices."]
    pub struct DeviceInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Type of the device."]
        pub device_type: ::std::option::Option<DeviceInfoDeviceTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A device-specific ID. The ID must be a unique identifier that distinguishes the device from other devices."]
        pub id: ::std::option::Option<::std::string::String>,
    }
    impl DeviceInfo {
        pub fn builder() -> DeviceInfoBuilder {
            DeviceInfoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Type of the device."]
    pub enum DeviceInfoDeviceTypeEnum {
        #[serde(rename = "DEVICE_TYPE_UNSPECIFIED")]
        #[doc = "The device type isn't specified."]
        DeviceTypeUnspecified,
        #[serde(rename = "WEB")]
        #[doc = "A desktop web browser, such as, Chrome, Firefox, Safari, or Internet Explorer)"]
        Web,
        #[serde(rename = "MOBILE_WEB")]
        #[doc = "A mobile device web browser, such as a phone or tablet with a Chrome browser."]
        MobileWeb,
        #[serde(rename = "ANDROID")]
        #[doc = "An Android device native application."]
        Android,
        #[serde(rename = "IOS")]
        #[doc = "An iOS device native application."]
        Ios,
        #[serde(rename = "BOT")]
        #[doc = "A bot, as opposed to a device operated by human beings, such as a web crawler."]
        Bot,
        #[serde(rename = "OTHER")]
        #[doc = "Other devices types."]
        Other,
    }
    impl ::std::default::Default for DeviceInfoDeviceTypeEnum {
        fn default() -> Self {
            Self::DeviceTypeUnspecified
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
    #[doc = "Input only. Histogram facets to be specified in SearchJobsRequest."]
    pub struct HistogramFacets {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compensationHistogramFacets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Specifies compensation field-based histogram requests. Duplicate values of CompensationHistogramRequest.type are not allowed."]
        pub compensation_histogram_facets:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CompensationHistogramRequest>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customAttributeHistogramFacets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Specifies the custom attributes histogram requests. Duplicate values of CustomAttributeHistogramRequest.key are not allowed."]
        pub custom_attribute_histogram_facets: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<CustomAttributeHistogramRequest>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "simpleHistogramFacets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Specifies the simple type of histogram facets, for example, `COMPANY_SIZE`, `EMPLOYMENT_TYPE` etc."]
        pub simple_histogram_facets:
            ::std::option::Option<::std::vec::Vec<HistogramFacetsSimpleHistogramFacetsEnum>>,
    }
    impl HistogramFacets {
        pub fn builder() -> HistogramFacetsBuilder {
            HistogramFacetsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum HistogramFacetsSimpleHistogramFacetsEnum {
        #[serde(rename = "SEARCH_TYPE_UNSPECIFIED")]
        #[doc = "The default value if search type is not specified."]
        SearchTypeUnspecified,
        #[serde(rename = "COMPANY_ID")]
        #[doc = "Filter by the company id field."]
        CompanyId,
        #[serde(rename = "EMPLOYMENT_TYPE")]
        #[doc = "Filter by the employment type field, such as `FULL_TIME` or `PART_TIME`."]
        EmploymentType,
        #[serde(rename = "COMPANY_SIZE")]
        #[doc = "Filter by the company size type field, such as `BIG`, `SMALL` or `BIGGER`."]
        CompanySize,
        #[serde(rename = "DATE_PUBLISHED")]
        #[doc = "Filter by the date published field. Possible return values are: * PAST_24_HOURS (The past 24 hours) * PAST_3_DAYS (The past 3 days) * PAST_WEEK (The past 7 days) * PAST_MONTH (The past 30 days) * PAST_YEAR (The past 365 days)"]
        DatePublished,
        #[serde(rename = "EDUCATION_LEVEL")]
        #[doc = "Filter by the required education level of the job."]
        EducationLevel,
        #[serde(rename = "EXPERIENCE_LEVEL")]
        #[doc = "Filter by the required experience level of the job."]
        ExperienceLevel,
        #[serde(rename = "ADMIN_1")]
        #[doc = "Filter by Admin1, which is a global placeholder for referring to state, province, or the particular term a country uses to define the geographic structure below the country level. Examples include states codes such as \"CA\", \"IL\", \"NY\", and provinces, such as \"BC\"."]
        Admin1,
        #[serde(rename = "COUNTRY")]
        #[doc = "Filter by the country code of job, such as US, JP, FR."]
        Country,
        #[serde(rename = "CITY")]
        #[doc = "Filter by the \"city name\", \"Admin1 code\", for example, \"Mountain View, CA\" or \"New York, NY\"."]
        City,
        #[serde(rename = "LOCALE")]
        #[doc = "Filter by the locale field of a job, such as \"en-US\", \"fr-FR\". This is the BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47)."]
        Locale,
        #[serde(rename = "LANGUAGE")]
        #[doc = "Filter by the language code portion of the locale field, such as \"en\" or \"fr\"."]
        Language,
        #[serde(rename = "CATEGORY")]
        #[doc = "Filter by the Category."]
        Category,
        #[serde(rename = "CITY_COORDINATE")]
        #[doc = "Filter by the city center GPS coordinate (latitude and longitude), for example, 37.4038522,-122.0987765. Since the coordinates of a city center can change, clients may need to refresh them periodically."]
        CityCoordinate,
        #[serde(rename = "ADMIN_1_COUNTRY")]
        #[doc = "A combination of state or province code with a country code. This field differs from `JOB_ADMIN1`, which can be used in multiple countries."]
        Admin1Country,
        #[serde(rename = "COMPANY_DISPLAY_NAME")]
        #[doc = "Company display name."]
        CompanyDisplayName,
        #[serde(rename = "BASE_COMPENSATION_UNIT")]
        #[doc = "Base compensation unit."]
        BaseCompensationUnit,
    }
    impl ::std::default::Default for HistogramFacetsSimpleHistogramFacetsEnum {
        fn default() -> Self {
            Self::SearchTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. Result of a histogram call. The response contains the histogram map for the search type specified by HistogramResult.field. The response is a map of each filter value to the corresponding count of jobs for that filter."]
    pub struct HistogramResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Histogram search filters."]
        pub search_type: ::std::option::Option<HistogramResultSearchTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map from the values of field to the number of jobs with that value in this search result. Key: search type (filter names, such as the companyName). Values: the count of jobs that match the filter for this search."]
        pub values:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::primitive::i64>>,
    }
    impl HistogramResult {
        pub fn builder() -> HistogramResultBuilder {
            HistogramResultBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The Histogram search filters."]
    pub enum HistogramResultSearchTypeEnum {
        #[serde(rename = "SEARCH_TYPE_UNSPECIFIED")]
        #[doc = "The default value if search type is not specified."]
        SearchTypeUnspecified,
        #[serde(rename = "COMPANY_ID")]
        #[doc = "Filter by the company id field."]
        CompanyId,
        #[serde(rename = "EMPLOYMENT_TYPE")]
        #[doc = "Filter by the employment type field, such as `FULL_TIME` or `PART_TIME`."]
        EmploymentType,
        #[serde(rename = "COMPANY_SIZE")]
        #[doc = "Filter by the company size type field, such as `BIG`, `SMALL` or `BIGGER`."]
        CompanySize,
        #[serde(rename = "DATE_PUBLISHED")]
        #[doc = "Filter by the date published field. Possible return values are: * PAST_24_HOURS (The past 24 hours) * PAST_3_DAYS (The past 3 days) * PAST_WEEK (The past 7 days) * PAST_MONTH (The past 30 days) * PAST_YEAR (The past 365 days)"]
        DatePublished,
        #[serde(rename = "EDUCATION_LEVEL")]
        #[doc = "Filter by the required education level of the job."]
        EducationLevel,
        #[serde(rename = "EXPERIENCE_LEVEL")]
        #[doc = "Filter by the required experience level of the job."]
        ExperienceLevel,
        #[serde(rename = "ADMIN_1")]
        #[doc = "Filter by Admin1, which is a global placeholder for referring to state, province, or the particular term a country uses to define the geographic structure below the country level. Examples include states codes such as \"CA\", \"IL\", \"NY\", and provinces, such as \"BC\"."]
        Admin1,
        #[serde(rename = "COUNTRY")]
        #[doc = "Filter by the country code of job, such as US, JP, FR."]
        Country,
        #[serde(rename = "CITY")]
        #[doc = "Filter by the \"city name\", \"Admin1 code\", for example, \"Mountain View, CA\" or \"New York, NY\"."]
        City,
        #[serde(rename = "LOCALE")]
        #[doc = "Filter by the locale field of a job, such as \"en-US\", \"fr-FR\". This is the BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47)."]
        Locale,
        #[serde(rename = "LANGUAGE")]
        #[doc = "Filter by the language code portion of the locale field, such as \"en\" or \"fr\"."]
        Language,
        #[serde(rename = "CATEGORY")]
        #[doc = "Filter by the Category."]
        Category,
        #[serde(rename = "CITY_COORDINATE")]
        #[doc = "Filter by the city center GPS coordinate (latitude and longitude), for example, 37.4038522,-122.0987765. Since the coordinates of a city center can change, clients may need to refresh them periodically."]
        CityCoordinate,
        #[serde(rename = "ADMIN_1_COUNTRY")]
        #[doc = "A combination of state or province code with a country code. This field differs from `JOB_ADMIN1`, which can be used in multiple countries."]
        Admin1Country,
        #[serde(rename = "COMPANY_DISPLAY_NAME")]
        #[doc = "Company display name."]
        CompanyDisplayName,
        #[serde(rename = "BASE_COMPENSATION_UNIT")]
        #[doc = "Base compensation unit."]
        BaseCompensationUnit,
    }
    impl ::std::default::Default for HistogramResultSearchTypeEnum {
        fn default() -> Self {
            Self::SearchTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. Histogram results that match HistogramFacets specified in SearchJobsRequest."]
    pub struct HistogramResults {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compensationHistogramResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies compensation field-based histogram results that match HistogramFacets.compensation_histogram_requests."]
        pub compensation_histogram_results:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CompensationHistogramResult>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customAttributeHistogramResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies histogram results for custom attributes that match HistogramFacets.custom_attribute_histogram_facets."]
        pub custom_attribute_histogram_results: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<CustomAttributeHistogramResult>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "simpleHistogramResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies histogram results that matches HistogramFacets.simple_histogram_facets."]
        pub simple_histogram_results:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HistogramResult>>>,
    }
    impl HistogramResults {
        pub fn builder() -> HistogramResultsBuilder {
            HistogramResultsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Job resource represents a job posting (also referred to as a \"job listing\" or \"job requisition\"). A job belongs to a Company, which is the hiring entity responsible for the job."]
    pub struct Job {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addresses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional but strongly recommended for the best service experience. Location(s) where the employer is looking to hire for this job posting. Specifying the full street address(es) of the hiring location enables better API results, especially job searches by commute time. At most 50 locations are allowed for best search performance. If a job has more locations, it is suggested to split it into multiple jobs with unique requisition_ids (e.g. 'ReqA' becomes 'ReqA-1', 'ReqA-2', etc.) as multiple jobs with the same company_name, language_code and requisition_id are not allowed. If the original requisition_id must be preserved, a custom field should be used for storage. It is also suggested to group the locations that close to each other in the same job for better search experience. The maximum number of allowed characters is 500."]
        pub addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applicationInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. At least one field within ApplicationInfo must be specified. Job application information."]
        pub application_info: ::std::option::Option<::std::boxed::Box<ApplicationInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "companyDisplayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Display name of the company listing the job."]
        pub company_display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "companyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The resource name of the company listing the job, such as \"projects/api-test-project/companies/foo\"."]
        pub company_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compensationInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Job compensation information."]
        pub compensation_info: ::std::option::Option<::std::boxed::Box<CompensationInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customAttributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A map of fields to hold both filterable and non-filterable custom job attributes that are not covered by the provided structured fields. The keys of the map are strings up to 64 bytes and must match the pattern: a-zA-Z*. For example, key0LikeThis or KEY_1_LIKE_THIS. At most 100 filterable and at most 100 unfilterable keys are supported. For filterable `string_values`, across all keys at most 200 values are allowed, with each string no more than 255 characters. For unfilterable `string_values`, the maximum total size of `string_values` across all keys is 50KB."]
        pub custom_attributes: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<CustomAttribute>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "degreeTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The desired education degrees for the job, such as Bachelors, Masters."]
        pub degree_types: ::std::option::Option<::std::vec::Vec<JobDegreeTypesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "department")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The department or functional area within the company with the open position. The maximum number of allowed characters is 255."]
        pub department: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "derivedInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Derived details about the job posting."]
        pub derived_info: ::std::option::Option<::std::boxed::Box<JobDerivedInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The description of the job, which typically includes a multi-paragraph description of the company and related information. Separate fields are provided on the job object for responsibilities, qualifications, and other job characteristics. Use of these separate job fields is recommended. This field accepts and sanitizes HTML input, and also accepts bold, italic, ordered list, and unordered list markup tags. The maximum number of allowed characters is 100,000."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "employmentTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The employment type(s) of a job, for example, full time or part time."]
        pub employment_types: ::std::option::Option<::std::vec::Vec<JobEmploymentTypesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "incentives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A description of bonus, commission, and other compensation incentives associated with the job not including salary or pay. The maximum number of allowed characters is 10,000."]
        pub incentives: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobBenefits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The benefits included with the job."]
        pub job_benefits: ::std::option::Option<::std::vec::Vec<JobJobBenefitsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobEndTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The end timestamp of the job. Typically this field is used for contracting engagements. Invalid timestamps are ignored."]
        pub job_end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The experience level associated with the job, such as \"Entry Level\"."]
        pub job_level: ::std::option::Option<JobJobLevelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The start timestamp of the job in UTC time zone. Typically this field is used for contracting engagements. Invalid timestamps are ignored."]
        pub job_start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The language of the posting. This field is distinct from any requirements for fluency that are associated with the job. Language codes must be in BCP-47 format, such as \"en-US\" or \"sr-Latn\". For more information, see [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47){: class=\"external\" target=\"_blank\" }. If this field is unspecified and Job.description is present, detected language code based on Job.description is assigned, otherwise defaults to 'en_US'."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required during job update. The resource name for the job. This is generated by the service when a job is created. The format is \"projects/{project_id}/jobs/{job_id}\", for example, \"projects/api-test-project/jobs/1234\". Use of this field in job queries and API calls is preferred over the use of requisition_id since this value is unique."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postingCreateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The timestamp when this job posting was created."]
        pub posting_create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postingExpireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional but strongly recommended for the best service experience. The expiration timestamp of the job. After this timestamp, the job is marked as expired, and it no longer appears in search results. The expired job can't be deleted or listed by the DeleteJob and ListJobs APIs, but it can be retrieved with the GetJob API or updated with the UpdateJob API. An expired job can be updated and opened again by using a future expiration timestamp. Updating an expired job fails if there is another existing open job with same company_name, language_code and requisition_id. The expired jobs are retained in our system for 90 days. However, the overall expired job count cannot exceed 3 times the maximum of open jobs count over the past week, otherwise jobs with earlier expire time are cleaned first. Expired jobs are no longer accessible after they are cleaned out. Invalid timestamps are ignored, and treated as expire time not provided. Timestamp before the instant request is made is considered valid, the job will be treated as expired immediately. If this value is not provided at the time of job creation or is invalid, the job posting expires after 30 days from the job's creation time. For example, if the job was created on 2017/01/01 13:00AM UTC with an unspecified expiration date, the job expires after 2017/01/31 13:00AM UTC. If this value is not provided on job update, it depends on the field masks set by UpdateJobRequest.update_mask. If the field masks include expiry_time, or the masks are empty meaning that every field is updated, the job posting expires after 30 days from the job's last update time. Otherwise the expiration date isn't updated."]
        pub posting_expire_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postingPublishTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The timestamp this job posting was most recently published. The default value is the time the request arrives at the server. Invalid timestamps are ignored."]
        pub posting_publish_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postingRegion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The job PostingRegion (for example, state, country) throughout which the job is available. If this field is set, a LocationFilter in a search query within the job region finds this job posting if an exact location match isn't specified. If this field is set to PostingRegion.NATION or PostingRegion.ADMINISTRATIVE_AREA, setting job Job.addresses to the same location level as this field is strongly recommended."]
        pub posting_region: ::std::option::Option<JobPostingRegionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postingUpdateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The timestamp when this job posting was last updated."]
        pub posting_update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processingOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Options for job processing."]
        pub processing_options: ::std::option::Option<::std::boxed::Box<ProcessingOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "promotionValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A promotion value of the job, as determined by the client. The value determines the sort order of the jobs returned when searching for jobs using the featured jobs search call, with higher promotional values being returned first and ties being resolved by relevance sort. Only the jobs with a promotionValue >0 are returned in a FEATURED_JOB_SEARCH. Default value is 0, and negative values are treated as 0."]
        pub promotion_value: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "qualifications")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A description of the qualifications required to perform the job. The use of this field is recommended as an alternative to using the more general description field. This field accepts and sanitizes HTML input, and also accepts bold, italic, ordered list, and unordered list markup tags. The maximum number of allowed characters is 10,000."]
        pub qualifications: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requisitionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The requisition ID, also referred to as the posting ID, assigned by the client to identify a job. This field is intended to be used by clients for client identification and tracking of postings. A job is not allowed to be created if there is another job with the same [company_name], language_code and requisition_id. The maximum number of allowed characters is 255."]
        pub requisition_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responsibilities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A description of job responsibilities. The use of this field is recommended as an alternative to using the more general description field. This field accepts and sanitizes HTML input, and also accepts bold, italic, ordered list, and unordered list markup tags. The maximum number of allowed characters is 10,000."]
        pub responsibilities: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The title of the job, such as \"Software Engineer\" The maximum number of allowed characters is 500."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibility")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The job is only visible to the owner. The visibility of the job. Defaults to Visibility.ACCOUNT_ONLY if not specified."]
        pub visibility: ::std::option::Option<JobVisibilityEnum>,
    }
    impl Job {
        pub fn builder() -> JobBuilder {
            JobBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum JobDegreeTypesEnum {
        #[serde(rename = "DEGREE_TYPE_UNSPECIFIED")]
        #[doc = "Default value. Represents no degree, or early childhood education. Maps to ISCED code 0. Ex) Kindergarten"]
        DegreeTypeUnspecified,
        #[serde(rename = "PRIMARY_EDUCATION")]
        #[doc = "Primary education which is typically the first stage of compulsory education. ISCED code 1. Ex) Elementary school"]
        PrimaryEducation,
        #[serde(rename = "LOWER_SECONDARY_EDUCATION")]
        #[doc = "Lower secondary education; First stage of secondary education building on primary education, typically with a more subject-oriented curriculum. ISCED code 2. Ex) Middle school"]
        LowerSecondaryEducation,
        #[serde(rename = "UPPER_SECONDARY_EDUCATION")]
        #[doc = "Middle education; Second/final stage of secondary education preparing for tertiary education and/or providing skills relevant to employment. Usually with an increased range of subject options and streams. ISCED code 3. Ex) High school"]
        UpperSecondaryEducation,
        #[serde(rename = "ADULT_REMEDIAL_EDUCATION")]
        #[doc = "Adult Remedial Education; Programmes providing learning experiences that build on secondary education and prepare for labour market entry and/or tertiary education. The content is broader than secondary but not as complex as tertiary education. ISCED code 4."]
        AdultRemedialEducation,
        #[serde(rename = "ASSOCIATES_OR_EQUIVALENT")]
        #[doc = "Associate's or equivalent; Short first tertiary programmes that are typically practically-based, occupationally-specific and prepare for labour market entry. These programmes may also provide a pathway to other tertiary programmes. ISCED code 5."]
        AssociatesOrEquivalent,
        #[serde(rename = "BACHELORS_OR_EQUIVALENT")]
        #[doc = "Bachelor's or equivalent; Programmes designed to provide intermediate academic and/or professional knowledge, skills and competencies leading to a first tertiary degree or equivalent qualification. ISCED code 6."]
        BachelorsOrEquivalent,
        #[serde(rename = "MASTERS_OR_EQUIVALENT")]
        #[doc = "Master's or equivalent; Programmes designed to provide advanced academic and/or professional knowledge, skills and competencies leading to a second tertiary degree or equivalent qualification. ISCED code 7."]
        MastersOrEquivalent,
        #[serde(rename = "DOCTORAL_OR_EQUIVALENT")]
        #[doc = "Doctoral or equivalent; Programmes designed primarily to lead to an advanced research qualification, usually concluding with the submission and defense of a substantive dissertation of publishable quality based on original research. ISCED code 8."]
        DoctoralOrEquivalent,
    }
    impl ::std::default::Default for JobDegreeTypesEnum {
        fn default() -> Self {
            Self::DegreeTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum JobEmploymentTypesEnum {
        #[serde(rename = "EMPLOYMENT_TYPE_UNSPECIFIED")]
        #[doc = "The default value if the employment type is not specified."]
        EmploymentTypeUnspecified,
        #[serde(rename = "FULL_TIME")]
        #[doc = "The job requires working a number of hours that constitute full time employment, typically 40 or more hours per week."]
        FullTime,
        #[serde(rename = "PART_TIME")]
        #[doc = "The job entails working fewer hours than a full time job, typically less than 40 hours a week."]
        PartTime,
        #[serde(rename = "CONTRACTOR")]
        #[doc = "The job is offered as a contracted, as opposed to a salaried employee, position."]
        Contractor,
        #[serde(rename = "CONTRACT_TO_HIRE")]
        #[doc = "The job is offered as a contracted position with the understanding that it's converted into a full-time position at the end of the contract. Jobs of this type are also returned by a search for EmploymentType.CONTRACTOR jobs."]
        ContractToHire,
        #[serde(rename = "TEMPORARY")]
        #[doc = "The job is offered as a temporary employment opportunity, usually a short-term engagement."]
        Temporary,
        #[serde(rename = "INTERN")]
        #[doc = "The job is a fixed-term opportunity for students or entry-level job seekers to obtain on-the-job training, typically offered as a summer position."]
        Intern,
        #[serde(rename = "VOLUNTEER")]
        #[doc = "The is an opportunity for an individual to volunteer, where there's no expectation of compensation for the provided services."]
        Volunteer,
        #[serde(rename = "PER_DIEM")]
        #[doc = "The job requires an employee to work on an as-needed basis with a flexible schedule."]
        PerDiem,
        #[serde(rename = "FLY_IN_FLY_OUT")]
        #[doc = "The job involves employing people in remote areas and flying them temporarily to the work site instead of relocating employees and their families permanently."]
        FlyInFlyOut,
        #[serde(rename = "OTHER_EMPLOYMENT_TYPE")]
        #[doc = "The job does not fit any of the other listed types."]
        OtherEmploymentType,
    }
    impl ::std::default::Default for JobEmploymentTypesEnum {
        fn default() -> Self {
            Self::EmploymentTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum JobJobBenefitsEnum {
        #[serde(rename = "JOB_BENEFIT_UNSPECIFIED")]
        #[doc = "Default value if the type is not specified."]
        JobBenefitUnspecified,
        #[serde(rename = "CHILD_CARE")]
        #[doc = "The job includes access to programs that support child care, such as daycare."]
        ChildCare,
        #[serde(rename = "DENTAL")]
        #[doc = "The job includes dental services covered by a dental insurance plan."]
        Dental,
        #[serde(rename = "DOMESTIC_PARTNER")]
        #[doc = "The job offers specific benefits to domestic partners."]
        DomesticPartner,
        #[serde(rename = "FLEXIBLE_HOURS")]
        #[doc = "The job allows for a flexible work schedule."]
        FlexibleHours,
        #[serde(rename = "MEDICAL")]
        #[doc = "The job includes health services covered by a medical insurance plan."]
        Medical,
        #[serde(rename = "LIFE_INSURANCE")]
        #[doc = "The job includes a life insurance plan provided by the employer or available for purchase by the employee."]
        LifeInsurance,
        #[serde(rename = "PARENTAL_LEAVE")]
        #[doc = "The job allows for a leave of absence to a parent to care for a newborn child."]
        ParentalLeave,
        #[serde(rename = "RETIREMENT_PLAN")]
        #[doc = "The job includes a workplace retirement plan provided by the employer or available for purchase by the employee."]
        RetirementPlan,
        #[serde(rename = "SICK_DAYS")]
        #[doc = "The job allows for paid time off due to illness."]
        SickDays,
        #[serde(rename = "VACATION")]
        #[doc = "The job includes paid time off for vacation."]
        Vacation,
        #[serde(rename = "VISION")]
        #[doc = "The job includes vision services covered by a vision insurance plan."]
        Vision,
    }
    impl ::std::default::Default for JobJobBenefitsEnum {
        fn default() -> Self {
            Self::JobBenefitUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The experience level associated with the job, such as \"Entry Level\"."]
    pub enum JobJobLevelEnum {
        #[serde(rename = "JOB_LEVEL_UNSPECIFIED")]
        #[doc = "The default value if the level is not specified."]
        JobLevelUnspecified,
        #[serde(rename = "ENTRY_LEVEL")]
        #[doc = "Entry-level individual contributors, typically with less than 2 years of experience in a similar role. Includes interns."]
        EntryLevel,
        #[serde(rename = "EXPERIENCED")]
        #[doc = "Experienced individual contributors, typically with 2+ years of experience in a similar role."]
        Experienced,
        #[serde(rename = "MANAGER")]
        #[doc = "Entry- to mid-level managers responsible for managing a team of people."]
        Manager,
        #[serde(rename = "DIRECTOR")]
        #[doc = "Senior-level managers responsible for managing teams of managers."]
        Director,
        #[serde(rename = "EXECUTIVE")]
        #[doc = "Executive-level managers and above, including C-level positions."]
        Executive,
    }
    impl ::std::default::Default for JobJobLevelEnum {
        fn default() -> Self {
            Self::JobLevelUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The job PostingRegion (for example, state, country) throughout which the job is available. If this field is set, a LocationFilter in a search query within the job region finds this job posting if an exact location match isn't specified. If this field is set to PostingRegion.NATION or PostingRegion.ADMINISTRATIVE_AREA, setting job Job.addresses to the same location level as this field is strongly recommended."]
    pub enum JobPostingRegionEnum {
        #[serde(rename = "POSTING_REGION_UNSPECIFIED")]
        #[doc = "If the region is unspecified, the job is only returned if it matches the LocationFilter."]
        PostingRegionUnspecified,
        #[serde(rename = "ADMINISTRATIVE_AREA")]
        #[doc = "In addition to exact location matching, job posting is returned when the LocationFilter in the search query is in the same administrative area as the returned job posting. For example, if a `ADMINISTRATIVE_AREA` job is posted in \"CA, USA\", it's returned if LocationFilter has \"Mountain View\". Administrative area refers to top-level administrative subdivision of this country. For example, US state, IT region, UK constituent nation and JP prefecture."]
        AdministrativeArea,
        #[serde(rename = "NATION")]
        #[doc = "In addition to exact location matching, job is returned when LocationFilter in search query is in the same country as this job. For example, if a `NATION_WIDE` job is posted in \"USA\", it's returned if LocationFilter has 'Mountain View'."]
        Nation,
        #[serde(rename = "TELECOMMUTE")]
        #[doc = "Job allows employees to work remotely (telecommute). If locations are provided with this value, the job is considered as having a location, but telecommuting is allowed."]
        Telecommute,
    }
    impl ::std::default::Default for JobPostingRegionEnum {
        fn default() -> Self {
            Self::PostingRegionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Deprecated. The job is only visible to the owner. The visibility of the job. Defaults to Visibility.ACCOUNT_ONLY if not specified."]
    pub enum JobVisibilityEnum {
        #[serde(rename = "VISIBILITY_UNSPECIFIED")]
        #[doc = "Default value."]
        VisibilityUnspecified,
        #[serde(rename = "ACCOUNT_ONLY")]
        #[doc = "The resource is only visible to the GCP account who owns it."]
        AccountOnly,
        #[serde(rename = "SHARED_WITH_GOOGLE")]
        #[doc = "The resource is visible to the owner and may be visible to other applications and processes at Google."]
        SharedWithGoogle,
        #[serde(rename = "SHARED_WITH_PUBLIC")]
        #[doc = "The resource is visible to the owner and may be visible to all other API clients."]
        SharedWithPublic,
    }
    impl ::std::default::Default for JobVisibilityEnum {
        fn default() -> Self {
            Self::VisibilityUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. Derived details about the job posting."]
    pub struct JobDerivedInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobCategories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Job categories derived from Job.title and Job.description."]
        pub job_categories: ::std::option::Option<::std::vec::Vec<JobDerivedInfoJobCategoriesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Structured locations of the job, resolved from Job.addresses. locations are exactly matched to Job.addresses in the same order."]
        pub locations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Location>>>,
    }
    impl JobDerivedInfo {
        pub fn builder() -> JobDerivedInfoBuilder {
            JobDerivedInfoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum JobDerivedInfoJobCategoriesEnum {
        #[serde(rename = "JOB_CATEGORY_UNSPECIFIED")]
        #[doc = "The default value if the category isn't specified."]
        JobCategoryUnspecified,
        #[serde(rename = "ACCOUNTING_AND_FINANCE")]
        #[doc = "An accounting and finance job, such as an Accountant."]
        AccountingAndFinance,
        #[serde(rename = "ADMINISTRATIVE_AND_OFFICE")]
        #[doc = "An administrative and office job, such as an Administrative Assistant."]
        AdministrativeAndOffice,
        #[serde(rename = "ADVERTISING_AND_MARKETING")]
        #[doc = "An advertising and marketing job, such as Marketing Manager."]
        AdvertisingAndMarketing,
        #[serde(rename = "ANIMAL_CARE")]
        #[doc = "An animal care job, such as Veterinarian."]
        AnimalCare,
        #[serde(rename = "ART_FASHION_AND_DESIGN")]
        #[doc = "An art, fashion, or design job, such as Designer."]
        ArtFashionAndDesign,
        #[serde(rename = "BUSINESS_OPERATIONS")]
        #[doc = "A business operations job, such as Business Operations Manager."]
        BusinessOperations,
        #[serde(rename = "CLEANING_AND_FACILITIES")]
        #[doc = "A cleaning and facilities job, such as Custodial Staff."]
        CleaningAndFacilities,
        #[serde(rename = "COMPUTER_AND_IT")]
        #[doc = "A computer and IT job, such as Systems Administrator."]
        ComputerAndIt,
        #[serde(rename = "CONSTRUCTION")]
        #[doc = "A construction job, such as General Laborer."]
        Construction,
        #[serde(rename = "CUSTOMER_SERVICE")]
        #[doc = "A customer service job, such s Cashier."]
        CustomerService,
        #[serde(rename = "EDUCATION")]
        #[doc = "An education job, such as School Teacher."]
        Education,
        #[serde(rename = "ENTERTAINMENT_AND_TRAVEL")]
        #[doc = "An entertainment and travel job, such as Flight Attendant."]
        EntertainmentAndTravel,
        #[serde(rename = "FARMING_AND_OUTDOORS")]
        #[doc = "A farming or outdoor job, such as Park Ranger."]
        FarmingAndOutdoors,
        #[serde(rename = "HEALTHCARE")]
        #[doc = "A healthcare job, such as Registered Nurse."]
        Healthcare,
        #[serde(rename = "HUMAN_RESOURCES")]
        #[doc = "A human resources job, such as Human Resources Director."]
        HumanResources,
        #[serde(rename = "INSTALLATION_MAINTENANCE_AND_REPAIR")]
        #[doc = "An installation, maintenance, or repair job, such as Electrician."]
        InstallationMaintenanceAndRepair,
        #[serde(rename = "LEGAL")]
        #[doc = "A legal job, such as Law Clerk."]
        Legal,
        #[serde(rename = "MANAGEMENT")]
        #[doc = "A management job, often used in conjunction with another category, such as Store Manager."]
        Management,
        #[serde(rename = "MANUFACTURING_AND_WAREHOUSE")]
        #[doc = "A manufacturing or warehouse job, such as Assembly Technician."]
        ManufacturingAndWarehouse,
        #[serde(rename = "MEDIA_COMMUNICATIONS_AND_WRITING")]
        #[doc = "A media, communications, or writing job, such as Media Relations."]
        MediaCommunicationsAndWriting,
        #[serde(rename = "OIL_GAS_AND_MINING")]
        #[doc = "An oil, gas or mining job, such as Offshore Driller."]
        OilGasAndMining,
        #[serde(rename = "PERSONAL_CARE_AND_SERVICES")]
        #[doc = "A personal care and services job, such as Hair Stylist."]
        PersonalCareAndServices,
        #[serde(rename = "PROTECTIVE_SERVICES")]
        #[doc = "A protective services job, such as Security Guard."]
        ProtectiveServices,
        #[serde(rename = "REAL_ESTATE")]
        #[doc = "A real estate job, such as Buyer's Agent."]
        RealEstate,
        #[serde(rename = "RESTAURANT_AND_HOSPITALITY")]
        #[doc = "A restaurant and hospitality job, such as Restaurant Server."]
        RestaurantAndHospitality,
        #[serde(rename = "SALES_AND_RETAIL")]
        #[doc = "A sales and/or retail job, such Sales Associate."]
        SalesAndRetail,
        #[serde(rename = "SCIENCE_AND_ENGINEERING")]
        #[doc = "A science and engineering job, such as Lab Technician."]
        ScienceAndEngineering,
        #[serde(rename = "SOCIAL_SERVICES_AND_NON_PROFIT")]
        #[doc = "A social services or non-profit job, such as Case Worker."]
        SocialServicesAndNonProfit,
        #[serde(rename = "SPORTS_FITNESS_AND_RECREATION")]
        #[doc = "A sports, fitness, or recreation job, such as Personal Trainer."]
        SportsFitnessAndRecreation,
        #[serde(rename = "TRANSPORTATION_AND_LOGISTICS")]
        #[doc = "A transportation or logistics job, such as Truck Driver."]
        TransportationAndLogistics,
    }
    impl ::std::default::Default for JobDerivedInfoJobCategoriesEnum {
        fn default() -> Self {
            Self::JobCategoryUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event issued when a job seeker interacts with the application that implements Cloud Talent Solution."]
    pub struct JobEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The job name(s) associated with this event. For example, if this is an impression event, this field contains the identifiers of all jobs shown to the job seeker. If this was a view event, this field contains the identifier of the viewed job."]
        pub jobs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The type of the event (see JobEventType)."]
        pub _type: ::std::option::Option<JobEventTypeEnum>,
    }
    impl JobEvent {
        pub fn builder() -> JobEventBuilder {
            JobEventBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The type of the event (see JobEventType)."]
    pub enum JobEventTypeEnum {
        #[serde(rename = "JOB_EVENT_TYPE_UNSPECIFIED")]
        #[doc = "The event is unspecified by other provided values."]
        JobEventTypeUnspecified,
        #[serde(rename = "IMPRESSION")]
        #[doc = "The job seeker or other entity interacting with the service has had a job rendered in their view, such as in a list of search results in a compressed or clipped format. This event is typically associated with the viewing of a jobs list on a single page by a job seeker."]
        Impression,
        #[serde(rename = "VIEW")]
        #[doc = "The job seeker, or other entity interacting with the service, has viewed the details of a job, including the full description. This event doesn't apply to the viewing a snippet of a job appearing as a part of the job search results. Viewing a snippet is associated with an impression)."]
        View,
        #[serde(rename = "VIEW_REDIRECT")]
        #[doc = "The job seeker or other entity interacting with the service performed an action to view a job and was redirected to a different website for job."]
        ViewRedirect,
        #[serde(rename = "APPLICATION_START")]
        #[doc = "The job seeker or other entity interacting with the service began the process or demonstrated the intention of applying for a job."]
        ApplicationStart,
        #[serde(rename = "APPLICATION_FINISH")]
        #[doc = "The job seeker or other entity interacting with the service submitted an application for a job."]
        ApplicationFinish,
        #[serde(rename = "APPLICATION_QUICK_SUBMISSION")]
        #[doc = "The job seeker or other entity interacting with the service submitted an application for a job with a single click without entering information. If a job seeker performs this action, send only this event to the service. Do not also send JobEventType.APPLICATION_START or JobEventType.APPLICATION_FINISH events."]
        ApplicationQuickSubmission,
        #[serde(rename = "APPLICATION_REDIRECT")]
        #[doc = "The job seeker or other entity interacting with the service performed an action to apply to a job and was redirected to a different website to complete the application."]
        ApplicationRedirect,
        #[serde(rename = "APPLICATION_START_FROM_SEARCH")]
        #[doc = "The job seeker or other entity interacting with the service began the process or demonstrated the intention of applying for a job from the search results page without viewing the details of the job posting. If sending this event, JobEventType.VIEW event shouldn't be sent."]
        ApplicationStartFromSearch,
        #[serde(rename = "APPLICATION_REDIRECT_FROM_SEARCH")]
        #[doc = "The job seeker, or other entity interacting with the service, performs an action with a single click from the search results page to apply to a job (without viewing the details of the job posting), and is redirected to a different website to complete the application. If a candidate performs this action, send only this event to the service. Do not also send JobEventType.APPLICATION_START, JobEventType.APPLICATION_FINISH or JobEventType.VIEW events."]
        ApplicationRedirectFromSearch,
        #[serde(rename = "APPLICATION_COMPANY_SUBMIT")]
        #[doc = "This event should be used when a company submits an application on behalf of a job seeker. This event is intended for use by staffing agencies attempting to place candidates."]
        ApplicationCompanySubmit,
        #[serde(rename = "BOOKMARK")]
        #[doc = "The job seeker or other entity interacting with the service demonstrated an interest in a job by bookmarking or saving it."]
        Bookmark,
        #[serde(rename = "NOTIFICATION")]
        #[doc = "The job seeker or other entity interacting with the service was sent a notification, such as an email alert or device notification, containing one or more jobs listings generated by the service."]
        Notification,
        #[serde(rename = "HIRED")]
        #[doc = "The job seeker or other entity interacting with the service was employed by the hiring entity (employer). Send this event only if the job seeker was hired through an application that was initiated by a search conducted through the Cloud Talent Solution service."]
        Hired,
        #[serde(rename = "SENT_CV")]
        #[doc = "A recruiter or staffing agency submitted an application on behalf of the candidate after interacting with the service to identify a suitable job posting."]
        SentCv,
        #[serde(rename = "INTERVIEW_GRANTED")]
        #[doc = "The entity interacting with the service (for example, the job seeker), was granted an initial interview by the hiring entity (employer). This event should only be sent if the job seeker was granted an interview as part of an application that was initiated by a search conducted through / recommendation provided by the Cloud Talent Solution service."]
        InterviewGranted,
        #[serde(rename = "NOT_INTERESTED")]
        #[doc = "The job seeker or other entity interacting with the service showed no interest in the job."]
        NotInterested,
    }
    impl ::std::default::Default for JobEventTypeEnum {
        fn default() -> Self {
            Self::JobEventTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Input only. The query required to perform a search query."]
    pub struct JobQuery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commuteFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Allows filtering jobs by commute time with different travel methods (for example, driving or public transit). Note: This only works with COMMUTE MODE. When specified, [JobQuery.location_filters] is ignored. Currently we don't support sorting by commute time."]
        pub commute_filter: ::std::option::Option<::std::boxed::Box<CommuteFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "companyDisplayNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. This filter specifies the exact company display name of the jobs to search against. If a value isn't specified, jobs within the search results are associated with any company. If multiple values are specified, jobs within the search results may be associated with any of the specified companies. At most 20 company display name filters are allowed."]
        pub company_display_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "companyNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. This filter specifies the company entities to search against. If a value isn't specified, jobs are searched for against all companies. If multiple values are specified, jobs are searched against the companies specified. The format is \"projects/{project_id}/companies/{company_id}\", for example, \"projects/api-test-project/companies/foo\". At most 20 company filters are allowed."]
        pub company_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compensationFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. This search filter is applied only to Job.compensation_info. For example, if the filter is specified as \"Hourly job with per-hour compensation > $15\", only jobs meeting these criteria are searched. If a filter isn't defined, all open jobs are searched."]
        pub compensation_filter: ::std::option::Option<::std::boxed::Box<CompensationFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customAttributeFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. This filter specifies a structured syntax to match against the Job.custom_attributes marked as `filterable`. The syntax for this expression is a subset of SQL syntax. Supported operators are: `=`, `!=`, `<`, `<=`, `>`, and `>=` where the left of the operator is a custom field key and the right of the operator is a number or a quoted string. You must escape backslash (\\\\) and quote (\\\") characters. Supported functions are `LOWER([field_name])` to perform a case insensitive match and `EMPTY([field_name])` to filter on the existence of a key. Boolean expressions (AND/OR/NOT) are supported up to 3 levels of nesting (for example, \"((A AND B AND C) OR NOT D) AND E\"), a maximum of 100 comparisons or functions are allowed in the expression. The expression must be < 6000 bytes in length. Sample Query: `(LOWER(driving_license)=\"class \\\"a\\\"\" OR EMPTY(driving_license)) AND driving_years > 10`"]
        pub custom_attribute_filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableSpellCheck")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. This flag controls the spell-check feature. If false, the service attempts to correct a misspelled query, for example, \"enginee\" is corrected to \"engineer\". Defaults to false: a spell check is performed."]
        pub disable_spell_check: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "employmentTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The employment type filter specifies the employment type of jobs to search against, such as EmploymentType.FULL_TIME. If a value is not specified, jobs in the search results includes any employment type. If multiple values are specified, jobs in the search results include any of the specified employment types."]
        pub employment_types: ::std::option::Option<::std::vec::Vec<JobQueryEmploymentTypesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobCategories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The category filter specifies the categories of jobs to search against. See Category for more information. If a value is not specified, jobs from any category are searched against. If multiple values are specified, jobs from any of the specified categories are searched against."]
        pub job_categories: ::std::option::Option<::std::vec::Vec<JobQueryJobCategoriesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCodes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. This filter specifies the locale of jobs to search against, for example, \"en-US\". If a value isn't specified, the search results can contain jobs in any locale. Language codes should be in BCP-47 format, such as \"en-US\" or \"sr-Latn\". For more information, see [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47). At most 10 language code filters are allowed."]
        pub language_codes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationFilters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The location filter specifies geo-regions containing the jobs to search against. See LocationFilter for more information. If a location value isn't specified, jobs fitting the other search criteria are retrieved regardless of where they're located. If multiple values are specified, jobs are retrieved from any of the specified locations. If different values are specified for the LocationFilter.distance_in_miles parameter, the maximum provided distance is used for all locations. At most 5 location filters are allowed."]
        pub location_filters:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LocationFilter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishTimeRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Jobs published within a range specified by this filter are searched against."]
        pub publish_time_range: ::std::option::Option<::std::boxed::Box<TimestampRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The query string that matches against the job title, description, and location fields. The maximum number of allowed characters is 255."]
        pub query: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryLanguageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language code of query. For example, \"en-US\". This field helps to better interpret the query. If a value isn't specified, the query language code is automatically detected, which may not be accurate. Language code should be in BCP-47 format, such as \"en-US\" or \"sr-Latn\". For more information, see [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47)."]
        pub query_language_code: ::std::option::Option<::std::string::String>,
    }
    impl JobQuery {
        pub fn builder() -> JobQueryBuilder {
            JobQueryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum JobQueryEmploymentTypesEnum {
        #[serde(rename = "EMPLOYMENT_TYPE_UNSPECIFIED")]
        #[doc = "The default value if the employment type is not specified."]
        EmploymentTypeUnspecified,
        #[serde(rename = "FULL_TIME")]
        #[doc = "The job requires working a number of hours that constitute full time employment, typically 40 or more hours per week."]
        FullTime,
        #[serde(rename = "PART_TIME")]
        #[doc = "The job entails working fewer hours than a full time job, typically less than 40 hours a week."]
        PartTime,
        #[serde(rename = "CONTRACTOR")]
        #[doc = "The job is offered as a contracted, as opposed to a salaried employee, position."]
        Contractor,
        #[serde(rename = "CONTRACT_TO_HIRE")]
        #[doc = "The job is offered as a contracted position with the understanding that it's converted into a full-time position at the end of the contract. Jobs of this type are also returned by a search for EmploymentType.CONTRACTOR jobs."]
        ContractToHire,
        #[serde(rename = "TEMPORARY")]
        #[doc = "The job is offered as a temporary employment opportunity, usually a short-term engagement."]
        Temporary,
        #[serde(rename = "INTERN")]
        #[doc = "The job is a fixed-term opportunity for students or entry-level job seekers to obtain on-the-job training, typically offered as a summer position."]
        Intern,
        #[serde(rename = "VOLUNTEER")]
        #[doc = "The is an opportunity for an individual to volunteer, where there's no expectation of compensation for the provided services."]
        Volunteer,
        #[serde(rename = "PER_DIEM")]
        #[doc = "The job requires an employee to work on an as-needed basis with a flexible schedule."]
        PerDiem,
        #[serde(rename = "FLY_IN_FLY_OUT")]
        #[doc = "The job involves employing people in remote areas and flying them temporarily to the work site instead of relocating employees and their families permanently."]
        FlyInFlyOut,
        #[serde(rename = "OTHER_EMPLOYMENT_TYPE")]
        #[doc = "The job does not fit any of the other listed types."]
        OtherEmploymentType,
    }
    impl ::std::default::Default for JobQueryEmploymentTypesEnum {
        fn default() -> Self {
            Self::EmploymentTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum JobQueryJobCategoriesEnum {
        #[serde(rename = "JOB_CATEGORY_UNSPECIFIED")]
        #[doc = "The default value if the category isn't specified."]
        JobCategoryUnspecified,
        #[serde(rename = "ACCOUNTING_AND_FINANCE")]
        #[doc = "An accounting and finance job, such as an Accountant."]
        AccountingAndFinance,
        #[serde(rename = "ADMINISTRATIVE_AND_OFFICE")]
        #[doc = "An administrative and office job, such as an Administrative Assistant."]
        AdministrativeAndOffice,
        #[serde(rename = "ADVERTISING_AND_MARKETING")]
        #[doc = "An advertising and marketing job, such as Marketing Manager."]
        AdvertisingAndMarketing,
        #[serde(rename = "ANIMAL_CARE")]
        #[doc = "An animal care job, such as Veterinarian."]
        AnimalCare,
        #[serde(rename = "ART_FASHION_AND_DESIGN")]
        #[doc = "An art, fashion, or design job, such as Designer."]
        ArtFashionAndDesign,
        #[serde(rename = "BUSINESS_OPERATIONS")]
        #[doc = "A business operations job, such as Business Operations Manager."]
        BusinessOperations,
        #[serde(rename = "CLEANING_AND_FACILITIES")]
        #[doc = "A cleaning and facilities job, such as Custodial Staff."]
        CleaningAndFacilities,
        #[serde(rename = "COMPUTER_AND_IT")]
        #[doc = "A computer and IT job, such as Systems Administrator."]
        ComputerAndIt,
        #[serde(rename = "CONSTRUCTION")]
        #[doc = "A construction job, such as General Laborer."]
        Construction,
        #[serde(rename = "CUSTOMER_SERVICE")]
        #[doc = "A customer service job, such s Cashier."]
        CustomerService,
        #[serde(rename = "EDUCATION")]
        #[doc = "An education job, such as School Teacher."]
        Education,
        #[serde(rename = "ENTERTAINMENT_AND_TRAVEL")]
        #[doc = "An entertainment and travel job, such as Flight Attendant."]
        EntertainmentAndTravel,
        #[serde(rename = "FARMING_AND_OUTDOORS")]
        #[doc = "A farming or outdoor job, such as Park Ranger."]
        FarmingAndOutdoors,
        #[serde(rename = "HEALTHCARE")]
        #[doc = "A healthcare job, such as Registered Nurse."]
        Healthcare,
        #[serde(rename = "HUMAN_RESOURCES")]
        #[doc = "A human resources job, such as Human Resources Director."]
        HumanResources,
        #[serde(rename = "INSTALLATION_MAINTENANCE_AND_REPAIR")]
        #[doc = "An installation, maintenance, or repair job, such as Electrician."]
        InstallationMaintenanceAndRepair,
        #[serde(rename = "LEGAL")]
        #[doc = "A legal job, such as Law Clerk."]
        Legal,
        #[serde(rename = "MANAGEMENT")]
        #[doc = "A management job, often used in conjunction with another category, such as Store Manager."]
        Management,
        #[serde(rename = "MANUFACTURING_AND_WAREHOUSE")]
        #[doc = "A manufacturing or warehouse job, such as Assembly Technician."]
        ManufacturingAndWarehouse,
        #[serde(rename = "MEDIA_COMMUNICATIONS_AND_WRITING")]
        #[doc = "A media, communications, or writing job, such as Media Relations."]
        MediaCommunicationsAndWriting,
        #[serde(rename = "OIL_GAS_AND_MINING")]
        #[doc = "An oil, gas or mining job, such as Offshore Driller."]
        OilGasAndMining,
        #[serde(rename = "PERSONAL_CARE_AND_SERVICES")]
        #[doc = "A personal care and services job, such as Hair Stylist."]
        PersonalCareAndServices,
        #[serde(rename = "PROTECTIVE_SERVICES")]
        #[doc = "A protective services job, such as Security Guard."]
        ProtectiveServices,
        #[serde(rename = "REAL_ESTATE")]
        #[doc = "A real estate job, such as Buyer's Agent."]
        RealEstate,
        #[serde(rename = "RESTAURANT_AND_HOSPITALITY")]
        #[doc = "A restaurant and hospitality job, such as Restaurant Server."]
        RestaurantAndHospitality,
        #[serde(rename = "SALES_AND_RETAIL")]
        #[doc = "A sales and/or retail job, such Sales Associate."]
        SalesAndRetail,
        #[serde(rename = "SCIENCE_AND_ENGINEERING")]
        #[doc = "A science and engineering job, such as Lab Technician."]
        ScienceAndEngineering,
        #[serde(rename = "SOCIAL_SERVICES_AND_NON_PROFIT")]
        #[doc = "A social services or non-profit job, such as Case Worker."]
        SocialServicesAndNonProfit,
        #[serde(rename = "SPORTS_FITNESS_AND_RECREATION")]
        #[doc = "A sports, fitness, or recreation job, such as Personal Trainer."]
        SportsFitnessAndRecreation,
        #[serde(rename = "TRANSPORTATION_AND_LOGISTICS")]
        #[doc = "A transportation or logistics job, such as Truck Driver."]
        TransportationAndLogistics,
    }
    impl ::std::default::Default for JobQueryJobCategoriesEnum {
        fn default() -> Self {
            Self::JobCategoryUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An object that represents a latitude/longitude pair. This is expressed as a pair of doubles to represent degrees latitude and degrees longitude. Unless specified otherwise, this must conform to the WGS84 standard. Values must be within normalized ranges."]
    pub struct LatLng {
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
    impl LatLng {
        pub fn builder() -> LatLngBuilder {
            LatLngBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. The List companies response object."]
    pub struct ListCompaniesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "companies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Companies for the current client."]
        pub companies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Company>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information for the API invocation, such as the request tracking id."]
        pub metadata: ::std::option::Option<::std::boxed::Box<ResponseMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListCompaniesResponse {
        pub fn builder() -> ListCompaniesResponseBuilder {
            ListCompaniesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. List jobs response."]
    pub struct ListJobsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Jobs for a given company. The maximum number of items returned is based on the limit field provided in the request."]
        pub jobs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Job>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information for the API invocation, such as the request tracking id."]
        pub metadata: ::std::option::Option<::std::boxed::Box<ResponseMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results."]
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
    #[doc = "Output only. A resource that represents a location with full geographic information."]
    pub struct Location {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latLng")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An object representing a latitude/longitude pair."]
        pub lat_lng: ::std::option::Option<::std::boxed::Box<LatLng>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of a location, which corresponds to the address lines field of PostalAddress. For example, \"Downtown, Atlanta, GA, USA\" has a type of LocationType#NEIGHBORHOOD, and \"Kansas City, KS, USA\" has a type of LocationType#LOCALITY."]
        pub location_type: ::std::option::Option<LocationLocationTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Postal address of the location that includes human readable information, such as postal delivery and payments addresses. Given a postal address, a postal service can deliver items to a premises, P.O. Box, or other delivery location."]
        pub postal_address: ::std::option::Option<::std::boxed::Box<PostalAddress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "radiusInMiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Radius in miles of the job location. This value is derived from the location bounding box in which a circle with the specified radius centered from LatLng covers the area associated with the job location. For example, currently, \"Mountain View, CA, USA\" has a radius of 6.17 miles."]
        pub radius_in_miles: ::std::option::Option<::std::primitive::f64>,
    }
    impl Location {
        pub fn builder() -> LocationBuilder {
            LocationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of a location, which corresponds to the address lines field of PostalAddress. For example, \"Downtown, Atlanta, GA, USA\" has a type of LocationType#NEIGHBORHOOD, and \"Kansas City, KS, USA\" has a type of LocationType#LOCALITY."]
    pub enum LocationLocationTypeEnum {
        #[serde(rename = "LOCATION_TYPE_UNSPECIFIED")]
        #[doc = "Default value if the type is not specified."]
        LocationTypeUnspecified,
        #[serde(rename = "COUNTRY")]
        #[doc = "A country level location."]
        Country,
        #[serde(rename = "ADMINISTRATIVE_AREA")]
        #[doc = "A state or equivalent level location."]
        AdministrativeArea,
        #[serde(rename = "SUB_ADMINISTRATIVE_AREA")]
        #[doc = "A county or equivalent level location."]
        SubAdministrativeArea,
        #[serde(rename = "LOCALITY")]
        #[doc = "A city or equivalent level location."]
        Locality,
        #[serde(rename = "POSTAL_CODE")]
        #[doc = "A postal code level location."]
        PostalCode,
        #[serde(rename = "SUB_LOCALITY")]
        #[doc = "A sublocality is a subdivision of a locality, for example a city borough, ward, or arrondissement. Sublocalities are usually recognized by a local political authority. For example, Manhattan and Brooklyn are recognized as boroughs by the City of New York, and are therefore modeled as sublocalities."]
        SubLocality,
        #[serde(rename = "SUB_LOCALITY_1")]
        #[doc = "A district or equivalent level location."]
        SubLocality1,
        #[serde(rename = "SUB_LOCALITY_2")]
        #[doc = "A smaller district or equivalent level display."]
        SubLocality2,
        #[serde(rename = "NEIGHBORHOOD")]
        #[doc = "A neighborhood level location."]
        Neighborhood,
        #[serde(rename = "STREET_ADDRESS")]
        #[doc = "A street address level location."]
        StreetAddress,
    }
    impl ::std::default::Default for LocationLocationTypeEnum {
        fn default() -> Self {
            Self::LocationTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Input only. Geographic region of the search."]
    pub struct LocationFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The address name, such as \"Mountain View\" or \"Bay Area\"."]
        pub address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "distanceInMiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The distance_in_miles is applied when the location being searched for is identified as a city or smaller. When the location being searched for is a state or larger, this field is ignored."]
        pub distance_in_miles: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latLng")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The latitude and longitude of the geographic center from which to search. This field's ignored if `address` is provided."]
        pub lat_lng: ::std::option::Option<::std::boxed::Box<LatLng>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. CLDR region code of the country/region of the address. This is used to address ambiguity of the user-input location, for example, \"Liverpool\" against \"Liverpool, NY, US\" or \"Liverpool, UK\". Set this field if all the jobs to search against are from a same region, or jobs are world-wide, but the job seeker is from a specific region. See http://cldr.unicode.org/ and http://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: \"CH\" for Switzerland."]
        pub region_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "telecommutePreference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Allows the client to return jobs without a set location, specifically, telecommuting jobs (telecommuting is considered by the service as a special location. Job.posting_region indicates if a job permits telecommuting. If this field is set to TelecommutePreference.TELECOMMUTE_ALLOWED, telecommuting jobs are searched, and address and lat_lng are ignored. If not set or set to TelecommutePreference.TELECOMMUTE_EXCLUDED, telecommute job are not searched. This filter can be used by itself to search exclusively for telecommuting jobs, or it can be combined with another location filter to search for a combination of job locations, such as \"Mountain View\" or \"telecommuting\" jobs. However, when used in combination with other location filters, telecommuting jobs can be treated as less relevant than other jobs in the search response."]
        pub telecommute_preference: ::std::option::Option<LocationFilterTelecommutePreferenceEnum>,
    }
    impl LocationFilter {
        pub fn builder() -> LocationFilterBuilder {
            LocationFilterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Allows the client to return jobs without a set location, specifically, telecommuting jobs (telecommuting is considered by the service as a special location. Job.posting_region indicates if a job permits telecommuting. If this field is set to TelecommutePreference.TELECOMMUTE_ALLOWED, telecommuting jobs are searched, and address and lat_lng are ignored. If not set or set to TelecommutePreference.TELECOMMUTE_EXCLUDED, telecommute job are not searched. This filter can be used by itself to search exclusively for telecommuting jobs, or it can be combined with another location filter to search for a combination of job locations, such as \"Mountain View\" or \"telecommuting\" jobs. However, when used in combination with other location filters, telecommuting jobs can be treated as less relevant than other jobs in the search response."]
    pub enum LocationFilterTelecommutePreferenceEnum {
        #[serde(rename = "TELECOMMUTE_PREFERENCE_UNSPECIFIED")]
        #[doc = "Default value if the telecommute preference is not specified."]
        TelecommutePreferenceUnspecified,
        #[serde(rename = "TELECOMMUTE_EXCLUDED")]
        #[doc = "Exclude telecommute jobs."]
        TelecommuteExcluded,
        #[serde(rename = "TELECOMMUTE_ALLOWED")]
        #[doc = "Allow telecommute jobs."]
        TelecommuteAllowed,
    }
    impl ::std::default::Default for LocationFilterTelecommutePreferenceEnum {
        fn default() -> Self {
            Self::TelecommutePreferenceUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. Job entry with metadata inside SearchJobsResponse."]
    pub struct MatchingJob {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commuteInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Commute information which is generated based on specified CommuteFilter."]
        pub commute_info: ::std::option::Option<::std::boxed::Box<CommuteInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "job")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Job resource that matches the specified SearchJobsRequest."]
        pub job: ::std::option::Option<::std::boxed::Box<Job>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobSummary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A summary of the job with core information that's displayed on the search results listing page."]
        pub job_summary: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobTitleSnippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains snippets of text from the Job.job_title field most closely matching a search query's keywords, if available. The matching query keywords are enclosed in HTML bold tags."]
        pub job_title_snippet: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchTextSnippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains snippets of text from the Job.description and similar fields that most closely match a search query's keywords, if available. All HTML tags in the original fields are stripped when returned in this field, and matching query keywords are enclosed in HTML bold tags."]
        pub search_text_snippet: ::std::option::Option<::std::string::String>,
    }
    impl MatchingJob {
        pub fn builder() -> MatchingJobBuilder {
            MatchingJobBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message representing input to a Mendel server for debug forcing. See go/mendel-debug-forcing for more details. Next ID: 2"]
    pub struct MendelDebugInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namespacedDebugInput")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When a request spans multiple servers, a MendelDebugInput may travel with the request and take effect in all the servers. This field is a map of namespaces to NamespacedMendelDebugInput protos. In a single server, up to two NamespacedMendelDebugInput protos are applied: 1. NamespacedMendelDebugInput with the global namespace (key == \"\"). 2. NamespacedMendelDebugInput with the server's namespace. When both NamespacedMendelDebugInput protos are present, they are merged. See go/mendel-debug-forcing for more details."]
        pub namespaced_debug_input: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<NamespacedDebugInput>>,
        >,
    }
    impl MendelDebugInput {
        pub fn builder() -> MendelDebugInputBuilder {
            MendelDebugInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an amount of money with its currency type."]
    pub struct Money {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The three-letter currency code defined in ISO 4217."]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nanos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
        pub nanos: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "units")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The whole units of the amount. For example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
        pub units: ::std::option::Option<::std::string::String>,
    }
    impl Money {
        pub fn builder() -> MoneyBuilder {
            MoneyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Next ID: 15"]
    pub struct NamespacedDebugInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "absolutelyForcedExpNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of experiment names to be absolutely forced. These experiments will be forced without evaluating the conditions."]
        pub absolutely_forced_exp_names:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "absolutelyForcedExpTags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of experiment tags to be absolutely forced. The experiments with these tags will be forced without evaluating the conditions."]
        pub absolutely_forced_exp_tags:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "absolutelyForcedExps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of experiment ids to be absolutely forced. These ids will be forced without evaluating the conditions."]
        pub absolutely_forced_exps: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conditionallyForcedExpNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of experiment names to be conditionally forced. These experiments will be forced only if their conditions and their parent domain's conditions are true."]
        pub conditionally_forced_exp_names:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conditionallyForcedExpTags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of experiment tags to be conditionally forced. The experiments with these tags will be forced only if their conditions and their parent domain's conditions are true."]
        pub conditionally_forced_exp_tags:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conditionallyForcedExps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of experiment ids to be conditionally forced. These ids will be forced only if their conditions and their parent domain's conditions are true."]
        pub conditionally_forced_exps:
            ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableAutomaticEnrollmentSelection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, disable automatic enrollment selection (at all diversion points). Automatic enrollment selection means experiment selection process based on the experiment's automatic enrollment condition. This does not disable selection of forced experiments."]
        pub disable_automatic_enrollment_selection: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableExpNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of experiment names to be disabled. If an experiment is disabled, it is never selected nor forced. If an aggregate experiment is disabled, its partitions are disabled together. If an experiment with an enrollment is disabled, the enrollment is disabled together. If a name corresponds to a domain, the domain itself and all descendant experiments and domains are disabled together."]
        pub disable_exp_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableExpTags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of experiment tags to be disabled. All experiments that are tagged with one or more of these tags are disabled. If an experiment is disabled, it is never selected nor forced. If an aggregate experiment is disabled, its partitions are disabled together. If an experiment with an enrollment is disabled, the enrollment is disabled together."]
        pub disable_exp_tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableExps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of experiment ids to be disabled. If an experiment is disabled, it is never selected nor forced. If an aggregate experiment is disabled, its partitions are disabled together. If an experiment with an enrollment is disabled, the enrollment is disabled together. If an ID corresponds to a domain, the domain itself and all descendant experiments and domains are disabled together."]
        pub disable_exps: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableManualEnrollmentSelection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, disable manual enrollment selection (at all diversion points). Manual enrollment selection means experiment selection process based on the request's manual enrollment states (a.k.a. opt-in experiments). This does not disable selection of forced experiments."]
        pub disable_manual_enrollment_selection: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableOrganicSelection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, disable organic experiment selection (at all diversion points). Organic selection means experiment selection process based on traffic allocation and diversion condition evaluation. This does not disable selection of forced experiments. This is useful in cases when it is not known whether experiment selection behavior is responsible for a error or breakage. Disabling organic selection may help to isolate the cause of a given problem."]
        pub disable_organic_selection: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "forcedFlags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flags to force in a particular experiment state. Map from flag name to flag value."]
        pub forced_flags:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "forcedRollouts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rollouts to force in a particular experiment state. Map from rollout name to rollout value."]
        pub forced_rollouts:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::primitive::bool>>,
    }
    impl NamespacedDebugInput {
        pub fn builder() -> NamespacedDebugInputBuilder {
            NamespacedDebugInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Input only. Use this field to specify bucketing option for the histogram search response."]
    pub struct NumericBucketingOption {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketBounds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Two adjacent values form a histogram bucket. Values should be in ascending order. For example, if [5, 10, 15] are provided, four buckets are created: (-inf, 5), 5, 10), [10, 15), [15, inf). At most 20 [buckets_bound is supported."]
        pub bucket_bounds: ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requiresMinMax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If set to true, the histogram result includes minimum/maximum value of the numeric field."]
        pub requires_min_max: ::std::option::Option<::std::primitive::bool>,
    }
    impl NumericBucketingOption {
        pub fn builder() -> NumericBucketingOptionBuilder {
            NumericBucketingOptionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. Custom numeric bucketing result."]
    pub struct NumericBucketingResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "counts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count within each bucket. Its size is the length of NumericBucketingOption.bucket_bounds plus 1."]
        pub counts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BucketizedCount>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Stores the maximum value of the numeric field. Is populated only if [NumericBucketingOption.requires_min_max] is set to true."]
        pub max_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Stores the minimum value of the numeric field. Will be populated only if [NumericBucketingOption.requires_min_max] is set to true."]
        pub min_value: ::std::option::Option<::std::primitive::f64>,
    }
    impl NumericBucketingResult {
        pub fn builder() -> NumericBucketingResultBuilder {
            NumericBucketingResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a postal address, e.g. for postal delivery or payments addresses. Given a postal address, a postal service can deliver items to a premise, P.O. Box or similar. It is not intended to model geographical locations (roads, towns, mountains). In typical usage an address would be created via user input or from importing existing data, depending on the type of process. Advice on address input / editing: - Use an i18n-ready address widget such as https://github.com/google/libaddressinput) - Users should not be presented with UI elements for input or editing of fields outside countries where that field is used. For more guidance on how to use this schema, please see: https://support.google.com/business/answer/6397478"]
    pub struct PostalAddress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addressLines")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unstructured address lines describing the lower levels of an address. Because values in address_lines do not have type information and may sometimes contain multiple values in a single field (e.g. \"Austin, TX\"), it is important that the line order is clear. The order of address lines should be \"envelope order\" for the country/region of the address. In places where this can vary (e.g. Japan), address_language is used to make it explicit (e.g. \"ja\" for large-to-small ordering and \"ja-Latn\" or \"en\" for small-to-large). This way, the most specific line of an address can be selected based on the language. The minimum permitted structural representation of an address consists of a region_code with all remaining information placed in the address_lines. It would be possible to format such an address very approximately without geocoding, but no semantic reasoning could be made about any of the address components until it was at least partially resolved. Creating an address only containing a region_code and address_lines, and then geocoding is the recommended way to handle completely unstructured addresses (as opposed to guessing which parts of the address should be localities or administrative areas)."]
        pub address_lines: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "administrativeArea")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Highest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state, a province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community (e.g. \"Barcelona\" and not \"Catalonia\"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland this should be left unpopulated."]
        pub administrative_area: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. BCP-47 language code of the contents of this address (if known). This is often the UI language of the input form or is expected to match one of the languages used in the address' country/region, or their transliterated equivalents. This can affect formatting in certain countries, but is not critical to the correctness of the data and will never affect any validation or other non-formatting related operations. If this value is not known, it should be omitted (rather than specifying a possibly incorrect default). Examples: \"zh-Hant\", \"ja\", \"ja-Latn\", \"en\"."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Generally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world where localities are not well defined or do not fit into this structure well, leave locality empty and use address_lines."]
        pub locality: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "organization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The name of the organization at the address."]
        pub organization: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Postal code of the address. Not all countries use or require postal codes to be present, but where they are used, they may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.)."]
        pub postal_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recipients")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The recipient at the address. This field may, under certain circumstances, contain multiline information. For example, it might contain \"care of\" information."]
        pub recipients: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to ensure the value is correct. See http://cldr.unicode.org/ and http://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: \"CH\" for Switzerland."]
        pub region_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The schema revision of the `PostalAddress`. This must be set to 0, which is the latest revision. All new revisions **must** be backward compatible with old revisions."]
        pub revision: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sortingCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Additional, country-specific, sorting code. This is not used in most regions. Where it is used, the value is either a string like \"CEDEX\", optionally followed by a number (e.g. \"CEDEX 7\"), or just a number alone, representing the \"sector code\" (Jamaica), \"delivery area indicator\" (Malawi) or \"post office indicator\" (e.g. Côte d'Ivoire)."]
        pub sorting_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sublocality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Sublocality of the address. For example, this can be neighborhoods, boroughs, districts."]
        pub sublocality: ::std::option::Option<::std::string::String>,
    }
    impl PostalAddress {
        pub fn builder() -> PostalAddressBuilder {
            PostalAddressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Input only. Options for job processing."]
    pub struct ProcessingOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableStreetAddressResolution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If set to `true`, the service does not attempt to resolve a more precise address for the job."]
        pub disable_street_address_resolution: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "htmlSanitization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Option for job HTML content sanitization. Applied fields are: * description * applicationInfo.instruction * incentives * qualifications * responsibilities HTML tags in these fields may be stripped if sanitiazation is not disabled. Defaults to HtmlSanitization.SIMPLE_FORMATTING_ONLY."]
        pub html_sanitization: ::std::option::Option<ProcessingOptionsHtmlSanitizationEnum>,
    }
    impl ProcessingOptions {
        pub fn builder() -> ProcessingOptionsBuilder {
            ProcessingOptionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Option for job HTML content sanitization. Applied fields are: * description * applicationInfo.instruction * incentives * qualifications * responsibilities HTML tags in these fields may be stripped if sanitiazation is not disabled. Defaults to HtmlSanitization.SIMPLE_FORMATTING_ONLY."]
    pub enum ProcessingOptionsHtmlSanitizationEnum {
        #[serde(rename = "HTML_SANITIZATION_UNSPECIFIED")]
        #[doc = "Default value."]
        HtmlSanitizationUnspecified,
        #[serde(rename = "HTML_SANITIZATION_DISABLED")]
        #[doc = "Disables sanitization on HTML input."]
        HtmlSanitizationDisabled,
        #[serde(rename = "SIMPLE_FORMATTING_ONLY")]
        #[doc = "Sanitizes HTML input, only accepts bold, italic, ordered list, and unordered list markup tags."]
        SimpleFormattingOnly,
    }
    impl ::std::default::Default for ProcessingOptionsHtmlSanitizationEnum {
        fn default() -> Self {
            Self::HtmlSanitizationUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Input only. Meta information related to the job searcher or entity conducting the job search. This information is used to improve the performance of the service."]
    pub struct RequestMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The type of device used by the job seeker at the time of the call to the service."]
        pub device_info: ::std::option::Option<::std::boxed::Box<DeviceInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The client-defined scope or source of the service call, which typically is the domain on which the service has been implemented and is currently being run. For example, if the service is being run by client *Foo, Inc.*, on job board www.foo.com and career site www.bar.com, then this field is set to \"foo.com\" for use on the job board, and \"bar.com\" for use on the career site. If this field isn't available for some reason, send \"UNKNOWN\". Any improvements to the model for a particular tenant site rely on this field being set correctly to a domain. The maximum number of allowed characters is 255."]
        pub domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A unique session identification string. A session is defined as the duration of an end user's interaction with the service over a certain period. Obfuscate this field for privacy concerns before providing it to the service. If this field is not available for some reason, send \"UNKNOWN\". Note that any improvements to the model for a particular tenant site, rely on this field being set correctly to some unique session_id. The maximum number of allowed characters is 255."]
        pub session_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A unique user identification string, as determined by the client. To have the strongest positive impact on search quality make sure the client-level is unique. Obfuscate this field for privacy concerns before providing it to the service. If this field is not available for some reason, send \"UNKNOWN\". Note that any improvements to the model for a particular tenant site, rely on this field being set correctly to a unique user_id. The maximum number of allowed characters is 255."]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl RequestMetadata {
        pub fn builder() -> RequestMetadataBuilder {
            RequestMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. Additional information returned to client, such as debugging information."]
    pub struct ResponseMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique id associated with this call. This id is logged for tracking purposes."]
        pub request_id: ::std::option::Option<::std::string::String>,
    }
    impl ResponseMetadata {
        pub fn builder() -> ResponseMetadataBuilder {
            ResponseMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Input only. The Request body of the `SearchJobs` call."]
    pub struct SearchJobsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableKeywordMatch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Controls whether to disable exact keyword match on Job.job_title, Job.description, Job.company_display_name, Job.locations, Job.qualifications. When disable keyword match is turned off, a keyword match returns jobs that do not match given category filters when there are matching keywords. For example, the query \"program manager,\" a result is returned even if the job posting has the title \"software developer,\" which does not fall into \"program manager\" ontology, but does have \"program manager\" appearing in its description. For queries like \"cloud\" that does not contain title or location specific ontology, jobs with \"cloud\" keyword matches are returned regardless of this flag's value. Please use Company.keyword_searchable_custom_fields or Company.keyword_searchable_custom_attributes if company specific globally matched custom field/attribute string values is needed. Enabling keyword match improves recall of subsequent search requests. Defaults to false."]
        pub disable_keyword_match: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diversificationLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Controls whether highly similar jobs are returned next to each other in the search results. Jobs are identified as highly similar based on their titles, job categories, and locations. Highly similar results are clustered so that only one representative job of the cluster is displayed to the job seeker higher up in the results, with the other jobs being displayed lower down in the results. Defaults to DiversificationLevel.SIMPLE if no value is specified."]
        pub diversification_level: ::std::option::Option<SearchJobsRequestDiversificationLevelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableBroadening")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Controls whether to broaden the search when it produces sparse results. Broadened queries append results to the end of the matching results list. Defaults to false."]
        pub enable_broadening: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "histogramFacets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Histogram requests for jobs matching JobQuery."]
        pub histogram_facets: ::std::option::Option<::std::boxed::Box<HistogramFacets>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobQuery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Query used to search against jobs, such as keyword, location filters, etc."]
        pub job_query: ::std::option::Option<::std::boxed::Box<JobQuery>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobView")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The desired job attributes returned for jobs in the search response. Defaults to JobView.SMALL if no value is specified."]
        pub job_view: ::std::option::Option<SearchJobsRequestJobViewEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An integer that specifies the current offset (that is, starting result location, amongst the jobs deemed by the API as relevant) in search results. This field is only considered if page_token is unset. The maximum allowed value is 5000. Otherwise an error is thrown. For example, 0 means to return results starting from the first matching job, and 10 means to return from the 11th job. This can be used for pagination, (for example, pageSize = 10 and offset = 10 means to return from the second page)."]
        pub offset: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orderBy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The criteria determining how search results are sorted. Default is \"relevance desc\". Supported options are: * `\"relevance desc\"`: By relevance descending, as determined by the API algorithms. Relevance thresholding of query results is only available with this ordering. * `\"posting_publish_time desc\"`: By Job.posting_publish_time descending. * `\"posting_update_time desc\"`: By Job.posting_update_time descending. * `\"title\"`: By Job.title ascending. * `\"title desc\"`: By Job.title descending. * `\"annualized_base_compensation\"`: By job's CompensationInfo.annualized_base_compensation_range ascending. Jobs whose annualized base compensation is unspecified are put at the end of search results. * `\"annualized_base_compensation desc\"`: By job's CompensationInfo.annualized_base_compensation_range descending. Jobs whose annualized base compensation is unspecified are put at the end of search results. * `\"annualized_total_compensation\"`: By job's CompensationInfo.annualized_total_compensation_range ascending. Jobs whose annualized base compensation is unspecified are put at the end of search results. * `\"annualized_total_compensation desc\"`: By job's CompensationInfo.annualized_total_compensation_range descending. Jobs whose annualized base compensation is unspecified are put at the end of search results."]
        pub order_by: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A limit on the number of jobs returned in the search results. Increasing this value above the default value of 10 can increase search response time. The value can be between 1 and 100."]
        pub page_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The token specifying the current offset within search results. See SearchJobsResponse.next_page_token for an explanation of how to obtain the next set of query results."]
        pub page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The meta information collected about the job searcher, used to improve the search quality of the service. The identifiers (such as `user_id`) are provided by users, and must be unique and consistent."]
        pub request_metadata: ::std::option::Option<::std::boxed::Box<RequestMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requirePreciseResultSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated."]
        pub require_precise_result_size: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Mode of a search. Defaults to SearchMode.JOB_SEARCH."]
        pub search_mode: ::std::option::Option<SearchJobsRequestSearchModeEnum>,
    }
    impl SearchJobsRequest {
        pub fn builder() -> SearchJobsRequestBuilder {
            SearchJobsRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Controls whether highly similar jobs are returned next to each other in the search results. Jobs are identified as highly similar based on their titles, job categories, and locations. Highly similar results are clustered so that only one representative job of the cluster is displayed to the job seeker higher up in the results, with the other jobs being displayed lower down in the results. Defaults to DiversificationLevel.SIMPLE if no value is specified."]
    pub enum SearchJobsRequestDiversificationLevelEnum {
        #[serde(rename = "DIVERSIFICATION_LEVEL_UNSPECIFIED")]
        #[doc = "The diversification level isn't specified. By default, jobs with this enum are ordered according to SIMPLE diversifying behavior."]
        DiversificationLevelUnspecified,
        #[serde(rename = "DISABLED")]
        #[doc = "Disables diversification. Jobs that would normally be pushed to the last page would not have their positions altered. This may result in highly similar jobs appearing in sequence in the search results."]
        Disabled,
        #[serde(rename = "SIMPLE")]
        #[doc = "Default diversifying behavior. The result list is ordered so that highly similar results are pushed to the end of the last page of search results."]
        Simple,
    }
    impl ::std::default::Default for SearchJobsRequestDiversificationLevelEnum {
        fn default() -> Self {
            Self::DiversificationLevelUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The desired job attributes returned for jobs in the search response. Defaults to JobView.SMALL if no value is specified."]
    pub enum SearchJobsRequestJobViewEnum {
        #[serde(rename = "JOB_VIEW_UNSPECIFIED")]
        #[doc = "Default value."]
        JobViewUnspecified,
        #[serde(rename = "JOB_VIEW_ID_ONLY")]
        #[doc = "A ID only view of job, with following attributes: Job.name, Job.requisition_id, Job.language_code."]
        JobViewIdOnly,
        #[serde(rename = "JOB_VIEW_MINIMAL")]
        #[doc = "A minimal view of the job, with the following attributes: Job.name, Job.requisition_id, Job.title, Job.company_name, Job.DerivedInfo.locations, Job.language_code."]
        JobViewMinimal,
        #[serde(rename = "JOB_VIEW_SMALL")]
        #[doc = "A small view of the job, with the following attributes in the search results: Job.name, Job.requisition_id, Job.title, Job.company_name, Job.DerivedInfo.locations, Job.visibility, Job.language_code, Job.description."]
        JobViewSmall,
        #[serde(rename = "JOB_VIEW_FULL")]
        #[doc = "All available attributes are included in the search results."]
        JobViewFull,
    }
    impl ::std::default::Default for SearchJobsRequestJobViewEnum {
        fn default() -> Self {
            Self::JobViewUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Mode of a search. Defaults to SearchMode.JOB_SEARCH."]
    pub enum SearchJobsRequestSearchModeEnum {
        #[serde(rename = "SEARCH_MODE_UNSPECIFIED")]
        #[doc = "The mode of the search method isn't specified. The default search behavior is identical to JOB_SEARCH search behavior."]
        SearchModeUnspecified,
        #[serde(rename = "JOB_SEARCH")]
        #[doc = "The job search matches against all jobs, and featured jobs (jobs with promotionValue > 0) are not specially handled."]
        JobSearch,
        #[serde(rename = "FEATURED_JOB_SEARCH")]
        #[doc = "The job search matches only against featured jobs (jobs with a promotionValue > 0). This method doesn't return any jobs having a promotionValue <= 0. The search results order is determined by the promotionValue (jobs with a higher promotionValue are returned higher up in the search results), with relevance being used as a tiebreaker."]
        FeaturedJobSearch,
    }
    impl ::std::default::Default for SearchJobsRequestSearchModeEnum {
        fn default() -> Self {
            Self::SearchModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. Response for SearchJob method."]
    pub struct SearchJobsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "broadenedQueryJobsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If query broadening is enabled, we may append additional results from the broadened query. This number indicates how many of the jobs returned in the jobs field are from the broadened query. These results are always at the end of the jobs list. In particular, a value of 0, or if the field isn't set, all the jobs in the jobs list are from the original (without broadening) query. If this field is non-zero, subsequent requests with offset after this result set should contain all broadened results."]
        pub broadened_query_jobs_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "estimatedTotalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An estimation of the number of jobs that match the specified query. This number is not guaranteed to be accurate. For accurate results, see SearchJobsResponse.total_size."]
        pub estimated_total_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "histogramResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The histogram results that match specified SearchJobsRequest.histogram_facets."]
        pub histogram_results: ::std::option::Option<::std::boxed::Box<HistogramResults>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationFilters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location filters that the service applied to the specified query. If any filters are lat-lng based, the JobLocation.location_type is JobLocation.LocationType#LOCATION_TYPE_UNSPECIFIED."]
        pub location_filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Location>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchingJobs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Job entities that match the specified SearchJobsRequest."]
        pub matching_jobs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MatchingJob>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information for the API invocation, such as the request tracking id."]
        pub metadata: ::std::option::Option<::std::boxed::Box<ResponseMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that specifies the starting position of the next page of results. This field is empty if there are no more results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spellCorrection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The spell checking result, and correction."]
        pub spell_correction: ::std::option::Option<::std::boxed::Box<SpellingCorrection>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The precise result count with limit 100,000."]
        pub total_size: ::std::option::Option<::std::primitive::i64>,
    }
    impl SearchJobsResponse {
        pub fn builder() -> SearchJobsResponseBuilder {
            SearchJobsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. Spell check result."]
    pub struct SpellingCorrection {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "corrected")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if the query was corrected by the spell checker."]
        pub corrected: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "correctedText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Correction output consisting of the corrected keyword string."]
        pub corrected_text: ::std::option::Option<::std::string::String>,
    }
    impl SpellingCorrection {
        pub fn builder() -> SpellingCorrectionBuilder {
            SpellingCorrectionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`."]
    pub struct TimeOfDay {
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
    impl TimeOfDay {
        pub fn builder() -> TimeOfDayBuilder {
            TimeOfDayBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message representing a period of time between two timestamps."]
    pub struct TimestampRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End of the period."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Begin of the period."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl TimestampRange {
        pub fn builder() -> TimestampRangeBuilder {
            TimestampRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Input only. Request for updating a specified company."]
    pub struct UpdateCompanyRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "company")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The company resource to replace the current resource in the system."]
        pub company: ::std::option::Option<::std::boxed::Box<Company>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional but strongly recommended for the best service experience. If update_mask is provided, only the specified fields in company are updated. Otherwise all the fields are updated. A field mask to specify the company fields to be updated. Only top level fields of Company are supported."]
        pub update_mask: ::std::option::Option<::std::string::String>,
    }
    impl UpdateCompanyRequest {
        pub fn builder() -> UpdateCompanyRequestBuilder {
            UpdateCompanyRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Input only. Update job request."]
    pub struct UpdateJobRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "job")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The Job to be updated."]
        pub job: ::std::option::Option<::std::boxed::Box<Job>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional but strongly recommended to be provided for the best service experience. If update_mask is provided, only the specified fields in job are updated. Otherwise all the fields are updated. A field mask to restrict the fields that are updated. Only top level fields of Job are supported."]
        pub update_mask: ::std::option::Option<::std::string::String>,
    }
    impl UpdateJobRequest {
        pub fn builder() -> UpdateJobRequestBuilder {
            UpdateJobRequestBuilder::default()
        }
    }
}
