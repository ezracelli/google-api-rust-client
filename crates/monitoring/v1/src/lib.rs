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
            pub mod dashboards {
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
                            #[doc = "A positive number that is the maximum number of results to return. If unspecified, a default of 1000 is used."]
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
        #[doc = "The alignment_period specifies a time interval, in seconds, that is used to divide the data in all the time series into consistent blocks of time. This will be done before the per-series aligner can be applied to the data.The value must be at least 60 seconds. If a per-series aligner other than ALIGN_NONE is specified, this field is required or an error is returned. If no per-series aligner is specified, or the aligner ALIGN_NONE is specified, then this field is ignored.The maximum value of the alignment_period is 2 years, or 104 weeks."]
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
    #[doc = "A chart axis."]
    pub struct Axis {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The label of the axis."]
        pub label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The axis scale. By default, a linear scale is used."]
        pub scale: ::std::option::Option<AxisScaleEnum>,
    }
    impl Axis {
        pub fn builder() -> AxisBuilder {
            AxisBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The axis scale. By default, a linear scale is used."]
    pub enum AxisScaleEnum {
        #[serde(rename = "SCALE_UNSPECIFIED")]
        #[doc = "Scale is unspecified. The view will default to LINEAR."]
        ScaleUnspecified,
        #[serde(rename = "LINEAR")]
        #[doc = "Linear scale."]
        Linear,
        #[serde(rename = "LOG10")]
        #[doc = "Logarithmic scale (base 10)."]
        Log10,
    }
    impl ::std::default::Default for AxisScaleEnum {
        fn default() -> Self {
            Self::ScaleUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options to control visual rendering of a chart."]
    pub struct ChartOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The chart mode."]
        pub mode: ::std::option::Option<ChartOptionsModeEnum>,
    }
    impl ChartOptions {
        pub fn builder() -> ChartOptionsBuilder {
            ChartOptionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The chart mode."]
    pub enum ChartOptionsModeEnum {
        #[serde(rename = "MODE_UNSPECIFIED")]
        #[doc = "Mode is unspecified. The view will default to COLOR."]
        ModeUnspecified,
        #[serde(rename = "COLOR")]
        #[doc = "The chart distinguishes data series using different color. Line colors may get reused when there are many lines in the chart."]
        Color,
        #[serde(rename = "X_RAY")]
        #[doc = "The chart uses the Stackdriver x-ray mode, in which each data set is plotted using the same semi-transparent color."]
        XRay,
        #[serde(rename = "STATS")]
        #[doc = "The chart displays statistics such as average, median, 95th percentile, and more."]
        Stats,
    }
    impl ::std::default::Default for ChartOptionsModeEnum {
        fn default() -> Self {
            Self::ModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines the layout properties and content for a column."]
    pub struct Column {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "weight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relative weight of this column. The column weight is used to adjust the width of columns on the screen (relative to peers). Greater the weight, greater the width of the column on the screen. If omitted, a value of 1 is used while rendering."]
        pub weight: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "widgets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display widgets arranged vertically in this column."]
        pub widgets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Widget>>>,
    }
    impl Column {
        pub fn builder() -> ColumnBuilder {
            ColumnBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A simplified layout that divides the available space into vertical columns and arranges a set of widgets vertically in each column."]
    pub struct ColumnLayout {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The columns of content to display."]
        pub columns: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Column>>>,
    }
    impl ColumnLayout {
        pub fn builder() -> ColumnLayoutBuilder {
            ColumnLayoutBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Google Stackdriver dashboard. Dashboards define the content and layout of pages in the Stackdriver web application."]
    pub struct Dashboard {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnLayout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content is divided into equally spaced columns and the widgets are arranged vertically."]
        pub column_layout: ::std::option::Option<::std::boxed::Box<ColumnLayout>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The mutable, human-readable name."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. An etag is returned in the response to GetDashboard, and users are expected to put that etag in the request to UpdateDashboard to ensure that their change will be applied to the same version of the Dashboard configuration. The field should not be passed during dashboard creation."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gridLayout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Content is arranged with a basic layout that re-flows a simple list of informational elements like widgets or tiles."]
        pub grid_layout: ::std::option::Option<::std::boxed::Box<GridLayout>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mosaicLayout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content is arranged as a grid of tiles, with each content widget occupying one or more grid blocks."]
        pub mosaic_layout: ::std::option::Option<::std::boxed::Box<MosaicLayout>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. The resource name of the dashboard."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowLayout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content is divided into equally spaced rows and the widgets are arranged horizontally."]
        pub row_layout: ::std::option::Option<::std::boxed::Box<RowLayout>>,
    }
    impl Dashboard {
        pub fn builder() -> DashboardBuilder {
            DashboardBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Groups a time series query definition with charting options."]
    pub struct DataSet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "legendTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A template string for naming TimeSeries in the resulting data set. This should be a string with interpolations of the form ${label_name}, which will resolve to the label's value."]
        pub legend_template: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minAlignmentPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The lower bound on data point frequency for this data set, implemented by specifying the minimum alignment period to use in a time series query For example, if the data is published once every 10 minutes, the min_alignment_period should be at least 10 minutes. It would not make sense to fetch and align data at one minute intervals."]
        pub min_alignment_period: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "plotType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How this data should be plotted on the chart."]
        pub plot_type: ::std::option::Option<DataSetPlotTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSeriesQuery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Fields for querying time series data from the Stackdriver metrics API."]
        pub time_series_query: ::std::option::Option<::std::boxed::Box<TimeSeriesQuery>>,
    }
    impl DataSet {
        pub fn builder() -> DataSetBuilder {
            DataSetBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How this data should be plotted on the chart."]
    pub enum DataSetPlotTypeEnum {
        #[serde(rename = "PLOT_TYPE_UNSPECIFIED")]
        #[doc = "Plot type is unspecified. The view will default to LINE."]
        PlotTypeUnspecified,
        #[serde(rename = "LINE")]
        #[doc = "The data is plotted as a set of lines (one line per series)."]
        Line,
        #[serde(rename = "STACKED_AREA")]
        #[doc = "The data is plotted as a set of filled areas (one area per series), with the areas stacked vertically (the base of each area is the top of its predecessor, and the base of the first area is the X axis). Since the areas do not overlap, each is filled with a different opaque color."]
        StackedArea,
        #[serde(rename = "STACKED_BAR")]
        #[doc = "The data is plotted as a set of rectangular boxes (one box per series), with the boxes stacked vertically (the base of each box is the top of its predecessor, and the base of the first box is the X axis). Since the boxes do not overlap, each is filled with a different opaque color."]
        StackedBar,
        #[serde(rename = "HEATMAP")]
        #[doc = "The data is plotted as a heatmap. The series being plotted must have a DISTRIBUTION value type. The value of each bucket in the distribution is displayed as a color. This type is not currently available in the Stackdriver Monitoring application."]
        Heatmap,
    }
    impl ::std::default::Default for DataSetPlotTypeEnum {
        fn default() -> Self {
            Self::PlotTypeUnspecified
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
    #[doc = "A gauge chart shows where the current value sits within a pre-defined range. The upper and lower bounds should define the possible range of values for the scorecard's query (inclusive)."]
    pub struct GaugeView {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lowerBound")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The lower bound for this gauge chart. The value of the chart should always be greater than or equal to this."]
        pub lower_bound: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upperBound")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The upper bound for this gauge chart. The value of the chart should always be less than or equal to this."]
        pub upper_bound: ::std::option::Option<::std::primitive::f64>,
    }
    impl GaugeView {
        pub fn builder() -> GaugeViewBuilder {
            GaugeViewBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A basic layout divides the available space into vertical columns of equal width and arranges a list of widgets using a row-first strategy."]
    pub struct GridLayout {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of columns into which the view's width is divided. If omitted or set to zero, a system default will be used while rendering."]
        pub columns: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "widgets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The informational elements that are arranged into the columns row-first."]
        pub widgets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Widget>>>,
    }
    impl GridLayout {
        pub fn builder() -> GridLayoutBuilder {
            GridLayoutBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The ListDashboards request."]
    pub struct ListDashboardsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dashboards")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of requested dashboards."]
        pub dashboards: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dashboard>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListDashboardsResponse {
        pub fn builder() -> ListDashboardsResponseBuilder {
            ListDashboardsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A mosaic layout divides the available space into a grid of blocks, and overlays the grid with tiles. Unlike GridLayout, tiles may span multiple grid blocks and can be placed at arbitrary locations in the grid."]
    pub struct MosaicLayout {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of columns in the mosaic grid. The number of columns must be between 1 and 12, inclusive."]
        pub columns: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tiles to display."]
        pub tiles: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Tile>>>,
    }
    impl MosaicLayout {
        pub fn builder() -> MosaicLayoutBuilder {
            MosaicLayoutBuilder::default()
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
    #[doc = "Describes a ranking-based time series filter. Each input time series is ranked with an aligner. The filter will allow up to num_time_series time series to pass through it, selecting them based on the relative ranking.For example, if ranking_method is METHOD_MEAN,direction is BOTTOM, and num_time_series is 3, then the 3 times series with the lowest mean values will pass through the filter."]
    pub struct PickTimeSeriesFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "direction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How to use the ranking to select time series that pass through the filter."]
        pub direction: ::std::option::Option<PickTimeSeriesFilterDirectionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numTimeSeries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How many time series to allow to pass through the filter."]
        pub num_time_series: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rankingMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ranking_method is applied to each time series independently to produce the value which will be used to compare the time series to other time series."]
        pub ranking_method: ::std::option::Option<PickTimeSeriesFilterRankingMethodEnum>,
    }
    impl PickTimeSeriesFilter {
        pub fn builder() -> PickTimeSeriesFilterBuilder {
            PickTimeSeriesFilterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How to use the ranking to select time series that pass through the filter."]
    pub enum PickTimeSeriesFilterDirectionEnum {
        #[serde(rename = "DIRECTION_UNSPECIFIED")]
        #[doc = "Not allowed. You must specify a different Direction if you specify a PickTimeSeriesFilter."]
        DirectionUnspecified,
        #[serde(rename = "TOP")]
        #[doc = "Pass the highest num_time_series ranking inputs."]
        Top,
        #[serde(rename = "BOTTOM")]
        #[doc = "Pass the lowest num_time_series ranking inputs."]
        Bottom,
    }
    impl ::std::default::Default for PickTimeSeriesFilterDirectionEnum {
        fn default() -> Self {
            Self::DirectionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "ranking_method is applied to each time series independently to produce the value which will be used to compare the time series to other time series."]
    pub enum PickTimeSeriesFilterRankingMethodEnum {
        #[serde(rename = "METHOD_UNSPECIFIED")]
        #[doc = "Not allowed. You must specify a different Method if you specify a PickTimeSeriesFilter."]
        MethodUnspecified,
        #[serde(rename = "METHOD_MEAN")]
        #[doc = "Select the mean of all values."]
        MethodMean,
        #[serde(rename = "METHOD_MAX")]
        #[doc = "Select the maximum value."]
        MethodMax,
        #[serde(rename = "METHOD_MIN")]
        #[doc = "Select the minimum value."]
        MethodMin,
        #[serde(rename = "METHOD_SUM")]
        #[doc = "Compute the sum of all values."]
        MethodSum,
        #[serde(rename = "METHOD_LATEST")]
        #[doc = "Select the most recent value."]
        MethodLatest,
    }
    impl ::std::default::Default for PickTimeSeriesFilterRankingMethodEnum {
        fn default() -> Self {
            Self::MethodUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a query to build the numerator or denominator of a TimeSeriesFilterRatio."]
    pub struct RatioPart {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aggregation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "By default, the raw time series data is returned. Use this field to combine multiple time series for different views of the data."]
        pub aggregation: ::std::option::Option<::std::boxed::Box<Aggregation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) that identifies the metric types, resources, and projects to query."]
        pub filter: ::std::option::Option<::std::string::String>,
    }
    impl RatioPart {
        pub fn builder() -> RatioPartBuilder {
            RatioPartBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines the layout properties and content for a row."]
    pub struct Row {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "weight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relative weight of this row. The row weight is used to adjust the height of rows on the screen (relative to peers). Greater the weight, greater the height of the row on the screen. If omitted, a value of 1 is used while rendering."]
        pub weight: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "widgets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display widgets arranged horizontally in this row."]
        pub widgets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Widget>>>,
    }
    impl Row {
        pub fn builder() -> RowBuilder {
            RowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A simplified layout that divides the available space into rows and arranges a set of widgets horizontally in each row."]
    pub struct RowLayout {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rows of content to display."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Row>>>,
    }
    impl RowLayout {
        pub fn builder() -> RowLayoutBuilder {
            RowLayoutBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A widget showing the latest value of a metric, and how this value relates to one or more thresholds."]
    pub struct Scorecard {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gaugeView")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Will cause the scorecard to show a gauge chart."]
        pub gauge_view: ::std::option::Option<::std::boxed::Box<GaugeView>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sparkChartView")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Will cause the scorecard to show a spark chart."]
        pub spark_chart_view: ::std::option::Option<::std::boxed::Box<SparkChartView>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thresholds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The thresholds used to determine the state of the scorecard given the time series' current value. For an actual value x, the scorecard is in a danger state if x is less than or equal to a danger threshold that triggers below, or greater than or equal to a danger threshold that triggers above. Similarly, if x is above/below a warning threshold that triggers above/below, then the scorecard is in a warning state - unless x also puts it in a danger state. (Danger trumps warning.)As an example, consider a scorecard with the following four thresholds: { value: 90, category: 'DANGER', trigger: 'ABOVE', }, { value: 70, category: 'WARNING', trigger: 'ABOVE', }, { value: 10, category: 'DANGER', trigger: 'BELOW', }, { value: 20, category: 'WARNING', trigger: 'BELOW', }Then: values less than or equal to 10 would put the scorecard in a DANGER state, values greater than 10 but less than or equal to 20 a WARNING state, values strictly between 20 and 70 an OK state, values greater than or equal to 70 but less than 90 a WARNING state, and values greater than or equal to 90 a DANGER state."]
        pub thresholds: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Threshold>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSeriesQuery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Fields for querying time series data from the Stackdriver metrics API."]
        pub time_series_query: ::std::option::Option<::std::boxed::Box<TimeSeriesQuery>>,
    }
    impl Scorecard {
        pub fn builder() -> ScorecardBuilder {
            ScorecardBuilder::default()
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
    #[doc = "A sparkChart is a small chart suitable for inclusion in a table-cell or inline in text. This message contains the configuration for a sparkChart to show up on a Scorecard, showing recent trends of the scorecard's timeseries."]
    pub struct SparkChartView {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minAlignmentPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The lower bound on data point frequency in the chart implemented by specifying the minimum alignment period to use in a time series query. For example, if the data is published once every 10 minutes it would not make sense to fetch and align data at one minute intervals. This field is optional and exists only as a hint."]
        pub min_alignment_period: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sparkChartType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The type of sparkchart to show in this chartView."]
        pub spark_chart_type: ::std::option::Option<SparkChartViewSparkChartTypeEnum>,
    }
    impl SparkChartView {
        pub fn builder() -> SparkChartViewBuilder {
            SparkChartViewBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The type of sparkchart to show in this chartView."]
    pub enum SparkChartViewSparkChartTypeEnum {
        #[serde(rename = "SPARK_CHART_TYPE_UNSPECIFIED")]
        #[doc = "Not allowed in well-formed requests."]
        SparkChartTypeUnspecified,
        #[serde(rename = "SPARK_LINE")]
        #[doc = "The sparkline will be rendered as a small line chart."]
        SparkLine,
        #[serde(rename = "SPARK_BAR")]
        #[doc = "The sparkbar will be rendered as a small bar chart."]
        SparkBar,
    }
    impl ::std::default::Default for SparkChartViewSparkChartTypeEnum {
        fn default() -> Self {
            Self::SparkChartTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A filter that ranks streams based on their statistical relation to other streams in a request. Note: This field is deprecated and completely ignored by the API."]
    pub struct StatisticalTimeSeriesFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numTimeSeries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How many time series to output."]
        pub num_time_series: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rankingMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "rankingMethod is applied to a set of time series, and then the produced value for each individual time series is used to compare a given time series to others. These are methods that cannot be applied stream-by-stream, but rather require the full context of a request to evaluate time series."]
        pub ranking_method: ::std::option::Option<StatisticalTimeSeriesFilterRankingMethodEnum>,
    }
    impl StatisticalTimeSeriesFilter {
        pub fn builder() -> StatisticalTimeSeriesFilterBuilder {
            StatisticalTimeSeriesFilterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "rankingMethod is applied to a set of time series, and then the produced value for each individual time series is used to compare a given time series to others. These are methods that cannot be applied stream-by-stream, but rather require the full context of a request to evaluate time series."]
    pub enum StatisticalTimeSeriesFilterRankingMethodEnum {
        #[serde(rename = "METHOD_UNSPECIFIED")]
        #[doc = "Not allowed in well-formed requests."]
        MethodUnspecified,
        #[serde(rename = "METHOD_CLUSTER_OUTLIER")]
        #[doc = "Compute the outlier score of each stream."]
        MethodClusterOutlier,
    }
    impl ::std::default::Default for StatisticalTimeSeriesFilterRankingMethodEnum {
        fn default() -> Self {
            Self::MethodUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A widget that displays textual content."]
    pub struct Text {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text content to be displayed."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "format")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How the text content is formatted."]
        pub format: ::std::option::Option<TextFormatEnum>,
    }
    impl Text {
        pub fn builder() -> TextBuilder {
            TextBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How the text content is formatted."]
    pub enum TextFormatEnum {
        #[serde(rename = "FORMAT_UNSPECIFIED")]
        #[doc = "Format is unspecified. Defaults to MARKDOWN."]
        FormatUnspecified,
        #[serde(rename = "MARKDOWN")]
        #[doc = "The text contains Markdown formatting."]
        Markdown,
        #[serde(rename = "RAW")]
        #[doc = "The text contains no special formatting."]
        Raw,
    }
    impl ::std::default::Default for TextFormatEnum {
        fn default() -> Self {
            Self::FormatUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines a threshold for categorizing time series values."]
    pub struct Threshold {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state color for this threshold. Color is not allowed in a XyChart."]
        pub color: ::std::option::Option<ThresholdColorEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "direction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The direction for the current threshold. Direction is not allowed in a XyChart."]
        pub direction: ::std::option::Option<ThresholdDirectionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A label for the threshold."]
        pub label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of the threshold. The value should be defined in the native scale of the metric."]
        pub value: ::std::option::Option<::std::primitive::f64>,
    }
    impl Threshold {
        pub fn builder() -> ThresholdBuilder {
            ThresholdBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state color for this threshold. Color is not allowed in a XyChart."]
    pub enum ThresholdColorEnum {
        #[serde(rename = "COLOR_UNSPECIFIED")]
        #[doc = "Color is unspecified. Not allowed in well-formed requests."]
        ColorUnspecified,
        #[serde(rename = "YELLOW")]
        #[doc = "Crossing the threshold is \"concerning\" behavior."]
        Yellow,
        #[serde(rename = "RED")]
        #[doc = "Crossing the threshold is \"emergency\" behavior."]
        Red,
    }
    impl ::std::default::Default for ThresholdColorEnum {
        fn default() -> Self {
            Self::ColorUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The direction for the current threshold. Direction is not allowed in a XyChart."]
    pub enum ThresholdDirectionEnum {
        #[serde(rename = "DIRECTION_UNSPECIFIED")]
        #[doc = "Not allowed in well-formed requests."]
        DirectionUnspecified,
        #[serde(rename = "ABOVE")]
        #[doc = "The threshold will be considered crossed if the actual value is above the threshold value."]
        Above,
        #[serde(rename = "BELOW")]
        #[doc = "The threshold will be considered crossed if the actual value is below the threshold value."]
        Below,
    }
    impl ::std::default::Default for ThresholdDirectionEnum {
        fn default() -> Self {
            Self::DirectionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single tile in the mosaic. The placement and size of the tile are configurable."]
    pub struct Tile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The height of the tile, measured in grid blocks. Tiles must have a minimum height of 1."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "widget")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The informational widget contained in the tile. For example an XyChart."]
        pub widget: ::std::option::Option<::std::boxed::Box<Widget>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the tile, measured in grid blocks. Tiles must have a minimum width of 1."]
        pub width: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "xPos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zero-indexed position of the tile in grid blocks relative to the left edge of the grid. Tiles must be contained within the specified number of columns. x_pos cannot be negative."]
        pub x_pos: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "yPos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zero-indexed position of the tile in grid blocks relative to the top edge of the grid. y_pos cannot be negative."]
        pub y_pos: ::std::option::Option<::std::primitive::i64>,
    }
    impl Tile {
        pub fn builder() -> TileBuilder {
            TileBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A filter that defines a subset of time series data that is displayed in a widget. Time series data is fetched using the ListTimeSeries (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list) method."]
    pub struct TimeSeriesFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aggregation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "By default, the raw time series data is returned. Use this field to combine multiple time series for different views of the data."]
        pub aggregation: ::std::option::Option<::std::boxed::Box<Aggregation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The monitoring filter (https://cloud.google.com/monitoring/api/v3/filters) that identifies the metric types, resources, and projects to query."]
        pub filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pickTimeSeriesFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ranking based time series filter."]
        pub pick_time_series_filter: ::std::option::Option<::std::boxed::Box<PickTimeSeriesFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secondaryAggregation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Apply a second aggregation after aggregation is applied."]
        pub secondary_aggregation: ::std::option::Option<::std::boxed::Box<Aggregation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statisticalTimeSeriesFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Statistics based time series filter. Note: This field is deprecated and completely ignored by the API."]
        pub statistical_time_series_filter:
            ::std::option::Option<::std::boxed::Box<StatisticalTimeSeriesFilter>>,
    }
    impl TimeSeriesFilter {
        pub fn builder() -> TimeSeriesFilterBuilder {
            TimeSeriesFilterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A pair of time series filters that define a ratio computation. The output time series is the pair-wise division of each aligned element from the numerator and denominator time series."]
    pub struct TimeSeriesFilterRatio {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "denominator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The denominator of the ratio."]
        pub denominator: ::std::option::Option<::std::boxed::Box<RatioPart>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numerator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The numerator of the ratio."]
        pub numerator: ::std::option::Option<::std::boxed::Box<RatioPart>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pickTimeSeriesFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ranking based time series filter."]
        pub pick_time_series_filter: ::std::option::Option<::std::boxed::Box<PickTimeSeriesFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secondaryAggregation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Apply a second aggregation after the ratio is computed."]
        pub secondary_aggregation: ::std::option::Option<::std::boxed::Box<Aggregation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statisticalTimeSeriesFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Statistics based time series filter. Note: This field is deprecated and completely ignored by the API."]
        pub statistical_time_series_filter:
            ::std::option::Option<::std::boxed::Box<StatisticalTimeSeriesFilter>>,
    }
    impl TimeSeriesFilterRatio {
        pub fn builder() -> TimeSeriesFilterRatioBuilder {
            TimeSeriesFilterRatioBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "TimeSeriesQuery collects the set of supported methods for querying time series data from the Stackdriver metrics API."]
    pub struct TimeSeriesQuery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSeriesFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Filter parameters to fetch time series."]
        pub time_series_filter: ::std::option::Option<::std::boxed::Box<TimeSeriesFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSeriesFilterRatio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters to fetch a ratio between two time series filters."]
        pub time_series_filter_ratio:
            ::std::option::Option<::std::boxed::Box<TimeSeriesFilterRatio>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSeriesQueryLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A query used to fetch time series."]
        pub time_series_query_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unitOverride")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unit of data contained in fetched time series. If non-empty, this unit will override any unit that accompanies fetched data. The format is the same as the unit (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.metricDescriptors) field in MetricDescriptor."]
        pub unit_override: ::std::option::Option<::std::string::String>,
    }
    impl TimeSeriesQuery {
        pub fn builder() -> TimeSeriesQueryBuilder {
            TimeSeriesQueryBuilder::default()
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
    #[doc = "Widget contains a single dashboard component and configuration of how to present the component in the dashboard."]
    pub struct Widget {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blank")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A blank space."]
        pub blank: ::std::option::Option<::std::boxed::Box<Empty>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scorecard")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A scorecard summarizing time series data."]
        pub scorecard: ::std::option::Option<::std::boxed::Box<Scorecard>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A raw string or markdown displaying textual content."]
        pub text: ::std::option::Option<::std::boxed::Box<Text>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The title of the widget."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "xyChart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A chart of time series data."]
        pub xy_chart: ::std::option::Option<::std::boxed::Box<XyChart>>,
    }
    impl Widget {
        pub fn builder() -> WidgetBuilder {
            WidgetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A chart that displays data on a 2D (X and Y axes) plane."]
    pub struct XyChart {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chartOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display options for the chart."]
        pub chart_options: ::std::option::Option<::std::boxed::Box<ChartOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The data displayed in this chart."]
        pub data_sets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataSet>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thresholds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Threshold lines drawn horizontally across the chart."]
        pub thresholds: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Threshold>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeshiftDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The duration used to display a comparison chart. A comparison chart simultaneously shows values from two similar-length time periods (e.g., week-over-week metrics). The duration must be positive, and it can only be applied to charts with data sets of LINE plot type."]
        pub timeshift_duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "xAxis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties applied to the X axis."]
        pub x_axis: ::std::option::Option<::std::boxed::Box<Axis>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "yAxis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties applied to the Y axis."]
        pub y_axis: ::std::option::Option<::std::boxed::Box<Axis>>,
    }
    impl XyChart {
        pub fn builder() -> XyChartBuilder {
            XyChartBuilder::default()
        }
    }
}
