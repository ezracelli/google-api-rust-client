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
    pub mod users {
        pub mod resources {
            pub mod data_sources {
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
                            #[serde(rename = "dataTypeName")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The names of data types to include in the list. If not specified, all data sources will be returned."]
                            pub data_type_name: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod data_point_changes {
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
                                    #[serde(rename = "limit")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "If specified, no more than this many data point changes will be included in the response."]
                                    pub limit: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of nextPageToken from the previous response."]
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
                                    #[serde(rename = "currentTimeMillis")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The client's current time in milliseconds since epoch."]
                                    pub current_time_millis:
                                        ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "modifiedTimeMillis")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "When the operation was performed on the client."]
                                    pub modified_time_millis:
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
                                    #[serde(rename = "limit")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "If specified, no more than this many data points will be included in the dataset. If there are more data points in the dataset, nextPageToken will be set in the dataset response. The limit is applied from the end of the time range. That is, if pageToken is absent, the limit most recent data points will be returned."]
                                    pub limit: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The continuation token, which is used to page through large datasets. To get the next page of a dataset, set this parameter to the value of nextPageToken from the previous response. Each subsequent call will yield a partial dataset with data point end timestamps that are strictly smaller than those in the previous partial response."]
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
                                    #[serde(rename = "currentTimeMillis")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The client's current time in milliseconds since epoch. Note that the minStartTimeNs and maxEndTimeNs properties in the request body are in nanoseconds instead of milliseconds."]
                                    pub current_time_millis:
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
            pub mod sessions {
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
                            #[serde(rename = "currentTimeMillis")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The client's current time in milliseconds since epoch."]
                            pub current_time_millis: ::std::option::Option<::std::string::String>,
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
                            #[serde(rename = "activityType")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If non-empty, only sessions with these activity types should be returned."]
                            pub activity_type: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "endTime")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An RFC3339 timestamp. Only sessions ending between the start and end times will be included in the response. If this time is omitted but startTime is specified, all sessions from startTime to the end of time will be returned."]
                            pub end_time: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "includeDeleted")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If true, and if both startTime and endTime are omitted, session deletions will be returned."]
                            pub include_deleted: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The continuation token, which is used for incremental syncing. To get the next batch of changes, set this parameter to the value of nextPageToken from the previous response. The page token is ignored if either start or end time is specified. If none of start time, end time, and the page token is specified, sessions modified in the last 30 days are returned."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startTime")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An RFC3339 timestamp. Only sessions ending between the start and end times will be included in the response. If this time is omitted but endTime is specified, all sessions from the start of time up to endTime will be returned."]
                            pub start_time: ::std::option::Option<::std::string::String>,
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
                            #[serde(rename = "currentTimeMillis")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The client's current time in milliseconds since epoch."]
                            pub current_time_millis: ::std::option::Option<::std::string::String>,
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
    pub struct AggregateBucket {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Available for Bucket.Type.ACTIVITY_TYPE, Bucket.Type.ACTIVITY_SEGMENT"]
        pub activity: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "There will be one dataset per AggregateBy in the request."]
        pub dataset: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dataset>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTimeMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end time for the aggregated data, in milliseconds since epoch, inclusive."]
        pub end_time_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "session")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Available for Bucket.Type.SESSION"]
        pub session: ::std::option::Option<::std::boxed::Box<Session>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTimeMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start time for the aggregated data, in milliseconds since epoch, inclusive."]
        pub start_time_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of a bucket signifies how the data aggregation is performed in the bucket."]
        pub _type: ::std::option::Option<AggregateBucketTypeEnum>,
    }
    impl AggregateBucket {
        pub fn builder() -> AggregateBucketBuilder {
            AggregateBucketBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of a bucket signifies how the data aggregation is performed in the bucket."]
    pub enum AggregateBucketTypeEnum {
        #[serde(rename = "unknown")]
        #[doc = ""]
        Unknown,
        #[serde(rename = "time")]
        #[doc = "Denotes that bucketing by time is requested. When this is specified, the timeBucketDurationMillis field is used to determine how many buckets will be returned."]
        Time,
        #[serde(rename = "session")]
        #[doc = "Denotes that bucketing by session is requested. When this is specified, only data that occurs within sessions that begin and end within the dataset time frame, is included in the results."]
        Session,
        #[serde(rename = "activityType")]
        #[doc = "Denotes that bucketing by activity type is requested. When this is specified, there will be one bucket for each unique activity type that a user participated in, during the dataset time frame of interest."]
        ActivityType,
        #[serde(rename = "activitySegment")]
        #[doc = "Denotes that bucketing by individual activity segment is requested. This will aggregate data by the time boundaries specified by each activity segment occurring within the dataset time frame of interest."]
        ActivitySegment,
    }
    impl ::std::default::Default for AggregateBucketTypeEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The specification of which data to aggregate."]
    pub struct AggregateBy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A data source ID to aggregate. Only data from the specified data source ID will be included in the aggregation. If specified, this data source must exist; the OAuth scopes in the supplied credentials must grant read access to this data type. The dataset in the response will have the same data source ID. Note: Data can be aggregated by either the dataTypeName or the dataSourceId, not both."]
        pub data_source_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataTypeName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data type to aggregate. All data sources providing this data type will contribute data to the aggregation. The response will contain a single dataset for this data type name. The dataset will have a data source ID of derived::com.google.android.gms:aggregated. If the user has no data for this data type, an empty data set will be returned. Note: Data can be aggregated by either the dataTypeName or the dataSourceId, not both."]
        pub data_type_name: ::std::option::Option<::std::string::String>,
    }
    impl AggregateBy {
        pub fn builder() -> AggregateByBuilder {
            AggregateByBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Next id: 10"]
    pub struct AggregateRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aggregateBy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The specification of data to be aggregated. At least one aggregateBy spec must be provided. All data that is specified will be aggregated using the same bucketing criteria. There will be one dataset in the response for every aggregateBy spec."]
        pub aggregate_by: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AggregateBy>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketByActivitySegment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies that data be aggregated each activity segment recorded for a user. Similar to bucketByActivitySegment, but bucketing is done for each activity segment rather than all segments of the same type. Mutually exclusive of other bucketing specifications."]
        pub bucket_by_activity_segment: ::std::option::Option<::std::boxed::Box<BucketByActivity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketByActivityType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies that data be aggregated by the type of activity being performed when the data was recorded. All data that was recorded during a certain activity type (.for the given time range) will be aggregated into the same bucket. Data that was recorded while the user was not active will not be included in the response. Mutually exclusive of other bucketing specifications."]
        pub bucket_by_activity_type: ::std::option::Option<::std::boxed::Box<BucketByActivity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketBySession")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies that data be aggregated by user sessions. Data that does not fall within the time range of a session will not be included in the response. Mutually exclusive of other bucketing specifications."]
        pub bucket_by_session: ::std::option::Option<::std::boxed::Box<BucketBySession>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketByTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies that data be aggregated by a single time interval. Mutually exclusive of other bucketing specifications."]
        pub bucket_by_time: ::std::option::Option<::std::boxed::Box<BucketByTime>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTimeMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end of a window of time. Data that intersects with this time window will be aggregated. The time is in milliseconds since epoch, inclusive."]
        pub end_time_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filteredDataQualityStandard")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DO NOT POPULATE THIS FIELD. It is ignored."]
        pub filtered_data_quality_standard:
            ::std::option::Option<::std::vec::Vec<AggregateRequestFilteredDataQualityStandardEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTimeMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start of a window of time. Data that intersects with this time window will be aggregated. The time is in milliseconds since epoch, inclusive."]
        pub start_time_millis: ::std::option::Option<::std::string::String>,
    }
    impl AggregateRequest {
        pub fn builder() -> AggregateRequestBuilder {
            AggregateRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum AggregateRequestFilteredDataQualityStandardEnum {
        #[serde(rename = "dataQualityUnknown")]
        #[doc = ""]
        DataQualityUnknown,
        #[serde(rename = "dataQualityBloodPressureEsh2002")]
        #[doc = ""]
        DataQualityBloodPressureEsh2002,
        #[serde(rename = "dataQualityBloodPressureEsh2010")]
        #[doc = ""]
        DataQualityBloodPressureEsh2010,
        #[serde(rename = "dataQualityBloodPressureAami")]
        #[doc = ""]
        DataQualityBloodPressureAami,
        #[serde(rename = "dataQualityBloodPressureBhsAA")]
        #[doc = ""]
        DataQualityBloodPressureBhsAa,
        #[serde(rename = "dataQualityBloodPressureBhsAB")]
        #[doc = ""]
        DataQualityBloodPressureBhsAb,
        #[serde(rename = "dataQualityBloodPressureBhsBA")]
        #[doc = ""]
        DataQualityBloodPressureBhsBa,
        #[serde(rename = "dataQualityBloodPressureBhsBB")]
        #[doc = ""]
        DataQualityBloodPressureBhsBb,
        #[serde(rename = "dataQualityBloodGlucoseIso151972003")]
        #[doc = ""]
        DataQualityBloodGlucoseIso151972003,
        #[serde(rename = "dataQualityBloodGlucoseIso151972013")]
        #[doc = ""]
        DataQualityBloodGlucoseIso151972013,
    }
    impl ::std::default::Default for AggregateRequestFilteredDataQualityStandardEnum {
        fn default() -> Self {
            Self::DataQualityUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AggregateResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucket")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of buckets containing the aggregated data."]
        pub bucket: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AggregateBucket>>>,
    }
    impl AggregateResponse {
        pub fn builder() -> AggregateResponseBuilder {
            AggregateResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Application {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detailsUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional URI that can be used to link back to the application."]
        pub details_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of this application. This is required for REST clients, but we do not enforce uniqueness of this name. It is provided as a matter of convenience for other developers who would like to identify which REST created an Application or Data Source."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Package name for this application. This is used as a unique identifier when created by Android applications, but cannot be specified by REST clients. REST clients will have their developer project number reflected into the Data Source data stream IDs, instead of the packageName."]
        pub package_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Version of the application. You should update this field whenever the application changes in a way that affects the computation of the data."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl Application {
        pub fn builder() -> ApplicationBuilder {
            ApplicationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BucketByActivity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activityDataSourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The default activity stream will be used if a specific activityDataSourceId is not specified."]
        pub activity_data_source_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minDurationMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies that only activity segments of duration longer than minDurationMillis are considered and used as a container for aggregated data."]
        pub min_duration_millis: ::std::option::Option<::std::string::String>,
    }
    impl BucketByActivity {
        pub fn builder() -> BucketByActivityBuilder {
            BucketByActivityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BucketBySession {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minDurationMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies that only sessions of duration longer than minDurationMillis are considered and used as a container for aggregated data."]
        pub min_duration_millis: ::std::option::Option<::std::string::String>,
    }
    impl BucketBySession {
        pub fn builder() -> BucketBySessionBuilder {
            BucketBySessionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BucketByTime {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "durationMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies that result buckets aggregate data by exactly durationMillis time frames. Time frames that contain no data will be included in the response with an empty dataset."]
        pub duration_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "period")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub period: ::std::option::Option<::std::boxed::Box<BucketByTimePeriod>>,
    }
    impl BucketByTime {
        pub fn builder() -> BucketByTimeBuilder {
            BucketByTimeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BucketByTimePeriod {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeZoneId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "org.joda.timezone.DateTimeZone"]
        pub time_zone_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub _type: ::std::option::Option<BucketByTimePeriodTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<::std::primitive::i64>,
    }
    impl BucketByTimePeriod {
        pub fn builder() -> BucketByTimePeriodBuilder {
            BucketByTimePeriodBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum BucketByTimePeriodTypeEnum {
        #[serde(rename = "day")]
        #[doc = ""]
        Day,
        #[serde(rename = "week")]
        #[doc = ""]
        Week,
        #[serde(rename = "month")]
        #[doc = ""]
        Month,
    }
    impl ::std::default::Default for BucketByTimePeriodTypeEnum {
        fn default() -> Self {
            Self::Day
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a single data point, generated by a particular data source. A data point holds a value for each field, an end timestamp and an optional start time. The exact semantics of each of these attributes are specified in the documentation for the particular data type. A data point can represent an instantaneous measurement, reading or input observation, as well as averages or aggregates over a time interval. Check the data type documentation to determine which is the case for a particular data type. Data points always contain one value for each field of the data type."]
    pub struct DataPoint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "computationTimeMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DO NOT USE THIS FIELD. It is ignored, and not stored."]
        pub computation_time_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataTypeName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data type defining the format of the values in this data point."]
        pub data_type_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTimeNanos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end time of the interval represented by this data point, in nanoseconds since epoch."]
        pub end_time_nanos: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modifiedTimeMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates the last time this data point was modified. Useful only in contexts where we are listing the data changes, rather than representing the current state of the data."]
        pub modified_time_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originDataSourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the data point is contained in a dataset for a derived data source, this field will be populated with the data source stream ID that created the data point originally. WARNING: do not rely on this field for anything other than debugging. The value of this field, if it is set at all, is an implementation detail and is not guaranteed to remain consistent."]
        pub origin_data_source_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rawTimestampNanos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The raw timestamp from the original SensorEvent."]
        pub raw_timestamp_nanos: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTimeNanos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start time of the interval represented by this data point, in nanoseconds since epoch."]
        pub start_time_nanos: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Values of each data type field for the data point. It is expected that each value corresponding to a data type field will occur in the same order that the field is listed with in the data type specified in a data source. Only one of integer and floating point fields will be populated, depending on the format enum value within data source's type field."]
        pub value: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Value>>>,
    }
    impl DataPoint {
        pub fn builder() -> DataPointBuilder {
            DataPointBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Definition of a unique source of sensor data. Data sources can expose raw data coming from hardware sensors on local or companion devices. They can also expose derived data, created by transforming or merging other data sources. Multiple data sources can exist for the same data type. Every data point inserted into or read from this service has an associated data source. The data source contains enough information to uniquely identify its data, including the hardware device and the application that collected and/or transformed the data. It also holds useful metadata, such as the hardware and application versions, and the device type. Each data source produces a unique stream of data, with a unique identifier. Not all changes to data source affect the stream identifier, so that data collected by updated versions of the same application/device can still be considered to belong to the same data stream."]
    pub struct DataSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "application")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about an application which feeds sensor data into the platform."]
        pub application: ::std::option::Option<::std::boxed::Box<Application>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataQualityStandard")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DO NOT POPULATE THIS FIELD. It is never populated in responses from the platform, and is ignored in queries. It will be removed in a future version entirely."]
        pub data_quality_standard:
            ::std::option::Option<::std::vec::Vec<DataSourceDataQualityStandardEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataStreamId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique identifier for the data stream produced by this data source. The identifier includes: - The physical device's manufacturer, model, and serial number (UID). - The application's package name or name. Package name is used when the data source was created by an Android application. The developer project number is used when the data source was created by a REST client. - The data source's type. - The data source's stream name. Note that not all attributes of the data source are used as part of the stream identifier. In particular, the version of the hardware/the application isn't used. This allows us to preserve the same stream through version updates. This also means that two DataSource objects may represent the same data stream even if they're not equal. The exact format of the data stream ID created by an Android application is: type:dataType.name:application.packageName:device.manufacturer:device.model:device.uid:dataStreamName The exact format of the data stream ID created by a REST client is: type:dataType.name:developer project number:device.manufacturer:device.model:device.uid:dataStreamName When any of the optional fields that make up the data stream ID are absent, they will be omitted from the data stream ID. The minimum viable data stream ID would be: type:dataType.name:developer project number Finally, the developer project number and device UID are obfuscated when read by any REST or Android client that did not create the data source. Only the data source creator will see the developer project number in clear and normal form. This means a client will see a different set of data_stream_ids than another client with different credentials."]
        pub data_stream_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataStreamName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stream name uniquely identifies this particular data source among other data sources of the same type from the same underlying producer. Setting the stream name is optional, but should be done whenever an application exposes two streams for the same data type, or when a device has two equivalent sensors."]
        pub data_stream_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data type defines the schema for a stream of data being collected by, inserted into, or queried from the Fitness API."]
        pub data_type: ::std::option::Option<::std::boxed::Box<DataType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "device")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Representation of an integrated device (such as a phone or a wearable) that can hold sensors."]
        pub device: ::std::option::Option<::std::boxed::Box<Device>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An end-user visible name for this data source."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A constant describing the type of this data source. Indicates whether this data source produces raw or derived data."]
        pub _type: ::std::option::Option<DataSourceTypeEnum>,
    }
    impl DataSource {
        pub fn builder() -> DataSourceBuilder {
            DataSourceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum DataSourceDataQualityStandardEnum {
        #[serde(rename = "dataQualityUnknown")]
        #[doc = ""]
        DataQualityUnknown,
        #[serde(rename = "dataQualityBloodPressureEsh2002")]
        #[doc = ""]
        DataQualityBloodPressureEsh2002,
        #[serde(rename = "dataQualityBloodPressureEsh2010")]
        #[doc = ""]
        DataQualityBloodPressureEsh2010,
        #[serde(rename = "dataQualityBloodPressureAami")]
        #[doc = ""]
        DataQualityBloodPressureAami,
        #[serde(rename = "dataQualityBloodPressureBhsAA")]
        #[doc = ""]
        DataQualityBloodPressureBhsAa,
        #[serde(rename = "dataQualityBloodPressureBhsAB")]
        #[doc = ""]
        DataQualityBloodPressureBhsAb,
        #[serde(rename = "dataQualityBloodPressureBhsBA")]
        #[doc = ""]
        DataQualityBloodPressureBhsBa,
        #[serde(rename = "dataQualityBloodPressureBhsBB")]
        #[doc = ""]
        DataQualityBloodPressureBhsBb,
        #[serde(rename = "dataQualityBloodGlucoseIso151972003")]
        #[doc = ""]
        DataQualityBloodGlucoseIso151972003,
        #[serde(rename = "dataQualityBloodGlucoseIso151972013")]
        #[doc = ""]
        DataQualityBloodGlucoseIso151972013,
    }
    impl ::std::default::Default for DataSourceDataQualityStandardEnum {
        fn default() -> Self {
            Self::DataQualityUnknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "A constant describing the type of this data source. Indicates whether this data source produces raw or derived data."]
    pub enum DataSourceTypeEnum {
        #[serde(rename = "raw")]
        #[doc = ""]
        Raw,
        #[serde(rename = "derived")]
        #[doc = ""]
        Derived,
    }
    impl ::std::default::Default for DataSourceTypeEnum {
        fn default() -> Self {
            Self::Raw
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DataType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A field represents one dimension of a data type."]
        pub field: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataTypeField>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Each data type has a unique, namespaced, name. All data types in the com.google namespace are shared as part of the platform."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl DataType {
        pub fn builder() -> DataTypeBuilder {
            DataTypeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "In case of multi-dimensional data (such as an accelerometer with x, y, and z axes) each field represents one dimension. Each data type field has a unique name which identifies it. The field also defines the format of the data (int, float, etc.). This message is only instantiated in code and not used for wire comms or stored in any way."]
    pub struct DataTypeField {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "format")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The different supported formats for each field in a data type."]
        pub format: ::std::option::Option<DataTypeFieldFormatEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the name and format of data. Unlike data type names, field names are not namespaced, and only need to be unique within the data type."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "optional")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub optional: ::std::option::Option<::std::primitive::bool>,
    }
    impl DataTypeField {
        pub fn builder() -> DataTypeFieldBuilder {
            DataTypeFieldBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The different supported formats for each field in a data type."]
    pub enum DataTypeFieldFormatEnum {
        #[serde(rename = "integer")]
        #[doc = ""]
        Integer,
        #[serde(rename = "floatPoint")]
        #[doc = ""]
        FloatPoint,
        #[serde(rename = "string")]
        #[doc = ""]
        String,
        #[serde(rename = "map")]
        #[doc = ""]
        Map,
        #[serde(rename = "integerList")]
        #[doc = ""]
        IntegerList,
        #[serde(rename = "floatList")]
        #[doc = ""]
        FloatList,
        #[serde(rename = "blob")]
        #[doc = ""]
        Blob,
    }
    impl ::std::default::Default for DataTypeFieldFormatEnum {
        fn default() -> Self {
            Self::Integer
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A dataset represents a projection container for data points. They do not carry any info of their own. Datasets represent a set of data points from a particular data source. A data point can be found in more than one dataset."]
    pub struct Dataset {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data stream ID of the data source that created the points in this dataset."]
        pub data_source_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxEndTimeNs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The largest end time of all data points in this possibly partial representation of the dataset. Time is in nanoseconds from epoch. This should also match the second part of the dataset identifier."]
        pub max_end_time_ns: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minStartTimeNs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The smallest start time of all data points in this possibly partial representation of the dataset. Time is in nanoseconds from epoch. This should also match the first part of the dataset identifier."]
        pub min_start_time_ns: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This token will be set when a dataset is received in response to a GET request and the dataset is too large to be included in a single response. Provide this value in a subsequent GET request to return the next page of data points within this dataset."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "point")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A partial list of data points contained in the dataset, ordered by endTimeNanos. This list is considered complete when retrieving a small dataset and partial when patching a dataset or retrieving a dataset that is too large to include in a single response."]
        pub point: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataPoint>>>,
    }
    impl Dataset {
        pub fn builder() -> DatasetBuilder {
            DatasetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Representation of an integrated device (such as a phone or a wearable) that can hold sensors. Each sensor is exposed as a data source. The main purpose of the device information contained in this class is to identify the hardware of a particular data source. This can be useful in different ways, including: - Distinguishing two similar sensors on different devices (the step counter on two nexus 5 phones, for instance) - Display the source of data to the user (by using the device make / model) - Treat data differently depending on sensor type (accelerometers on a watch may give different patterns than those on a phone) - Build different analysis models for each device/version. "]
    pub struct Device {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manufacturer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Manufacturer of the product/hardware."]
        pub manufacturer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End-user visible model name for the device."]
        pub model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A constant representing the type of the device."]
        pub _type: ::std::option::Option<DeviceTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The serial number or other unique ID for the hardware. This field is obfuscated when read by any REST or Android client that did not create the data source. Only the data source creator will see the uid field in clear and normal form. The obfuscation preserves equality; that is, given two IDs, if id1 == id2, obfuscated(id1) == obfuscated(id2)."]
        pub uid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Version string for the device hardware/software."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl Device {
        pub fn builder() -> DeviceBuilder {
            DeviceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "A constant representing the type of the device."]
    pub enum DeviceTypeEnum {
        #[serde(rename = "unknown")]
        #[doc = "Device type is not known."]
        Unknown,
        #[serde(rename = "phone")]
        #[doc = "An Android phone."]
        Phone,
        #[serde(rename = "tablet")]
        #[doc = "An Android tablet."]
        Tablet,
        #[serde(rename = "watch")]
        #[doc = "A watch or other wrist-mounted band."]
        Watch,
        #[serde(rename = "chestStrap")]
        #[doc = "A chest strap."]
        ChestStrap,
        #[serde(rename = "scale")]
        #[doc = "A scale."]
        Scale,
        #[serde(rename = "headMounted")]
        #[doc = "Glass or other head-mounted device."]
        HeadMounted,
        #[serde(rename = "smartDisplay")]
        #[doc = "A smart display e.g. Nest device."]
        SmartDisplay,
    }
    impl ::std::default::Default for DeviceTypeEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListDataPointChangesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data stream ID of the data source with data point changes."]
        pub data_source_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deletedDataPoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deleted data points for the user. Note, for modifications this should be parsed before handling insertions."]
        pub deleted_data_point:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataPoint>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertedDataPoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Inserted data points for the user."]
        pub inserted_data_point:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataPoint>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListDataPointChangesResponse {
        pub fn builder() -> ListDataPointChangesResponseBuilder {
            ListDataPointChangesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListDataSourcesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A previously created data source."]
        pub data_source: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataSource>>>,
    }
    impl ListDataSourcesResponse {
        pub fn builder() -> ListDataSourcesResponseBuilder {
            ListDataSourcesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListSessionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deletedSession")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If includeDeleted is set to true in the request, and startTime and endTime are omitted, this will include sessions which were deleted since the last sync."]
        pub deleted_session: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Session>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasMoreData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag to indicate server has more data to transfer. DO NOT USE THIS FIELD. It is never populated in responses from the server."]
        pub has_more_data: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sync token which is used to sync further changes. This will only be provided if both startTime and endTime are omitted from the request."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "session")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sessions with an end time that is between startTime and endTime of the request."]
        pub session: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Session>>>,
    }
    impl ListSessionsResponse {
        pub fn builder() -> ListSessionsResponseBuilder {
            ListSessionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Holder object for the value of an entry in a map field of a data point. A map value supports a subset of the formats that the regular Value supports."]
    pub struct MapValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fpVal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Floating point value."]
        pub fp_val: ::std::option::Option<::std::primitive::f64>,
    }
    impl MapValue {
        pub fn builder() -> MapValueBuilder {
            MapValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Sessions contain metadata, such as a user-friendly name and time interval information."]
    pub struct Session {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activeTimeMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Session active time. While start_time_millis and end_time_millis define the full session time, the active time can be shorter and specified by active_time_millis. If the inactive time during the session is known, it should also be inserted via a com.google.activity.segment data point with a STILL activity value"]
        pub active_time_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activityType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of activity this session represents."]
        pub activity_type: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "application")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The application that created the session."]
        pub application: ::std::option::Option<::std::boxed::Box<Application>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description for this session."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTimeMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An end time, in milliseconds since epoch, inclusive."]
        pub end_time_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A client-generated identifier that is unique across all sessions owned by this particular user."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modifiedTimeMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A timestamp that indicates when the session was last modified."]
        pub modified_time_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A human readable name of the session."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTimeMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A start time, in milliseconds since epoch, inclusive."]
        pub start_time_millis: ::std::option::Option<::std::string::String>,
    }
    impl Session {
        pub fn builder() -> SessionBuilder {
            SessionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Holder object for the value of a single field in a data point. A field value has a particular format and is only ever set to one of an integer or a floating point value."]
    pub struct Value {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fpVal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Floating point value. When this is set, other values must not be set."]
        pub fp_val: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intVal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Integer value. When this is set, other values must not be set."]
        pub int_val: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mapVal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map value. The valid key space and units for the corresponding value of each entry should be documented as part of the data type definition. Keys should be kept small whenever possible. Data streams with large keys and high data frequency may be down sampled."]
        pub map_val: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ValueMapValEntry>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringVal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "String value. When this is set, other values must not be set. Strings should be kept small whenever possible. Data streams with large string values and high data frequency may be down sampled."]
        pub string_val: ::std::option::Option<::std::string::String>,
    }
    impl Value {
        pub fn builder() -> ValueBuilder {
            ValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ValueMapValEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<::std::boxed::Box<MapValue>>,
    }
    impl ValueMapValEntry {
        pub fn builder() -> ValueMapValEntryBuilder {
            ValueMapValEntryBuilder::default()
        }
    }
}
