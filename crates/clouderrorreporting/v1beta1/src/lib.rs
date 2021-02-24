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
            pub mod events {
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
                            #[serde(rename = "groupId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. The group for which events shall be returned."]
                            pub group_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The maximum number of results to return per response."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. A `next_page_token` provided by a previous response."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "serviceFilter.resourceType")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The exact value to match against [`ServiceContext.resource_type`](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.resource_type)."]
                            pub service_filter_resource_type:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "serviceFilter.service")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The exact value to match against [`ServiceContext.service`](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.service)."]
                            pub service_filter_service:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "serviceFilter.version")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The exact value to match against [`ServiceContext.version`](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.version)."]
                            pub service_filter_version:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "timeRange.period")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Restricts the query to the specified time range."]
                            pub time_range_period:
                                ::std::option::Option<QueryParametersTimeRangePeriodEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Restricts the query to the specified time range."]
                        pub enum QueryParametersTimeRangePeriodEnum {
                            #[serde(rename = "PERIOD_UNSPECIFIED")]
                            #[doc = "Do not use."]
                            PeriodUnspecified,
                            #[serde(rename = "PERIOD_1_HOUR")]
                            #[doc = "Retrieve data for the last hour. Recommended minimum timed count duration: 1 min."]
                            Period1Hour,
                            #[serde(rename = "PERIOD_6_HOURS")]
                            #[doc = "Retrieve data for the last 6 hours. Recommended minimum timed count duration: 10 min."]
                            Period6Hours,
                            #[serde(rename = "PERIOD_1_DAY")]
                            #[doc = "Retrieve data for the last day. Recommended minimum timed count duration: 1 hour."]
                            Period1Day,
                            #[serde(rename = "PERIOD_1_WEEK")]
                            #[doc = "Retrieve data for the last week. Recommended minimum timed count duration: 6 hours."]
                            Period1Week,
                            #[serde(rename = "PERIOD_30_DAYS")]
                            #[doc = "Retrieve data for the last 30 days. Recommended minimum timed count duration: 1 day."]
                            Period30Days,
                        }
                        impl ::std::default::Default for QueryParametersTimeRangePeriodEnum {
                            fn default() -> Self {
                                Self::PeriodUnspecified
                            }
                        }
                    }
                }
            }
            pub mod group_stats {
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
                            #[serde(rename = "alignment")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The alignment of the timed counts to be returned. Default is `ALIGNMENT_EQUAL_AT_END`."]
                            pub alignment: ::std::option::Option<QueryParametersAlignmentEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "alignmentTime")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Time where the timed counts shall be aligned if rounded alignment is chosen. Default is 00:00 UTC."]
                            pub alignment_time: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "groupId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. List all ErrorGroupStats with these IDs."]
                            pub group_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "order")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The sort order in which the results are returned. Default is `COUNT_DESC`."]
                            pub order: ::std::option::Option<QueryParametersOrderEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The maximum number of results to return per response. Default is 20."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. A `next_page_token` provided by a previous response. To view additional results, pass this token along with the identical query parameters as the first request."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "serviceFilter.resourceType")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The exact value to match against [`ServiceContext.resource_type`](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.resource_type)."]
                            pub service_filter_resource_type:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "serviceFilter.service")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The exact value to match against [`ServiceContext.service`](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.service)."]
                            pub service_filter_service:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "serviceFilter.version")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The exact value to match against [`ServiceContext.version`](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.version)."]
                            pub service_filter_version:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "timeRange.period")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Restricts the query to the specified time range."]
                            pub time_range_period:
                                ::std::option::Option<QueryParametersTimeRangePeriodEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "timedCountDuration")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The preferred duration for a single returned `TimedCount`. If not set, no timed counts are returned."]
                            pub timed_count_duration: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Optional. The alignment of the timed counts to be returned. Default is `ALIGNMENT_EQUAL_AT_END`."]
                        pub enum QueryParametersAlignmentEnum {
                            #[serde(rename = "ERROR_COUNT_ALIGNMENT_UNSPECIFIED")]
                            #[doc = "No alignment specified."]
                            ErrorCountAlignmentUnspecified,
                            #[serde(rename = "ALIGNMENT_EQUAL_ROUNDED")]
                            #[doc = "The time periods shall be consecutive, have width equal to the requested duration, and be aligned at the `alignment_time` provided in the request. The `alignment_time` does not have to be inside the query period but even if it is outside, only time periods are returned which overlap with the query period. A rounded alignment will typically result in a different size of the first or the last time period."]
                            AlignmentEqualRounded,
                            #[serde(rename = "ALIGNMENT_EQUAL_AT_END")]
                            #[doc = "The time periods shall be consecutive, have width equal to the requested duration, and be aligned at the end of the requested time period. This can result in a different size of the first time period."]
                            AlignmentEqualAtEnd,
                        }
                        impl ::std::default::Default for QueryParametersAlignmentEnum {
                            fn default() -> Self {
                                Self::ErrorCountAlignmentUnspecified
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Optional. The sort order in which the results are returned. Default is `COUNT_DESC`."]
                        pub enum QueryParametersOrderEnum {
                            #[serde(rename = "GROUP_ORDER_UNSPECIFIED")]
                            #[doc = "No group order specified."]
                            GroupOrderUnspecified,
                            #[serde(rename = "COUNT_DESC")]
                            #[doc = "Total count of errors in the given time window in descending order."]
                            CountDesc,
                            #[serde(rename = "LAST_SEEN_DESC")]
                            #[doc = "Timestamp when the group was last seen in the given time window in descending order."]
                            LastSeenDesc,
                            #[serde(rename = "CREATED_DESC")]
                            #[doc = "Timestamp when the group was created in descending order."]
                            CreatedDesc,
                            #[serde(rename = "AFFECTED_USERS_DESC")]
                            #[doc = "Number of affected users in the given time window in descending order."]
                            AffectedUsersDesc,
                        }
                        impl ::std::default::Default for QueryParametersOrderEnum {
                            fn default() -> Self {
                                Self::GroupOrderUnspecified
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Restricts the query to the specified time range."]
                        pub enum QueryParametersTimeRangePeriodEnum {
                            #[serde(rename = "PERIOD_UNSPECIFIED")]
                            #[doc = "Do not use."]
                            PeriodUnspecified,
                            #[serde(rename = "PERIOD_1_HOUR")]
                            #[doc = "Retrieve data for the last hour. Recommended minimum timed count duration: 1 min."]
                            Period1Hour,
                            #[serde(rename = "PERIOD_6_HOURS")]
                            #[doc = "Retrieve data for the last 6 hours. Recommended minimum timed count duration: 10 min."]
                            Period6Hours,
                            #[serde(rename = "PERIOD_1_DAY")]
                            #[doc = "Retrieve data for the last day. Recommended minimum timed count duration: 1 hour."]
                            Period1Day,
                            #[serde(rename = "PERIOD_1_WEEK")]
                            #[doc = "Retrieve data for the last week. Recommended minimum timed count duration: 6 hours."]
                            Period1Week,
                            #[serde(rename = "PERIOD_30_DAYS")]
                            #[doc = "Retrieve data for the last 30 days. Recommended minimum timed count duration: 1 day."]
                            Period30Days,
                        }
                        impl ::std::default::Default for QueryParametersTimeRangePeriodEnum {
                            fn default() -> Self {
                                Self::PeriodUnspecified
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
    #[doc = "Response message for deleting error events."]
    pub struct DeleteEventsResponse {}
    impl DeleteEventsResponse {
        pub fn builder() -> DeleteEventsResponseBuilder {
            DeleteEventsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A description of the context in which an error occurred. This data should be provided by the application when reporting an error, unless the error report has been generated automatically from Google App Engine logs."]
    pub struct ErrorContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpRequest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP request which was processed when the error was triggered."]
        pub http_request: ::std::option::Option<::std::boxed::Box<HttpRequestContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location in the source code where the decision was made to report the error, usually the place where it was logged. For a logged exception this would be the source line where the exception is logged, usually close to the place where it was caught."]
        pub report_location: ::std::option::Option<::std::boxed::Box<SourceLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceReferences")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Source code that was used to build the executable which has caused the given error message."]
        pub source_references:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SourceReference>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user who caused or was affected by the crash. This can be a user ID, an email address, or an arbitrary token that uniquely identifies the user. When sending an error report, leave this field empty if the user was not logged in. In this case the Error Reporting system will use other data, such as remote IP address, to distinguish affected users. See `affected_users_count` in `ErrorGroupStats`."]
        pub user: ::std::option::Option<::std::string::String>,
    }
    impl ErrorContext {
        pub fn builder() -> ErrorContextBuilder {
            ErrorContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An error event which is returned by the Error Reporting system."]
    pub struct ErrorEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data about the context in which the error occurred."]
        pub context: ::std::option::Option<::std::boxed::Box<ErrorContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time when the event occurred as provided in the error report. If the report did not contain a timestamp, the time the error was received by the Error Reporting system is used."]
        pub event_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stack trace that was reported or logged by the service."]
        pub message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `ServiceContext` for which this error was reported."]
        pub service_context: ::std::option::Option<::std::boxed::Box<ServiceContext>>,
    }
    impl ErrorEvent {
        pub fn builder() -> ErrorEventBuilder {
            ErrorEventBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Description of a group of similar error events."]
    pub struct ErrorGroup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Group IDs are unique for a given project. If the same kind of error occurs in different service contexts, it will receive the same group ID."]
        pub group_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The group resource name. Example: projects/my-project-123/groups/CNSgkpnppqKCUw"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resolutionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error group's resolution status. An unspecified resolution status will be interpreted as OPEN"]
        pub resolution_status: ::std::option::Option<ErrorGroupResolutionStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackingIssues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Associated tracking issues."]
        pub tracking_issues:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TrackingIssue>>>,
    }
    impl ErrorGroup {
        pub fn builder() -> ErrorGroupBuilder {
            ErrorGroupBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Error group's resolution status. An unspecified resolution status will be interpreted as OPEN"]
    pub enum ErrorGroupResolutionStatusEnum {
        #[serde(rename = "RESOLUTION_STATUS_UNSPECIFIED")]
        #[doc = "Status is unknown. When left unspecified in requests, it is treated like OPEN."]
        ResolutionStatusUnspecified,
        #[serde(rename = "OPEN")]
        #[doc = "The error group is not being addressed. This is the default for new groups. It is also used for errors re-occurring after marked RESOLVED."]
        Open,
        #[serde(rename = "ACKNOWLEDGED")]
        #[doc = "Error Group manually acknowledged, it can have an issue link attached."]
        Acknowledged,
        #[serde(rename = "RESOLVED")]
        #[doc = "Error Group manually resolved, more events for this group are not expected to occur."]
        Resolved,
        #[serde(rename = "MUTED")]
        #[doc = "The error group is muted and excluded by default on group stats requests."]
        Muted,
    }
    impl ::std::default::Default for ErrorGroupResolutionStatusEnum {
        fn default() -> Self {
            Self::ResolutionStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data extracted for a specific group based on certain filter criteria, such as a given time period and/or service filter."]
    pub struct ErrorGroupStats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "affectedServices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Service contexts with a non-zero error count for the given filter criteria. This list can be truncated if multiple services are affected. Refer to `num_affected_services` for the total count."]
        pub affected_services:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ServiceContext>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "affectedUsersCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Approximate number of affected users in the given group that match the filter criteria. Users are distinguished by data in the `ErrorContext` of the individual error events, such as their login name or their remote IP address in case of HTTP requests. The number of affected users can be zero even if the number of errors is non-zero if no data was provided from which the affected user could be deduced. Users are counted based on data in the request context that was provided in the error report. If more users are implicitly affected, such as due to a crash of the whole service, this is not reflected here."]
        pub affected_users_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Approximate total number of events in the given group that match the filter criteria."]
        pub count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstSeenTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Approximate first occurrence that was ever seen for this group and which matches the given filter criteria, ignoring the time_range that was specified in the request."]
        pub first_seen_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "group")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Group data that is independent of the filter criteria."]
        pub group: ::std::option::Option<::std::boxed::Box<ErrorGroup>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastSeenTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Approximate last occurrence that was ever seen for this group and which matches the given filter criteria, ignoring the time_range that was specified in the request."]
        pub last_seen_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numAffectedServices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of services with a non-zero error count for the given filter criteria."]
        pub num_affected_services: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "representative")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An arbitrary event that is chosen as representative for the whole group. The representative event is intended to be used as a quick preview for the whole group. Events in the group are usually sufficiently similar to each other such that showing an arbitrary representative provides insight into the characteristics of the group as a whole."]
        pub representative: ::std::option::Option<::std::boxed::Box<ErrorEvent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timedCounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Approximate number of occurrences over time. Timed counts returned by ListGroups are guaranteed to be: - Inside the requested time interval - Non-overlapping, and - Ordered by ascending time."]
        pub timed_counts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TimedCount>>>,
    }
    impl ErrorGroupStats {
        pub fn builder() -> ErrorGroupStatsBuilder {
            ErrorGroupStatsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "HTTP request data that is related to a reported error. This data should be provided by the application when reporting an error, unless the error report has been generated automatically from Google App Engine logs."]
    pub struct HttpRequestContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of HTTP request, such as `GET`, `POST`, etc."]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referrer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The referrer information that is provided with the request."]
        pub referrer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "remoteIp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IP address from which the request originated. This can be IPv4, IPv6, or a token which is derived from the IP address, depending on the data that has been provided in the error report."]
        pub remote_ip: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseStatusCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP response status code for the request."]
        pub response_status_code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the request."]
        pub url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userAgent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user agent information that is provided with the request."]
        pub user_agent: ::std::option::Option<::std::string::String>,
    }
    impl HttpRequestContext {
        pub fn builder() -> HttpRequestContextBuilder {
            HttpRequestContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains a set of requested error events."]
    pub struct ListEventsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorEvents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error events which match the given request."]
        pub error_events: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ErrorEvent>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If non-empty, more results are available. Pass this token, along with the same query parameters as the first request, to view the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeRangeBegin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp specifies the start time to which the request was restricted."]
        pub time_range_begin: ::std::option::Option<::std::string::String>,
    }
    impl ListEventsResponse {
        pub fn builder() -> ListEventsResponseBuilder {
            ListEventsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains a set of requested error group stats."]
    pub struct ListGroupStatsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorGroupStats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error group stats which match the given request."]
        pub error_group_stats:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ErrorGroupStats>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If non-empty, more results are available. Pass this token, along with the same query parameters as the first request, to view the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeRangeBegin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp specifies the start time to which the request was restricted. The start time is set based on the requested time range. It may be adjusted to a later time if a project has exceeded the storage quota and older data has been deleted."]
        pub time_range_begin: ::std::option::Option<::std::string::String>,
    }
    impl ListGroupStatsResponse {
        pub fn builder() -> ListGroupStatsResponseBuilder {
            ListGroupStatsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for reporting an individual error event. Data may be added to this message in the future."]
    pub struct ReportErrorEventResponse {}
    impl ReportErrorEventResponse {
        pub fn builder() -> ReportErrorEventResponseBuilder {
            ReportErrorEventResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An error event which is reported to the Error Reporting system."]
    pub struct ReportedErrorEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A description of the context in which the error occurred."]
        pub context: ::std::option::Option<::std::boxed::Box<ErrorContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Time when the event occurred. If not provided, the time when the event was received by the Error Reporting system will be used."]
        pub event_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The error message. If no `context.reportLocation` is provided, the message must contain a header (typically consisting of the exception type name and an error message) and an exception stack trace in one of the supported programming languages and formats. Supported languages are Java, Python, JavaScript, Ruby, C#, PHP, and Go. Supported stack trace formats are: * **Java**: Must be the return value of [`Throwable.printStackTrace()`](https://docs.oracle.com/javase/7/docs/api/java/lang/Throwable.html#printStackTrace%28%29). * **Python**: Must be the return value of [`traceback.format_exc()`](https://docs.python.org/2/library/traceback.html#traceback.format_exc). * **JavaScript**: Must be the value of [`error.stack`](https://github.com/v8/v8/wiki/Stack-Trace-API) as returned by V8. * **Ruby**: Must contain frames returned by [`Exception.backtrace`](https://ruby-doc.org/core-2.2.0/Exception.html#method-i-backtrace). * **C#**: Must be the return value of [`Exception.ToString()`](https://msdn.microsoft.com/en-us/library/system.exception.tostring.aspx). * **PHP**: Must start with `PHP (Notice|Parse error|Fatal error|Warning)` and contain the result of [`(string)$exception`](http://php.net/manual/en/exception.tostring.php). * **Go**: Must be the return value of [`runtime.Stack()`](https://golang.org/pkg/runtime/debug/#Stack)."]
        pub message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The service context in which this error has occurred."]
        pub service_context: ::std::option::Option<::std::boxed::Box<ServiceContext>>,
    }
    impl ReportedErrorEvent {
        pub fn builder() -> ReportedErrorEventBuilder {
            ReportedErrorEventBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a running service that sends errors. Its version changes over time and multiple versions can run in parallel."]
    pub struct ServiceContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the MonitoredResource. List of possible values: https://cloud.google.com/monitoring/api/resources Value is set automatically for incoming errors and must not be set when reporting errors."]
        pub resource_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "service")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An identifier of the service, such as the name of the executable, job, or Google App Engine service name. This field is expected to have a low number of values that are relatively stable over time, as opposed to `version`, which can be changed whenever new code is deployed. Contains the service name for error reports extracted from Google App Engine logs or `default` if the App Engine default service is used."]
        pub service: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents the source code version that the developer provided, which could represent a version label or a Git SHA-1 hash, for example. For App Engine standard environment, the version is set to the version of the app."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl ServiceContext {
        pub fn builder() -> ServiceContextBuilder {
            ServiceContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Indicates a location in the source code of the service for which errors are reported. `functionName` must be provided by the application when reporting an error, unless the error report contains a `message` with a supported exception stack trace. All fields are optional for the later case."]
    pub struct SourceLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source code filename, which can include a truncated relative path, or a full path from a production machine."]
        pub file_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "functionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human-readable name of a function or method. The value can include optional context like the class or package name. For example, `my.package.MyClass.method` in case of Java."]
        pub function_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "1-based. 0 indicates that the line number is unknown."]
        pub line_number: ::std::option::Option<::std::primitive::i64>,
    }
    impl SourceLocation {
        pub fn builder() -> SourceLocationBuilder {
            SourceLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A reference to a particular snapshot of the source tree used to build and deploy an application."]
    pub struct SourceReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "repository")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A URI string identifying the repository. Example: \"https://github.com/GoogleCloudPlatform/kubernetes.git\""]
        pub repository: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The canonical and persistent identifier of the deployed revision. Example (git): \"0035781c50ec7aa23385dc841529ce8a4b70db1b\""]
        pub revision_id: ::std::option::Option<::std::string::String>,
    }
    impl SourceReference {
        pub fn builder() -> SourceReferenceBuilder {
            SourceReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The number of errors in a given time period. All numbers are approximate since the error events are sampled before counting them."]
    pub struct TimedCount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Approximate number of occurrences in the given time period."]
        pub count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End of the time period to which `count` refers (excluded)."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start of the time period to which `count` refers (included)."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl TimedCount {
        pub fn builder() -> TimedCountBuilder {
            TimedCountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information related to tracking the progress on resolving the error."]
    pub struct TrackingIssue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL pointing to a related entry in an issue tracking system. Example: `https://github.com/user/project/issues/4`"]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl TrackingIssue {
        pub fn builder() -> TrackingIssueBuilder {
            TrackingIssueBuilder::default()
        }
    }
}
