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
    pub mod folders {
        pub mod resources {
            pub mod time_series {
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
                            #[serde(rename = "aggregation.alignmentPeriod")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The alignment_period specifies a time interval, in seconds, that is used to divide the data in all the time series into consistent blocks of time. This will be done before the per-series aligner can be applied to the data.The value must be at least 60 seconds. If a per-series aligner other than ALIGN_NONE is specified, this field is required or an error is returned. If no per-series aligner is specified, or the aligner ALIGN_NONE is specified, then this field is ignored.The maximum value of the alignment_period is 104 weeks (2 years) for charts, and 90,000 seconds (25 hours) for alerting policies."]
                            pub aggregation_alignment_period:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "aggregation.crossSeriesReducer")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned."]
                            pub aggregation_cross_series_reducer: ::std::option::Option<
                                QueryParametersAggregationCrossSeriesReducerEnum,
                            >,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "aggregation.groupByFields")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The set of fields to preserve when cross_series_reducer is specified. The group_by_fields determine how the time series are partitioned into subsets prior to applying the aggregation operation. Each subset contains time series that have the same value for each of the grouping fields. Each individual time series is a member of exactly one subset. The cross_series_reducer is applied to each subset of time series. It is not possible to reduce across different resource types, so this field implicitly contains resource.type. Fields not specified in group_by_fields are aggregated away. If group_by_fields is not specified and all the time series have the same resource type, then the time series are aggregated into a single output time series. If cross_series_reducer is not defined, this field is ignored."]
                            pub aggregation_group_by_fields:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "aggregation.perSeriesAligner")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned."]
                            pub aggregation_per_series_aligner: ::std::option::Option<
                                QueryParametersAggregationPerSeriesAlignerEnum,
                            >,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. A monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) that specifies which time series should be returned. The filter must specify a single metric type, and can additionally specify metric labels and other information. For example: metric.type = \"compute.googleapis.com/instance/cpu/usage_time\" AND metric.labels.instance_name = \"my-instance-name\" "]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "interval.endTime")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. The end of the time interval."]
                            pub interval_end_time: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "interval.startTime")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The beginning of the time interval. The default value for the start time is the end time. The start time must not be later than the end time."]
                            pub interval_start_time: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Unsupported: must be left blank. The points in each time series are currently returned in reverse time order (most recent to oldest)."]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A positive number that is the maximum number of results to return. If page_size is empty or more than 100,000 results, the effective page_size is 100,000 results. If view is set to FULL, this is the maximum number of Points returned. If view is set to HEADERS, this is the maximum number of TimeSeries returned."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If this field is not empty then it must contain the nextPageToken value returned by a previous call to this method. Using this field causes the method to return additional results from the previous method call."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "secondaryAggregation.alignmentPeriod")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The alignment_period specifies a time interval, in seconds, that is used to divide the data in all the time series into consistent blocks of time. This will be done before the per-series aligner can be applied to the data.The value must be at least 60 seconds. If a per-series aligner other than ALIGN_NONE is specified, this field is required or an error is returned. If no per-series aligner is specified, or the aligner ALIGN_NONE is specified, then this field is ignored.The maximum value of the alignment_period is 104 weeks (2 years) for charts, and 90,000 seconds (25 hours) for alerting policies."]
                            pub secondary_aggregation_alignment_period:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "secondaryAggregation.crossSeriesReducer")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned."]
                            pub secondary_aggregation_cross_series_reducer: ::std::option::Option<
                                QueryParametersSecondaryAggregationCrossSeriesReducerEnum,
                            >,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "secondaryAggregation.groupByFields")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The set of fields to preserve when cross_series_reducer is specified. The group_by_fields determine how the time series are partitioned into subsets prior to applying the aggregation operation. Each subset contains time series that have the same value for each of the grouping fields. Each individual time series is a member of exactly one subset. The cross_series_reducer is applied to each subset of time series. It is not possible to reduce across different resource types, so this field implicitly contains resource.type. Fields not specified in group_by_fields are aggregated away. If group_by_fields is not specified and all the time series have the same resource type, then the time series are aggregated into a single output time series. If cross_series_reducer is not defined, this field is ignored."]
                            pub secondary_aggregation_group_by_fields:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "secondaryAggregation.perSeriesAligner")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned."]
                            pub secondary_aggregation_per_series_aligner: ::std::option::Option<
                                QueryParametersSecondaryAggregationPerSeriesAlignerEnum,
                            >,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. Specifies which information is returned about the time series."]
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
                        #[doc = "The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned."]
                        pub enum QueryParametersAggregationCrossSeriesReducerEnum {
                            #[serde(rename = "REDUCE_NONE")]
                            #[doc = "No cross-time series reduction. The output of the Aligner is returned."]
                            ReduceNone,
                            #[serde(rename = "REDUCE_MEAN")]
                            #[doc = "Reduce by computing the mean value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE."]
                            ReduceMean,
                            #[serde(rename = "REDUCE_MIN")]
                            #[doc = "Reduce by computing the minimum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input."]
                            ReduceMin,
                            #[serde(rename = "REDUCE_MAX")]
                            #[doc = "Reduce by computing the maximum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input."]
                            ReduceMax,
                            #[serde(rename = "REDUCE_SUM")]
                            #[doc = "Reduce by computing the sum across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric and distribution values. The value_type of the output is the same as the value_type of the input."]
                            ReduceSum,
                            #[serde(rename = "REDUCE_STDDEV")]
                            #[doc = "Reduce by computing the standard deviation across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE."]
                            ReduceStddev,
                            #[serde(rename = "REDUCE_COUNT")]
                            #[doc = "Reduce by computing the number of data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of numeric, Boolean, distribution, and string value_type. The value_type of the output is INT64."]
                            ReduceCount,
                            #[serde(rename = "REDUCE_COUNT_TRUE")]
                            #[doc = "Reduce by computing the number of True-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64."]
                            ReduceCountTrue,
                            #[serde(rename = "REDUCE_COUNT_FALSE")]
                            #[doc = "Reduce by computing the number of False-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64."]
                            ReduceCountFalse,
                            #[serde(rename = "REDUCE_FRACTION_TRUE")]
                            #[doc = "Reduce by computing the ratio of the number of True-valued data points to the total number of data points for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The output value is in the range 0.0, 1.0 and has value_type DOUBLE."]
                            ReduceFractionTrue,
                            #[serde(rename = "REDUCE_PERCENTILE_99")]
                            #[doc = "Reduce by computing the 99th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile99,
                            #[serde(rename = "REDUCE_PERCENTILE_95")]
                            #[doc = "Reduce by computing the 95th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile95,
                            #[serde(rename = "REDUCE_PERCENTILE_50")]
                            #[doc = "Reduce by computing the 50th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile50,
                            #[serde(rename = "REDUCE_PERCENTILE_05")]
                            #[doc = "Reduce by computing the 5th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile05,
                        }
                        impl ::std::default::Default for QueryParametersAggregationCrossSeriesReducerEnum {
                            fn default() -> Self {
                                Self::ReduceNone
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned."]
                        pub enum QueryParametersAggregationPerSeriesAlignerEnum {
                            #[serde(rename = "ALIGN_NONE")]
                            #[doc = "No alignment. Raw data is returned. Not valid if cross-series reduction is requested. The value_type of the result is the same as the value_type of the input."]
                            AlignNone,
                            #[serde(rename = "ALIGN_DELTA")]
                            #[doc = "Align and convert to DELTA. The output is delta = y1 - y0.This alignment is valid for CUMULATIVE and DELTA metrics. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignDelta,
                            #[serde(rename = "ALIGN_RATE")]
                            #[doc = "Align and convert to a rate. The result is computed as rate = (y1 - y0)/(t1 - t0), or \"delta over time\". Think of this aligner as providing the slope of the line that passes through the value at the start and at the end of the alignment_period.This aligner is valid for CUMULATIVE and DELTA metrics with numeric values. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The output is a GAUGE metric with value_type DOUBLE.If, by \"rate\", you mean \"percentage change\", see the ALIGN_PERCENT_CHANGE aligner instead."]
                            AlignRate,
                            #[serde(rename = "ALIGN_INTERPOLATE")]
                            #[doc = "Align by interpolating between adjacent points around the alignment period boundary. This aligner is valid for GAUGE metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignInterpolate,
                            #[serde(rename = "ALIGN_NEXT_OLDER")]
                            #[doc = "Align by moving the most recent data point before the end of the alignment period to the boundary at the end of the alignment period. This aligner is valid for GAUGE metrics. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignNextOlder,
                            #[serde(rename = "ALIGN_MIN")]
                            #[doc = "Align the time series by returning the minimum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignMin,
                            #[serde(rename = "ALIGN_MAX")]
                            #[doc = "Align the time series by returning the maximum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignMax,
                            #[serde(rename = "ALIGN_MEAN")]
                            #[doc = "Align the time series by returning the mean value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is DOUBLE."]
                            AlignMean,
                            #[serde(rename = "ALIGN_COUNT")]
                            #[doc = "Align the time series by returning the number of values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric or Boolean values. The value_type of the aligned result is INT64."]
                            AlignCount,
                            #[serde(rename = "ALIGN_SUM")]
                            #[doc = "Align the time series by returning the sum of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric and distribution values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignSum,
                            #[serde(rename = "ALIGN_STDDEV")]
                            #[doc = "Align the time series by returning the standard deviation of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the output is DOUBLE."]
                            AlignStddev,
                            #[serde(rename = "ALIGN_COUNT_TRUE")]
                            #[doc = "Align the time series by returning the number of True values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64."]
                            AlignCountTrue,
                            #[serde(rename = "ALIGN_COUNT_FALSE")]
                            #[doc = "Align the time series by returning the number of False values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64."]
                            AlignCountFalse,
                            #[serde(rename = "ALIGN_FRACTION_TRUE")]
                            #[doc = "Align the time series by returning the ratio of the number of True values to the total number of values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The output value is in the range 0.0, 1.0 and has value_type DOUBLE."]
                            AlignFractionTrue,
                            #[serde(rename = "ALIGN_PERCENTILE_99")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 99th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile99,
                            #[serde(rename = "ALIGN_PERCENTILE_95")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 95th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile95,
                            #[serde(rename = "ALIGN_PERCENTILE_50")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 50th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile50,
                            #[serde(rename = "ALIGN_PERCENTILE_05")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 5th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile05,
                            #[serde(rename = "ALIGN_PERCENT_CHANGE")]
                            #[doc = "Align and convert to a percentage change. This aligner is valid for GAUGE and DELTA metrics with numeric values. This alignment returns ((current - previous)/previous) * 100, where the value of previous is determined based on the alignment_period.If the values of current and previous are both 0, then the returned value is 0. If only previous is 0, the returned value is infinity.A 10-minute moving mean is computed at each point of the alignment period prior to the above calculation to smooth the metric and prevent false positives from very short-lived spikes. The moving mean is only applicable for data whose values are >= 0. Any values < 0 are treated as a missing datapoint, and are ignored. While DELTA metrics are accepted by this alignment, special care should be taken that the values for the metric will always be positive. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentChange,
                        }
                        impl ::std::default::Default for QueryParametersAggregationPerSeriesAlignerEnum {
                            fn default() -> Self {
                                Self::AlignNone
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned."]
                        pub enum QueryParametersSecondaryAggregationCrossSeriesReducerEnum {
                            #[serde(rename = "REDUCE_NONE")]
                            #[doc = "No cross-time series reduction. The output of the Aligner is returned."]
                            ReduceNone,
                            #[serde(rename = "REDUCE_MEAN")]
                            #[doc = "Reduce by computing the mean value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE."]
                            ReduceMean,
                            #[serde(rename = "REDUCE_MIN")]
                            #[doc = "Reduce by computing the minimum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input."]
                            ReduceMin,
                            #[serde(rename = "REDUCE_MAX")]
                            #[doc = "Reduce by computing the maximum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input."]
                            ReduceMax,
                            #[serde(rename = "REDUCE_SUM")]
                            #[doc = "Reduce by computing the sum across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric and distribution values. The value_type of the output is the same as the value_type of the input."]
                            ReduceSum,
                            #[serde(rename = "REDUCE_STDDEV")]
                            #[doc = "Reduce by computing the standard deviation across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE."]
                            ReduceStddev,
                            #[serde(rename = "REDUCE_COUNT")]
                            #[doc = "Reduce by computing the number of data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of numeric, Boolean, distribution, and string value_type. The value_type of the output is INT64."]
                            ReduceCount,
                            #[serde(rename = "REDUCE_COUNT_TRUE")]
                            #[doc = "Reduce by computing the number of True-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64."]
                            ReduceCountTrue,
                            #[serde(rename = "REDUCE_COUNT_FALSE")]
                            #[doc = "Reduce by computing the number of False-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64."]
                            ReduceCountFalse,
                            #[serde(rename = "REDUCE_FRACTION_TRUE")]
                            #[doc = "Reduce by computing the ratio of the number of True-valued data points to the total number of data points for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The output value is in the range 0.0, 1.0 and has value_type DOUBLE."]
                            ReduceFractionTrue,
                            #[serde(rename = "REDUCE_PERCENTILE_99")]
                            #[doc = "Reduce by computing the 99th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile99,
                            #[serde(rename = "REDUCE_PERCENTILE_95")]
                            #[doc = "Reduce by computing the 95th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile95,
                            #[serde(rename = "REDUCE_PERCENTILE_50")]
                            #[doc = "Reduce by computing the 50th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile50,
                            #[serde(rename = "REDUCE_PERCENTILE_05")]
                            #[doc = "Reduce by computing the 5th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile05,
                        }
                        impl ::std::default::Default for QueryParametersSecondaryAggregationCrossSeriesReducerEnum {
                            fn default() -> Self {
                                Self::ReduceNone
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned."]
                        pub enum QueryParametersSecondaryAggregationPerSeriesAlignerEnum {
                            #[serde(rename = "ALIGN_NONE")]
                            #[doc = "No alignment. Raw data is returned. Not valid if cross-series reduction is requested. The value_type of the result is the same as the value_type of the input."]
                            AlignNone,
                            #[serde(rename = "ALIGN_DELTA")]
                            #[doc = "Align and convert to DELTA. The output is delta = y1 - y0.This alignment is valid for CUMULATIVE and DELTA metrics. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignDelta,
                            #[serde(rename = "ALIGN_RATE")]
                            #[doc = "Align and convert to a rate. The result is computed as rate = (y1 - y0)/(t1 - t0), or \"delta over time\". Think of this aligner as providing the slope of the line that passes through the value at the start and at the end of the alignment_period.This aligner is valid for CUMULATIVE and DELTA metrics with numeric values. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The output is a GAUGE metric with value_type DOUBLE.If, by \"rate\", you mean \"percentage change\", see the ALIGN_PERCENT_CHANGE aligner instead."]
                            AlignRate,
                            #[serde(rename = "ALIGN_INTERPOLATE")]
                            #[doc = "Align by interpolating between adjacent points around the alignment period boundary. This aligner is valid for GAUGE metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignInterpolate,
                            #[serde(rename = "ALIGN_NEXT_OLDER")]
                            #[doc = "Align by moving the most recent data point before the end of the alignment period to the boundary at the end of the alignment period. This aligner is valid for GAUGE metrics. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignNextOlder,
                            #[serde(rename = "ALIGN_MIN")]
                            #[doc = "Align the time series by returning the minimum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignMin,
                            #[serde(rename = "ALIGN_MAX")]
                            #[doc = "Align the time series by returning the maximum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignMax,
                            #[serde(rename = "ALIGN_MEAN")]
                            #[doc = "Align the time series by returning the mean value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is DOUBLE."]
                            AlignMean,
                            #[serde(rename = "ALIGN_COUNT")]
                            #[doc = "Align the time series by returning the number of values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric or Boolean values. The value_type of the aligned result is INT64."]
                            AlignCount,
                            #[serde(rename = "ALIGN_SUM")]
                            #[doc = "Align the time series by returning the sum of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric and distribution values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignSum,
                            #[serde(rename = "ALIGN_STDDEV")]
                            #[doc = "Align the time series by returning the standard deviation of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the output is DOUBLE."]
                            AlignStddev,
                            #[serde(rename = "ALIGN_COUNT_TRUE")]
                            #[doc = "Align the time series by returning the number of True values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64."]
                            AlignCountTrue,
                            #[serde(rename = "ALIGN_COUNT_FALSE")]
                            #[doc = "Align the time series by returning the number of False values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64."]
                            AlignCountFalse,
                            #[serde(rename = "ALIGN_FRACTION_TRUE")]
                            #[doc = "Align the time series by returning the ratio of the number of True values to the total number of values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The output value is in the range 0.0, 1.0 and has value_type DOUBLE."]
                            AlignFractionTrue,
                            #[serde(rename = "ALIGN_PERCENTILE_99")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 99th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile99,
                            #[serde(rename = "ALIGN_PERCENTILE_95")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 95th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile95,
                            #[serde(rename = "ALIGN_PERCENTILE_50")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 50th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile50,
                            #[serde(rename = "ALIGN_PERCENTILE_05")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 5th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile05,
                            #[serde(rename = "ALIGN_PERCENT_CHANGE")]
                            #[doc = "Align and convert to a percentage change. This aligner is valid for GAUGE and DELTA metrics with numeric values. This alignment returns ((current - previous)/previous) * 100, where the value of previous is determined based on the alignment_period.If the values of current and previous are both 0, then the returned value is 0. If only previous is 0, the returned value is infinity.A 10-minute moving mean is computed at each point of the alignment period prior to the above calculation to smooth the metric and prevent false positives from very short-lived spikes. The moving mean is only applicable for data whose values are >= 0. Any values < 0 are treated as a missing datapoint, and are ignored. While DELTA metrics are accepted by this alignment, special care should be taken that the values for the metric will always be positive. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentChange,
                        }
                        impl ::std::default::Default for QueryParametersSecondaryAggregationPerSeriesAlignerEnum {
                            fn default() -> Self {
                                Self::AlignNone
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Required. Specifies which information is returned about the time series."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "FULL")]
                            #[doc = "Returns the identity of the metric(s), the time series, and the time series data."]
                            Full,
                            #[serde(rename = "HEADERS")]
                            #[doc = "Returns the identity of the metric and the time series resource, but not the time series data."]
                            Headers,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::Full
                            }
                        }
                    }
                }
            }
        }
    }
    pub mod organizations {
        pub mod resources {
            pub mod time_series {
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
                            #[serde(rename = "aggregation.alignmentPeriod")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The alignment_period specifies a time interval, in seconds, that is used to divide the data in all the time series into consistent blocks of time. This will be done before the per-series aligner can be applied to the data.The value must be at least 60 seconds. If a per-series aligner other than ALIGN_NONE is specified, this field is required or an error is returned. If no per-series aligner is specified, or the aligner ALIGN_NONE is specified, then this field is ignored.The maximum value of the alignment_period is 104 weeks (2 years) for charts, and 90,000 seconds (25 hours) for alerting policies."]
                            pub aggregation_alignment_period:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "aggregation.crossSeriesReducer")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned."]
                            pub aggregation_cross_series_reducer: ::std::option::Option<
                                QueryParametersAggregationCrossSeriesReducerEnum,
                            >,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "aggregation.groupByFields")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The set of fields to preserve when cross_series_reducer is specified. The group_by_fields determine how the time series are partitioned into subsets prior to applying the aggregation operation. Each subset contains time series that have the same value for each of the grouping fields. Each individual time series is a member of exactly one subset. The cross_series_reducer is applied to each subset of time series. It is not possible to reduce across different resource types, so this field implicitly contains resource.type. Fields not specified in group_by_fields are aggregated away. If group_by_fields is not specified and all the time series have the same resource type, then the time series are aggregated into a single output time series. If cross_series_reducer is not defined, this field is ignored."]
                            pub aggregation_group_by_fields:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "aggregation.perSeriesAligner")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned."]
                            pub aggregation_per_series_aligner: ::std::option::Option<
                                QueryParametersAggregationPerSeriesAlignerEnum,
                            >,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. A monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) that specifies which time series should be returned. The filter must specify a single metric type, and can additionally specify metric labels and other information. For example: metric.type = \"compute.googleapis.com/instance/cpu/usage_time\" AND metric.labels.instance_name = \"my-instance-name\" "]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "interval.endTime")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. The end of the time interval."]
                            pub interval_end_time: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "interval.startTime")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The beginning of the time interval. The default value for the start time is the end time. The start time must not be later than the end time."]
                            pub interval_start_time: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Unsupported: must be left blank. The points in each time series are currently returned in reverse time order (most recent to oldest)."]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A positive number that is the maximum number of results to return. If page_size is empty or more than 100,000 results, the effective page_size is 100,000 results. If view is set to FULL, this is the maximum number of Points returned. If view is set to HEADERS, this is the maximum number of TimeSeries returned."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If this field is not empty then it must contain the nextPageToken value returned by a previous call to this method. Using this field causes the method to return additional results from the previous method call."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "secondaryAggregation.alignmentPeriod")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The alignment_period specifies a time interval, in seconds, that is used to divide the data in all the time series into consistent blocks of time. This will be done before the per-series aligner can be applied to the data.The value must be at least 60 seconds. If a per-series aligner other than ALIGN_NONE is specified, this field is required or an error is returned. If no per-series aligner is specified, or the aligner ALIGN_NONE is specified, then this field is ignored.The maximum value of the alignment_period is 104 weeks (2 years) for charts, and 90,000 seconds (25 hours) for alerting policies."]
                            pub secondary_aggregation_alignment_period:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "secondaryAggregation.crossSeriesReducer")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned."]
                            pub secondary_aggregation_cross_series_reducer: ::std::option::Option<
                                QueryParametersSecondaryAggregationCrossSeriesReducerEnum,
                            >,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "secondaryAggregation.groupByFields")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The set of fields to preserve when cross_series_reducer is specified. The group_by_fields determine how the time series are partitioned into subsets prior to applying the aggregation operation. Each subset contains time series that have the same value for each of the grouping fields. Each individual time series is a member of exactly one subset. The cross_series_reducer is applied to each subset of time series. It is not possible to reduce across different resource types, so this field implicitly contains resource.type. Fields not specified in group_by_fields are aggregated away. If group_by_fields is not specified and all the time series have the same resource type, then the time series are aggregated into a single output time series. If cross_series_reducer is not defined, this field is ignored."]
                            pub secondary_aggregation_group_by_fields:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "secondaryAggregation.perSeriesAligner")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned."]
                            pub secondary_aggregation_per_series_aligner: ::std::option::Option<
                                QueryParametersSecondaryAggregationPerSeriesAlignerEnum,
                            >,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. Specifies which information is returned about the time series."]
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
                        #[doc = "The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned."]
                        pub enum QueryParametersAggregationCrossSeriesReducerEnum {
                            #[serde(rename = "REDUCE_NONE")]
                            #[doc = "No cross-time series reduction. The output of the Aligner is returned."]
                            ReduceNone,
                            #[serde(rename = "REDUCE_MEAN")]
                            #[doc = "Reduce by computing the mean value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE."]
                            ReduceMean,
                            #[serde(rename = "REDUCE_MIN")]
                            #[doc = "Reduce by computing the minimum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input."]
                            ReduceMin,
                            #[serde(rename = "REDUCE_MAX")]
                            #[doc = "Reduce by computing the maximum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input."]
                            ReduceMax,
                            #[serde(rename = "REDUCE_SUM")]
                            #[doc = "Reduce by computing the sum across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric and distribution values. The value_type of the output is the same as the value_type of the input."]
                            ReduceSum,
                            #[serde(rename = "REDUCE_STDDEV")]
                            #[doc = "Reduce by computing the standard deviation across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE."]
                            ReduceStddev,
                            #[serde(rename = "REDUCE_COUNT")]
                            #[doc = "Reduce by computing the number of data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of numeric, Boolean, distribution, and string value_type. The value_type of the output is INT64."]
                            ReduceCount,
                            #[serde(rename = "REDUCE_COUNT_TRUE")]
                            #[doc = "Reduce by computing the number of True-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64."]
                            ReduceCountTrue,
                            #[serde(rename = "REDUCE_COUNT_FALSE")]
                            #[doc = "Reduce by computing the number of False-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64."]
                            ReduceCountFalse,
                            #[serde(rename = "REDUCE_FRACTION_TRUE")]
                            #[doc = "Reduce by computing the ratio of the number of True-valued data points to the total number of data points for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The output value is in the range 0.0, 1.0 and has value_type DOUBLE."]
                            ReduceFractionTrue,
                            #[serde(rename = "REDUCE_PERCENTILE_99")]
                            #[doc = "Reduce by computing the 99th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile99,
                            #[serde(rename = "REDUCE_PERCENTILE_95")]
                            #[doc = "Reduce by computing the 95th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile95,
                            #[serde(rename = "REDUCE_PERCENTILE_50")]
                            #[doc = "Reduce by computing the 50th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile50,
                            #[serde(rename = "REDUCE_PERCENTILE_05")]
                            #[doc = "Reduce by computing the 5th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile05,
                        }
                        impl ::std::default::Default for QueryParametersAggregationCrossSeriesReducerEnum {
                            fn default() -> Self {
                                Self::ReduceNone
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned."]
                        pub enum QueryParametersAggregationPerSeriesAlignerEnum {
                            #[serde(rename = "ALIGN_NONE")]
                            #[doc = "No alignment. Raw data is returned. Not valid if cross-series reduction is requested. The value_type of the result is the same as the value_type of the input."]
                            AlignNone,
                            #[serde(rename = "ALIGN_DELTA")]
                            #[doc = "Align and convert to DELTA. The output is delta = y1 - y0.This alignment is valid for CUMULATIVE and DELTA metrics. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignDelta,
                            #[serde(rename = "ALIGN_RATE")]
                            #[doc = "Align and convert to a rate. The result is computed as rate = (y1 - y0)/(t1 - t0), or \"delta over time\". Think of this aligner as providing the slope of the line that passes through the value at the start and at the end of the alignment_period.This aligner is valid for CUMULATIVE and DELTA metrics with numeric values. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The output is a GAUGE metric with value_type DOUBLE.If, by \"rate\", you mean \"percentage change\", see the ALIGN_PERCENT_CHANGE aligner instead."]
                            AlignRate,
                            #[serde(rename = "ALIGN_INTERPOLATE")]
                            #[doc = "Align by interpolating between adjacent points around the alignment period boundary. This aligner is valid for GAUGE metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignInterpolate,
                            #[serde(rename = "ALIGN_NEXT_OLDER")]
                            #[doc = "Align by moving the most recent data point before the end of the alignment period to the boundary at the end of the alignment period. This aligner is valid for GAUGE metrics. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignNextOlder,
                            #[serde(rename = "ALIGN_MIN")]
                            #[doc = "Align the time series by returning the minimum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignMin,
                            #[serde(rename = "ALIGN_MAX")]
                            #[doc = "Align the time series by returning the maximum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignMax,
                            #[serde(rename = "ALIGN_MEAN")]
                            #[doc = "Align the time series by returning the mean value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is DOUBLE."]
                            AlignMean,
                            #[serde(rename = "ALIGN_COUNT")]
                            #[doc = "Align the time series by returning the number of values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric or Boolean values. The value_type of the aligned result is INT64."]
                            AlignCount,
                            #[serde(rename = "ALIGN_SUM")]
                            #[doc = "Align the time series by returning the sum of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric and distribution values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignSum,
                            #[serde(rename = "ALIGN_STDDEV")]
                            #[doc = "Align the time series by returning the standard deviation of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the output is DOUBLE."]
                            AlignStddev,
                            #[serde(rename = "ALIGN_COUNT_TRUE")]
                            #[doc = "Align the time series by returning the number of True values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64."]
                            AlignCountTrue,
                            #[serde(rename = "ALIGN_COUNT_FALSE")]
                            #[doc = "Align the time series by returning the number of False values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64."]
                            AlignCountFalse,
                            #[serde(rename = "ALIGN_FRACTION_TRUE")]
                            #[doc = "Align the time series by returning the ratio of the number of True values to the total number of values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The output value is in the range 0.0, 1.0 and has value_type DOUBLE."]
                            AlignFractionTrue,
                            #[serde(rename = "ALIGN_PERCENTILE_99")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 99th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile99,
                            #[serde(rename = "ALIGN_PERCENTILE_95")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 95th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile95,
                            #[serde(rename = "ALIGN_PERCENTILE_50")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 50th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile50,
                            #[serde(rename = "ALIGN_PERCENTILE_05")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 5th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile05,
                            #[serde(rename = "ALIGN_PERCENT_CHANGE")]
                            #[doc = "Align and convert to a percentage change. This aligner is valid for GAUGE and DELTA metrics with numeric values. This alignment returns ((current - previous)/previous) * 100, where the value of previous is determined based on the alignment_period.If the values of current and previous are both 0, then the returned value is 0. If only previous is 0, the returned value is infinity.A 10-minute moving mean is computed at each point of the alignment period prior to the above calculation to smooth the metric and prevent false positives from very short-lived spikes. The moving mean is only applicable for data whose values are >= 0. Any values < 0 are treated as a missing datapoint, and are ignored. While DELTA metrics are accepted by this alignment, special care should be taken that the values for the metric will always be positive. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentChange,
                        }
                        impl ::std::default::Default for QueryParametersAggregationPerSeriesAlignerEnum {
                            fn default() -> Self {
                                Self::AlignNone
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned."]
                        pub enum QueryParametersSecondaryAggregationCrossSeriesReducerEnum {
                            #[serde(rename = "REDUCE_NONE")]
                            #[doc = "No cross-time series reduction. The output of the Aligner is returned."]
                            ReduceNone,
                            #[serde(rename = "REDUCE_MEAN")]
                            #[doc = "Reduce by computing the mean value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE."]
                            ReduceMean,
                            #[serde(rename = "REDUCE_MIN")]
                            #[doc = "Reduce by computing the minimum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input."]
                            ReduceMin,
                            #[serde(rename = "REDUCE_MAX")]
                            #[doc = "Reduce by computing the maximum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input."]
                            ReduceMax,
                            #[serde(rename = "REDUCE_SUM")]
                            #[doc = "Reduce by computing the sum across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric and distribution values. The value_type of the output is the same as the value_type of the input."]
                            ReduceSum,
                            #[serde(rename = "REDUCE_STDDEV")]
                            #[doc = "Reduce by computing the standard deviation across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE."]
                            ReduceStddev,
                            #[serde(rename = "REDUCE_COUNT")]
                            #[doc = "Reduce by computing the number of data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of numeric, Boolean, distribution, and string value_type. The value_type of the output is INT64."]
                            ReduceCount,
                            #[serde(rename = "REDUCE_COUNT_TRUE")]
                            #[doc = "Reduce by computing the number of True-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64."]
                            ReduceCountTrue,
                            #[serde(rename = "REDUCE_COUNT_FALSE")]
                            #[doc = "Reduce by computing the number of False-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64."]
                            ReduceCountFalse,
                            #[serde(rename = "REDUCE_FRACTION_TRUE")]
                            #[doc = "Reduce by computing the ratio of the number of True-valued data points to the total number of data points for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The output value is in the range 0.0, 1.0 and has value_type DOUBLE."]
                            ReduceFractionTrue,
                            #[serde(rename = "REDUCE_PERCENTILE_99")]
                            #[doc = "Reduce by computing the 99th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile99,
                            #[serde(rename = "REDUCE_PERCENTILE_95")]
                            #[doc = "Reduce by computing the 95th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile95,
                            #[serde(rename = "REDUCE_PERCENTILE_50")]
                            #[doc = "Reduce by computing the 50th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile50,
                            #[serde(rename = "REDUCE_PERCENTILE_05")]
                            #[doc = "Reduce by computing the 5th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile05,
                        }
                        impl ::std::default::Default for QueryParametersSecondaryAggregationCrossSeriesReducerEnum {
                            fn default() -> Self {
                                Self::ReduceNone
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned."]
                        pub enum QueryParametersSecondaryAggregationPerSeriesAlignerEnum {
                            #[serde(rename = "ALIGN_NONE")]
                            #[doc = "No alignment. Raw data is returned. Not valid if cross-series reduction is requested. The value_type of the result is the same as the value_type of the input."]
                            AlignNone,
                            #[serde(rename = "ALIGN_DELTA")]
                            #[doc = "Align and convert to DELTA. The output is delta = y1 - y0.This alignment is valid for CUMULATIVE and DELTA metrics. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignDelta,
                            #[serde(rename = "ALIGN_RATE")]
                            #[doc = "Align and convert to a rate. The result is computed as rate = (y1 - y0)/(t1 - t0), or \"delta over time\". Think of this aligner as providing the slope of the line that passes through the value at the start and at the end of the alignment_period.This aligner is valid for CUMULATIVE and DELTA metrics with numeric values. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The output is a GAUGE metric with value_type DOUBLE.If, by \"rate\", you mean \"percentage change\", see the ALIGN_PERCENT_CHANGE aligner instead."]
                            AlignRate,
                            #[serde(rename = "ALIGN_INTERPOLATE")]
                            #[doc = "Align by interpolating between adjacent points around the alignment period boundary. This aligner is valid for GAUGE metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignInterpolate,
                            #[serde(rename = "ALIGN_NEXT_OLDER")]
                            #[doc = "Align by moving the most recent data point before the end of the alignment period to the boundary at the end of the alignment period. This aligner is valid for GAUGE metrics. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignNextOlder,
                            #[serde(rename = "ALIGN_MIN")]
                            #[doc = "Align the time series by returning the minimum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignMin,
                            #[serde(rename = "ALIGN_MAX")]
                            #[doc = "Align the time series by returning the maximum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignMax,
                            #[serde(rename = "ALIGN_MEAN")]
                            #[doc = "Align the time series by returning the mean value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is DOUBLE."]
                            AlignMean,
                            #[serde(rename = "ALIGN_COUNT")]
                            #[doc = "Align the time series by returning the number of values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric or Boolean values. The value_type of the aligned result is INT64."]
                            AlignCount,
                            #[serde(rename = "ALIGN_SUM")]
                            #[doc = "Align the time series by returning the sum of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric and distribution values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignSum,
                            #[serde(rename = "ALIGN_STDDEV")]
                            #[doc = "Align the time series by returning the standard deviation of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the output is DOUBLE."]
                            AlignStddev,
                            #[serde(rename = "ALIGN_COUNT_TRUE")]
                            #[doc = "Align the time series by returning the number of True values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64."]
                            AlignCountTrue,
                            #[serde(rename = "ALIGN_COUNT_FALSE")]
                            #[doc = "Align the time series by returning the number of False values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64."]
                            AlignCountFalse,
                            #[serde(rename = "ALIGN_FRACTION_TRUE")]
                            #[doc = "Align the time series by returning the ratio of the number of True values to the total number of values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The output value is in the range 0.0, 1.0 and has value_type DOUBLE."]
                            AlignFractionTrue,
                            #[serde(rename = "ALIGN_PERCENTILE_99")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 99th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile99,
                            #[serde(rename = "ALIGN_PERCENTILE_95")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 95th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile95,
                            #[serde(rename = "ALIGN_PERCENTILE_50")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 50th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile50,
                            #[serde(rename = "ALIGN_PERCENTILE_05")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 5th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile05,
                            #[serde(rename = "ALIGN_PERCENT_CHANGE")]
                            #[doc = "Align and convert to a percentage change. This aligner is valid for GAUGE and DELTA metrics with numeric values. This alignment returns ((current - previous)/previous) * 100, where the value of previous is determined based on the alignment_period.If the values of current and previous are both 0, then the returned value is 0. If only previous is 0, the returned value is infinity.A 10-minute moving mean is computed at each point of the alignment period prior to the above calculation to smooth the metric and prevent false positives from very short-lived spikes. The moving mean is only applicable for data whose values are >= 0. Any values < 0 are treated as a missing datapoint, and are ignored. While DELTA metrics are accepted by this alignment, special care should be taken that the values for the metric will always be positive. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentChange,
                        }
                        impl ::std::default::Default for QueryParametersSecondaryAggregationPerSeriesAlignerEnum {
                            fn default() -> Self {
                                Self::AlignNone
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Required. Specifies which information is returned about the time series."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "FULL")]
                            #[doc = "Returns the identity of the metric(s), the time series, and the time series data."]
                            Full,
                            #[serde(rename = "HEADERS")]
                            #[doc = "Returns the identity of the metric and the time series resource, but not the time series data."]
                            Headers,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::Full
                            }
                        }
                    }
                }
            }
        }
    }
    pub mod projects {
        pub mod resources {
            pub mod alert_policies {
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
                            #[doc = "If provided, this field specifies the criteria that must be met by alert policies to be included in the response.For more details, see sorting and filtering (https://cloud.google.com/monitoring/api/v3/sorting-and-filtering)."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A comma-separated list of fields by which to sort the result. Supports the same set of field references as the filter field. Entries can be prefixed with a minus sign to sort by the field in descending order.For more details, see sorting and filtering (https://cloud.google.com/monitoring/api/v3/sorting-and-filtering)."]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of results to return in a single response."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If this field is not empty then it must contain the nextPageToken value returned by a previous call to this method. Using this field causes the method to return more results from the previous method call."]
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
                            #[doc = "Optional. A list of alerting policy field names. If this field is not empty, each listed field in the existing alerting policy is set to the value of the corresponding field in the supplied policy (alert_policy), or to the field's default value if the field is not in the supplied alerting policy. Fields not listed retain their previous value.Examples of valid field masks include display_name, documentation, documentation.content, documentation.mime_type, user_labels, user_label.nameofkey, enabled, conditions, combiner, etc.If this field is empty, then the supplied alerting policy replaces the existing policy. It is the same as deleting the existing policy and adding the supplied policy, except for the following: The new policy will have the same [ALERT_POLICY_ID] as the former policy. This gives you continuity with the former policy in your notifications and incidents. Conditions in the new policy will keep their former [CONDITION_ID] if the supplied condition includes the name field with that [CONDITION_ID]. If the supplied condition omits the name field, then a new [CONDITION_ID] is created."]
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
            pub mod groups {
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
                            #[serde(rename = "validateOnly")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If true, validate this request but do not create the group."]
                            pub validate_only: ::std::option::Option<::std::primitive::bool>,
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
                            #[serde(rename = "recursive")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If this field is true, then the request means to delete a group with all its descendants. Otherwise, the request means to delete a group only when it has no descendants. The default value is false."]
                            pub recursive: ::std::option::Option<::std::primitive::bool>,
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
                            #[serde(rename = "ancestorsOfGroup")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A group name. The format is: projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID] Returns groups that are ancestors of the specified group. The groups are returned in order, starting with the immediate parent and ending with the most distant ancestor. If the specified group has no immediate parent, the results are empty."]
                            pub ancestors_of_group: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "childrenOfGroup")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A group name. The format is: projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID] Returns groups whose parent_name field contains the group name. If no groups have this parent, the results are empty."]
                            pub children_of_group: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "descendantsOfGroup")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A group name. The format is: projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID] Returns the descendants of the specified group. This is a superset of the results returned by the children_of_group filter, and includes children-of-children, and so forth."]
                            pub descendants_of_group: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A positive number that is the maximum number of results to return."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If this field is not empty then it must contain the next_page_token value returned by a previous call to this method. Using this field causes the method to return additional results from the previous method call."]
                            pub page_token: ::std::option::Option<::std::string::String>,
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
                            #[serde(rename = "validateOnly")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If true, validate this request but do not update the existing group."]
                            pub validate_only: ::std::option::Option<::std::primitive::bool>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "filter")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "An optional list filter (https://cloud.google.com/monitoring/api/learn_more#filtering) describing the members to be returned. The filter may reference the type, labels, and metadata of monitored resources that comprise the group. For example, to return only resources representing Compute Engine VM instances, use this filter: `resource.type = \"gce_instance\"` "]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "interval.endTime")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Required. The end of the time interval."]
                                    pub interval_end_time:
                                        ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "interval.startTime")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. The beginning of the time interval. The default value for the start time is the end time. The start time must not be later than the end time."]
                                    pub interval_start_time:
                                        ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A positive number that is the maximum number of results to return."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "If this field is not empty then it must contain the next_page_token value returned by a previous call to this method. Using this field causes the method to return additional results from the previous method call."]
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
            pub mod metric_descriptors {
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
                            #[doc = "If this field is empty, all custom and system-defined metric descriptors are returned. Otherwise, the filter (https://cloud.google.com/monitoring/api/v3/filters) specifies which metric descriptors are to be returned. For example, the following filter matches all custom metrics (https://cloud.google.com/monitoring/custom-metrics): metric.type = starts_with(\"custom.googleapis.com/\") "]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A positive number that is the maximum number of results to return."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If this field is not empty then it must contain the nextPageToken value returned by a previous call to this method. Using this field causes the method to return additional results from the previous method call."]
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
            pub mod monitored_resource_descriptors {
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
                            #[doc = "An optional filter (https://cloud.google.com/monitoring/api/v3/filters) describing the descriptors to be returned. The filter can reference the descriptor's type and labels. For example, the following filter returns only Google Compute Engine descriptors that have an id label: resource.type = starts_with(\"gce_\") AND resource.label:id "]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A positive number that is the maximum number of results to return."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If this field is not empty then it must contain the nextPageToken value returned by a previous call to this method. Using this field causes the method to return additional results from the previous method call."]
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
            pub mod notification_channel_descriptors {
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
                            #[doc = "The maximum number of results to return in a single response. If not set to a positive number, a reasonable value will be chosen by the service."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If non-empty, page_token must contain a value returned as the next_page_token in a previous response to request the next set of results."]
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
            pub mod notification_channels {
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
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If true, the notification channel will be deleted regardless of its use in alert policies (the policies will be updated to remove the channel). If false, channels that are still referenced by an existing alerting policy will fail to be deleted in a delete operation."]
                            pub force: ::std::option::Option<::std::primitive::bool>,
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
                            #[doc = "If provided, this field specifies the criteria that must be met by notification channels to be included in the response.For more details, see sorting and filtering (https://cloud.google.com/monitoring/api/v3/sorting-and-filtering)."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A comma-separated list of fields by which to sort the result. Supports the same set of fields as in filter. Entries can be prefixed with a minus sign to sort in descending rather than ascending order.For more details, see sorting and filtering (https://cloud.google.com/monitoring/api/v3/sorting-and-filtering)."]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of results to return in a single response. If not set to a positive number, a reasonable value will be chosen by the service."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If non-empty, page_token must contain a value returned as the next_page_token in a previous response to request the next set of results."]
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
                            #[doc = "The fields to update."]
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
            pub mod time_series {
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
                            #[serde(rename = "aggregation.alignmentPeriod")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The alignment_period specifies a time interval, in seconds, that is used to divide the data in all the time series into consistent blocks of time. This will be done before the per-series aligner can be applied to the data.The value must be at least 60 seconds. If a per-series aligner other than ALIGN_NONE is specified, this field is required or an error is returned. If no per-series aligner is specified, or the aligner ALIGN_NONE is specified, then this field is ignored.The maximum value of the alignment_period is 104 weeks (2 years) for charts, and 90,000 seconds (25 hours) for alerting policies."]
                            pub aggregation_alignment_period:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "aggregation.crossSeriesReducer")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned."]
                            pub aggregation_cross_series_reducer: ::std::option::Option<
                                QueryParametersAggregationCrossSeriesReducerEnum,
                            >,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "aggregation.groupByFields")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The set of fields to preserve when cross_series_reducer is specified. The group_by_fields determine how the time series are partitioned into subsets prior to applying the aggregation operation. Each subset contains time series that have the same value for each of the grouping fields. Each individual time series is a member of exactly one subset. The cross_series_reducer is applied to each subset of time series. It is not possible to reduce across different resource types, so this field implicitly contains resource.type. Fields not specified in group_by_fields are aggregated away. If group_by_fields is not specified and all the time series have the same resource type, then the time series are aggregated into a single output time series. If cross_series_reducer is not defined, this field is ignored."]
                            pub aggregation_group_by_fields:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "aggregation.perSeriesAligner")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned."]
                            pub aggregation_per_series_aligner: ::std::option::Option<
                                QueryParametersAggregationPerSeriesAlignerEnum,
                            >,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. A monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) that specifies which time series should be returned. The filter must specify a single metric type, and can additionally specify metric labels and other information. For example: metric.type = \"compute.googleapis.com/instance/cpu/usage_time\" AND metric.labels.instance_name = \"my-instance-name\" "]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "interval.endTime")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. The end of the time interval."]
                            pub interval_end_time: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "interval.startTime")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The beginning of the time interval. The default value for the start time is the end time. The start time must not be later than the end time."]
                            pub interval_start_time: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Unsupported: must be left blank. The points in each time series are currently returned in reverse time order (most recent to oldest)."]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A positive number that is the maximum number of results to return. If page_size is empty or more than 100,000 results, the effective page_size is 100,000 results. If view is set to FULL, this is the maximum number of Points returned. If view is set to HEADERS, this is the maximum number of TimeSeries returned."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If this field is not empty then it must contain the nextPageToken value returned by a previous call to this method. Using this field causes the method to return additional results from the previous method call."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "secondaryAggregation.alignmentPeriod")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The alignment_period specifies a time interval, in seconds, that is used to divide the data in all the time series into consistent blocks of time. This will be done before the per-series aligner can be applied to the data.The value must be at least 60 seconds. If a per-series aligner other than ALIGN_NONE is specified, this field is required or an error is returned. If no per-series aligner is specified, or the aligner ALIGN_NONE is specified, then this field is ignored.The maximum value of the alignment_period is 104 weeks (2 years) for charts, and 90,000 seconds (25 hours) for alerting policies."]
                            pub secondary_aggregation_alignment_period:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "secondaryAggregation.crossSeriesReducer")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned."]
                            pub secondary_aggregation_cross_series_reducer: ::std::option::Option<
                                QueryParametersSecondaryAggregationCrossSeriesReducerEnum,
                            >,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "secondaryAggregation.groupByFields")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The set of fields to preserve when cross_series_reducer is specified. The group_by_fields determine how the time series are partitioned into subsets prior to applying the aggregation operation. Each subset contains time series that have the same value for each of the grouping fields. Each individual time series is a member of exactly one subset. The cross_series_reducer is applied to each subset of time series. It is not possible to reduce across different resource types, so this field implicitly contains resource.type. Fields not specified in group_by_fields are aggregated away. If group_by_fields is not specified and all the time series have the same resource type, then the time series are aggregated into a single output time series. If cross_series_reducer is not defined, this field is ignored."]
                            pub secondary_aggregation_group_by_fields:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "secondaryAggregation.perSeriesAligner")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned."]
                            pub secondary_aggregation_per_series_aligner: ::std::option::Option<
                                QueryParametersSecondaryAggregationPerSeriesAlignerEnum,
                            >,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. Specifies which information is returned about the time series."]
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
                        #[doc = "The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned."]
                        pub enum QueryParametersAggregationCrossSeriesReducerEnum {
                            #[serde(rename = "REDUCE_NONE")]
                            #[doc = "No cross-time series reduction. The output of the Aligner is returned."]
                            ReduceNone,
                            #[serde(rename = "REDUCE_MEAN")]
                            #[doc = "Reduce by computing the mean value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE."]
                            ReduceMean,
                            #[serde(rename = "REDUCE_MIN")]
                            #[doc = "Reduce by computing the minimum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input."]
                            ReduceMin,
                            #[serde(rename = "REDUCE_MAX")]
                            #[doc = "Reduce by computing the maximum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input."]
                            ReduceMax,
                            #[serde(rename = "REDUCE_SUM")]
                            #[doc = "Reduce by computing the sum across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric and distribution values. The value_type of the output is the same as the value_type of the input."]
                            ReduceSum,
                            #[serde(rename = "REDUCE_STDDEV")]
                            #[doc = "Reduce by computing the standard deviation across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE."]
                            ReduceStddev,
                            #[serde(rename = "REDUCE_COUNT")]
                            #[doc = "Reduce by computing the number of data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of numeric, Boolean, distribution, and string value_type. The value_type of the output is INT64."]
                            ReduceCount,
                            #[serde(rename = "REDUCE_COUNT_TRUE")]
                            #[doc = "Reduce by computing the number of True-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64."]
                            ReduceCountTrue,
                            #[serde(rename = "REDUCE_COUNT_FALSE")]
                            #[doc = "Reduce by computing the number of False-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64."]
                            ReduceCountFalse,
                            #[serde(rename = "REDUCE_FRACTION_TRUE")]
                            #[doc = "Reduce by computing the ratio of the number of True-valued data points to the total number of data points for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The output value is in the range 0.0, 1.0 and has value_type DOUBLE."]
                            ReduceFractionTrue,
                            #[serde(rename = "REDUCE_PERCENTILE_99")]
                            #[doc = "Reduce by computing the 99th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile99,
                            #[serde(rename = "REDUCE_PERCENTILE_95")]
                            #[doc = "Reduce by computing the 95th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile95,
                            #[serde(rename = "REDUCE_PERCENTILE_50")]
                            #[doc = "Reduce by computing the 50th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile50,
                            #[serde(rename = "REDUCE_PERCENTILE_05")]
                            #[doc = "Reduce by computing the 5th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile05,
                        }
                        impl ::std::default::Default for QueryParametersAggregationCrossSeriesReducerEnum {
                            fn default() -> Self {
                                Self::ReduceNone
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned."]
                        pub enum QueryParametersAggregationPerSeriesAlignerEnum {
                            #[serde(rename = "ALIGN_NONE")]
                            #[doc = "No alignment. Raw data is returned. Not valid if cross-series reduction is requested. The value_type of the result is the same as the value_type of the input."]
                            AlignNone,
                            #[serde(rename = "ALIGN_DELTA")]
                            #[doc = "Align and convert to DELTA. The output is delta = y1 - y0.This alignment is valid for CUMULATIVE and DELTA metrics. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignDelta,
                            #[serde(rename = "ALIGN_RATE")]
                            #[doc = "Align and convert to a rate. The result is computed as rate = (y1 - y0)/(t1 - t0), or \"delta over time\". Think of this aligner as providing the slope of the line that passes through the value at the start and at the end of the alignment_period.This aligner is valid for CUMULATIVE and DELTA metrics with numeric values. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The output is a GAUGE metric with value_type DOUBLE.If, by \"rate\", you mean \"percentage change\", see the ALIGN_PERCENT_CHANGE aligner instead."]
                            AlignRate,
                            #[serde(rename = "ALIGN_INTERPOLATE")]
                            #[doc = "Align by interpolating between adjacent points around the alignment period boundary. This aligner is valid for GAUGE metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignInterpolate,
                            #[serde(rename = "ALIGN_NEXT_OLDER")]
                            #[doc = "Align by moving the most recent data point before the end of the alignment period to the boundary at the end of the alignment period. This aligner is valid for GAUGE metrics. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignNextOlder,
                            #[serde(rename = "ALIGN_MIN")]
                            #[doc = "Align the time series by returning the minimum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignMin,
                            #[serde(rename = "ALIGN_MAX")]
                            #[doc = "Align the time series by returning the maximum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignMax,
                            #[serde(rename = "ALIGN_MEAN")]
                            #[doc = "Align the time series by returning the mean value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is DOUBLE."]
                            AlignMean,
                            #[serde(rename = "ALIGN_COUNT")]
                            #[doc = "Align the time series by returning the number of values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric or Boolean values. The value_type of the aligned result is INT64."]
                            AlignCount,
                            #[serde(rename = "ALIGN_SUM")]
                            #[doc = "Align the time series by returning the sum of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric and distribution values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignSum,
                            #[serde(rename = "ALIGN_STDDEV")]
                            #[doc = "Align the time series by returning the standard deviation of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the output is DOUBLE."]
                            AlignStddev,
                            #[serde(rename = "ALIGN_COUNT_TRUE")]
                            #[doc = "Align the time series by returning the number of True values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64."]
                            AlignCountTrue,
                            #[serde(rename = "ALIGN_COUNT_FALSE")]
                            #[doc = "Align the time series by returning the number of False values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64."]
                            AlignCountFalse,
                            #[serde(rename = "ALIGN_FRACTION_TRUE")]
                            #[doc = "Align the time series by returning the ratio of the number of True values to the total number of values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The output value is in the range 0.0, 1.0 and has value_type DOUBLE."]
                            AlignFractionTrue,
                            #[serde(rename = "ALIGN_PERCENTILE_99")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 99th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile99,
                            #[serde(rename = "ALIGN_PERCENTILE_95")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 95th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile95,
                            #[serde(rename = "ALIGN_PERCENTILE_50")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 50th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile50,
                            #[serde(rename = "ALIGN_PERCENTILE_05")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 5th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile05,
                            #[serde(rename = "ALIGN_PERCENT_CHANGE")]
                            #[doc = "Align and convert to a percentage change. This aligner is valid for GAUGE and DELTA metrics with numeric values. This alignment returns ((current - previous)/previous) * 100, where the value of previous is determined based on the alignment_period.If the values of current and previous are both 0, then the returned value is 0. If only previous is 0, the returned value is infinity.A 10-minute moving mean is computed at each point of the alignment period prior to the above calculation to smooth the metric and prevent false positives from very short-lived spikes. The moving mean is only applicable for data whose values are >= 0. Any values < 0 are treated as a missing datapoint, and are ignored. While DELTA metrics are accepted by this alignment, special care should be taken that the values for the metric will always be positive. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentChange,
                        }
                        impl ::std::default::Default for QueryParametersAggregationPerSeriesAlignerEnum {
                            fn default() -> Self {
                                Self::AlignNone
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned."]
                        pub enum QueryParametersSecondaryAggregationCrossSeriesReducerEnum {
                            #[serde(rename = "REDUCE_NONE")]
                            #[doc = "No cross-time series reduction. The output of the Aligner is returned."]
                            ReduceNone,
                            #[serde(rename = "REDUCE_MEAN")]
                            #[doc = "Reduce by computing the mean value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE."]
                            ReduceMean,
                            #[serde(rename = "REDUCE_MIN")]
                            #[doc = "Reduce by computing the minimum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input."]
                            ReduceMin,
                            #[serde(rename = "REDUCE_MAX")]
                            #[doc = "Reduce by computing the maximum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input."]
                            ReduceMax,
                            #[serde(rename = "REDUCE_SUM")]
                            #[doc = "Reduce by computing the sum across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric and distribution values. The value_type of the output is the same as the value_type of the input."]
                            ReduceSum,
                            #[serde(rename = "REDUCE_STDDEV")]
                            #[doc = "Reduce by computing the standard deviation across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE."]
                            ReduceStddev,
                            #[serde(rename = "REDUCE_COUNT")]
                            #[doc = "Reduce by computing the number of data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of numeric, Boolean, distribution, and string value_type. The value_type of the output is INT64."]
                            ReduceCount,
                            #[serde(rename = "REDUCE_COUNT_TRUE")]
                            #[doc = "Reduce by computing the number of True-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64."]
                            ReduceCountTrue,
                            #[serde(rename = "REDUCE_COUNT_FALSE")]
                            #[doc = "Reduce by computing the number of False-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64."]
                            ReduceCountFalse,
                            #[serde(rename = "REDUCE_FRACTION_TRUE")]
                            #[doc = "Reduce by computing the ratio of the number of True-valued data points to the total number of data points for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The output value is in the range 0.0, 1.0 and has value_type DOUBLE."]
                            ReduceFractionTrue,
                            #[serde(rename = "REDUCE_PERCENTILE_99")]
                            #[doc = "Reduce by computing the 99th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile99,
                            #[serde(rename = "REDUCE_PERCENTILE_95")]
                            #[doc = "Reduce by computing the 95th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile95,
                            #[serde(rename = "REDUCE_PERCENTILE_50")]
                            #[doc = "Reduce by computing the 50th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile50,
                            #[serde(rename = "REDUCE_PERCENTILE_05")]
                            #[doc = "Reduce by computing the 5th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
                            ReducePercentile05,
                        }
                        impl ::std::default::Default for QueryParametersSecondaryAggregationCrossSeriesReducerEnum {
                            fn default() -> Self {
                                Self::ReduceNone
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned."]
                        pub enum QueryParametersSecondaryAggregationPerSeriesAlignerEnum {
                            #[serde(rename = "ALIGN_NONE")]
                            #[doc = "No alignment. Raw data is returned. Not valid if cross-series reduction is requested. The value_type of the result is the same as the value_type of the input."]
                            AlignNone,
                            #[serde(rename = "ALIGN_DELTA")]
                            #[doc = "Align and convert to DELTA. The output is delta = y1 - y0.This alignment is valid for CUMULATIVE and DELTA metrics. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignDelta,
                            #[serde(rename = "ALIGN_RATE")]
                            #[doc = "Align and convert to a rate. The result is computed as rate = (y1 - y0)/(t1 - t0), or \"delta over time\". Think of this aligner as providing the slope of the line that passes through the value at the start and at the end of the alignment_period.This aligner is valid for CUMULATIVE and DELTA metrics with numeric values. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The output is a GAUGE metric with value_type DOUBLE.If, by \"rate\", you mean \"percentage change\", see the ALIGN_PERCENT_CHANGE aligner instead."]
                            AlignRate,
                            #[serde(rename = "ALIGN_INTERPOLATE")]
                            #[doc = "Align by interpolating between adjacent points around the alignment period boundary. This aligner is valid for GAUGE metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignInterpolate,
                            #[serde(rename = "ALIGN_NEXT_OLDER")]
                            #[doc = "Align by moving the most recent data point before the end of the alignment period to the boundary at the end of the alignment period. This aligner is valid for GAUGE metrics. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignNextOlder,
                            #[serde(rename = "ALIGN_MIN")]
                            #[doc = "Align the time series by returning the minimum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignMin,
                            #[serde(rename = "ALIGN_MAX")]
                            #[doc = "Align the time series by returning the maximum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignMax,
                            #[serde(rename = "ALIGN_MEAN")]
                            #[doc = "Align the time series by returning the mean value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is DOUBLE."]
                            AlignMean,
                            #[serde(rename = "ALIGN_COUNT")]
                            #[doc = "Align the time series by returning the number of values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric or Boolean values. The value_type of the aligned result is INT64."]
                            AlignCount,
                            #[serde(rename = "ALIGN_SUM")]
                            #[doc = "Align the time series by returning the sum of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric and distribution values. The value_type of the aligned result is the same as the value_type of the input."]
                            AlignSum,
                            #[serde(rename = "ALIGN_STDDEV")]
                            #[doc = "Align the time series by returning the standard deviation of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the output is DOUBLE."]
                            AlignStddev,
                            #[serde(rename = "ALIGN_COUNT_TRUE")]
                            #[doc = "Align the time series by returning the number of True values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64."]
                            AlignCountTrue,
                            #[serde(rename = "ALIGN_COUNT_FALSE")]
                            #[doc = "Align the time series by returning the number of False values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64."]
                            AlignCountFalse,
                            #[serde(rename = "ALIGN_FRACTION_TRUE")]
                            #[doc = "Align the time series by returning the ratio of the number of True values to the total number of values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The output value is in the range 0.0, 1.0 and has value_type DOUBLE."]
                            AlignFractionTrue,
                            #[serde(rename = "ALIGN_PERCENTILE_99")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 99th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile99,
                            #[serde(rename = "ALIGN_PERCENTILE_95")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 95th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile95,
                            #[serde(rename = "ALIGN_PERCENTILE_50")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 50th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile50,
                            #[serde(rename = "ALIGN_PERCENTILE_05")]
                            #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 5th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentile05,
                            #[serde(rename = "ALIGN_PERCENT_CHANGE")]
                            #[doc = "Align and convert to a percentage change. This aligner is valid for GAUGE and DELTA metrics with numeric values. This alignment returns ((current - previous)/previous) * 100, where the value of previous is determined based on the alignment_period.If the values of current and previous are both 0, then the returned value is 0. If only previous is 0, the returned value is infinity.A 10-minute moving mean is computed at each point of the alignment period prior to the above calculation to smooth the metric and prevent false positives from very short-lived spikes. The moving mean is only applicable for data whose values are >= 0. Any values < 0 are treated as a missing datapoint, and are ignored. While DELTA metrics are accepted by this alignment, special care should be taken that the values for the metric will always be positive. The output is a GAUGE metric with value_type DOUBLE."]
                            AlignPercentChange,
                        }
                        impl ::std::default::Default for QueryParametersSecondaryAggregationPerSeriesAlignerEnum {
                            fn default() -> Self {
                                Self::AlignNone
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Required. Specifies which information is returned about the time series."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "FULL")]
                            #[doc = "Returns the identity of the metric(s), the time series, and the time series data."]
                            Full,
                            #[serde(rename = "HEADERS")]
                            #[doc = "Returns the identity of the metric and the time series resource, but not the time series data."]
                            Headers,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::Full
                            }
                        }
                    }
                }
            }
            pub mod uptime_check_configs {
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
                            #[doc = "The maximum number of results to return in a single response. The server may further constrain the maximum number of results returned in a single page. If the page_size is <=0, the server will decide the number of results to be returned."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If this field is not empty then it must contain the nextPageToken value returned by a previous call to this method. Using this field causes the method to return more results from the previous method call."]
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
                            #[doc = "Optional. If present, only the listed fields in the current Uptime check configuration are updated with values from the new configuration. If this field is empty, then the current configuration is completely replaced with the new configuration."]
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
    pub mod services {
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "serviceId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. The Service id to use for this Service. If omitted, an id will be generated instead. Must match the pattern [a-z0-9\\-]+"]
                    pub service_id: ::std::option::Option<::std::string::String>,
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
                    #[doc = "A filter specifying what Services to return. The filter currently supports the following fields: - `identifier_case` - `app_engine.module_id` - `cloud_endpoints.service` (reserved for future use) - `mesh_istio.mesh_uid` - `mesh_istio.service_namespace` - `mesh_istio.service_name` - `cluster_istio.location` (deprecated) - `cluster_istio.cluster_name` (deprecated) - `cluster_istio.service_namespace` (deprecated) - `cluster_istio.service_name` (deprecated) identifier_case refers to which option in the identifier oneof is populated. For example, the filter identifier_case = \"CUSTOM\" would match all services with a value for the custom field. Valid options are \"CUSTOM\", \"APP_ENGINE\", \"MESH_ISTIO\", plus \"CLUSTER_ISTIO\" (deprecated) and \"CLOUD_ENDPOINTS\" (reserved for future use)."]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A non-negative number that is the maximum number of results to return. When 0, use default page size."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If this field is not empty then it must contain the nextPageToken value returned by a previous call to this method. Using this field causes the method to return additional results from the previous method call."]
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "updateMask")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A set of field paths defining which fields to use for the update."]
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
            pub mod service_level_objectives {
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
                            #[serde(rename = "serviceLevelObjectiveId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The ServiceLevelObjective id to use for this ServiceLevelObjective. If omitted, an id will be generated instead. Must match the pattern [a-z0-9\\-]+"]
                            pub service_level_objective_id:
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
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "View of the ServiceLevelObjective to return. If DEFAULT, return the ServiceLevelObjective as originally defined. If EXPLICIT and the ServiceLevelObjective is defined in terms of a BasicSli, replace the BasicSli with a RequestBasedSli spelling out how the SLI is computed."]
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
                        #[doc = "View of the ServiceLevelObjective to return. If DEFAULT, return the ServiceLevelObjective as originally defined. If EXPLICIT and the ServiceLevelObjective is defined in terms of a BasicSli, replace the BasicSli with a RequestBasedSli spelling out how the SLI is computed."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "VIEW_UNSPECIFIED")]
                            #[doc = "Same as FULL."]
                            ViewUnspecified,
                            #[serde(rename = "FULL")]
                            #[doc = "Return the embedded ServiceLevelIndicator in the form in which it was defined. If it was defined using a BasicSli, return that BasicSli."]
                            Full,
                            #[serde(rename = "EXPLICIT")]
                            #[doc = "For ServiceLevelIndicators using BasicSli articulation, instead return the ServiceLevelIndicator with its mode of computation fully spelled out as a RequestBasedSli. For ServiceLevelIndicators using RequestBasedSli or WindowsBasedSli, return the ServiceLevelIndicator as it was provided."]
                            Explicit,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::ViewUnspecified
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
                            #[doc = "A filter specifying what ServiceLevelObjectives to return."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A non-negative number that is the maximum number of results to return. When 0, use default page size."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If this field is not empty then it must contain the nextPageToken value returned by a previous call to this method. Using this field causes the method to return additional results from the previous method call."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "View of the ServiceLevelObjectives to return. If DEFAULT, return each ServiceLevelObjective as originally defined. If EXPLICIT and the ServiceLevelObjective is defined in terms of a BasicSli, replace the BasicSli with a RequestBasedSli spelling out how the SLI is computed."]
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
                        #[doc = "View of the ServiceLevelObjectives to return. If DEFAULT, return each ServiceLevelObjective as originally defined. If EXPLICIT and the ServiceLevelObjective is defined in terms of a BasicSli, replace the BasicSli with a RequestBasedSli spelling out how the SLI is computed."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "VIEW_UNSPECIFIED")]
                            #[doc = "Same as FULL."]
                            ViewUnspecified,
                            #[serde(rename = "FULL")]
                            #[doc = "Return the embedded ServiceLevelIndicator in the form in which it was defined. If it was defined using a BasicSli, return that BasicSli."]
                            Full,
                            #[serde(rename = "EXPLICIT")]
                            #[doc = "For ServiceLevelIndicators using BasicSli articulation, instead return the ServiceLevelIndicator with its mode of computation fully spelled out as a RequestBasedSli. For ServiceLevelIndicators using RequestBasedSli or WindowsBasedSli, return the ServiceLevelIndicator as it was provided."]
                            Explicit,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::ViewUnspecified
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
                            #[doc = "A set of field paths defining which fields to use for the update."]
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
    pub mod uptime_check_ips {
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
                    #[doc = "The maximum number of results to return in a single response. The server may further constrain the maximum number of results returned in a single page. If the page_size is <=0, the server will decide the number of results to be returned. NOTE: this field is not yet implemented"]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If this field is not empty then it must contain the nextPageToken value returned by a previous call to this method. Using this field causes the method to return more results from the previous method call. NOTE: this field is not yet implemented"]
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
    #[doc = "Describes how to combine multiple time series to provide a different view of the data. Aggregation of time series is done in two steps. First, each time series in the set is aligned to the same time interval boundaries, then the set of time series is optionally reduced in number.Alignment consists of applying the per_series_aligner operation to each time series after its data has been divided into regular alignment_period time intervals. This process takes all of the data points in an alignment period, applies a mathematical transformation such as averaging, minimum, maximum, delta, etc., and converts them into a single data point per period.Reduction is when the aligned and transformed time series can optionally be combined, reducing the number of time series through similar mathematical transformations. Reduction involves applying a cross_series_reducer to all the time series, optionally sorting the time series into subsets with group_by_fields, and applying the reducer to each subset.The raw time series data can contain a huge amount of information from multiple sources. Alignment and reduction transforms this mass of data into a more manageable and representative collection of data, for example \"the 95% latency across the average of all tasks in a cluster\". This representative data can be more easily graphed and comprehended, and the individual time series data is still available for later drilldown. For more details, see Filtering and aggregation (https://cloud.google.com/monitoring/api/v3/aggregation)."]
    pub struct Aggregation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alignmentPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The alignment_period specifies a time interval, in seconds, that is used to divide the data in all the time series into consistent blocks of time. This will be done before the per-series aligner can be applied to the data.The value must be at least 60 seconds. If a per-series aligner other than ALIGN_NONE is specified, this field is required or an error is returned. If no per-series aligner is specified, or the aligner ALIGN_NONE is specified, then this field is ignored.The maximum value of the alignment_period is 104 weeks (2 years) for charts, and 90,000 seconds (25 hours) for alerting policies."]
        pub alignment_period: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "crossSeriesReducer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned."]
        pub cross_series_reducer: ::std::option::Option<AggregationCrossSeriesReducerEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupByFields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of fields to preserve when cross_series_reducer is specified. The group_by_fields determine how the time series are partitioned into subsets prior to applying the aggregation operation. Each subset contains time series that have the same value for each of the grouping fields. Each individual time series is a member of exactly one subset. The cross_series_reducer is applied to each subset of time series. It is not possible to reduce across different resource types, so this field implicitly contains resource.type. Fields not specified in group_by_fields are aggregated away. If group_by_fields is not specified and all the time series have the same resource type, then the time series are aggregated into a single output time series. If cross_series_reducer is not defined, this field is ignored."]
        pub group_by_fields: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "perSeriesAligner")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned."]
        pub per_series_aligner: ::std::option::Option<AggregationPerSeriesAlignerEnum>,
    }
    impl Aggregation {
        pub fn builder() -> AggregationBuilder {
            AggregationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned."]
    pub enum AggregationCrossSeriesReducerEnum {
        #[serde(rename = "REDUCE_NONE")]
        #[doc = "No cross-time series reduction. The output of the Aligner is returned."]
        ReduceNone,
        #[serde(rename = "REDUCE_MEAN")]
        #[doc = "Reduce by computing the mean value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE."]
        ReduceMean,
        #[serde(rename = "REDUCE_MIN")]
        #[doc = "Reduce by computing the minimum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input."]
        ReduceMin,
        #[serde(rename = "REDUCE_MAX")]
        #[doc = "Reduce by computing the maximum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input."]
        ReduceMax,
        #[serde(rename = "REDUCE_SUM")]
        #[doc = "Reduce by computing the sum across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric and distribution values. The value_type of the output is the same as the value_type of the input."]
        ReduceSum,
        #[serde(rename = "REDUCE_STDDEV")]
        #[doc = "Reduce by computing the standard deviation across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE."]
        ReduceStddev,
        #[serde(rename = "REDUCE_COUNT")]
        #[doc = "Reduce by computing the number of data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of numeric, Boolean, distribution, and string value_type. The value_type of the output is INT64."]
        ReduceCount,
        #[serde(rename = "REDUCE_COUNT_TRUE")]
        #[doc = "Reduce by computing the number of True-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64."]
        ReduceCountTrue,
        #[serde(rename = "REDUCE_COUNT_FALSE")]
        #[doc = "Reduce by computing the number of False-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64."]
        ReduceCountFalse,
        #[serde(rename = "REDUCE_FRACTION_TRUE")]
        #[doc = "Reduce by computing the ratio of the number of True-valued data points to the total number of data points for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The output value is in the range 0.0, 1.0 and has value_type DOUBLE."]
        ReduceFractionTrue,
        #[serde(rename = "REDUCE_PERCENTILE_99")]
        #[doc = "Reduce by computing the 99th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
        ReducePercentile99,
        #[serde(rename = "REDUCE_PERCENTILE_95")]
        #[doc = "Reduce by computing the 95th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
        ReducePercentile95,
        #[serde(rename = "REDUCE_PERCENTILE_50")]
        #[doc = "Reduce by computing the 50th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
        ReducePercentile50,
        #[serde(rename = "REDUCE_PERCENTILE_05")]
        #[doc = "Reduce by computing the 5th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE."]
        ReducePercentile05,
    }
    impl ::std::default::Default for AggregationCrossSeriesReducerEnum {
        fn default() -> Self {
            Self::ReduceNone
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned."]
    pub enum AggregationPerSeriesAlignerEnum {
        #[serde(rename = "ALIGN_NONE")]
        #[doc = "No alignment. Raw data is returned. Not valid if cross-series reduction is requested. The value_type of the result is the same as the value_type of the input."]
        AlignNone,
        #[serde(rename = "ALIGN_DELTA")]
        #[doc = "Align and convert to DELTA. The output is delta = y1 - y0.This alignment is valid for CUMULATIVE and DELTA metrics. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The value_type of the aligned result is the same as the value_type of the input."]
        AlignDelta,
        #[serde(rename = "ALIGN_RATE")]
        #[doc = "Align and convert to a rate. The result is computed as rate = (y1 - y0)/(t1 - t0), or \"delta over time\". Think of this aligner as providing the slope of the line that passes through the value at the start and at the end of the alignment_period.This aligner is valid for CUMULATIVE and DELTA metrics with numeric values. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The output is a GAUGE metric with value_type DOUBLE.If, by \"rate\", you mean \"percentage change\", see the ALIGN_PERCENT_CHANGE aligner instead."]
        AlignRate,
        #[serde(rename = "ALIGN_INTERPOLATE")]
        #[doc = "Align by interpolating between adjacent points around the alignment period boundary. This aligner is valid for GAUGE metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
        AlignInterpolate,
        #[serde(rename = "ALIGN_NEXT_OLDER")]
        #[doc = "Align by moving the most recent data point before the end of the alignment period to the boundary at the end of the alignment period. This aligner is valid for GAUGE metrics. The value_type of the aligned result is the same as the value_type of the input."]
        AlignNextOlder,
        #[serde(rename = "ALIGN_MIN")]
        #[doc = "Align the time series by returning the minimum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
        AlignMin,
        #[serde(rename = "ALIGN_MAX")]
        #[doc = "Align the time series by returning the maximum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input."]
        AlignMax,
        #[serde(rename = "ALIGN_MEAN")]
        #[doc = "Align the time series by returning the mean value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is DOUBLE."]
        AlignMean,
        #[serde(rename = "ALIGN_COUNT")]
        #[doc = "Align the time series by returning the number of values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric or Boolean values. The value_type of the aligned result is INT64."]
        AlignCount,
        #[serde(rename = "ALIGN_SUM")]
        #[doc = "Align the time series by returning the sum of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric and distribution values. The value_type of the aligned result is the same as the value_type of the input."]
        AlignSum,
        #[serde(rename = "ALIGN_STDDEV")]
        #[doc = "Align the time series by returning the standard deviation of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the output is DOUBLE."]
        AlignStddev,
        #[serde(rename = "ALIGN_COUNT_TRUE")]
        #[doc = "Align the time series by returning the number of True values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64."]
        AlignCountTrue,
        #[serde(rename = "ALIGN_COUNT_FALSE")]
        #[doc = "Align the time series by returning the number of False values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64."]
        AlignCountFalse,
        #[serde(rename = "ALIGN_FRACTION_TRUE")]
        #[doc = "Align the time series by returning the ratio of the number of True values to the total number of values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The output value is in the range 0.0, 1.0 and has value_type DOUBLE."]
        AlignFractionTrue,
        #[serde(rename = "ALIGN_PERCENTILE_99")]
        #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 99th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
        AlignPercentile99,
        #[serde(rename = "ALIGN_PERCENTILE_95")]
        #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 95th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
        AlignPercentile95,
        #[serde(rename = "ALIGN_PERCENTILE_50")]
        #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 50th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
        AlignPercentile50,
        #[serde(rename = "ALIGN_PERCENTILE_05")]
        #[doc = "Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 5th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE."]
        AlignPercentile05,
        #[serde(rename = "ALIGN_PERCENT_CHANGE")]
        #[doc = "Align and convert to a percentage change. This aligner is valid for GAUGE and DELTA metrics with numeric values. This alignment returns ((current - previous)/previous) * 100, where the value of previous is determined based on the alignment_period.If the values of current and previous are both 0, then the returned value is 0. If only previous is 0, the returned value is infinity.A 10-minute moving mean is computed at each point of the alignment period prior to the above calculation to smooth the metric and prevent false positives from very short-lived spikes. The moving mean is only applicable for data whose values are >= 0. Any values < 0 are treated as a missing datapoint, and are ignored. While DELTA metrics are accepted by this alignment, special care should be taken that the values for the metric will always be positive. The output is a GAUGE metric with value_type DOUBLE."]
        AlignPercentChange,
    }
    impl ::std::default::Default for AggregationPerSeriesAlignerEnum {
        fn default() -> Self {
            Self::AlignNone
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A description of the conditions under which some aspect of your system is considered to be \"unhealthy\" and the ways to notify people or services about this state. For an overview of alert policies, see Introduction to Alerting (https://cloud.google.com/monitoring/alerts/)."]
    pub struct AlertPolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "combiner")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How to combine the results of multiple conditions to determine if an incident should be opened. If condition_time_series_query_language is present, this must be COMBINE_UNSPECIFIED."]
        pub combiner: ::std::option::Option<AlertPolicyCombinerEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conditions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of conditions for the policy. The conditions are combined by AND or OR according to the combiner field. If the combined conditions evaluate to true, then an incident is created. A policy can have from one to six conditions. If condition_time_series_query_language is present, it must be the only condition."]
        pub conditions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Condition>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationRecord")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A read-only record of the creation of the alerting policy. If provided in a call to create or update, this field will be ignored."]
        pub creation_record: ::std::option::Option<::std::boxed::Box<MutationRecord>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short name or phrase used to identify the policy in dashboards, notifications, and incidents. To avoid confusion, don't use the same display name for multiple policies in the same project. The name is limited to 512 Unicode characters."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "documentation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Documentation that is included with notifications and incidents related to this policy. Best practice is for the documentation to include information to help responders understand, mitigate, escalate, and correct the underlying problems detected by the alerting policy. Notification channels that have limited capacity might not show this documentation."]
        pub documentation: ::std::option::Option<::std::boxed::Box<Documentation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not the policy is enabled. On write, the default interpretation if unset is that the policy is enabled. On read, clients should not make any assumption about the state if it has not been populated. The field should always be populated on List and Get operations, unless a field projection has been specified that strips it out."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mutationRecord")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A read-only record of the most recent change to the alerting policy. If provided in a call to create or update, this field will be ignored."]
        pub mutation_record: ::std::option::Option<::std::boxed::Box<MutationRecord>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required if the policy exists. The resource name for this policy. The format is: projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[ALERT_POLICY_ID] [ALERT_POLICY_ID] is assigned by Stackdriver Monitoring when the policy is created. When calling the alertPolicies.create method, do not include the name field in the alerting policy passed as part of the request."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notificationChannels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies the notification channels to which notifications should be sent when incidents are opened or closed or when new violations occur on an already opened incident. Each element of this array corresponds to the name field in each of the NotificationChannel objects that are returned from the ListNotificationChannels method. The format of the entries in this field is: projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID] "]
        pub notification_channels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-supplied key/value data to be used for organizing and identifying the AlertPolicy objects.The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter."]
        pub user_labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Read-only description of how the alert policy is invalid. OK if the alert policy is valid. If not OK, the alert policy will not generate incidents."]
        pub validity: ::std::option::Option<::std::boxed::Box<Status>>,
    }
    impl AlertPolicy {
        pub fn builder() -> AlertPolicyBuilder {
            AlertPolicyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How to combine the results of multiple conditions to determine if an incident should be opened. If condition_time_series_query_language is present, this must be COMBINE_UNSPECIFIED."]
    pub enum AlertPolicyCombinerEnum {
        #[serde(rename = "COMBINE_UNSPECIFIED")]
        #[doc = "An unspecified combiner."]
        CombineUnspecified,
        #[serde(rename = "AND")]
        #[doc = "Combine conditions using the logical AND operator. An incident is created only if all the conditions are met simultaneously. This combiner is satisfied if all conditions are met, even if they are met on completely different resources."]
        And,
        #[serde(rename = "OR")]
        #[doc = "Combine conditions using the logical OR operator. An incident is created if any of the listed conditions is met."]
        Or,
        #[serde(rename = "AND_WITH_MATCHING_RESOURCE")]
        #[doc = "Combine conditions using logical AND operator, but unlike the regular AND option, an incident is created only if all conditions are met simultaneously on at least one resource."]
        AndWithMatchingResource,
    }
    impl ::std::default::Default for AlertPolicyCombinerEnum {
        fn default() -> Self {
            Self::CombineUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "App Engine service. Learn more at https://cloud.google.com/appengine."]
    pub struct AppEngine {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "moduleId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the App Engine module underlying this service. Corresponds to the module_id resource label in the gae_app monitored resource: https://cloud.google.com/monitoring/api/resources#tag_gae_app"]
        pub module_id: ::std::option::Option<::std::string::String>,
    }
    impl AppEngine {
        pub fn builder() -> AppEngineBuilder {
            AppEngineBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Future parameters for the availability SLI."]
    pub struct AvailabilityCriteria {}
    impl AvailabilityCriteria {
        pub fn builder() -> AvailabilityCriteriaBuilder {
            AvailabilityCriteriaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The authentication parameters to provide to the specified resource or URL that requires a username and password. Currently, only Basic HTTP authentication (https://tools.ietf.org/html/rfc7617) is supported in Uptime checks."]
    pub struct BasicAuthentication {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "password")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The password to use when authenticating with the HTTP server."]
        pub password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The username to use when authenticating with the HTTP server."]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl BasicAuthentication {
        pub fn builder() -> BasicAuthenticationBuilder {
            BasicAuthenticationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An SLI measuring performance on a well-known service type. Performance will be computed on the basis of pre-defined metrics. The type of the service_resource determines the metrics to use and the service_resource.labels and metric_labels are used to construct a monitoring filter to filter that metric down to just the data relevant to this service."]
    pub struct BasicSli {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "availability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Good service is defined to be the count of requests made to this service that return successfully."]
        pub availability: ::std::option::Option<::std::boxed::Box<AvailabilityCriteria>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Good service is defined to be the count of requests made to this service that are fast enough with respect to latency.threshold."]
        pub latency: ::std::option::Option<::std::boxed::Box<LatencyCriteria>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OPTIONAL: The set of locations to which this SLI is relevant. Telemetry from other locations will not be used to calculate performance for this SLI. If omitted, this SLI applies to all locations in which the Service has activity. For service types that don't support breaking down by location, setting this field will result in an error."]
        pub location: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OPTIONAL: The set of RPCs to which this SLI is relevant. Telemetry from other methods will not be used to calculate performance for this SLI. If omitted, this SLI applies to all the Service's methods. For service types that don't support breaking down by method, setting this field will result in an error."]
        pub method: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OPTIONAL: The set of API versions to which this SLI is relevant. Telemetry from other API versions will not be used to calculate performance for this SLI. If omitted, this SLI applies to all API versions. For service types that don't support breaking down by version, setting this field will result in an error."]
        pub version: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl BasicSli {
        pub fn builder() -> BasicSliBuilder {
            BasicSliBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "BucketOptions describes the bucket boundaries used to create a histogram for the distribution. The buckets can be in a linear sequence, an exponential sequence, or each bucket can be specified explicitly. BucketOptions does not include the number of values in each bucket.A bucket has an inclusive lower bound and exclusive upper bound for the values that are counted for that bucket. The upper bound of a bucket must be strictly greater than the lower bound. The sequence of N buckets for a distribution consists of an underflow bucket (number 0), zero or more finite buckets (number 1 through N - 2) and an overflow bucket (number N - 1). The buckets are contiguous: the lower bound of bucket i (i > 0) is the same as the upper bound of bucket i - 1. The buckets span the whole range of finite values: lower bound of the underflow bucket is -infinity and the upper bound of the overflow bucket is +infinity. The finite buckets are so-called because both bounds are finite."]
    pub struct BucketOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explicitBuckets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explicit buckets."]
        pub explicit_buckets: ::std::option::Option<::std::boxed::Box<Explicit>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exponentialBuckets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The exponential buckets."]
        pub exponential_buckets: ::std::option::Option<::std::boxed::Box<Exponential>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linearBuckets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The linear bucket."]
        pub linear_buckets: ::std::option::Option<::std::boxed::Box<Linear>>,
    }
    impl BucketOptions {
        pub fn builder() -> BucketOptionsBuilder {
            BucketOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Cloud Endpoints service. Learn more at https://cloud.google.com/endpoints."]
    pub struct CloudEndpoints {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "service")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the Cloud Endpoints service underlying this service. Corresponds to the service resource label in the api monitored resource: https://cloud.google.com/monitoring/api/resources#tag_api"]
        pub service: ::std::option::Option<::std::string::String>,
    }
    impl CloudEndpoints {
        pub fn builder() -> CloudEndpointsBuilder {
            CloudEndpointsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Istio service scoped to a single Kubernetes cluster. Learn more at https://istio.io. Clusters running OSS Istio will have their services ingested as this type."]
    pub struct ClusterIstio {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the Kubernetes cluster in which this Istio service is defined. Corresponds to the cluster_name resource label in k8s_cluster resources."]
        pub cluster_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location of the Kubernetes cluster in which this Istio service is defined. Corresponds to the location resource label in k8s_cluster resources."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the Istio service underlying this service. Corresponds to the destination_service_name metric label in Istio metrics."]
        pub service_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceNamespace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The namespace of the Istio service underlying this service. Corresponds to the destination_service_namespace metric label in Istio metrics."]
        pub service_namespace: ::std::option::Option<::std::string::String>,
    }
    impl ClusterIstio {
        pub fn builder() -> ClusterIstioBuilder {
            ClusterIstioBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A collection of data points sent from a collectd-based plugin. See the collectd documentation for more information."]
    pub struct CollectdPayload {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end time of the interval."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The measurement metadata. Example: \"process_id\" -> 12345"]
        pub metadata: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<TypedValue>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "plugin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the plugin. Example: \"disk\"."]
        pub plugin: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pluginInstance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The instance name of the plugin Example: \"hdcl\"."]
        pub plugin_instance: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start time of the interval."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The measurement type. Example: \"memory\"."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "typeInstance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The measurement type instance. Example: \"used\"."]
        pub type_instance: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The measured values during this time interval. Each value must have a different data_source_name."]
        pub values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CollectdValue>>>,
    }
    impl CollectdPayload {
        pub fn builder() -> CollectdPayloadBuilder {
            CollectdPayloadBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes the error status for payloads that were not written."]
    pub struct CollectdPayloadError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Records the error status for the payload. If this field is present, the partial errors for nested values won't be populated."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zero-based index in CreateCollectdTimeSeriesRequest.collectd_payloads."]
        pub index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Records the error status for values that were not written due to an error.Failed payloads for which nothing is written will not include partial value errors."]
        pub value_errors:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CollectdValueError>>>,
    }
    impl CollectdPayloadError {
        pub fn builder() -> CollectdPayloadErrorBuilder {
            CollectdPayloadErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single data point from a collectd-based plugin."]
    pub struct CollectdValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data source for the collectd value. For example, there are two data sources for network measurements: \"rx\" and \"tx\"."]
        pub data_source_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of measurement."]
        pub data_source_type: ::std::option::Option<CollectdValueDataSourceTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The measurement value."]
        pub value: ::std::option::Option<::std::boxed::Box<TypedValue>>,
    }
    impl CollectdValue {
        pub fn builder() -> CollectdValueBuilder {
            CollectdValueBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of measurement."]
    pub enum CollectdValueDataSourceTypeEnum {
        #[serde(rename = "UNSPECIFIED_DATA_SOURCE_TYPE")]
        #[doc = "An unspecified data source type. This corresponds to google.api.MetricDescriptor.MetricKind.METRIC_KIND_UNSPECIFIED."]
        UnspecifiedDataSourceType,
        #[serde(rename = "GAUGE")]
        #[doc = "An instantaneous measurement of a varying quantity. This corresponds to google.api.MetricDescriptor.MetricKind.GAUGE."]
        Gauge,
        #[serde(rename = "COUNTER")]
        #[doc = "A cumulative value over time. This corresponds to google.api.MetricDescriptor.MetricKind.CUMULATIVE."]
        Counter,
        #[serde(rename = "DERIVE")]
        #[doc = "A rate of change of the measurement."]
        Derive,
        #[serde(rename = "ABSOLUTE")]
        #[doc = "An amount of change since the last measurement interval. This corresponds to google.api.MetricDescriptor.MetricKind.DELTA."]
        Absolute,
    }
    impl ::std::default::Default for CollectdValueDataSourceTypeEnum {
        fn default() -> Self {
            Self::UnspecifiedDataSourceType
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes the error status for values that were not written."]
    pub struct CollectdValueError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Records the error status for the value."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zero-based index in CollectdPayload.values within the parent CreateCollectdTimeSeriesRequest.collectd_payloads."]
        pub index: ::std::option::Option<::std::primitive::i64>,
    }
    impl CollectdValueError {
        pub fn builder() -> CollectdValueErrorBuilder {
            CollectdValueErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A condition is a true/false test that determines when an alerting policy should open an incident. If a condition evaluates to true, it signifies that something is wrong."]
    pub struct Condition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conditionAbsent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A condition that checks that a time series continues to receive new data points."]
        pub condition_absent: ::std::option::Option<::std::boxed::Box<MetricAbsence>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conditionMonitoringQueryLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A condition that uses the Monitoring Query Language to define alerts."]
        pub condition_monitoring_query_language:
            ::std::option::Option<::std::boxed::Box<MonitoringQueryLanguageCondition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conditionThreshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A condition that compares a time series against a threshold."]
        pub condition_threshold: ::std::option::Option<::std::boxed::Box<MetricThreshold>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short name or phrase used to identify the condition in dashboards, notifications, and incidents. To avoid confusion, don't use the same display name for multiple conditions in the same policy."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required if the condition exists. The unique resource name for this condition. Its format is: projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[POLICY_ID]/conditions/[CONDITION_ID] [CONDITION_ID] is assigned by Stackdriver Monitoring when the condition is created as part of a new or updated alerting policy.When calling the alertPolicies.create method, do not include the name field in the conditions of the requested alerting policy. Stackdriver Monitoring creates the condition identifiers and includes them in the new policy.When calling the alertPolicies.update method to update a policy, including a condition name causes the existing condition to be updated. Conditions without names are added to the updated policy. Existing conditions are deleted if they are not updated.Best practice is to preserve [CONDITION_ID] if you make only small changes, such as those to condition thresholds, durations, or trigger values. Otherwise, treat the change as a new condition and let the existing condition be deleted."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Condition {
        pub fn builder() -> ConditionBuilder {
            ConditionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Optional. Used to perform content matching. This allows matching based on substrings and regular expressions, together with their negations. Only the first 4 MB of an HTTP or HTTPS check's response (and the first 1 MB of a TCP check's response) are examined for purposes of content matching."]
    pub struct ContentMatcher {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "String or regex content to match. Maximum 1024 bytes. An empty content string indicates no content matching is to be performed."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matcher")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of content matcher that will be applied to the server output, compared to the content string when the check is run."]
        pub matcher: ::std::option::Option<ContentMatcherMatcherEnum>,
    }
    impl ContentMatcher {
        pub fn builder() -> ContentMatcherBuilder {
            ContentMatcherBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of content matcher that will be applied to the server output, compared to the content string when the check is run."]
    pub enum ContentMatcherMatcherEnum {
        #[serde(rename = "CONTENT_MATCHER_OPTION_UNSPECIFIED")]
        #[doc = "No content matcher type specified (maintained for backward compatibility, but deprecated for future use). Treated as CONTAINS_STRING."]
        ContentMatcherOptionUnspecified,
        #[serde(rename = "CONTAINS_STRING")]
        #[doc = "Selects substring matching. The match succeeds if the output contains the content string. This is the default value for checks without a matcher option, or where the value of matcher is CONTENT_MATCHER_OPTION_UNSPECIFIED."]
        ContainsString,
        #[serde(rename = "NOT_CONTAINS_STRING")]
        #[doc = "Selects negation of substring matching. The match succeeds if the output does NOT contain the content string."]
        NotContainsString,
        #[serde(rename = "MATCHES_REGEX")]
        #[doc = "Selects regular-expression matching. The match succeeds of the output matches the regular expression specified in the content string."]
        MatchesRegex,
        #[serde(rename = "NOT_MATCHES_REGEX")]
        #[doc = "Selects negation of regular-expression matching. The match succeeds if the output does NOT match the regular expression specified in the content string."]
        NotMatchesRegex,
    }
    impl ::std::default::Default for ContentMatcherMatcherEnum {
        fn default() -> Self {
            Self::ContentMatcherOptionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The CreateCollectdTimeSeries request."]
    pub struct CreateCollectdTimeSeriesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "collectdPayloads")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collectd payloads representing the time series data. You must not include more than a single point for each time series, so no two payloads can have the same values for all of the fields plugin, plugin_instance, type, and type_instance."]
        pub collectd_payloads:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CollectdPayload>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "collectdVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of collectd that collected the data. Example: \"5.3.0-192.el6\"."]
        pub collectd_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The monitored resource associated with the time series."]
        pub resource: ::std::option::Option<::std::boxed::Box<MonitoredResource>>,
    }
    impl CreateCollectdTimeSeriesRequest {
        pub fn builder() -> CreateCollectdTimeSeriesRequestBuilder {
            CreateCollectdTimeSeriesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The CreateCollectdTimeSeries response."]
    pub struct CreateCollectdTimeSeriesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payloadErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Records the error status for points that were not written due to an error in the request.Failed requests for which nothing is written will return an error response instead. Requests where data points were rejected by the backend will set summary instead."]
        pub payload_errors:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CollectdPayloadError>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "summary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aggregate statistics from writing the payloads. This field is omitted if all points were successfully written, so that the response is empty. This is for backwards compatibility with clients that log errors on any non-empty response."]
        pub summary: ::std::option::Option<::std::boxed::Box<CreateTimeSeriesSummary>>,
    }
    impl CreateCollectdTimeSeriesResponse {
        pub fn builder() -> CreateCollectdTimeSeriesResponseBuilder {
            CreateCollectdTimeSeriesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The CreateTimeSeries request."]
    pub struct CreateTimeSeriesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSeries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The new data to be added to a list of time series. Adds at most one data point to each of several time series. The new data point must be more recent than any other point in its time series. Each TimeSeries value must fully specify a unique time series by supplying all label values for the metric and the monitored resource.The maximum number of TimeSeries objects per Create request is 200."]
        pub time_series: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TimeSeries>>>,
    }
    impl CreateTimeSeriesRequest {
        pub fn builder() -> CreateTimeSeriesRequestBuilder {
            CreateTimeSeriesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Summary of the result of a failed request to write data to a time series."]
    pub struct CreateTimeSeriesSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of points that failed to be written. Order is not guaranteed."]
        pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Error>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "successPointCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of points that were successfully written."]
        pub success_point_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalPointCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of points in the request."]
        pub total_point_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl CreateTimeSeriesSummary {
        pub fn builder() -> CreateTimeSeriesSummaryBuilder {
            CreateTimeSeriesSummaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Custom view of service telemetry. Currently a place-holder pending final design."]
    pub struct Custom {}
    impl Custom {
        pub fn builder() -> CustomBuilder {
            CustomBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Distribution contains summary statistics for a population of values. It optionally contains a histogram representing the distribution of those values across a set of buckets.The summary statistics are the count, mean, sum of the squared deviation from the mean, the minimum, and the maximum of the set of population of values. The histogram is based on a sequence of buckets and gives a count of values that fall into each bucket. The boundaries of the buckets are given either explicitly or by formulas for buckets of fixed or exponentially increasing widths.Although it is not forbidden, it is generally a bad idea to include non-finite values (infinities or NaNs) in the population of values, as this will render the mean and sum_of_squared_deviation fields meaningless."]
    pub struct Distribution {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketCounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required in the Cloud Monitoring API v3. The values for each bucket specified in bucket_options. The sum of the values in bucketCounts must equal the value in the count field of the Distribution object. The order of the bucket counts follows the numbering schemes described for the three bucket types. The underflow bucket has number 0; the finite buckets, if any, have numbers 1 through N-2; and the overflow bucket has number N-1. The size of bucket_counts must not be greater than N. If the size is less than N, then the remaining buckets are assigned values of zero."]
        pub bucket_counts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required in the Cloud Monitoring API v3. Defines the histogram bucket boundaries."]
        pub bucket_options: ::std::option::Option<::std::boxed::Box<BucketOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of values in the population. Must be non-negative. This value must equal the sum of the values in bucket_counts if a histogram is provided."]
        pub count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exemplars")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Must be in increasing order of value field."]
        pub exemplars: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Exemplar>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mean")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The arithmetic mean of the values in the population. If count is zero then this field must be zero."]
        pub mean: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, contains the range of the population values. The field must not be present if the count is zero. This field is presently ignored by the Cloud Monitoring API v3."]
        pub range: ::std::option::Option<::std::boxed::Box<Range>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sumOfSquaredDeviation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sum of squared deviations from the mean of the values in the population. For values x_i this is: Sum[i=1..n]((x_i - mean)^2) Knuth, \"The Art of Computer Programming\", Vol. 2, page 232, 3rd edition describes Welford's method for accumulating this sum in one pass.If count is zero then this field must be zero."]
        pub sum_of_squared_deviation: ::std::option::Option<::std::primitive::f64>,
    }
    impl Distribution {
        pub fn builder() -> DistributionBuilder {
            DistributionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A DistributionCut defines a TimeSeries and thresholds used for measuring good service and total service. The TimeSeries must have ValueType = DISTRIBUTION and MetricKind = DELTA or MetricKind = CUMULATIVE. The computed good_service will be the count of values x in the Distribution such that range.min <= x < range.max."]
    pub struct DistributionCut {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "distributionFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) specifying a TimeSeries aggregating values. Must have ValueType = DISTRIBUTION and MetricKind = DELTA or MetricKind = CUMULATIVE."]
        pub distribution_filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Range of values considered \"good.\" For a one-sided range, set one bound to an infinite value."]
        pub range: ::std::option::Option<::std::boxed::Box<GoogleMonitoringV3Range>>,
    }
    impl DistributionCut {
        pub fn builder() -> DistributionCutBuilder {
            DistributionCutBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A content string and a MIME type that describes the content string's format."]
    pub struct Documentation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text of the documentation, interpreted according to mime_type. The content may not exceed 8,192 Unicode characters and may not exceed more than 10,240 bytes when encoded in UTF-8 format, whichever is smaller."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The format of the content field. Presently, only the value \"text/markdown\" is supported. See Markdown (https://en.wikipedia.org/wiki/Markdown) for more information."]
        pub mime_type: ::std::option::Option<::std::string::String>,
    }
    impl Documentation {
        pub fn builder() -> DocumentationBuilder {
            DocumentationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A set of (label, value) pairs that were removed from a Distribution time series during aggregation and then added as an attachment to a Distribution.Exemplar.The full label set for the exemplars is constructed by using the dropped pairs in combination with the label values that remain on the aggregated Distribution time series. The constructed full label set can be used to identify the specific entity, such as the instance or job, which might be contributing to a long-tail. However, with dropped labels, the storage requirements are reduced because only the aggregated distribution values for a large group of time series are stored.Note that there are no guarantees on ordering of the labels from exemplar-to-exemplar and from distribution-to-distribution in the same stream, and there may be duplicates. It is up to clients to resolve any ambiguities."]
    pub struct DroppedLabels {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map from label to its value, for all labels dropped in any aggregation."]
        pub label:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl DroppedLabels {
        pub fn builder() -> DroppedLabelsBuilder {
            DroppedLabelsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for Empty is empty JSON object {}."]
    pub struct Empty {}
    impl Empty {
        pub fn builder() -> EmptyBuilder {
            EmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detailed information about an error category."]
    pub struct Error {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pointCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of points that couldn't be written because of status."]
        pub point_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the requested write operation."]
        pub status: ::std::option::Option<::std::boxed::Box<Status>>,
    }
    impl Error {
        pub fn builder() -> ErrorBuilder {
            ErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Exemplars are example points that may be used to annotate aggregated distribution values. They are metadata that gives information about a particular value added to a Distribution bucket, such as a trace ID that was active when a value was added. They may contain further information, such as a example values and timestamps, origin, etc."]
    pub struct Exemplar {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attachments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contextual information about the example value. Examples are:Trace: type.googleapis.com/google.monitoring.v3.SpanContextLiteral string: type.googleapis.com/google.protobuf.StringValueLabels dropped during aggregation: type.googleapis.com/google.monitoring.v3.DroppedLabelsThere may be only a single attachment of any given message type in a single exemplar, and this is enforced by the system."]
        pub attachments: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The observation (sampling) time of the above value."]
        pub timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of the exemplar point. This value determines to which bucket the exemplar belongs."]
        pub value: ::std::option::Option<::std::primitive::f64>,
    }
    impl Exemplar {
        pub fn builder() -> ExemplarBuilder {
            ExemplarBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies a set of buckets with arbitrary widths.There are size(bounds) + 1 (= N) buckets. Bucket i has the following boundaries:Upper bound (0 <= i < N-1): boundsi Lower bound (1 <= i < N); boundsi - 1The bounds field must contain at least one element. If bounds has only one element, then there are no finite buckets, and that single element is the common boundary of the overflow and underflow buckets."]
    pub struct Explicit {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bounds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values must be monotonically increasing."]
        pub bounds: ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
    }
    impl Explicit {
        pub fn builder() -> ExplicitBuilder {
            ExplicitBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies an exponential sequence of buckets that have a width that is proportional to the value of the lower bound. Each bucket represents a constant relative uncertainty on a specific value in the bucket.There are num_finite_buckets + 2 (= N) buckets. Bucket i has the following boundaries:Upper bound (0 <= i < N-1): scale * (growth_factor ^ i). Lower bound (1 <= i < N): scale * (growth_factor ^ (i - 1))."]
    pub struct Exponential {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "growthFactor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Must be greater than 1."]
        pub growth_factor: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numFiniteBuckets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Must be greater than 0."]
        pub num_finite_buckets: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Must be greater than 0."]
        pub scale: ::std::option::Option<::std::primitive::f64>,
    }
    impl Exponential {
        pub fn builder() -> ExponentialBuilder {
            ExponentialBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single field of a message type."]
    pub struct Field {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cardinality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The field cardinality."]
        pub cardinality: ::std::option::Option<FieldCardinalityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The string value of the default value of this field. Proto2 syntax only."]
        pub default_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jsonName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The field JSON name."]
        pub json_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The field type."]
        pub kind: ::std::option::Option<FieldKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The field name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "number")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The field number."]
        pub number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oneofIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the field type in Type.oneofs, for message or enumeration types. The first type has index 1; zero means the type is not in the list."]
        pub oneof_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "options")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The protocol buffer options."]
        pub options: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Option>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to use alternative packed wire representation."]
        pub packed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "typeUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The field type URL, without the scheme, for message or enumeration types. Example: \"type.googleapis.com/google.protobuf.Timestamp\"."]
        pub type_url: ::std::option::Option<::std::string::String>,
    }
    impl Field {
        pub fn builder() -> FieldBuilder {
            FieldBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The field cardinality."]
    pub enum FieldCardinalityEnum {
        #[serde(rename = "CARDINALITY_UNKNOWN")]
        #[doc = "For fields with unknown cardinality."]
        CardinalityUnknown,
        #[serde(rename = "CARDINALITY_OPTIONAL")]
        #[doc = "For optional fields."]
        CardinalityOptional,
        #[serde(rename = "CARDINALITY_REQUIRED")]
        #[doc = "For required fields. Proto2 syntax only."]
        CardinalityRequired,
        #[serde(rename = "CARDINALITY_REPEATED")]
        #[doc = "For repeated fields."]
        CardinalityRepeated,
    }
    impl ::std::default::Default for FieldCardinalityEnum {
        fn default() -> Self {
            Self::CardinalityUnknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The field type."]
    pub enum FieldKindEnum {
        #[serde(rename = "TYPE_UNKNOWN")]
        #[doc = "Field type unknown."]
        TypeUnknown,
        #[serde(rename = "TYPE_DOUBLE")]
        #[doc = "Field type double."]
        TypeDouble,
        #[serde(rename = "TYPE_FLOAT")]
        #[doc = "Field type float."]
        TypeFloat,
        #[serde(rename = "TYPE_INT64")]
        #[doc = "Field type int64."]
        TypeInt64,
        #[serde(rename = "TYPE_UINT64")]
        #[doc = "Field type uint64."]
        TypeUint64,
        #[serde(rename = "TYPE_INT32")]
        #[doc = "Field type int32."]
        TypeInt32,
        #[serde(rename = "TYPE_FIXED64")]
        #[doc = "Field type fixed64."]
        TypeFixed64,
        #[serde(rename = "TYPE_FIXED32")]
        #[doc = "Field type fixed32."]
        TypeFixed32,
        #[serde(rename = "TYPE_BOOL")]
        #[doc = "Field type bool."]
        TypeBool,
        #[serde(rename = "TYPE_STRING")]
        #[doc = "Field type string."]
        TypeString,
        #[serde(rename = "TYPE_GROUP")]
        #[doc = "Field type group. Proto2 syntax only, and deprecated."]
        TypeGroup,
        #[serde(rename = "TYPE_MESSAGE")]
        #[doc = "Field type message."]
        TypeMessage,
        #[serde(rename = "TYPE_BYTES")]
        #[doc = "Field type bytes."]
        TypeBytes,
        #[serde(rename = "TYPE_UINT32")]
        #[doc = "Field type uint32."]
        TypeUint32,
        #[serde(rename = "TYPE_ENUM")]
        #[doc = "Field type enum."]
        TypeEnum,
        #[serde(rename = "TYPE_SFIXED32")]
        #[doc = "Field type sfixed32."]
        TypeSfixed32,
        #[serde(rename = "TYPE_SFIXED64")]
        #[doc = "Field type sfixed64."]
        TypeSfixed64,
        #[serde(rename = "TYPE_SINT32")]
        #[doc = "Field type sint32."]
        TypeSint32,
        #[serde(rename = "TYPE_SINT64")]
        #[doc = "Field type sint64."]
        TypeSint64,
    }
    impl ::std::default::Default for FieldKindEnum {
        fn default() -> Self {
            Self::TypeUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The GetNotificationChannelVerificationCode request."]
    pub struct GetNotificationChannelVerificationCodeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired expiration time. If specified, the API will guarantee that the returned code will not be valid after the specified timestamp; however, the API cannot guarantee that the returned code will be valid for at least as long as the requested time (the API puts an upper bound on the amount of time for which a code may be valid). If omitted, a default expiration will be used, which may be less than the max permissible expiration (so specifying an expiration may extend the code's lifetime over omitting an expiration, even though the API does impose an upper limit on the maximum expiration that is permitted)."]
        pub expire_time: ::std::option::Option<::std::string::String>,
    }
    impl GetNotificationChannelVerificationCodeRequest {
        pub fn builder() -> GetNotificationChannelVerificationCodeRequestBuilder {
            GetNotificationChannelVerificationCodeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The GetNotificationChannelVerificationCode request."]
    pub struct GetNotificationChannelVerificationCodeResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The verification code, which may be used to verify other channels that have an equivalent identity (i.e. other channels of the same type with the same fingerprint such as other email channels with the same email address or other sms channels with the same number)."]
        pub code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The expiration time associated with the code that was returned. If an expiration was provided in the request, this is the minimum of the requested expiration in the request and the max permitted expiration."]
        pub expire_time: ::std::option::Option<::std::string::String>,
    }
    impl GetNotificationChannelVerificationCodeResponse {
        pub fn builder() -> GetNotificationChannelVerificationCodeResponseBuilder {
            GetNotificationChannelVerificationCodeResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Range of numerical values, inclusive of min and exclusive of max. If the open range \"< range.max\" is desired, set range.min = -infinity. If the open range \">= range.min\" is desired, set range.max = infinity."]
    pub struct GoogleMonitoringV3Range {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "max")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Range maximum."]
        pub max: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "min")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Range minimum."]
        pub min: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleMonitoringV3Range {
        pub fn builder() -> GoogleMonitoringV3RangeBuilder {
            GoogleMonitoringV3RangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The description of a dynamic collection of monitored resources. Each group has a filter that is matched against monitored resources and their associated metadata. If a group's filter matches an available monitored resource, then that resource is a member of that group. Groups can contain any number of monitored resources, and each monitored resource can be a member of any number of groups.Groups can be nested in parent-child hierarchies. The parentName field identifies an optional parent for each group. If a group has a parent, then the only monitored resources available to be matched by the group's filter are the resources contained in the parent group. In other words, a group contains the monitored resources that match its filter and the filters of all the group's ancestors. A group without a parent can contain any monitored resource.For example, consider an infrastructure running a set of instances with two user-defined tags: \"environment\" and \"role\". A parent group has a filter, environment=\"production\". A child of that parent group has a filter, role=\"transcoder\". The parent group contains all instances in the production environment, regardless of their roles. The child group contains instances that have the transcoder role and are in the production environment.The monitored resources contained in a group can change at any moment, depending on what resources exist and what filters are associated with the group and its ancestors."]
    pub struct Group {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A user-assigned name for this group, used only for display purposes."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filter used to determine which monitored resources belong to this group."]
        pub filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isCluster")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, the members of this group are considered to be a cluster. The system can perform additional analysis on groups that are clusters."]
        pub is_cluster: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of this group. The format is: projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID] When creating a group, this field is ignored and a new name is created consisting of the project specified in the call to CreateGroup and a unique [GROUP_ID] that is generated automatically."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the group's parent, if it has one. The format is: projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID] For groups with no parent, parent_name is the empty string, \"\"."]
        pub parent_name: ::std::option::Option<::std::string::String>,
    }
    impl Group {
        pub fn builder() -> GroupBuilder {
            GroupBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information involved in an HTTP/HTTPS Uptime check request."]
    pub struct HttpCheck {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The authentication information. Optional when creating an HTTP check; defaults to empty."]
        pub auth_info: ::std::option::Option<::std::boxed::Box<BasicAuthentication>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "body")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request body associated with the HTTP POST request. If content_type is URL_ENCODED, the body passed in must be URL-encoded. Users can provide a Content-Length header via the headers field or the API will do so. If the request_method is GET and body is not empty, the API will return an error. The maximum byte size is 1 megabyte. Note: As with all bytes fields, JSON representations are base64 encoded. e.g.: \"foo=bar\" in URL-encoded form is \"foo%3Dbar\" and in base64 encoding is \"Zm9vJTI1M0RiYXI=\"."]
        pub body: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content type header to use for the check. The following configurations result in errors: 1. Content type is specified in both the headers field and the content_type field. 2. Request method is GET and content_type is not TYPE_UNSPECIFIED 3. Request method is POST and content_type is TYPE_UNSPECIFIED. 4. Request method is POST and a \"Content-Type\" header is provided via headers field. The content_type field should be used instead."]
        pub content_type: ::std::option::Option<HttpCheckContentTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of headers to send as part of the Uptime check request. If two headers have the same key and different values, they should be entered as a single header, with the value being a comma-separated list of all the desired values as described at https://www.w3.org/Protocols/rfc2616/rfc2616.txt (page 31). Entering two separate headers with the same key in a Create call will cause the first to be overwritten by the second. The maximum number of headers allowed is 100."]
        pub headers:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maskHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boolean specifying whether to encrypt the header information. Encryption should be specified for any headers related to authentication that you do not wish to be seen when retrieving the configuration. The server will be responsible for encrypting the headers. On Get/List calls, if mask_headers is set to true then the headers will be obscured with ******."]
        pub mask_headers: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional (defaults to \"/\"). The path to the page against which to run the check. Will be combined with the host (specified within the monitored_resource) and port to construct the full URL. If the provided path does not begin with \"/\", a \"/\" will be prepended automatically."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "port")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional (defaults to 80 when use_ssl is false, and 443 when use_ssl is true). The TCP port on the HTTP server against which to run the check. Will be combined with host (specified within the monitored_resource) and path to construct the full URL."]
        pub port: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP request method to use for the check. If set to METHOD_UNSPECIFIED then request_method defaults to GET."]
        pub request_method: ::std::option::Option<HttpCheckRequestMethodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useSsl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, use HTTPS instead of HTTP to run the check."]
        pub use_ssl: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validateSsl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boolean specifying whether to include SSL certificate validation as a part of the Uptime check. Only applies to checks where monitored_resource is set to uptime_url. If use_ssl is false, setting validate_ssl to true has no effect."]
        pub validate_ssl: ::std::option::Option<::std::primitive::bool>,
    }
    impl HttpCheck {
        pub fn builder() -> HttpCheckBuilder {
            HttpCheckBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The content type header to use for the check. The following configurations result in errors: 1. Content type is specified in both the headers field and the content_type field. 2. Request method is GET and content_type is not TYPE_UNSPECIFIED 3. Request method is POST and content_type is TYPE_UNSPECIFIED. 4. Request method is POST and a \"Content-Type\" header is provided via headers field. The content_type field should be used instead."]
    pub enum HttpCheckContentTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "No content type specified."]
        TypeUnspecified,
        #[serde(rename = "URL_ENCODED")]
        #[doc = "body is in URL-encoded form. Equivalent to setting the Content-Type to application/x-www-form-urlencoded in the HTTP request."]
        UrlEncoded,
    }
    impl ::std::default::Default for HttpCheckContentTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The HTTP request method to use for the check. If set to METHOD_UNSPECIFIED then request_method defaults to GET."]
    pub enum HttpCheckRequestMethodEnum {
        #[serde(rename = "METHOD_UNSPECIFIED")]
        #[doc = "No request method specified."]
        MethodUnspecified,
        #[serde(rename = "GET")]
        #[doc = "GET request."]
        Get,
        #[serde(rename = "POST")]
        #[doc = "POST request."]
        Post,
    }
    impl ::std::default::Default for HttpCheckRequestMethodEnum {
        fn default() -> Self {
            Self::MethodUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An internal checker allows Uptime checks to run on private/internal GCP resources."]
    pub struct InternalChecker {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The checker's human-readable name. The display name should be unique within a Stackdriver Workspace in order to make it easier to identify; however, uniqueness is not enforced."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcpZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GCP zone the Uptime check should egress from. Only respected for internal Uptime checks, where internal_network is specified."]
        pub gcp_zone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique resource name for this InternalChecker. The format is: projects/[PROJECT_ID_OR_NUMBER]/internalCheckers/[INTERNAL_CHECKER_ID] [PROJECT_ID_OR_NUMBER] is the Stackdriver Workspace project for the Uptime check config associated with the internal checker."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GCP VPC network (https://cloud.google.com/vpc/docs/vpc) where the internal resource lives (ex: \"default\")."]
        pub network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "peerProjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GCP project ID where the internal checker lives. Not necessary the same as the Workspace project."]
        pub peer_project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current operational state of the internal checker."]
        pub state: ::std::option::Option<InternalCheckerStateEnum>,
    }
    impl InternalChecker {
        pub fn builder() -> InternalCheckerBuilder {
            InternalCheckerBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The current operational state of the internal checker."]
    pub enum InternalCheckerStateEnum {
        #[serde(rename = "UNSPECIFIED")]
        #[doc = "An internal checker should never be in the unspecified state."]
        Unspecified,
        #[serde(rename = "CREATING")]
        #[doc = "The checker is being created, provisioned, and configured. A checker in this state can be returned by ListInternalCheckers or GetInternalChecker, as well as by examining the long running Operation (https://cloud.google.com/apis/design/design_patterns#long_running_operations) that created it."]
        Creating,
        #[serde(rename = "RUNNING")]
        #[doc = "The checker is running and available for use. A checker in this state can be returned by ListInternalCheckers or GetInternalChecker as well as by examining the long running Operation (https://cloud.google.com/apis/design/design_patterns#long_running_operations) that created it. If a checker is being torn down, it is neither visible nor usable, so there is no \"deleting\" or \"down\" state."]
        Running,
    }
    impl ::std::default::Default for InternalCheckerStateEnum {
        fn default() -> Self {
            Self::Unspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Canonical service scoped to an Istio mesh. Anthos clusters running ASM >= 1.6.8 will have their services ingested as this type."]
    pub struct IstioCanonicalService {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canonicalService")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the canonical service underlying this service. Corresponds to the destination_canonical_service_name metric label in label in Istio metrics (https://cloud.google.com/monitoring/api/metrics_istio)."]
        pub canonical_service: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canonicalServiceNamespace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The namespace of the canonical service underlying this service. Corresponds to the destination_canonical_service_namespace metric label in Istio metrics (https://cloud.google.com/monitoring/api/metrics_istio)."]
        pub canonical_service_namespace: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meshUid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier for the Istio mesh in which this canonical service is defined. Corresponds to the mesh_uid metric label in Istio metrics (https://cloud.google.com/monitoring/api/metrics_istio)."]
        pub mesh_uid: ::std::option::Option<::std::string::String>,
    }
    impl IstioCanonicalService {
        pub fn builder() -> IstioCanonicalServiceBuilder {
            IstioCanonicalServiceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A description of a label."]
    pub struct LabelDescriptor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A human-readable description for the label."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key for this label. The key must meet the following criteria: Does not exceed 100 characters. Matches the following regular expression: [a-zA-Z][a-zA-Z0-9_]* The first character must be an upper- or lower-case letter. The remaining characters must be letters, digits, or underscores."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of data that can be assigned to the label."]
        pub value_type: ::std::option::Option<LabelDescriptorValueTypeEnum>,
    }
    impl LabelDescriptor {
        pub fn builder() -> LabelDescriptorBuilder {
            LabelDescriptorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of data that can be assigned to the label."]
    pub enum LabelDescriptorValueTypeEnum {
        #[serde(rename = "STRING")]
        #[doc = "A variable-length string, not to exceed 1,024 characters. This is the default value type."]
        String,
        #[serde(rename = "BOOL")]
        #[doc = "Boolean; true or false."]
        Bool,
        #[serde(rename = "INT64")]
        #[doc = "A 64-bit signed integer."]
        Int64,
    }
    impl ::std::default::Default for LabelDescriptorValueTypeEnum {
        fn default() -> Self {
            Self::String
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A label value."]
    pub struct LabelValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boolValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A bool label value."]
        pub bool_value: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "int64Value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An int64 label value."]
        pub int64_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A string label value."]
        pub string_value: ::std::option::Option<::std::string::String>,
    }
    impl LabelValue {
        pub fn builder() -> LabelValueBuilder {
            LabelValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters for a latency threshold SLI."]
    pub struct LatencyCriteria {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Good service is defined to be the count of requests made to this service that return in no more than threshold."]
        pub threshold: ::std::option::Option<::std::string::String>,
    }
    impl LatencyCriteria {
        pub fn builder() -> LatencyCriteriaBuilder {
            LatencyCriteriaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies a linear sequence of buckets that all have the same width (except overflow and underflow). Each bucket represents a constant absolute uncertainty on the specific value in the bucket.There are num_finite_buckets + 2 (= N) buckets. Bucket i has the following boundaries:Upper bound (0 <= i < N-1): offset + (width * i). Lower bound (1 <= i < N): offset + (width * (i - 1))."]
    pub struct Linear {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numFiniteBuckets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Must be greater than 0."]
        pub num_finite_buckets: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Lower bound of the first bucket."]
        pub offset: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Must be greater than 0."]
        pub width: ::std::option::Option<::std::primitive::f64>,
    }
    impl Linear {
        pub fn builder() -> LinearBuilder {
            LinearBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The protocol for the ListAlertPolicies response."]
    pub struct ListAlertPoliciesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alertPolicies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The returned alert policies."]
        pub alert_policies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AlertPolicy>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If there might be more results than were returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of alert policies in all pages. This number is only an estimate, and may change in subsequent pages. https://aip.dev/158"]
        pub total_size: ::std::option::Option<::std::primitive::i64>,
    }
    impl ListAlertPoliciesResponse {
        pub fn builder() -> ListAlertPoliciesResponseBuilder {
            ListAlertPoliciesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The ListGroupMembers response."]
    pub struct ListGroupMembersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "members")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A set of monitored resources in the group."]
        pub members: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MonitoredResource>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of elements matching this request."]
        pub total_size: ::std::option::Option<::std::primitive::i64>,
    }
    impl ListGroupMembersResponse {
        pub fn builder() -> ListGroupMembersResponseBuilder {
            ListGroupMembersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The ListGroups response."]
    pub struct ListGroupsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "group")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The groups that match the specified filters."]
        pub group: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Group>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListGroupsResponse {
        pub fn builder() -> ListGroupsResponseBuilder {
            ListGroupsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The ListMetricDescriptors response."]
    pub struct ListMetricDescriptorsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricDescriptors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metric descriptors that are available to the project and that match the value of filter, if present."]
        pub metric_descriptors:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricDescriptor>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListMetricDescriptorsResponse {
        pub fn builder() -> ListMetricDescriptorsResponseBuilder {
            ListMetricDescriptorsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The ListMonitoredResourceDescriptors response."]
    pub struct ListMonitoredResourceDescriptorsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceDescriptors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The monitored resource descriptors that are available to this project and that match filter, if present."]
        pub resource_descriptors:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MonitoredResourceDescriptor>>>,
    }
    impl ListMonitoredResourceDescriptorsResponse {
        pub fn builder() -> ListMonitoredResourceDescriptorsResponseBuilder {
            ListMonitoredResourceDescriptorsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The ListNotificationChannelDescriptors response."]
    pub struct ListNotificationChannelDescriptorsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelDescriptors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The monitored resource descriptors supported for the specified project, optionally filtered."]
        pub channel_descriptors: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<NotificationChannelDescriptor>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If not empty, indicates that there may be more results that match the request. Use the value in the page_token field in a subsequent request to fetch the next set of results. If empty, all results have been returned."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListNotificationChannelDescriptorsResponse {
        pub fn builder() -> ListNotificationChannelDescriptorsResponseBuilder {
            ListNotificationChannelDescriptorsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The ListNotificationChannels response."]
    pub struct ListNotificationChannelsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If not empty, indicates that there may be more results that match the request. Use the value in the page_token field in a subsequent request to fetch the next set of results. If empty, all results have been returned."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notificationChannels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The notification channels defined for the specified project."]
        pub notification_channels:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NotificationChannel>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of notification channels in all pages. This number is only an estimate, and may change in subsequent pages. https://aip.dev/158"]
        pub total_size: ::std::option::Option<::std::primitive::i64>,
    }
    impl ListNotificationChannelsResponse {
        pub fn builder() -> ListNotificationChannelsResponseBuilder {
            ListNotificationChannelsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The ListServiceLevelObjectives response."]
    pub struct ListServiceLevelObjectivesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceLevelObjectives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ServiceLevelObjectives matching the specified filter."]
        pub service_level_objectives:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ServiceLevelObjective>>>,
    }
    impl ListServiceLevelObjectivesResponse {
        pub fn builder() -> ListServiceLevelObjectivesResponseBuilder {
            ListServiceLevelObjectivesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The ListServices response."]
    pub struct ListServicesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "services")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Services matching the specified filter."]
        pub services: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Service>>>,
    }
    impl ListServicesResponse {
        pub fn builder() -> ListServicesResponseBuilder {
            ListServicesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The ListTimeSeries response."]
    pub struct ListTimeSeriesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Query execution errors that may have caused the time series data returned to be incomplete."]
        pub execution_errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Status>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSeries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "One or more time series that match the filter included in the request."]
        pub time_series: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TimeSeries>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unit in which all time_series point values are reported. unit follows the UCUM format for units as seen in https://unitsofmeasure.org/ucum.html. If different time_series have different units (for example, because they come from different metric types, or a unit is absent), then unit will be \"{not_a_unit}\"."]
        pub unit: ::std::option::Option<::std::string::String>,
    }
    impl ListTimeSeriesResponse {
        pub fn builder() -> ListTimeSeriesResponseBuilder {
            ListTimeSeriesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The protocol for the ListUptimeCheckConfigs response."]
    pub struct ListUptimeCheckConfigsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field represents the pagination token to retrieve the next page of results. If the value is empty, it means no further results for the request. To retrieve the next page of results, the value of the next_page_token is passed to the subsequent List method call (in the request message's page_token field)."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of Uptime check configurations for the project, irrespective of any pagination."]
        pub total_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uptimeCheckConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The returned Uptime check configurations."]
        pub uptime_check_configs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UptimeCheckConfig>>>,
    }
    impl ListUptimeCheckConfigsResponse {
        pub fn builder() -> ListUptimeCheckConfigsResponseBuilder {
            ListUptimeCheckConfigsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The protocol for the ListUptimeCheckIps response."]
    pub struct ListUptimeCheckIpsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field represents the pagination token to retrieve the next page of results. If the value is empty, it means no further results for the request. To retrieve the next page of results, the value of the next_page_token is passed to the subsequent List method call (in the request message's page_token field). NOTE: this field is not yet implemented"]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uptimeCheckIps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The returned list of IP addresses (including region and location) that the checkers run from."]
        pub uptime_check_ips:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UptimeCheckIp>>>,
    }
    impl ListUptimeCheckIpsResponse {
        pub fn builder() -> ListUptimeCheckIpsResponseBuilder {
            ListUptimeCheckIpsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Istio service scoped to an Istio mesh. Anthos clusters running ASM < 1.6.8 will have their services ingested as this type."]
    pub struct MeshIstio {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meshUid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier for the mesh in which this Istio service is defined. Corresponds to the mesh_uid metric label in Istio metrics."]
        pub mesh_uid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the Istio service underlying this service. Corresponds to the destination_service_name metric label in Istio metrics."]
        pub service_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceNamespace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The namespace of the Istio service underlying this service. Corresponds to the destination_service_namespace metric label in Istio metrics."]
        pub service_namespace: ::std::option::Option<::std::string::String>,
    }
    impl MeshIstio {
        pub fn builder() -> MeshIstioBuilder {
            MeshIstioBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A specific metric, identified by specifying values for all of the labels of a MetricDescriptor."]
    pub struct Metric {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of label values that uniquely identify this metric. All labels listed in the MetricDescriptor must be assigned values."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An existing metric type, see google.api.MetricDescriptor. For example, custom.googleapis.com/invoice/paid/amount."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl Metric {
        pub fn builder() -> MetricBuilder {
            MetricBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A condition type that checks that monitored resources are reporting data. The configuration defines a metric and a set of monitored resources. The predicate is considered in violation when a time series for the specified metric of a monitored resource does not include any data in the specified duration."]
    pub struct MetricAbsence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aggregations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the alignment of data points in individual time series as well as how to combine the retrieved time series together (such as when aggregating multiple streams on each resource to a single stream for each resource or when aggregating streams across all members of a group of resrouces). Multiple aggregations are applied in the order specified.This field is similar to the one in the ListTimeSeries request (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list). It is advisable to use the ListTimeSeries method when debugging this field."]
        pub aggregations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Aggregation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of time that a time series must fail to report new data to be considered failing. The minimum value of this field is 120 seconds. Larger values that are a multiple of a minute--for example, 240 or 300 seconds--are supported. If an invalid value is given, an error will be returned. The Duration.nanos field is ignored."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A filter (https://cloud.google.com/monitoring/api/v3/filters) that identifies which time series should be compared with the threshold.The filter is similar to the one that is specified in the ListTimeSeries request (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list) (that call is useful to verify the time series that will be retrieved / processed). The filter must specify the metric type and the resource type. Optionally, it can specify resource labels and metric labels. This field must not exceed 2048 Unicode characters in length."]
        pub filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trigger")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number/percent of time series for which the comparison must hold in order for the condition to trigger. If unspecified, then the condition will trigger if the comparison is true for any of the time series that have been identified by filter and aggregations."]
        pub trigger: ::std::option::Option<::std::boxed::Box<Trigger>>,
    }
    impl MetricAbsence {
        pub fn builder() -> MetricAbsenceBuilder {
            MetricAbsenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines a metric type and its schema. Once a metric descriptor is created, deleting or altering it stops data collection and makes the metric type's existing data unusable."]
    pub struct MetricDescriptor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A detailed description of the metric, which can be used in documentation."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A concise name for the metric, which can be displayed in user interfaces. Use sentence case without an ending period, for example \"Request count\". This field is optional but it is recommended to be set for any metrics associated with user-visible concepts, such as Quota."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of labels that can be used to describe a specific instance of this metric type. For example, the appengine.googleapis.com/http/server/response_latencies metric type has a label for the HTTP response code, response_code, so you can look at latencies for successful responses or just for responses that failed."]
        pub labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LabelDescriptor>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "launchStage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The launch stage of the metric definition."]
        pub launch_stage: ::std::option::Option<MetricDescriptorLaunchStageEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Metadata which can be used to guide usage of the metric."]
        pub metadata: ::std::option::Option<::std::boxed::Box<MetricDescriptorMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricKind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the metric records instantaneous values, changes to a value, etc. Some combinations of metric_kind and value_type might not be supported."]
        pub metric_kind: ::std::option::Option<MetricDescriptorMetricKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "monitoredResourceTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Read-only. If present, then a time series, which is identified partially by a metric type and a MonitoredResourceDescriptor, that is associated with this metric type can only be associated with one of the monitored resource types listed here."]
        pub monitored_resource_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the metric descriptor."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metric type, including its DNS name prefix. The type is not URL-encoded. All user-defined metric types have the DNS name custom.googleapis.com or external.googleapis.com. Metric types should use a natural hierarchical grouping. For example: \"custom.googleapis.com/invoice/paid/amount\" \"external.googleapis.com/prometheus/up\" \"appengine.googleapis.com/http/server/response_latencies\" "]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The units in which the metric value is reported. It is only applicable if the value_type is INT64, DOUBLE, or DISTRIBUTION. The unit defines the representation of the stored metric values.Different systems might scale the values to be more easily displayed (so a value of 0.02kBy might be displayed as 20By, and a value of 3523kBy might be displayed as 3.5MBy). However, if the unit is kBy, then the value of the metric is always in thousands of bytes, no matter how it might be displayed.If you want a custom metric to record the exact number of CPU-seconds used by a job, you can create an INT64 CUMULATIVE metric whose unit is s{CPU} (or equivalently 1s{CPU} or just s). If the job uses 12,005 CPU-seconds, then the value is written as 12005.Alternatively, if you want a custom metric to record data in a more granular way, you can create a DOUBLE CUMULATIVE metric whose unit is ks{CPU}, and then write the value 12.005 (which is 12005/1000), or use Kis{CPU} and write 11.723 (which is 12005/1024).The supported units are a subset of The Unified Code for Units of Measure (https://unitsofmeasure.org/ucum.html) standard:Basic units (UNIT) bit bit By byte s second min minute h hour d day 1 dimensionlessPrefixes (PREFIX) k kilo (10^3) M mega (10^6) G giga (10^9) T tera (10^12) P peta (10^15) E exa (10^18) Z zetta (10^21) Y yotta (10^24) m milli (10^-3) u micro (10^-6) n nano (10^-9) p pico (10^-12) f femto (10^-15) a atto (10^-18) z zepto (10^-21) y yocto (10^-24) Ki kibi (2^10) Mi mebi (2^20) Gi gibi (2^30) Ti tebi (2^40) Pi pebi (2^50)GrammarThe grammar also includes these connectors: / division or ratio (as an infix operator). For examples, kBy/{email} or MiBy/10ms (although you should almost never have /s in a metric unit; rates should always be computed at query time from the underlying cumulative or delta value). . multiplication or composition (as an infix operator). For examples, GBy.d or k{watt}.h.The grammar for a unit is as follows: Expression = Component { \".\" Component } { \"/\" Component } ; Component = ( [ PREFIX ] UNIT | \"%\" ) [ Annotation ] | Annotation | \"1\" ; Annotation = \"{\" NAME \"}\" ; Notes: Annotation is just a comment if it follows a UNIT. If the annotation is used alone, then the unit is equivalent to 1. For examples, {request}/s == 1/s, By{transmitted}/s == By/s. NAME is a sequence of non-blank printable ASCII characters not containing { or }. 1 represents a unitary dimensionless unit (https://en.wikipedia.org/wiki/Dimensionless_quantity) of 1, such as in 1/s. It is typically used when none of the basic units are appropriate. For example, \"new users per day\" can be represented as 1/d or {new-users}/d (and a metric value 5 would mean \"5 new users). Alternatively, \"thousands of page views per day\" would be represented as 1000/d or k1/d or k{page_views}/d (and a metric value of 5.3 would mean \"5300 page views per day\"). % represents dimensionless value of 1/100, and annotates values giving a percentage (so the metric values are typically in the range of 0..100, and a metric value 3 means \"3 percent\"). 10^2.% indicates a metric contains a ratio, typically in the range 0..1, that will be multiplied by 100 and displayed as a percentage (so a metric value 0.03 means \"3 percent\")."]
        pub unit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the measurement is an integer, a floating-point number, etc. Some combinations of metric_kind and value_type might not be supported."]
        pub value_type: ::std::option::Option<MetricDescriptorValueTypeEnum>,
    }
    impl MetricDescriptor {
        pub fn builder() -> MetricDescriptorBuilder {
            MetricDescriptorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The launch stage of the metric definition."]
    pub enum MetricDescriptorLaunchStageEnum {
        #[serde(rename = "LAUNCH_STAGE_UNSPECIFIED")]
        #[doc = "Do not use this default value."]
        LaunchStageUnspecified,
        #[serde(rename = "UNIMPLEMENTED")]
        #[doc = "The feature is not yet implemented. Users can not use it."]
        Unimplemented,
        #[serde(rename = "PRELAUNCH")]
        #[doc = "Prelaunch features are hidden from users and are only visible internally."]
        Prelaunch,
        #[serde(rename = "EARLY_ACCESS")]
        #[doc = "Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released."]
        EarlyAccess,
        #[serde(rename = "ALPHA")]
        #[doc = "Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don’t have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases."]
        Alpha,
        #[serde(rename = "BETA")]
        #[doc = "Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases."]
        Beta,
        #[serde(rename = "GA")]
        #[doc = "GA features are open to all developers and are considered stable and fully qualified for production use."]
        Ga,
        #[serde(rename = "DEPRECATED")]
        #[doc = "Deprecated features are scheduled to be shut down and removed. For more information, see the “Deprecation Policy” section of our Terms of Service (https://cloud.google.com/terms/) and the Google Cloud Platform Subject to the Deprecation Policy (https://cloud.google.com/terms/deprecation) documentation."]
        Deprecated,
    }
    impl ::std::default::Default for MetricDescriptorLaunchStageEnum {
        fn default() -> Self {
            Self::LaunchStageUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Whether the metric records instantaneous values, changes to a value, etc. Some combinations of metric_kind and value_type might not be supported."]
    pub enum MetricDescriptorMetricKindEnum {
        #[serde(rename = "METRIC_KIND_UNSPECIFIED")]
        #[doc = "Do not use this default value."]
        MetricKindUnspecified,
        #[serde(rename = "GAUGE")]
        #[doc = "An instantaneous measurement of a value."]
        Gauge,
        #[serde(rename = "DELTA")]
        #[doc = "The change in a value during a time interval."]
        Delta,
        #[serde(rename = "CUMULATIVE")]
        #[doc = "A value accumulated over a time interval. Cumulative measurements in a time series should have the same start time and increasing end times, until an event resets the cumulative value to zero and sets a new start time for the following points."]
        Cumulative,
    }
    impl ::std::default::Default for MetricDescriptorMetricKindEnum {
        fn default() -> Self {
            Self::MetricKindUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Whether the measurement is an integer, a floating-point number, etc. Some combinations of metric_kind and value_type might not be supported."]
    pub enum MetricDescriptorValueTypeEnum {
        #[serde(rename = "VALUE_TYPE_UNSPECIFIED")]
        #[doc = "Do not use this default value."]
        ValueTypeUnspecified,
        #[serde(rename = "BOOL")]
        #[doc = "The value is a boolean. This value type can be used only if the metric kind is GAUGE."]
        Bool,
        #[serde(rename = "INT64")]
        #[doc = "The value is a signed 64-bit integer."]
        Int64,
        #[serde(rename = "DOUBLE")]
        #[doc = "The value is a double precision floating point number."]
        Double,
        #[serde(rename = "STRING")]
        #[doc = "The value is a text string. This value type can be used only if the metric kind is GAUGE."]
        String,
        #[serde(rename = "DISTRIBUTION")]
        #[doc = "The value is a Distribution."]
        Distribution,
        #[serde(rename = "MONEY")]
        #[doc = "The value is money."]
        Money,
    }
    impl ::std::default::Default for MetricDescriptorValueTypeEnum {
        fn default() -> Self {
            Self::ValueTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional annotations that can be used to guide the usage of a metric."]
    pub struct MetricDescriptorMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ingestDelay")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The delay of data points caused by ingestion. Data points older than this age are guaranteed to be ingested and available to be read, excluding data loss due to errors."]
        pub ingest_delay: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "launchStage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Must use the MetricDescriptor.launch_stage instead."]
        pub launch_stage: ::std::option::Option<MetricDescriptorMetadataLaunchStageEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "samplePeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sampling period of metric data points. For metrics which are written periodically, consecutive data points are stored at this time interval, excluding data loss due to errors. Metrics with a higher granularity have a smaller sampling period."]
        pub sample_period: ::std::option::Option<::std::string::String>,
    }
    impl MetricDescriptorMetadata {
        pub fn builder() -> MetricDescriptorMetadataBuilder {
            MetricDescriptorMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Deprecated. Must use the MetricDescriptor.launch_stage instead."]
    pub enum MetricDescriptorMetadataLaunchStageEnum {
        #[serde(rename = "LAUNCH_STAGE_UNSPECIFIED")]
        #[doc = "Do not use this default value."]
        LaunchStageUnspecified,
        #[serde(rename = "UNIMPLEMENTED")]
        #[doc = "The feature is not yet implemented. Users can not use it."]
        Unimplemented,
        #[serde(rename = "PRELAUNCH")]
        #[doc = "Prelaunch features are hidden from users and are only visible internally."]
        Prelaunch,
        #[serde(rename = "EARLY_ACCESS")]
        #[doc = "Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released."]
        EarlyAccess,
        #[serde(rename = "ALPHA")]
        #[doc = "Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don’t have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases."]
        Alpha,
        #[serde(rename = "BETA")]
        #[doc = "Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases."]
        Beta,
        #[serde(rename = "GA")]
        #[doc = "GA features are open to all developers and are considered stable and fully qualified for production use."]
        Ga,
        #[serde(rename = "DEPRECATED")]
        #[doc = "Deprecated features are scheduled to be shut down and removed. For more information, see the “Deprecation Policy” section of our Terms of Service (https://cloud.google.com/terms/) and the Google Cloud Platform Subject to the Deprecation Policy (https://cloud.google.com/terms/deprecation) documentation."]
        Deprecated,
    }
    impl ::std::default::Default for MetricDescriptorMetadataLaunchStageEnum {
        fn default() -> Self {
            Self::LaunchStageUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A MetricRange is used when each window is good when the value x of a single TimeSeries satisfies range.min <= x < range.max. The provided TimeSeries must have ValueType = INT64 or ValueType = DOUBLE and MetricKind = GAUGE."]
    pub struct MetricRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Range of values considered \"good.\" For a one-sided range, set one bound to an infinite value."]
        pub range: ::std::option::Option<::std::boxed::Box<GoogleMonitoringV3Range>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSeries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) specifying the TimeSeries to use for evaluating window quality."]
        pub time_series: ::std::option::Option<::std::string::String>,
    }
    impl MetricRange {
        pub fn builder() -> MetricRangeBuilder {
            MetricRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A condition type that compares a collection of time series against a threshold."]
    pub struct MetricThreshold {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aggregations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the alignment of data points in individual time series as well as how to combine the retrieved time series together (such as when aggregating multiple streams on each resource to a single stream for each resource or when aggregating streams across all members of a group of resrouces). Multiple aggregations are applied in the order specified.This field is similar to the one in the ListTimeSeries request (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list). It is advisable to use the ListTimeSeries method when debugging this field."]
        pub aggregations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Aggregation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comparison")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The comparison to apply between the time series (indicated by filter and aggregation) and the threshold (indicated by threshold_value). The comparison is applied on each time series, with the time series on the left-hand side and the threshold on the right-hand side.Only COMPARISON_LT and COMPARISON_GT are supported currently."]
        pub comparison: ::std::option::Option<MetricThresholdComparisonEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "denominatorAggregations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the alignment of data points in individual time series selected by denominatorFilter as well as how to combine the retrieved time series together (such as when aggregating multiple streams on each resource to a single stream for each resource or when aggregating streams across all members of a group of resources).When computing ratios, the aggregations and denominator_aggregations fields must use the same alignment period and produce time series that have the same periodicity and labels."]
        pub denominator_aggregations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Aggregation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "denominatorFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A filter (https://cloud.google.com/monitoring/api/v3/filters) that identifies a time series that should be used as the denominator of a ratio that will be compared with the threshold. If a denominator_filter is specified, the time series specified by the filter field will be used as the numerator.The filter must specify the metric type and optionally may contain restrictions on resource type, resource labels, and metric labels. This field may not exceed 2048 Unicode characters in length."]
        pub denominator_filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of time that a time series must violate the threshold to be considered failing. Currently, only values that are a multiple of a minute--e.g., 0, 60, 120, or 300 seconds--are supported. If an invalid value is given, an error will be returned. When choosing a duration, it is useful to keep in mind the frequency of the underlying time series data (which may also be affected by any alignments specified in the aggregations field); a good duration is long enough so that a single outlier does not generate spurious alerts, but short enough that unhealthy states are detected and alerted on quickly."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A filter (https://cloud.google.com/monitoring/api/v3/filters) that identifies which time series should be compared with the threshold.The filter is similar to the one that is specified in the ListTimeSeries request (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list) (that call is useful to verify the time series that will be retrieved / processed). The filter must specify the metric type and the resource type. Optionally, it can specify resource labels and metric labels. This field must not exceed 2048 Unicode characters in length."]
        pub filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thresholdValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A value against which to compare the time series."]
        pub threshold_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trigger")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number/percent of time series for which the comparison must hold in order for the condition to trigger. If unspecified, then the condition will trigger if the comparison is true for any of the time series that have been identified by filter and aggregations, or by the ratio, if denominator_filter and denominator_aggregations are specified."]
        pub trigger: ::std::option::Option<::std::boxed::Box<Trigger>>,
    }
    impl MetricThreshold {
        pub fn builder() -> MetricThresholdBuilder {
            MetricThresholdBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The comparison to apply between the time series (indicated by filter and aggregation) and the threshold (indicated by threshold_value). The comparison is applied on each time series, with the time series on the left-hand side and the threshold on the right-hand side.Only COMPARISON_LT and COMPARISON_GT are supported currently."]
    pub enum MetricThresholdComparisonEnum {
        #[serde(rename = "COMPARISON_UNSPECIFIED")]
        #[doc = "No ordering relationship is specified."]
        ComparisonUnspecified,
        #[serde(rename = "COMPARISON_GT")]
        #[doc = "True if the left argument is greater than the right argument."]
        ComparisonGt,
        #[serde(rename = "COMPARISON_GE")]
        #[doc = "True if the left argument is greater than or equal to the right argument."]
        ComparisonGe,
        #[serde(rename = "COMPARISON_LT")]
        #[doc = "True if the left argument is less than the right argument."]
        ComparisonLt,
        #[serde(rename = "COMPARISON_LE")]
        #[doc = "True if the left argument is less than or equal to the right argument."]
        ComparisonLe,
        #[serde(rename = "COMPARISON_EQ")]
        #[doc = "True if the left argument is equal to the right argument."]
        ComparisonEq,
        #[serde(rename = "COMPARISON_NE")]
        #[doc = "True if the left argument is not equal to the right argument."]
        ComparisonNe,
    }
    impl ::std::default::Default for MetricThresholdComparisonEnum {
        fn default() -> Self {
            Self::ComparisonUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An object representing a resource that can be used for monitoring, logging, billing, or other purposes. Examples include virtual machine instances, databases, and storage devices such as disks. The type field identifies a MonitoredResourceDescriptor object that describes the resource's schema. Information in the labels field identifies the actual resource and its attributes according to the schema. For example, a particular Compute Engine VM instance could be represented by the following object, because the MonitoredResourceDescriptor for \"gce_instance\" has labels \"instance_id\" and \"zone\": { \"type\": \"gce_instance\", \"labels\": { \"instance_id\": \"12345678901234\", \"zone\": \"us-central1-a\" }} "]
    pub struct MonitoredResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Values for all of the labels listed in the associated monitored resource descriptor. For example, Compute Engine VM instances use the labels \"project_id\", \"instance_id\", and \"zone\"."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The monitored resource type. This field must match the type field of a MonitoredResourceDescriptor object. For example, the type of a Compute Engine VM instance is gce_instance. For a list of types, see Monitoring resource types and Logging resource types."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl MonitoredResource {
        pub fn builder() -> MonitoredResourceBuilder {
            MonitoredResourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An object that describes the schema of a MonitoredResource object using a type name and a set of labels. For example, the monitored resource descriptor for Google Compute Engine VM instances has a type of \"gce_instance\" and specifies the use of the labels \"instance_id\" and \"zone\" to identify particular VM instances.Different APIs can support different monitored resource types. APIs generally provide a list method that returns the monitored resource descriptors used by the API."]
    pub struct MonitoredResourceDescriptor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A detailed description of the monitored resource type that might be used in documentation."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A concise name for the monitored resource type that might be displayed in user interfaces. It should be a Title Cased Noun Phrase, without any article or other determiners. For example, \"Google Cloud SQL Database\"."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A set of labels used to describe instances of this monitored resource type. For example, an individual Google Cloud SQL database is identified by values for the labels \"database_id\" and \"zone\"."]
        pub labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LabelDescriptor>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "launchStage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The launch stage of the monitored resource definition."]
        pub launch_stage: ::std::option::Option<MonitoredResourceDescriptorLaunchStageEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The resource name of the monitored resource descriptor: \"projects/{project_id}/monitoredResourceDescriptors/{type}\" where {type} is the value of the type field in this object and {project_id} is a project ID that provides API-specific context for accessing the type. APIs that do not use project information can use the resource name format \"monitoredResourceDescriptors/{type}\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The monitored resource type. For example, the type \"cloudsql_database\" represents databases in Google Cloud SQL."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl MonitoredResourceDescriptor {
        pub fn builder() -> MonitoredResourceDescriptorBuilder {
            MonitoredResourceDescriptorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The launch stage of the monitored resource definition."]
    pub enum MonitoredResourceDescriptorLaunchStageEnum {
        #[serde(rename = "LAUNCH_STAGE_UNSPECIFIED")]
        #[doc = "Do not use this default value."]
        LaunchStageUnspecified,
        #[serde(rename = "UNIMPLEMENTED")]
        #[doc = "The feature is not yet implemented. Users can not use it."]
        Unimplemented,
        #[serde(rename = "PRELAUNCH")]
        #[doc = "Prelaunch features are hidden from users and are only visible internally."]
        Prelaunch,
        #[serde(rename = "EARLY_ACCESS")]
        #[doc = "Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released."]
        EarlyAccess,
        #[serde(rename = "ALPHA")]
        #[doc = "Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don’t have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases."]
        Alpha,
        #[serde(rename = "BETA")]
        #[doc = "Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases."]
        Beta,
        #[serde(rename = "GA")]
        #[doc = "GA features are open to all developers and are considered stable and fully qualified for production use."]
        Ga,
        #[serde(rename = "DEPRECATED")]
        #[doc = "Deprecated features are scheduled to be shut down and removed. For more information, see the “Deprecation Policy” section of our Terms of Service (https://cloud.google.com/terms/) and the Google Cloud Platform Subject to the Deprecation Policy (https://cloud.google.com/terms/deprecation) documentation."]
        Deprecated,
    }
    impl ::std::default::Default for MonitoredResourceDescriptorLaunchStageEnum {
        fn default() -> Self {
            Self::LaunchStageUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Auxiliary metadata for a MonitoredResource object. MonitoredResource objects contain the minimum set of information to uniquely identify a monitored resource instance. There is some other useful auxiliary metadata. Monitoring and Logging use an ingestion pipeline to extract metadata for cloud resources of all types, and store the metadata in this message."]
    pub struct MonitoredResourceMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Values for predefined system metadata labels. System labels are a kind of metadata extracted by Google, including \"machine_image\", \"vpc\", \"subnet_id\", \"security_group\", \"name\", etc. System label values can be only strings, Boolean values, or a list of strings. For example: { \"name\": \"my-test-instance\", \"security_group\": [\"a\", \"b\", \"c\"], \"spot_instance\": false } "]
        pub system_labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A map of user-defined metadata labels."]
        pub user_labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl MonitoredResourceMetadata {
        pub fn builder() -> MonitoredResourceMetadataBuilder {
            MonitoredResourceMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A condition type that allows alert policies to be defined using Monitoring Query Language (https://cloud.google.com/monitoring/mql)."]
    pub struct MonitoringQueryLanguageCondition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of time that a time series must violate the threshold to be considered failing. Currently, only values that are a multiple of a minute--e.g., 0, 60, 120, or 300 seconds--are supported. If an invalid value is given, an error will be returned. When choosing a duration, it is useful to keep in mind the frequency of the underlying time series data (which may also be affected by any alignments specified in the aggregations field); a good duration is long enough so that a single outlier does not generate spurious alerts, but short enough that unhealthy states are detected and alerted on quickly."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Monitoring Query Language (https://cloud.google.com/monitoring/mql) query that outputs a boolean stream."]
        pub query: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trigger")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number/percent of time series for which the comparison must hold in order for the condition to trigger. If unspecified, then the condition will trigger if the comparison is true for any of the time series that have been identified by filter and aggregations, or by the ratio, if denominator_filter and denominator_aggregations are specified."]
        pub trigger: ::std::option::Option<::std::boxed::Box<Trigger>>,
    }
    impl MonitoringQueryLanguageCondition {
        pub fn builder() -> MonitoringQueryLanguageConditionBuilder {
            MonitoringQueryLanguageConditionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a change made to a configuration."]
    pub struct MutationRecord {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mutateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the change occurred."]
        pub mutate_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mutatedBy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of the user making the change."]
        pub mutated_by: ::std::option::Option<::std::string::String>,
    }
    impl MutationRecord {
        pub fn builder() -> MutationRecordBuilder {
            MutationRecordBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A NotificationChannel is a medium through which an alert is delivered when a policy violation is detected. Examples of channels include email, SMS, and third-party messaging applications. Fields containing sensitive information like authentication tokens or contact info are only partially populated on retrieval."]
    pub struct NotificationChannel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationRecord")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Record of the creation of this channel."]
        pub creation_record: ::std::option::Option<::std::boxed::Box<MutationRecord>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional human-readable description of this notification channel. This description may provide additional details, beyond the display name, for the channel. This may not exceed 1024 Unicode characters."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional human-readable name for this notification channel. It is recommended that you specify a non-empty and unique name in order to make it easier to identify the channels in your project, though this is not enforced. The display name is limited to 512 Unicode characters."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether notifications are forwarded to the described channel. This makes it possible to disable delivery of notifications to a particular channel without removing the channel from all alerting policies that reference the channel. This is a more convenient approach when the change is temporary and you want to receive notifications from the same set of alerting policies on the channel at some point in the future."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration fields that define the channel and its behavior. The permissible and required labels are specified in the NotificationChannelDescriptor.labels of the NotificationChannelDescriptor corresponding to the type field."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mutationRecords")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Records of the modification of this channel."]
        pub mutation_records:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MutationRecord>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full REST resource name for this channel. The format is: projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID] The [CHANNEL_ID] is automatically assigned by the server on creation."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the notification channel. This field matches the value of the NotificationChannelDescriptor.type field."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-supplied key/value data that does not need to conform to the corresponding NotificationChannelDescriptor's schema, unlike the labels field. This field is intended to be used for organizing and identifying the NotificationChannel objects.The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter."]
        pub user_labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verificationStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether this channel has been verified or not. On a ListNotificationChannels or GetNotificationChannel operation, this field is expected to be populated.If the value is UNVERIFIED, then it indicates that the channel is non-functioning (it both requires verification and lacks verification); otherwise, it is assumed that the channel works.If the channel is neither VERIFIED nor UNVERIFIED, it implies that the channel is of a type that does not require verification or that this specific channel has been exempted from verification because it was created prior to verification being required for channels of this type.This field cannot be modified using a standard UpdateNotificationChannel operation. To change the value of this field, you must call VerifyNotificationChannel."]
        pub verification_status: ::std::option::Option<NotificationChannelVerificationStatusEnum>,
    }
    impl NotificationChannel {
        pub fn builder() -> NotificationChannelBuilder {
            NotificationChannelBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates whether this channel has been verified or not. On a ListNotificationChannels or GetNotificationChannel operation, this field is expected to be populated.If the value is UNVERIFIED, then it indicates that the channel is non-functioning (it both requires verification and lacks verification); otherwise, it is assumed that the channel works.If the channel is neither VERIFIED nor UNVERIFIED, it implies that the channel is of a type that does not require verification or that this specific channel has been exempted from verification because it was created prior to verification being required for channels of this type.This field cannot be modified using a standard UpdateNotificationChannel operation. To change the value of this field, you must call VerifyNotificationChannel."]
    pub enum NotificationChannelVerificationStatusEnum {
        #[serde(rename = "VERIFICATION_STATUS_UNSPECIFIED")]
        #[doc = "Sentinel value used to indicate that the state is unknown, omitted, or is not applicable (as in the case of channels that neither support nor require verification in order to function)."]
        VerificationStatusUnspecified,
        #[serde(rename = "UNVERIFIED")]
        #[doc = "The channel has yet to be verified and requires verification to function. Note that this state also applies to the case where the verification process has been initiated by sending a verification code but where the verification code has not been submitted to complete the process."]
        Unverified,
        #[serde(rename = "VERIFIED")]
        #[doc = "It has been proven that notifications can be received on this notification channel and that someone on the project has access to messages that are delivered to that channel."]
        Verified,
    }
    impl ::std::default::Default for NotificationChannelVerificationStatusEnum {
        fn default() -> Self {
            Self::VerificationStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A description of a notification channel. The descriptor includes the properties of the channel and the set of labels or fields that must be specified to configure channels of a given type."]
    pub struct NotificationChannelDescriptor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A human-readable description of the notification channel type. The description may include a description of the properties of the channel and pointers to external documentation."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A human-readable name for the notification channel type. This form of the name is suitable for a user interface."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of labels that must be defined to identify a particular channel of the corresponding type. Each label includes a description for how that field should be populated."]
        pub labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LabelDescriptor>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "launchStage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The product launch stage for channels of this type."]
        pub launch_stage: ::std::option::Option<NotificationChannelDescriptorLaunchStageEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full REST resource name for this descriptor. The format is: projects/[PROJECT_ID_OR_NUMBER]/notificationChannelDescriptors/[TYPE] In the above, [TYPE] is the value of the type field."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of notification channel, such as \"email\" and \"sms\". To view the full list of channels, see Channel descriptors (https://cloud.google.com/monitoring/alerts/using-channels-api#ncd). Notification channel types are globally unique."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl NotificationChannelDescriptor {
        pub fn builder() -> NotificationChannelDescriptorBuilder {
            NotificationChannelDescriptorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The product launch stage for channels of this type."]
    pub enum NotificationChannelDescriptorLaunchStageEnum {
        #[serde(rename = "LAUNCH_STAGE_UNSPECIFIED")]
        #[doc = "Do not use this default value."]
        LaunchStageUnspecified,
        #[serde(rename = "UNIMPLEMENTED")]
        #[doc = "The feature is not yet implemented. Users can not use it."]
        Unimplemented,
        #[serde(rename = "PRELAUNCH")]
        #[doc = "Prelaunch features are hidden from users and are only visible internally."]
        Prelaunch,
        #[serde(rename = "EARLY_ACCESS")]
        #[doc = "Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released."]
        EarlyAccess,
        #[serde(rename = "ALPHA")]
        #[doc = "Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don’t have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases."]
        Alpha,
        #[serde(rename = "BETA")]
        #[doc = "Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases."]
        Beta,
        #[serde(rename = "GA")]
        #[doc = "GA features are open to all developers and are considered stable and fully qualified for production use."]
        Ga,
        #[serde(rename = "DEPRECATED")]
        #[doc = "Deprecated features are scheduled to be shut down and removed. For more information, see the “Deprecation Policy” section of our Terms of Service (https://cloud.google.com/terms/) and the Google Cloud Platform Subject to the Deprecation Policy (https://cloud.google.com/terms/deprecation) documentation."]
        Deprecated,
    }
    impl ::std::default::Default for NotificationChannelDescriptorLaunchStageEnum {
        fn default() -> Self {
            Self::LaunchStageUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A protocol buffer option, which can be attached to a message, field, enumeration, etc."]
    pub struct Option {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The option's name. For protobuf built-in options (options defined in descriptor.proto), this is the short name. For example, \"map_entry\". For custom options, it should be the fully-qualified name. For example, \"google.api.http\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The option's value packed in an Any message. If the value is a primitive, the corresponding wrapper type defined in google/protobuf/wrappers.proto should be used. If the value is an enum, it should be stored as an int32 value using the google.protobuf.Int32Value type."]
        pub value: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl Option {
        pub fn builder() -> OptionBuilder {
            OptionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A PerformanceThreshold is used when each window is good when that window has a sufficiently high performance."]
    pub struct PerformanceThreshold {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicSliPerformance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "BasicSli to evaluate to judge window quality."]
        pub basic_sli_performance: ::std::option::Option<::std::boxed::Box<BasicSli>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "performance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RequestBasedSli to evaluate to judge window quality."]
        pub performance: ::std::option::Option<::std::boxed::Box<RequestBasedSli>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If window performance >= threshold, the window is counted as good."]
        pub threshold: ::std::option::Option<::std::primitive::f64>,
    }
    impl PerformanceThreshold {
        pub fn builder() -> PerformanceThresholdBuilder {
            PerformanceThresholdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single data point in a time series."]
    pub struct Point {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "interval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time interval to which the data point applies. For GAUGE metrics, the start time is optional, but if it is supplied, it must equal the end time. For DELTA metrics, the start and end time should specify a non-zero interval, with subsequent points specifying contiguous and non-overlapping intervals. For CUMULATIVE metrics, the start and end time should specify a non-zero interval, with subsequent points specifying the same start time and increasing end times, until an event resets the cumulative value to zero and sets a new start time for the following points."]
        pub interval: ::std::option::Option<::std::boxed::Box<TimeInterval>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of the data point."]
        pub value: ::std::option::Option<::std::boxed::Box<TypedValue>>,
    }
    impl Point {
        pub fn builder() -> PointBuilder {
            PointBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A point's value columns and time interval. Each point has one or more point values corresponding to the entries in point_descriptors field in the TimeSeriesDescriptor associated with this object."]
    pub struct PointData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeInterval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time interval associated with the point."]
        pub time_interval: ::std::option::Option<::std::boxed::Box<TimeInterval>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values that make up the point."]
        pub values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TypedValue>>>,
    }
    impl PointData {
        pub fn builder() -> PointDataBuilder {
            PointDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The QueryTimeSeries request."]
    pub struct QueryTimeSeriesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A positive number that is the maximum number of time_series_data to return."]
        pub page_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this field is not empty then it must contain the nextPageToken value returned by a previous call to this method. Using this field causes the method to return additional results from the previous method call."]
        pub page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The query in the Monitoring Query Language (https://cloud.google.com/monitoring/mql/reference) format. The default time zone is in UTC."]
        pub query: ::std::option::Option<::std::string::String>,
    }
    impl QueryTimeSeriesRequest {
        pub fn builder() -> QueryTimeSeriesRequestBuilder {
            QueryTimeSeriesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The QueryTimeSeries response."]
    pub struct QueryTimeSeriesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Query execution errors that may have caused the time series data returned to be incomplete. The available data will be available in the response."]
        pub partial_errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Status>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSeriesData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time series data."]
        pub time_series_data:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TimeSeriesData>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSeriesDescriptor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The descriptor for the time series data."]
        pub time_series_descriptor: ::std::option::Option<::std::boxed::Box<TimeSeriesDescriptor>>,
    }
    impl QueryTimeSeriesResponse {
        pub fn builder() -> QueryTimeSeriesResponseBuilder {
            QueryTimeSeriesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The range of the population values."]
    pub struct Range {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "max")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum of the population values."]
        pub max: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "min")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum of the population values."]
        pub min: ::std::option::Option<::std::primitive::f64>,
    }
    impl Range {
        pub fn builder() -> RangeBuilder {
            RangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Service Level Indicators for which atomic units of service are counted directly."]
    pub struct RequestBasedSli {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "distributionCut")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "distribution_cut is used when good_service is a count of values aggregated in a Distribution that fall into a good range. The total_service is the total count of all values aggregated in the Distribution."]
        pub distribution_cut: ::std::option::Option<::std::boxed::Box<DistributionCut>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "goodTotalRatio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "good_total_ratio is used when the ratio of good_service to total_service is computed from two TimeSeries."]
        pub good_total_ratio: ::std::option::Option<::std::boxed::Box<TimeSeriesRatio>>,
    }
    impl RequestBasedSli {
        pub fn builder() -> RequestBasedSliBuilder {
            RequestBasedSliBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The resource submessage for group checks. It can be used instead of a monitored resource, when multiple resources are being monitored."]
    pub struct ResourceGroup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The group of resources being monitored. Should be only the [GROUP_ID], and not the full-path projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID]."]
        pub group_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource type of the group members."]
        pub resource_type: ::std::option::Option<ResourceGroupResourceTypeEnum>,
    }
    impl ResourceGroup {
        pub fn builder() -> ResourceGroupBuilder {
            ResourceGroupBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The resource type of the group members."]
    pub enum ResourceGroupResourceTypeEnum {
        #[serde(rename = "RESOURCE_TYPE_UNSPECIFIED")]
        #[doc = "Default value (not valid)."]
        ResourceTypeUnspecified,
        #[serde(rename = "INSTANCE")]
        #[doc = "A group of instances from Google Cloud Platform (GCP) or Amazon Web Services (AWS)."]
        Instance,
        #[serde(rename = "AWS_ELB_LOAD_BALANCER")]
        #[doc = "A group of Amazon ELB load balancers."]
        AwsElbLoadBalancer,
    }
    impl ::std::default::Default for ResourceGroupResourceTypeEnum {
        fn default() -> Self {
            Self::ResourceTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The SendNotificationChannelVerificationCode request."]
    pub struct SendNotificationChannelVerificationCodeRequest {}
    impl SendNotificationChannelVerificationCodeRequest {
        pub fn builder() -> SendNotificationChannelVerificationCodeRequestBuilder {
            SendNotificationChannelVerificationCodeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Service is a discrete, autonomous, and network-accessible unit, designed to solve an individual concern (Wikipedia (https://en.wikipedia.org/wiki/Service-orientation)). In Cloud Monitoring, a Service acts as the root resource under which operational aspects of the service are accessible."]
    pub struct Service {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appEngine")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type used for App Engine services."]
        pub app_engine: ::std::option::Option<::std::boxed::Box<AppEngine>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudEndpoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type used for Cloud Endpoints services."]
        pub cloud_endpoints: ::std::option::Option<::std::boxed::Box<CloudEndpoints>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterIstio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type used for Istio services that live in a Kubernetes cluster."]
        pub cluster_istio: ::std::option::Option<::std::boxed::Box<ClusterIstio>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "custom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom service type."]
        pub custom: ::std::option::Option<::std::boxed::Box<Custom>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name used for UI elements listing this Service."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "istioCanonicalService")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type used for canonical services scoped to an Istio mesh. Metrics for Istio are documented here (https://istio.io/latest/docs/reference/config/metrics/)"]
        pub istio_canonical_service:
            ::std::option::Option<::std::boxed::Box<IstioCanonicalService>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meshIstio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type used for Istio services scoped to an Istio mesh."]
        pub mesh_istio: ::std::option::Option<::std::boxed::Box<MeshIstio>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name for this Service. The format is: projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID] "]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "telemetry")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for how to query telemetry on a Service."]
        pub telemetry: ::std::option::Option<::std::boxed::Box<Telemetry>>,
    }
    impl Service {
        pub fn builder() -> ServiceBuilder {
            ServiceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Service-Level Indicator (SLI) describes the \"performance\" of a service. For some services, the SLI is well-defined. In such cases, the SLI can be described easily by referencing the well-known SLI and providing the needed parameters. Alternatively, a \"custom\" SLI can be defined with a query to the underlying metric store. An SLI is defined to be good_service / total_service over any queried time interval. The value of performance always falls into the range 0 <= performance <= 1. A custom SLI describes how to compute this ratio, whether this is by dividing values from a pair of time series, cutting a Distribution into good and bad counts, or counting time windows in which the service complies with a criterion. For separation of concerns, a single Service-Level Indicator measures performance for only one aspect of service quality, such as fraction of successful queries or fast-enough queries."]
    pub struct ServiceLevelIndicator {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicSli")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic SLI on a well-known service type."]
        pub basic_sli: ::std::option::Option<::std::boxed::Box<BasicSli>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestBased")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Request-based SLIs"]
        pub request_based: ::std::option::Option<::std::boxed::Box<RequestBasedSli>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "windowsBased")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Windows-based SLIs"]
        pub windows_based: ::std::option::Option<::std::boxed::Box<WindowsBasedSli>>,
    }
    impl ServiceLevelIndicator {
        pub fn builder() -> ServiceLevelIndicatorBuilder {
            ServiceLevelIndicatorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Service-Level Objective (SLO) describes a level of desired good service. It consists of a service-level indicator (SLI), a performance goal, and a period over which the objective is to be evaluated against that goal. The SLO can use SLIs defined in a number of different manners. Typical SLOs might include \"99% of requests in each rolling week have latency below 200 milliseconds\" or \"99.5% of requests in each calendar month return successfully.\""]
    pub struct ServiceLevelObjective {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "calendarPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A calendar period, semantically \"since the start of the current \". At this time, only DAY, WEEK, FORTNIGHT, and MONTH are supported."]
        pub calendar_period: ::std::option::Option<ServiceLevelObjectiveCalendarPeriodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name used for UI elements listing this SLO."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "goal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fraction of service that must be good in order for this objective to be met. 0 < goal <= 0.999."]
        pub goal: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name for this ServiceLevelObjective. The format is: projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]/serviceLevelObjectives/[SLO_NAME] "]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rollingPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A rolling time period, semantically \"in the past \". Must be an integer multiple of 1 day no larger than 30 days."]
        pub rolling_period: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceLevelIndicator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The definition of good service, used to measure and calculate the quality of the Service's performance with respect to a single aspect of service quality."]
        pub service_level_indicator:
            ::std::option::Option<::std::boxed::Box<ServiceLevelIndicator>>,
    }
    impl ServiceLevelObjective {
        pub fn builder() -> ServiceLevelObjectiveBuilder {
            ServiceLevelObjectiveBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "A calendar period, semantically \"since the start of the current \". At this time, only DAY, WEEK, FORTNIGHT, and MONTH are supported."]
    pub enum ServiceLevelObjectiveCalendarPeriodEnum {
        #[serde(rename = "CALENDAR_PERIOD_UNSPECIFIED")]
        #[doc = "Undefined period, raises an error."]
        CalendarPeriodUnspecified,
        #[serde(rename = "DAY")]
        #[doc = "A day."]
        Day,
        #[serde(rename = "WEEK")]
        #[doc = "A week. Weeks begin on Monday, following ISO 8601 (https://en.wikipedia.org/wiki/ISO_week_date)."]
        Week,
        #[serde(rename = "FORTNIGHT")]
        #[doc = "A fortnight. The first calendar fortnight of the year begins at the start of week 1 according to ISO 8601 (https://en.wikipedia.org/wiki/ISO_week_date)."]
        Fortnight,
        #[serde(rename = "MONTH")]
        #[doc = "A month."]
        Month,
        #[serde(rename = "QUARTER")]
        #[doc = "A quarter. Quarters start on dates 1-Jan, 1-Apr, 1-Jul, and 1-Oct of each year."]
        Quarter,
        #[serde(rename = "HALF")]
        #[doc = "A half-year. Half-years start on dates 1-Jan and 1-Jul."]
        Half,
        #[serde(rename = "YEAR")]
        #[doc = "A year."]
        Year,
    }
    impl ::std::default::Default for ServiceLevelObjectiveCalendarPeriodEnum {
        fn default() -> Self {
            Self::CalendarPeriodUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SourceContext represents information about the source of a protobuf element, like the file in which it is defined."]
    pub struct SourceContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The path-qualified name of the .proto file that contained the associated protobuf element. For example: \"google/protobuf/source_context.proto\"."]
        pub file_name: ::std::option::Option<::std::string::String>,
    }
    impl SourceContext {
        pub fn builder() -> SourceContextBuilder {
            SourceContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The context of a span, attached to Exemplars in Distribution values during aggregation.It contains the name of a span with format: projects/[PROJECT_ID_OR_NUMBER]/traces/[TRACE_ID]/spans/[SPAN_ID] "]
    pub struct SpanContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spanName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the span. The format is: projects/[PROJECT_ID_OR_NUMBER]/traces/[TRACE_ID]/spans/[SPAN_ID] [TRACE_ID] is a unique identifier for a trace within a project; it is a 32-character hexadecimal encoding of a 16-byte array.[SPAN_ID] is a unique identifier for a span within a trace; it is a 16-character hexadecimal encoding of an 8-byte array."]
        pub span_name: ::std::option::Option<::std::string::String>,
    }
    impl SpanContext {
        pub fn builder() -> SpanContextBuilder {
            SpanContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Status type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC (https://github.com/grpc). Each Status message contains three pieces of data: error code, error message, and error details.You can find out more about this error model and how to work with it in the API Design Guide (https://cloud.google.com/apis/design/errors)."]
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
    #[doc = "Information required for a TCP Uptime check request."]
    pub struct TcpCheck {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "port")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The TCP port on the server against which to run the check. Will be combined with host (specified within the monitored_resource) to construct the full URL. Required."]
        pub port: ::std::option::Option<::std::primitive::i64>,
    }
    impl TcpCheck {
        pub fn builder() -> TcpCheckBuilder {
            TcpCheckBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for how to query telemetry on a Service."]
    pub struct Telemetry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full name of the resource that defines this service. Formatted as described in https://cloud.google.com/apis/design/resource_names."]
        pub resource_name: ::std::option::Option<::std::string::String>,
    }
    impl Telemetry {
        pub fn builder() -> TelemetryBuilder {
            TelemetryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A closed time interval. It extends from the start time to the end time, and includes both: [startTime, endTime]. Valid time intervals depend on the MetricKind of the metric value. The end time must not be earlier than the start time. When writing data points, the start time must not be more than 25 hours in the past and the end time must not be more than five minutes in the future. For GAUGE metrics, the startTime value is technically optional; if no value is specified, the start time defaults to the value of the end time, and the interval represents a single point in time. If both start and end times are specified, they must be identical. Such an interval is valid only for GAUGE metrics, which are point-in-time measurements. The end time of a new interval must be at least a millisecond after the end time of the previous interval. For DELTA metrics, the start time and end time must specify a non-zero interval, with subsequent points specifying contiguous and non-overlapping intervals. For DELTA metrics, the start time of the next interval must be at least a millisecond after the end time of the previous interval. For CUMULATIVE metrics, the start time and end time must specify a a non-zero interval, with subsequent points specifying the same start time and increasing end times, until an event resets the cumulative value to zero and sets a new start time for the following points. The new start time must be at least a millisecond after the end time of the previous interval. The start time of a new interval must be at least a millisecond after the end time of the previous interval because intervals are closed. If the start time of a new interval is the same as the end time of the previous interval, then data written at the new start time could overwrite data written at the previous end time."]
    pub struct TimeInterval {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The end of the time interval."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The beginning of the time interval. The default value for the start time is the end time. The start time must not be later than the end time."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl TimeInterval {
        pub fn builder() -> TimeIntervalBuilder {
            TimeIntervalBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A collection of data points that describes the time-varying values of a metric. A time series is identified by a combination of a fully-specified monitored resource and a fully-specified metric. This type is used for both listing and creating time series."]
    pub struct TimeSeries {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The associated monitored resource metadata. When reading a time series, this field will include metadata labels that are explicitly named in the reduction. When creating a time series, this field is ignored."]
        pub metadata: ::std::option::Option<::std::boxed::Box<MonitoredResourceMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metric")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The associated metric. A fully-specified metric used to identify the time series."]
        pub metric: ::std::option::Option<::std::boxed::Box<Metric>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricKind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metric kind of the time series. When listing time series, this metric kind might be different from the metric kind of the associated metric if this time series is an alignment or reduction of other time series.When creating a time series, this field is optional. If present, it must be the same as the metric kind of the associated metric. If the associated metric's descriptor must be auto-created, then this field specifies the metric kind of the new descriptor and must be either GAUGE (the default) or CUMULATIVE."]
        pub metric_kind: ::std::option::Option<TimeSeriesMetricKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "points")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data points of this time series. When listing time series, points are returned in reverse time order.When creating a time series, this field must contain exactly one point and the point's type must be the same as the value type of the associated metric. If the associated metric's descriptor must be auto-created, then the value type of the descriptor is determined by the point's type, which must be BOOL, INT64, DOUBLE, or DISTRIBUTION."]
        pub points: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Point>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The associated monitored resource. Custom metrics can use only certain monitored resource types in their time series data."]
        pub resource: ::std::option::Option<::std::boxed::Box<MonitoredResource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The units in which the metric value is reported. It is only applicable if the value_type is INT64, DOUBLE, or DISTRIBUTION. The unit defines the representation of the stored metric values."]
        pub unit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value type of the time series. When listing time series, this value type might be different from the value type of the associated metric if this time series is an alignment or reduction of other time series.When creating a time series, this field is optional. If present, it must be the same as the type of the data in the points field."]
        pub value_type: ::std::option::Option<TimeSeriesValueTypeEnum>,
    }
    impl TimeSeries {
        pub fn builder() -> TimeSeriesBuilder {
            TimeSeriesBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The metric kind of the time series. When listing time series, this metric kind might be different from the metric kind of the associated metric if this time series is an alignment or reduction of other time series.When creating a time series, this field is optional. If present, it must be the same as the metric kind of the associated metric. If the associated metric's descriptor must be auto-created, then this field specifies the metric kind of the new descriptor and must be either GAUGE (the default) or CUMULATIVE."]
    pub enum TimeSeriesMetricKindEnum {
        #[serde(rename = "METRIC_KIND_UNSPECIFIED")]
        #[doc = "Do not use this default value."]
        MetricKindUnspecified,
        #[serde(rename = "GAUGE")]
        #[doc = "An instantaneous measurement of a value."]
        Gauge,
        #[serde(rename = "DELTA")]
        #[doc = "The change in a value during a time interval."]
        Delta,
        #[serde(rename = "CUMULATIVE")]
        #[doc = "A value accumulated over a time interval. Cumulative measurements in a time series should have the same start time and increasing end times, until an event resets the cumulative value to zero and sets a new start time for the following points."]
        Cumulative,
    }
    impl ::std::default::Default for TimeSeriesMetricKindEnum {
        fn default() -> Self {
            Self::MetricKindUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The value type of the time series. When listing time series, this value type might be different from the value type of the associated metric if this time series is an alignment or reduction of other time series.When creating a time series, this field is optional. If present, it must be the same as the type of the data in the points field."]
    pub enum TimeSeriesValueTypeEnum {
        #[serde(rename = "VALUE_TYPE_UNSPECIFIED")]
        #[doc = "Do not use this default value."]
        ValueTypeUnspecified,
        #[serde(rename = "BOOL")]
        #[doc = "The value is a boolean. This value type can be used only if the metric kind is GAUGE."]
        Bool,
        #[serde(rename = "INT64")]
        #[doc = "The value is a signed 64-bit integer."]
        Int64,
        #[serde(rename = "DOUBLE")]
        #[doc = "The value is a double precision floating point number."]
        Double,
        #[serde(rename = "STRING")]
        #[doc = "The value is a text string. This value type can be used only if the metric kind is GAUGE."]
        String,
        #[serde(rename = "DISTRIBUTION")]
        #[doc = "The value is a Distribution."]
        Distribution,
        #[serde(rename = "MONEY")]
        #[doc = "The value is money."]
        Money,
    }
    impl ::std::default::Default for TimeSeriesValueTypeEnum {
        fn default() -> Self {
            Self::ValueTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the values of a time series associated with a TimeSeriesDescriptor."]
    pub struct TimeSeriesData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values of the labels in the time series identifier, given in the same order as the label_descriptors field of the TimeSeriesDescriptor associated with this object. Each value must have a value of the type given in the corresponding entry of label_descriptors."]
        pub label_values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LabelValue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pointData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The points in the time series."]
        pub point_data: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PointData>>>,
    }
    impl TimeSeriesData {
        pub fn builder() -> TimeSeriesDataBuilder {
            TimeSeriesDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A descriptor for the labels and points in a time series."]
    pub struct TimeSeriesDescriptor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelDescriptors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Descriptors for the labels."]
        pub label_descriptors:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LabelDescriptor>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pointDescriptors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Descriptors for the point data value columns."]
        pub point_descriptors:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ValueDescriptor>>>,
    }
    impl TimeSeriesDescriptor {
        pub fn builder() -> TimeSeriesDescriptorBuilder {
            TimeSeriesDescriptorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A TimeSeriesRatio specifies two TimeSeries to use for computing the good_service / total_service ratio. The specified TimeSeries must have ValueType = DOUBLE or ValueType = INT64 and must have MetricKind = DELTA or MetricKind = CUMULATIVE. The TimeSeriesRatio must specify exactly two of good, bad, and total, and the relationship good_service + bad_service = total_service will be assumed."]
    pub struct TimeSeriesRatio {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "badServiceFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) specifying a TimeSeries quantifying bad service, either demanded service that was not provided or demanded service that was of inadequate quality. Must have ValueType = DOUBLE or ValueType = INT64 and must have MetricKind = DELTA or MetricKind = CUMULATIVE."]
        pub bad_service_filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "goodServiceFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) specifying a TimeSeries quantifying good service provided. Must have ValueType = DOUBLE or ValueType = INT64 and must have MetricKind = DELTA or MetricKind = CUMULATIVE."]
        pub good_service_filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalServiceFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) specifying a TimeSeries quantifying total demanded service. Must have ValueType = DOUBLE or ValueType = INT64 and must have MetricKind = DELTA or MetricKind = CUMULATIVE."]
        pub total_service_filter: ::std::option::Option<::std::string::String>,
    }
    impl TimeSeriesRatio {
        pub fn builder() -> TimeSeriesRatioBuilder {
            TimeSeriesRatioBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies how many time series must fail a predicate to trigger a condition. If not specified, then a {count: 1} trigger is used."]
    pub struct Trigger {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The absolute number of time series that must fail the predicate for the condition to be triggered."]
        pub count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "percent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The percentage of time series that must fail the predicate for the condition to be triggered."]
        pub percent: ::std::option::Option<::std::primitive::f64>,
    }
    impl Trigger {
        pub fn builder() -> TriggerBuilder {
            TriggerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A protocol buffer message type."]
    pub struct Type {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of fields."]
        pub fields: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Field>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fully qualified message name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oneofs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of types appearing in oneof definitions in this type."]
        pub oneofs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "options")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The protocol buffer options."]
        pub options: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Option>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source context."]
        pub source_context: ::std::option::Option<::std::boxed::Box<SourceContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "syntax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source syntax."]
        pub syntax: ::std::option::Option<TypeSyntaxEnum>,
    }
    impl Type {
        pub fn builder() -> TypeBuilder {
            TypeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The source syntax."]
    pub enum TypeSyntaxEnum {
        #[serde(rename = "SYNTAX_PROTO2")]
        #[doc = "Syntax proto2."]
        SyntaxProto2,
        #[serde(rename = "SYNTAX_PROTO3")]
        #[doc = "Syntax proto3."]
        SyntaxProto3,
    }
    impl ::std::default::Default for TypeSyntaxEnum {
        fn default() -> Self {
            Self::SyntaxProto2
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single strongly-typed value."]
    pub struct TypedValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boolValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Boolean value: true or false."]
        pub bool_value: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "distributionValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A distribution value."]
        pub distribution_value: ::std::option::Option<::std::boxed::Box<Distribution>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "doubleValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A 64-bit double-precision floating-point number. Its magnitude is approximately ±10±300 and it has 16 significant digits of precision."]
        pub double_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "int64Value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A 64-bit integer. Its range is approximately ±9.2x1018."]
        pub int64_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A variable-length string value."]
        pub string_value: ::std::option::Option<::std::string::String>,
    }
    impl TypedValue {
        pub fn builder() -> TypedValueBuilder {
            TypedValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This message configures which resources and services to monitor for availability."]
    pub struct UptimeCheckConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentMatchers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content that is expected to appear in the data returned by the target server against which the check is run. Currently, only the first entry in the content_matchers list is supported, and additional entries will be ignored. This field is optional and should only be specified if a content match is required as part of the/ Uptime check."]
        pub content_matchers:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContentMatcher>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A human-friendly name for the Uptime check configuration. The display name should be unique within a Stackdriver Workspace in order to make it easier to identify; however, uniqueness is not enforced. Required."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpCheck")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains information needed to make an HTTP or HTTPS check."]
        pub http_check: ::std::option::Option<::std::boxed::Box<HttpCheck>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "internalCheckers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The internal checkers that this check will egress from. If is_internal is true and this list is empty, the check will egress from all the InternalCheckers configured for the project that owns this UptimeCheckConfig."]
        pub internal_checkers:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InternalChecker>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isInternal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this is true, then checks are made only from the 'internal_checkers'. If it is false, then checks are made only from the 'selected_regions'. It is an error to provide 'selected_regions' when is_internal is true, or to provide 'internal_checkers' when is_internal is false."]
        pub is_internal: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "monitoredResource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The monitored resource (https://cloud.google.com/monitoring/api/resources) associated with the configuration. The following monitored resource types are supported for Uptime checks: uptime_url, gce_instance, gae_app, aws_ec2_instance, aws_elb_load_balancer"]
        pub monitored_resource: ::std::option::Option<::std::boxed::Box<MonitoredResource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique resource name for this Uptime check configuration. The format is: projects/[PROJECT_ID_OR_NUMBER]/uptimeCheckConfigs/[UPTIME_CHECK_ID] [PROJECT_ID_OR_NUMBER] is the Workspace host project associated with the Uptime check.This field should be omitted when creating the Uptime check configuration; on create, the resource name is assigned by the server and included in the response."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "period")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How often, in seconds, the Uptime check is performed. Currently, the only supported values are 60s (1 minute), 300s (5 minutes), 600s (10 minutes), and 900s (15 minutes). Optional, defaults to 60s."]
        pub period: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The group resource associated with the configuration."]
        pub resource_group: ::std::option::Option<::std::boxed::Box<ResourceGroup>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selectedRegions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of regions from which the check will be run. Some regions contain one location, and others contain more than one. If this field is specified, enough regions must be provided to include a minimum of 3 locations. Not specifying this field will result in Uptime checks running from all available regions."]
        pub selected_regions:
            ::std::option::Option<::std::vec::Vec<UptimeCheckConfigSelectedRegionsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tcpCheck")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains information needed to make a TCP check."]
        pub tcp_check: ::std::option::Option<::std::boxed::Box<TcpCheck>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum amount of time to wait for the request to complete (must be between 1 and 60 seconds). Required."]
        pub timeout: ::std::option::Option<::std::string::String>,
    }
    impl UptimeCheckConfig {
        pub fn builder() -> UptimeCheckConfigBuilder {
            UptimeCheckConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum UptimeCheckConfigSelectedRegionsEnum {
        #[serde(rename = "REGION_UNSPECIFIED")]
        #[doc = "Default value if no region is specified. Will result in Uptime checks running from all regions."]
        RegionUnspecified,
        #[serde(rename = "USA")]
        #[doc = "Allows checks to run from locations within the United States of America."]
        Usa,
        #[serde(rename = "EUROPE")]
        #[doc = "Allows checks to run from locations within the continent of Europe."]
        Europe,
        #[serde(rename = "SOUTH_AMERICA")]
        #[doc = "Allows checks to run from locations within the continent of South America."]
        SouthAmerica,
        #[serde(rename = "ASIA_PACIFIC")]
        #[doc = "Allows checks to run from locations within the Asia Pacific area (ex: Singapore)."]
        AsiaPacific,
    }
    impl ::std::default::Default for UptimeCheckConfigSelectedRegionsEnum {
        fn default() -> Self {
            Self::RegionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains the region, location, and list of IP addresses where checkers in the location run from."]
    pub struct UptimeCheckIp {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IP address from which the Uptime check originates. This is a fully specified IP address (not an IP address range). Most IP addresses, as of this publication, are in IPv4 format; however, one should not rely on the IP addresses being in IPv4 format indefinitely, and should support interpreting this field in either IPv4 or IPv6 format."]
        pub ip_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A more specific location within the region that typically encodes a particular city/town/metro (and its containing state/province or country) within the broader umbrella region category."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A broad region category in which the IP address is located."]
        pub region: ::std::option::Option<UptimeCheckIpRegionEnum>,
    }
    impl UptimeCheckIp {
        pub fn builder() -> UptimeCheckIpBuilder {
            UptimeCheckIpBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "A broad region category in which the IP address is located."]
    pub enum UptimeCheckIpRegionEnum {
        #[serde(rename = "REGION_UNSPECIFIED")]
        #[doc = "Default value if no region is specified. Will result in Uptime checks running from all regions."]
        RegionUnspecified,
        #[serde(rename = "USA")]
        #[doc = "Allows checks to run from locations within the United States of America."]
        Usa,
        #[serde(rename = "EUROPE")]
        #[doc = "Allows checks to run from locations within the continent of Europe."]
        Europe,
        #[serde(rename = "SOUTH_AMERICA")]
        #[doc = "Allows checks to run from locations within the continent of South America."]
        SouthAmerica,
        #[serde(rename = "ASIA_PACIFIC")]
        #[doc = "Allows checks to run from locations within the Asia Pacific area (ex: Singapore)."]
        AsiaPacific,
    }
    impl ::std::default::Default for UptimeCheckIpRegionEnum {
        fn default() -> Self {
            Self::RegionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A descriptor for the value columns in a data point."]
    pub struct ValueDescriptor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value key."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricKind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value stream kind."]
        pub metric_kind: ::std::option::Option<ValueDescriptorMetricKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unit in which time_series point values are reported. unit follows the UCUM format for units as seen in https://unitsofmeasure.org/ucum.html. unit is only valid if value_type is INTEGER, DOUBLE, DISTRIBUTION."]
        pub unit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value type."]
        pub value_type: ::std::option::Option<ValueDescriptorValueTypeEnum>,
    }
    impl ValueDescriptor {
        pub fn builder() -> ValueDescriptorBuilder {
            ValueDescriptorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The value stream kind."]
    pub enum ValueDescriptorMetricKindEnum {
        #[serde(rename = "METRIC_KIND_UNSPECIFIED")]
        #[doc = "Do not use this default value."]
        MetricKindUnspecified,
        #[serde(rename = "GAUGE")]
        #[doc = "An instantaneous measurement of a value."]
        Gauge,
        #[serde(rename = "DELTA")]
        #[doc = "The change in a value during a time interval."]
        Delta,
        #[serde(rename = "CUMULATIVE")]
        #[doc = "A value accumulated over a time interval. Cumulative measurements in a time series should have the same start time and increasing end times, until an event resets the cumulative value to zero and sets a new start time for the following points."]
        Cumulative,
    }
    impl ::std::default::Default for ValueDescriptorMetricKindEnum {
        fn default() -> Self {
            Self::MetricKindUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The value type."]
    pub enum ValueDescriptorValueTypeEnum {
        #[serde(rename = "VALUE_TYPE_UNSPECIFIED")]
        #[doc = "Do not use this default value."]
        ValueTypeUnspecified,
        #[serde(rename = "BOOL")]
        #[doc = "The value is a boolean. This value type can be used only if the metric kind is GAUGE."]
        Bool,
        #[serde(rename = "INT64")]
        #[doc = "The value is a signed 64-bit integer."]
        Int64,
        #[serde(rename = "DOUBLE")]
        #[doc = "The value is a double precision floating point number."]
        Double,
        #[serde(rename = "STRING")]
        #[doc = "The value is a text string. This value type can be used only if the metric kind is GAUGE."]
        String,
        #[serde(rename = "DISTRIBUTION")]
        #[doc = "The value is a Distribution."]
        Distribution,
        #[serde(rename = "MONEY")]
        #[doc = "The value is money."]
        Money,
    }
    impl ::std::default::Default for ValueDescriptorValueTypeEnum {
        fn default() -> Self {
            Self::ValueTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The VerifyNotificationChannel request."]
    pub struct VerifyNotificationChannelRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The verification code that was delivered to the channel as a result of invoking the SendNotificationChannelVerificationCode API method or that was retrieved from a verified channel via GetNotificationChannelVerificationCode. For example, one might have \"G-123456\" or \"TKNZGhhd2EyN3I1MnRnMjRv\" (in general, one is only guaranteed that the code is valid UTF-8; one should not make any assumptions regarding the structure or format of the code)."]
        pub code: ::std::option::Option<::std::string::String>,
    }
    impl VerifyNotificationChannelRequest {
        pub fn builder() -> VerifyNotificationChannelRequestBuilder {
            VerifyNotificationChannelRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A WindowsBasedSli defines good_service as the count of time windows for which the provided service was of good quality. Criteria for determining if service was good are embedded in the window_criterion."]
    pub struct WindowsBasedSli {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "goodBadMetricFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) specifying a TimeSeries with ValueType = BOOL. The window is good if any true values appear in the window."]
        pub good_bad_metric_filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "goodTotalRatioThreshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A window is good if its performance is high enough."]
        pub good_total_ratio_threshold:
            ::std::option::Option<::std::boxed::Box<PerformanceThreshold>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricMeanInRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A window is good if the metric's value is in a good range, averaged across returned streams."]
        pub metric_mean_in_range: ::std::option::Option<::std::boxed::Box<MetricRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricSumInRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A window is good if the metric's value is in a good range, summed across returned streams."]
        pub metric_sum_in_range: ::std::option::Option<::std::boxed::Box<MetricRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "windowPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Duration over which window quality is evaluated. Must be an integer fraction of a day and at least 60s."]
        pub window_period: ::std::option::Option<::std::string::String>,
    }
    impl WindowsBasedSli {
        pub fn builder() -> WindowsBasedSliBuilder {
            WindowsBasedSliBuilder::default()
        }
    }
}
