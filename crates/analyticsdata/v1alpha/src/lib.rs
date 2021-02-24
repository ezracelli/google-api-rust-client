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
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The batch request containing multiple pivot report requests."]
    pub struct BatchRunPivotReportsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A property whose events are tracked. This entity must be specified for the batch. The entity within RunPivotReportRequest may either be unspecified or consistent with this entity."]
        pub entity: ::std::option::Option<::std::boxed::Box<Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Individual requests. Each request has a separate pivot report response. Each batch request is allowed up to 5 requests."]
        pub requests:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RunPivotReportRequest>>>,
    }
    impl BatchRunPivotReportsRequest {
        pub fn builder() -> BatchRunPivotReportsRequestBuilder {
            BatchRunPivotReportsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The batch response containing multiple pivot reports."]
    pub struct BatchRunPivotReportsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pivotReports")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Individual responses. Each response has a separate pivot report request."]
        pub pivot_reports:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RunPivotReportResponse>>>,
    }
    impl BatchRunPivotReportsResponse {
        pub fn builder() -> BatchRunPivotReportsResponseBuilder {
            BatchRunPivotReportsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The batch request containing multiple report requests."]
    pub struct BatchRunReportsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A property whose events are tracked. This entity must be specified for the batch. The entity within RunReportRequest may either be unspecified or consistent with this entity."]
        pub entity: ::std::option::Option<::std::boxed::Box<Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Individual requests. Each request has a separate report response. Each batch request is allowed up to 5 requests."]
        pub requests: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RunReportRequest>>>,
    }
    impl BatchRunReportsRequest {
        pub fn builder() -> BatchRunReportsRequestBuilder {
            BatchRunReportsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The batch response containing multiple reports."]
    pub struct BatchRunReportsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reports")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Individual responses. Each response has a separate report request."]
        pub reports: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RunReportResponse>>>,
    }
    impl BatchRunReportsResponse {
        pub fn builder() -> BatchRunReportsResponseBuilder {
            BatchRunReportsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "To express that the result needs to be between two numbers (inclusive)."]
    pub struct BetweenFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fromValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Begins with this number."]
        pub from_value: ::std::option::Option<::std::boxed::Box<NumericValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "toValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ends with this number."]
        pub to_value: ::std::option::Option<::std::boxed::Box<NumericValue>>,
    }
    impl BetweenFilter {
        pub fn builder() -> BetweenFilterBuilder {
            BetweenFilterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Used to convert a dimension value to a single case."]
    pub struct CaseExpression {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of a dimension. The name must refer back to a name in dimensions field of the request."]
        pub dimension_name: ::std::option::Option<::std::string::String>,
    }
    impl CaseExpression {
        pub fn builder() -> CaseExpressionBuilder {
            CaseExpressionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines a cohort selection criteria. A cohort is a group of users who share a common characteristic. For example, users with the same `firstSessionDate` belong to the same cohort."]
    pub struct Cohort {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cohort selects users whose first touch date is between start date and end date defined in the `dateRange`. This `dateRange` does not specify the full date range of event data that is present in a cohort report. In a cohort report, this `dateRange` is extended by the granularity and offset present in the `cohortsRange`; event data for the extended reporting date range is present in a cohort report. In a cohort request, this `dateRange` is required and the `dateRanges` in the `RunReportRequest` or `RunPivotReportRequest` must be unspecified. This `dateRange` should generally be aligned with the cohort's granularity. If `CohortsRange` uses daily granularity, this `dateRange` can be a single day. If `CohortsRange` uses weekly granularity, this `dateRange` can be aligned to a week boundary, starting at Sunday and ending Saturday. If `CohortsRange` uses monthly granularity, this `dateRange` can be aligned to a month, starting at the first and ending on the last day of the month."]
        pub date_range: ::std::option::Option<::std::boxed::Box<DateRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dimension used by the cohort. Required and only supports `firstSessionDate`."]
        pub dimension: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Assigns a name to this cohort. The dimension `cohort` is valued to this name in a report response. If set, cannot begin with `cohort_` or `RESERVED_`. If not set, cohorts are named by their zero based index `cohort_0`, `cohort_1`, etc."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Cohort {
        pub fn builder() -> CohortBuilder {
            CohortBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Optional settings of a cohort report."]
    pub struct CohortReportSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accumulate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, accumulates the result from first touch day to the end day. Not supported in `RunReportRequest`."]
        pub accumulate: ::std::option::Option<::std::primitive::bool>,
    }
    impl CohortReportSettings {
        pub fn builder() -> CohortReportSettingsBuilder {
            CohortReportSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The specification of cohorts for a cohort report. Cohort reports create a time series of user retention for the cohort. For example, you could select the cohort of users that were acquired in the first week of September and follow that cohort for the next six weeks. Selecting the users acquired in the first week of September cohort is specified in the `cohort` object. Following that cohort for the next six weeks is specified in the `cohortsRange` object. For examples, see [Cohort Report Examples](https://developers.google.com/analytics/devguides/reporting/data/v1/advanced#cohort_report_examples). The report response could show a weekly time series where say your app has retained 60% of this cohort after three weeks and 25% of this cohort after six weeks. These two percentages can be calculated by the metric `cohortActiveUsers/cohortTotalUsers` and will be separate rows in the report."]
    pub struct CohortSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cohortReportSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional settings for a cohort report."]
        pub cohort_report_settings: ::std::option::Option<::std::boxed::Box<CohortReportSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cohorts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the selection criteria to group users into cohorts. Most cohort reports define only a single cohort. If multiple cohorts are specified, each cohort can be recognized in the report by their name."]
        pub cohorts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Cohort>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cohortsRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cohort reports follow cohorts over an extended reporting date range. This range specifies an offset duration to follow the cohorts over."]
        pub cohorts_range: ::std::option::Option<::std::boxed::Box<CohortsRange>>,
    }
    impl CohortSpec {
        pub fn builder() -> CohortSpecBuilder {
            CohortSpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configures the extended reporting date range for a cohort report. Specifies an offset duration to follow the cohorts over."]
    pub struct CohortsRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. `endOffset` specifies the end date of the extended reporting date range for a cohort report. `endOffset` can be any positive integer but is commonly set to 5 to 10 so that reports contain data on the cohort for the next several granularity time periods. If `granularity` is `DAILY`, the `endDate` of the extended reporting date range is `endDate` of the cohort plus `endOffset` days. If `granularity` is `WEEKLY`, the `endDate` of the extended reporting date range is `endDate` of the cohort plus `endOffset * 7` days. If `granularity` is `MONTHLY`, the `endDate` of the extended reporting date range is `endDate` of the cohort plus `endOffset * 30` days."]
        pub end_offset: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "granularity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The granularity used to interpret the `startOffset` and `endOffset` for the extended reporting date range for a cohort report."]
        pub granularity: ::std::option::Option<CohortsRangeGranularityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "`startOffset` specifies the start date of the extended reporting date range for a cohort report. `startOffset` is commonly set to 0 so that reports contain data from the acquisition of the cohort forward. If `granularity` is `DAILY`, the `startDate` of the extended reporting date range is `startDate` of the cohort plus `startOffset` days. If `granularity` is `WEEKLY`, the `startDate` of the extended reporting date range is `startDate` of the cohort plus `startOffset * 7` days. If `granularity` is `MONTHLY`, the `startDate` of the extended reporting date range is `startDate` of the cohort plus `startOffset * 30` days."]
        pub start_offset: ::std::option::Option<::std::primitive::i64>,
    }
    impl CohortsRange {
        pub fn builder() -> CohortsRangeBuilder {
            CohortsRangeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The granularity used to interpret the `startOffset` and `endOffset` for the extended reporting date range for a cohort report."]
    pub enum CohortsRangeGranularityEnum {
        #[serde(rename = "GRANULARITY_UNSPECIFIED")]
        #[doc = "Should never be specified."]
        GranularityUnspecified,
        #[serde(rename = "DAILY")]
        #[doc = "Daily granularity. Commonly used if the cohort's `dateRange` is a single day and the request contains `cohortNthDay`."]
        Daily,
        #[serde(rename = "WEEKLY")]
        #[doc = "Weekly granularity. Commonly used if the cohort's `dateRange` is a week in duration (starting on Sunday and ending on Saturday) and the request contains `cohortNthWeek`."]
        Weekly,
        #[serde(rename = "MONTHLY")]
        #[doc = "Monthly granularity. Commonly used if the cohort's `dateRange` is a month in duration and the request contains `cohortNthMonth`."]
        Monthly,
    }
    impl ::std::default::Default for CohortsRangeGranularityEnum {
        fn default() -> Self {
            Self::GranularityUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Used to combine dimension values to a single dimension."]
    pub struct ConcatenateExpression {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delimiter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The delimiter placed between dimension names. Delimiters are often single characters such as \"|\" or \",\" but can be longer strings. If a dimension value contains the delimiter, both will be present in response with no distinction. For example if dimension 1 value = \"US,FR\", dimension 2 value = \"JP\", and delimiter = \",\", then the response will contain \"US,FR,JP\"."]
        pub delimiter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Names of dimensions. The names must refer back to names in the dimensions field of the request."]
        pub dimension_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ConcatenateExpression {
        pub fn builder() -> ConcatenateExpressionBuilder {
            ConcatenateExpressionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A contiguous set of days: startDate, startDate + 1, ..., endDate. Requests are allowed up to 4 date ranges."]
    pub struct DateRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The inclusive end date for the query in the format `YYYY-MM-DD`. Cannot be before `start_date`. The format `NdaysAgo`, `yesterday`, or `today` is also accepted, and in that case, the date is inferred based on the property's reporting time zone."]
        pub end_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Assigns a name to this date range. The dimension `dateRange` is valued to this name in a report response. If set, cannot begin with `date_range_` or `RESERVED_`. If not set, date ranges are named by their zero based index in the request: `date_range_0`, `date_range_1`, etc."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The inclusive start date for the query in the format `YYYY-MM-DD`. Cannot be after `end_date`. The format `NdaysAgo`, `yesterday`, or `today` is also accepted, and in that case, the date is inferred based on the property's reporting time zone."]
        pub start_date: ::std::option::Option<::std::string::String>,
    }
    impl DateRange {
        pub fn builder() -> DateRangeBuilder {
            DateRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Dimensions are attributes of your data. For example, the dimension city indicates the city from which an event originates. Dimension values in report responses are strings; for example, city could be \"Paris\" or \"New York\". Requests are allowed up to 8 dimensions."]
    pub struct Dimension {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionExpression")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "One dimension can be the result of an expression of multiple dimensions. For example, dimension \"country, city\": concatenate(country, \", \", city)."]
        pub dimension_expression: ::std::option::Option<::std::boxed::Box<DimensionExpression>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the dimension. See the [API Dimensions](https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema#dimensions) for the list of dimension names. If `dimensionExpression` is specified, `name` can be any string that you would like. For example if a `dimensionExpression` concatenates `country` and `city`, you could call that dimension `countryAndCity`. Dimensions are referenced by `name` in `dimensionFilter`, `orderBys`, `dimensionExpression`, and `pivots`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Dimension {
        pub fn builder() -> DimensionBuilder {
            DimensionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Used to express a dimension which is the result of a formula of multiple dimensions. Example usages: 1) lower_case(dimension) 2) concatenate(dimension1, symbol, dimension2)."]
    pub struct DimensionExpression {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "concatenate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Used to combine dimension values to a single dimension. For example, dimension \"country, city\": concatenate(country, \", \", city)."]
        pub concatenate: ::std::option::Option<::std::boxed::Box<ConcatenateExpression>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lowerCase")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Used to convert a dimension value to lower case."]
        pub lower_case: ::std::option::Option<::std::boxed::Box<CaseExpression>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upperCase")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Used to convert a dimension value to upper case."]
        pub upper_case: ::std::option::Option<::std::boxed::Box<CaseExpression>>,
    }
    impl DimensionExpression {
        pub fn builder() -> DimensionExpressionBuilder {
            DimensionExpressionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a dimension column in the report. Dimensions requested in a report produce column entries within rows and DimensionHeaders. However, dimensions used exclusively within filters or expressions do not produce columns in a report; correspondingly, those dimensions do not produce headers."]
    pub struct DimensionHeader {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimension's name."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl DimensionHeader {
        pub fn builder() -> DimensionHeaderBuilder {
            DimensionHeaderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Explains a dimension."]
    pub struct DimensionMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This dimension's name. Useable in [Dimension](#Dimension)'s `name`. For example, `eventName`."]
        pub api_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customDefinition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the dimension is a custom dimension for this property."]
        pub custom_definition: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deprecatedApiNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Still usable but deprecated names for this dimension. If populated, this dimension is available by either `apiName` or one of `deprecatedApiNames` for a period of time. After the deprecation period, the dimension will be available only by `apiName`."]
        pub deprecated_api_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of how this dimension is used and calculated."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uiName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This dimension's name within the Google Analytics user interface. For example, `Event name`."]
        pub ui_name: ::std::option::Option<::std::string::String>,
    }
    impl DimensionMetadata {
        pub fn builder() -> DimensionMetadataBuilder {
            DimensionMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Sorts by dimension values."]
    pub struct DimensionOrderBy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A dimension name in the request to order by."]
        pub dimension_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orderType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Controls the rule for dimension value ordering."]
        pub order_type: ::std::option::Option<DimensionOrderByOrderTypeEnum>,
    }
    impl DimensionOrderBy {
        pub fn builder() -> DimensionOrderByBuilder {
            DimensionOrderByBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Controls the rule for dimension value ordering."]
    pub enum DimensionOrderByOrderTypeEnum {
        #[serde(rename = "ORDER_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified."]
        OrderTypeUnspecified,
        #[serde(rename = "ALPHANUMERIC")]
        #[doc = "Alphanumeric sort by Unicode code point. For example, \"2\" < \"A\" < \"X\" < \"b\" < \"z\"."]
        Alphanumeric,
        #[serde(rename = "CASE_INSENSITIVE_ALPHANUMERIC")]
        #[doc = "Case insensitive alphanumeric sort by lower case Unicode code point. For example, \"2\" < \"A\" < \"b\" < \"X\" < \"z\"."]
        CaseInsensitiveAlphanumeric,
        #[serde(rename = "NUMERIC")]
        #[doc = "Dimension values are converted to numbers before sorting. For example in NUMERIC sort, \"25\" < \"100\", and in `ALPHANUMERIC` sort, \"100\" < \"25\". Non-numeric dimension values all have equal ordering value below all numeric values."]
        Numeric,
    }
    impl ::std::default::Default for DimensionOrderByOrderTypeEnum {
        fn default() -> Self {
            Self::OrderTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The value of a dimension."]
    pub struct DimensionValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value as a string if the dimension type is a string."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl DimensionValue {
        pub fn builder() -> DimensionValueBuilder {
            DimensionValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The unique identifier of the property whose events are tracked."]
    pub struct Entity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "propertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Google Analytics GA4 property id. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id)."]
        pub property_id: ::std::option::Option<::std::string::String>,
    }
    impl Entity {
        pub fn builder() -> EntityBuilder {
            EntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An expression to filter dimension or metric values."]
    pub struct Filter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "betweenFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A filter for two values."]
        pub between_filter: ::std::option::Option<::std::boxed::Box<BetweenFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimension name or metric name. Must be a name defined in dimensions or metrics."]
        pub field_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inListFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A filter for in list values."]
        pub in_list_filter: ::std::option::Option<::std::boxed::Box<InListFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numericFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A filter for numeric or date values."]
        pub numeric_filter: ::std::option::Option<::std::boxed::Box<NumericFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Strings related filter."]
        pub string_filter: ::std::option::Option<::std::boxed::Box<StringFilter>>,
    }
    impl Filter {
        pub fn builder() -> FilterBuilder {
            FilterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "To express dimension or metric filters. The fields in the same FilterExpression need to be either all dimensions or all metrics."]
    pub struct FilterExpression {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "andGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The FilterExpressions in and_group have an AND relationship."]
        pub and_group: ::std::option::Option<::std::boxed::Box<FilterExpressionList>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A primitive filter. All fields in filter in same FilterExpression needs to be either all dimensions or metrics."]
        pub filter: ::std::option::Option<::std::boxed::Box<Filter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notExpression")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The FilterExpression is NOT of not_expression."]
        pub not_expression: ::std::option::Option<::std::boxed::Box<FilterExpression>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The FilterExpressions in or_group have an OR relationship."]
        pub or_group: ::std::option::Option<::std::boxed::Box<FilterExpressionList>>,
    }
    impl FilterExpression {
        pub fn builder() -> FilterExpressionBuilder {
            FilterExpressionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of filter expressions."]
    pub struct FilterExpressionList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expressions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of filter expressions."]
        pub expressions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FilterExpression>>>,
    }
    impl FilterExpressionList {
        pub fn builder() -> FilterExpressionListBuilder {
            FilterExpressionListBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result needs to be in a list of string values."]
    pub struct InListFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "caseSensitive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, the string value is case sensitive."]
        pub case_sensitive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of string values. Must be non-empty."]
        pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl InListFilter {
        pub fn builder() -> InListFilterBuilder {
            InListFilterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The dimensions and metrics currently accepted in reporting methods."]
    pub struct Metadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimension descriptions."]
        pub dimensions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionMetadata>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metric descriptions."]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricMetadata>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of this metadata."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Metadata {
        pub fn builder() -> MetadataBuilder {
            MetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The quantitative measurements of a report. For example, the metric `eventCount` is the total number of events. Requests are allowed up to 10 metrics."]
    pub struct Metric {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expression")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A mathematical expression for derived metrics. For example, the metric Event count per user is `eventCount/totalUsers`."]
        pub expression: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invisible")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if a metric is invisible in the report response. If a metric is invisible, the metric will not produce a column in the response, but can be used in `metricFilter`, `orderBys`, or a metric `expression`."]
        pub invisible: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the metric. See the [API Metrics](https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema#metrics) for the list of metric names. If `expression` is specified, `name` can be any string that you would like. For example if `expression` is `screenPageViews/sessions`, you could call that metric's name = `viewsPerSession`. Metrics are referenced by `name` in `metricFilter`, `orderBys`, and metric `expression`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Metric {
        pub fn builder() -> MetricBuilder {
            MetricBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a metric column in the report. Visible metrics requested in a report produce column entries within rows and MetricHeaders. However, metrics used exclusively within filters or expressions do not produce columns in a report; correspondingly, those metrics do not produce headers."]
    pub struct MetricHeader {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metric's name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metric's data type."]
        pub _type: ::std::option::Option<MetricHeaderTypeEnum>,
    }
    impl MetricHeader {
        pub fn builder() -> MetricHeaderBuilder {
            MetricHeaderBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The metric's data type."]
    pub enum MetricHeaderTypeEnum {
        #[serde(rename = "METRIC_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified type."]
        MetricTypeUnspecified,
        #[serde(rename = "TYPE_INTEGER")]
        #[doc = "Integer type."]
        TypeInteger,
        #[serde(rename = "TYPE_FLOAT")]
        #[doc = "Floating point type."]
        TypeFloat,
        #[serde(rename = "TYPE_SECONDS")]
        #[doc = "A duration of seconds; a special floating point type."]
        TypeSeconds,
        #[serde(rename = "TYPE_MILLISECONDS")]
        #[doc = "A duration in milliseconds; a special floating point type."]
        TypeMilliseconds,
        #[serde(rename = "TYPE_MINUTES")]
        #[doc = "A duration in minutes; a special floating point type."]
        TypeMinutes,
        #[serde(rename = "TYPE_HOURS")]
        #[doc = "A duration in hours; a special floating point type."]
        TypeHours,
        #[serde(rename = "TYPE_STANDARD")]
        #[doc = "A custom metric of standard type; a special floating point type."]
        TypeStandard,
        #[serde(rename = "TYPE_CURRENCY")]
        #[doc = "An amount of money; a special floating point type."]
        TypeCurrency,
        #[serde(rename = "TYPE_FEET")]
        #[doc = "A length in feet; a special floating point type."]
        TypeFeet,
        #[serde(rename = "TYPE_MILES")]
        #[doc = "A length in miles; a special floating point type."]
        TypeMiles,
        #[serde(rename = "TYPE_METERS")]
        #[doc = "A length in meters; a special floating point type."]
        TypeMeters,
        #[serde(rename = "TYPE_KILOMETERS")]
        #[doc = "A length in kilometers; a special floating point type."]
        TypeKilometers,
    }
    impl ::std::default::Default for MetricHeaderTypeEnum {
        fn default() -> Self {
            Self::MetricTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Explains a metric."]
    pub struct MetricMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A metric name. Useable in [Metric](#Metric)'s `name`. For example, `eventCount`."]
        pub api_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customDefinition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the metric is a custom metric for this property."]
        pub custom_definition: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deprecatedApiNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Still usable but deprecated names for this metric. If populated, this metric is available by either `apiName` or one of `deprecatedApiNames` for a period of time. After the deprecation period, the metric will be available only by `apiName`."]
        pub deprecated_api_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of how this metric is used and calculated."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expression")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The mathematical expression for this derived metric. Can be used in [Metric](#Metric)'s `expression` field for equivalent reports. Most metrics are not expressions, and for non-expressions, this field is empty."]
        pub expression: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of this metric."]
        pub _type: ::std::option::Option<MetricMetadataTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uiName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This metric's name within the Google Analytics user interface. For example, `Event count`."]
        pub ui_name: ::std::option::Option<::std::string::String>,
    }
    impl MetricMetadata {
        pub fn builder() -> MetricMetadataBuilder {
            MetricMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of this metric."]
    pub enum MetricMetadataTypeEnum {
        #[serde(rename = "METRIC_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified type."]
        MetricTypeUnspecified,
        #[serde(rename = "TYPE_INTEGER")]
        #[doc = "Integer type."]
        TypeInteger,
        #[serde(rename = "TYPE_FLOAT")]
        #[doc = "Floating point type."]
        TypeFloat,
        #[serde(rename = "TYPE_SECONDS")]
        #[doc = "A duration of seconds; a special floating point type."]
        TypeSeconds,
        #[serde(rename = "TYPE_MILLISECONDS")]
        #[doc = "A duration in milliseconds; a special floating point type."]
        TypeMilliseconds,
        #[serde(rename = "TYPE_MINUTES")]
        #[doc = "A duration in minutes; a special floating point type."]
        TypeMinutes,
        #[serde(rename = "TYPE_HOURS")]
        #[doc = "A duration in hours; a special floating point type."]
        TypeHours,
        #[serde(rename = "TYPE_STANDARD")]
        #[doc = "A custom metric of standard type; a special floating point type."]
        TypeStandard,
        #[serde(rename = "TYPE_CURRENCY")]
        #[doc = "An amount of money; a special floating point type."]
        TypeCurrency,
        #[serde(rename = "TYPE_FEET")]
        #[doc = "A length in feet; a special floating point type."]
        TypeFeet,
        #[serde(rename = "TYPE_MILES")]
        #[doc = "A length in miles; a special floating point type."]
        TypeMiles,
        #[serde(rename = "TYPE_METERS")]
        #[doc = "A length in meters; a special floating point type."]
        TypeMeters,
        #[serde(rename = "TYPE_KILOMETERS")]
        #[doc = "A length in kilometers; a special floating point type."]
        TypeKilometers,
    }
    impl ::std::default::Default for MetricMetadataTypeEnum {
        fn default() -> Self {
            Self::MetricTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Sorts by metric values."]
    pub struct MetricOrderBy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A metric name in the request to order by."]
        pub metric_name: ::std::option::Option<::std::string::String>,
    }
    impl MetricOrderBy {
        pub fn builder() -> MetricOrderByBuilder {
            MetricOrderByBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The value of a metric."]
    pub struct MetricValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Measurement value. See MetricHeader for type."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl MetricValue {
        pub fn builder() -> MetricValueBuilder {
            MetricValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Filters for numeric or date values."]
    pub struct NumericFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operation type for this filter."]
        pub operation: ::std::option::Option<NumericFilterOperationEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A numeric value or a date value."]
        pub value: ::std::option::Option<::std::boxed::Box<NumericValue>>,
    }
    impl NumericFilter {
        pub fn builder() -> NumericFilterBuilder {
            NumericFilterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The operation type for this filter."]
    pub enum NumericFilterOperationEnum {
        #[serde(rename = "OPERATION_UNSPECIFIED")]
        #[doc = "Unspecified."]
        OperationUnspecified,
        #[serde(rename = "EQUAL")]
        #[doc = "Equal"]
        Equal,
        #[serde(rename = "LESS_THAN")]
        #[doc = "Less than"]
        LessThan,
        #[serde(rename = "LESS_THAN_OR_EQUAL")]
        #[doc = "Less than or equal"]
        LessThanOrEqual,
        #[serde(rename = "GREATER_THAN")]
        #[doc = "Greater than"]
        GreaterThan,
        #[serde(rename = "GREATER_THAN_OR_EQUAL")]
        #[doc = "Greater than or equal"]
        GreaterThanOrEqual,
    }
    impl ::std::default::Default for NumericFilterOperationEnum {
        fn default() -> Self {
            Self::OperationUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "To represent a number."]
    pub struct NumericValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "doubleValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Double value"]
        pub double_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "int64Value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Integer value"]
        pub int64_value: ::std::option::Option<::std::string::String>,
    }
    impl NumericValue {
        pub fn builder() -> NumericValueBuilder {
            NumericValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The sort options."]
    pub struct OrderBy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desc")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, sorts by descending order."]
        pub desc: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sorts results by a dimension's values."]
        pub dimension: ::std::option::Option<::std::boxed::Box<DimensionOrderBy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metric")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sorts results by a metric's values."]
        pub metric: ::std::option::Option<::std::boxed::Box<MetricOrderBy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pivot")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sorts results by a metric's values within a pivot column group."]
        pub pivot: ::std::option::Option<::std::boxed::Box<PivotOrderBy>>,
    }
    impl OrderBy {
        pub fn builder() -> OrderByBuilder {
            OrderByBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes the visible dimension columns and rows in the report response."]
    pub struct Pivot {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dimension names for visible columns in the report response. Including \"dateRange\" produces a date range column; for each row in the response, dimension values in the date range column will indicate the corresponding date range from the request."]
        pub field_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of rows to return in this pivot. If the `limit` parameter is unspecified, up to 10,000 rows are returned. The product of the `limit` for each `pivot` in a `RunPivotReportRequest` must not exceed 100,000. For example, a two pivot request with `limit: 1000` in each pivot will fail because the product is `1,000,000`."]
        pub limit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricAggregations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aggregate the metrics by dimensions in this pivot using the specified metric_aggregations."]
        pub metric_aggregations:
            ::std::option::Option<::std::vec::Vec<PivotMetricAggregationsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The row count of the start row. The first row is counted as row 0."]
        pub offset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orderBys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies how dimensions are ordered in the pivot. In the first Pivot, the OrderBys determine Row and PivotDimensionHeader ordering; in subsequent Pivots, the OrderBys determine only PivotDimensionHeader ordering. Dimensions specified in these OrderBys must be a subset of Pivot.field_names."]
        pub order_bys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderBy>>>,
    }
    impl Pivot {
        pub fn builder() -> PivotBuilder {
            PivotBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum PivotMetricAggregationsEnum {
        #[serde(rename = "METRIC_AGGREGATION_UNSPECIFIED")]
        #[doc = "Unspecified operator."]
        MetricAggregationUnspecified,
        #[serde(rename = "TOTAL")]
        #[doc = "SUM operator."]
        Total,
        #[serde(rename = "MINIMUM")]
        #[doc = "Minimum operator."]
        Minimum,
        #[serde(rename = "MAXIMUM")]
        #[doc = "Maximum operator."]
        Maximum,
        #[serde(rename = "COUNT")]
        #[doc = "Count operator."]
        Count,
    }
    impl ::std::default::Default for PivotMetricAggregationsEnum {
        fn default() -> Self {
            Self::MetricAggregationUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Summarizes dimension values from a row for this pivot."]
    pub struct PivotDimensionHeader {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Values of multiple dimensions in a pivot."]
        pub dimension_values:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionValue>>>,
    }
    impl PivotDimensionHeader {
        pub fn builder() -> PivotDimensionHeaderBuilder {
            PivotDimensionHeaderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Dimensions' values in a single pivot."]
    pub struct PivotHeader {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pivotDimensionHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size is the same as the cardinality of the corresponding dimension combinations."]
        pub pivot_dimension_headers:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PivotDimensionHeader>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cardinality of the pivot. The total number of rows for this pivot's fields regardless of how the parameters `offset` and `limit` are specified in the request."]
        pub row_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl PivotHeader {
        pub fn builder() -> PivotHeaderBuilder {
            PivotHeaderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Sorts by a pivot column group."]
    pub struct PivotOrderBy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "In the response to order by, order rows by this column. Must be a metric name from the request."]
        pub metric_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pivotSelections")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Used to select a dimension name and value pivot. If multiple pivot selections are given, the sort occurs on rows where all pivot selection dimension name and value pairs match the row's dimension name and value pair."]
        pub pivot_selections:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PivotSelection>>>,
    }
    impl PivotOrderBy {
        pub fn builder() -> PivotOrderByBuilder {
            PivotOrderByBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A pair of dimension names and values. Rows with this dimension pivot pair are ordered by the metric's value. For example if pivots = {{\"browser\", \"Chrome\"}} and metric_name = \"Sessions\", then the rows will be sorted based on Sessions in Chrome. ---------|----------|----------------|----------|---------------- | Chrome | Chrome | Safari | Safari ---------|----------|----------------|----------|---------------- Country | Sessions | Pages/Sessions | Sessions | Pages/Sessions ---------|----------|----------------|----------|---------------- US | 2 | 2 | 3 | 1 ---------|----------|----------------|----------|---------------- Canada | 3 | 1 | 4 | 1 ---------|----------|----------------|----------|----------------"]
    pub struct PivotSelection {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Must be a dimension name from the request."]
        pub dimension_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Order by only when the named dimension is this value."]
        pub dimension_value: ::std::option::Option<::std::string::String>,
    }
    impl PivotSelection {
        pub fn builder() -> PivotSelectionBuilder {
            PivotSelectionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Current state of all quotas for this Analytics Property. If any quota for a property is exhausted, all requests to that property will return Resource Exhausted errors."]
    pub struct PropertyQuota {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "concurrentRequests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Standard Analytics Properties can send up to 10 concurrent requests; Analytics 360 Properties can use up to 50 concurrent requests."]
        pub concurrent_requests: ::std::option::Option<::std::boxed::Box<QuotaStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serverErrorsPerProjectPerHour")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Standard Analytics Properties and cloud project pairs can have up to 10 server errors per hour; Analytics 360 Properties and cloud project pairs can have up to 50 server errors per hour."]
        pub server_errors_per_project_per_hour:
            ::std::option::Option<::std::boxed::Box<QuotaStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokensPerDay")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Standard Analytics Properties can use up to 25,000 tokens per day; Analytics 360 Properties can use 250,000 tokens per day. Most requests consume fewer than 10 tokens."]
        pub tokens_per_day: ::std::option::Option<::std::boxed::Box<QuotaStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokensPerHour")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Standard Analytics Properties can use up to 5,000 tokens per hour; Analytics 360 Properties can use 50,000 tokens per hour. An API request consumes a single number of tokens, and that number is deducted from both the hourly and daily quotas."]
        pub tokens_per_hour: ::std::option::Option<::std::boxed::Box<QuotaStatus>>,
    }
    impl PropertyQuota {
        pub fn builder() -> PropertyQuotaBuilder {
            PropertyQuotaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Current state for a particular quota group."]
    pub struct QuotaStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Quota consumed by this request."]
        pub consumed: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "remaining")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Quota remaining after this request."]
        pub remaining: ::std::option::Option<::std::primitive::i64>,
    }
    impl QuotaStatus {
        pub fn builder() -> QuotaStatusBuilder {
            QuotaStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response's metadata carrying additional information about the report content."]
    pub struct ResponseMetaData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataLossFromOtherRow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, indicates some buckets of dimension combinations are rolled into \"(other)\" row. This can happen for high cardinality reports."]
        pub data_loss_from_other_row: ::std::option::Option<::std::primitive::bool>,
    }
    impl ResponseMetaData {
        pub fn builder() -> ResponseMetaDataBuilder {
            ResponseMetaDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Report data for each row. For example if RunReportRequest contains: ```none \"dimensions\": [ { \"name\": \"eventName\" }, { \"name\": \"countryId\" } ], \"metrics\": [ { \"name\": \"eventCount\" } ] ``` One row with 'in_app_purchase' as the eventName, 'JP' as the countryId, and 15 as the eventCount, would be: ```none \"dimensionValues\": [ { \"value\": \"in_app_purchase\" }, { \"value\": \"JP\" } ], \"metricValues\": [ { \"value\": \"15\" } ] ```"]
    pub struct Row {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of requested dimension values. In a PivotReport, dimension_values are only listed for dimensions included in a pivot."]
        pub dimension_values:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionValue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of requested visible metric values."]
        pub metric_values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricValue>>>,
    }
    impl Row {
        pub fn builder() -> RowBuilder {
            RowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request to generate a pivot report."]
    pub struct RunPivotReportRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cohortSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cohort group associated with this request. If there is a cohort group in the request the 'cohort' dimension must be present."]
        pub cohort_spec: ::std::option::Option<::std::boxed::Box<CohortSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A currency code in ISO4217 format, such as \"AED\", \"USD\", \"JPY\". If the field is empty, the report uses the entity's default currency."]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date range to retrieve event data for the report. If multiple date ranges are specified, event data from each date range is used in the report. A special dimension with field name \"dateRange\" can be included in a Pivot's field names; if included, the report compares between date ranges. In a cohort request, this `dateRanges` must be unspecified."]
        pub date_ranges: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DateRange>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filter clause of dimensions. Dimensions must be requested to be used in this filter. Metrics cannot be used in this filter."]
        pub dimension_filter: ::std::option::Option<::std::boxed::Box<FilterExpression>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimensions requested. All defined dimensions must be used by one of the following: dimension_expression, dimension_filter, pivots, order_bys."]
        pub dimensions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dimension>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A property whose events are tracked. Within a batch request, this entity should either be unspecified or consistent with the batch-level entity."]
        pub entity: ::std::option::Option<::std::boxed::Box<Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keepEmptyRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If false or unspecified, each row with all metrics equal to 0 will not be returned. If true, these rows will be returned if they are not separately removed by a filter."]
        pub keep_empty_rows: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filter clause of metrics. Applied at post aggregation phase, similar to SQL having-clause. Metrics must be requested to be used in this filter. Dimensions cannot be used in this filter."]
        pub metric_filter: ::std::option::Option<::std::boxed::Box<FilterExpression>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metrics requested, at least one metric needs to be specified. All defined metrics must be used by one of the following: metric_expression, metric_filter, order_bys."]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metric>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pivots")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes the visual format of the report's dimensions in columns or rows. The union of the fieldNames (dimension names) in all pivots must be a subset of dimension names defined in Dimensions. No two pivots can share a dimension. A dimension is only visible if it appears in a pivot."]
        pub pivots: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Pivot>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnPropertyQuota")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Toggles whether to return the current state of this Analytics Property's quota. Quota is returned in [PropertyQuota](#PropertyQuota)."]
        pub return_property_quota: ::std::option::Option<::std::primitive::bool>,
    }
    impl RunPivotReportRequest {
        pub fn builder() -> RunPivotReportRequestBuilder {
            RunPivotReportRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response pivot report table corresponding to a pivot request."]
    pub struct RunPivotReportResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aggregates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aggregation of metric values. Can be totals, minimums, or maximums. The returned aggregations are controlled by the metric_aggregations in the pivot. The type of aggregation returned in each row is shown by the dimension_values which are set to \"RESERVED_\"."]
        pub aggregates: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Row>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes dimension columns. The number of DimensionHeaders and ordering of DimensionHeaders matches the dimensions present in rows."]
        pub dimension_headers:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionHeader>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata for the report."]
        pub metadata: ::std::option::Option<::std::boxed::Box<ResponseMetaData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes metric columns. The number of MetricHeaders and ordering of MetricHeaders matches the metrics present in rows."]
        pub metric_headers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricHeader>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pivotHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Summarizes the columns and rows created by a pivot. Each pivot in the request produces one header in the response. If we have a request like this: \"pivots\": [{ \"fieldNames\": [\"country\", \"city\"] }, { \"fieldNames\": \"eventName\" }] We will have the following `pivotHeaders` in the response: \"pivotHeaders\" : [{ \"dimensionHeaders\": [{ \"dimensionValues\": [ { \"value\": \"United Kingdom\" }, { \"value\": \"London\" } ] }, { \"dimensionValues\": [ { \"value\": \"Japan\" }, { \"value\": \"Osaka\" } ] }] }, { \"dimensionHeaders\": [{ \"dimensionValues\": [{ \"value\": \"session_start\" }] }, { \"dimensionValues\": [{ \"value\": \"scroll\" }] }] }]"]
        pub pivot_headers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PivotHeader>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "propertyQuota")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This Analytics Property's quota state including this request."]
        pub property_quota: ::std::option::Option<::std::boxed::Box<PropertyQuota>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rows of dimension value combinations and metric values in the report."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Row>>>,
    }
    impl RunPivotReportResponse {
        pub fn builder() -> RunPivotReportResponseBuilder {
            RunPivotReportResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request to generate a realtime report."]
    pub struct RunRealtimeReportRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filter clause of dimensions. Dimensions must be requested to be used in this filter. Metrics cannot be used in this filter."]
        pub dimension_filter: ::std::option::Option<::std::boxed::Box<FilterExpression>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimensions requested and displayed."]
        pub dimensions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dimension>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of rows to return. If the `limit` parameter is unspecified, 10,000 rows are returned. The API returns a maximum of 100,000 rows per request, no matter how many you ask for."]
        pub limit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricAggregations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aggregation of metrics. Aggregated metric values will be shown in rows where the dimension_values are set to \"RESERVED_(MetricAggregation)\"."]
        pub metric_aggregations:
            ::std::option::Option<::std::vec::Vec<RunRealtimeReportRequestMetricAggregationsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filter clause of metrics. Applied at post aggregation phase, similar to SQL having-clause. Metrics must be requested to be used in this filter. Dimensions cannot be used in this filter."]
        pub metric_filter: ::std::option::Option<::std::boxed::Box<FilterExpression>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metrics requested and displayed."]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metric>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orderBys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies how rows are ordered in the response."]
        pub order_bys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderBy>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnPropertyQuota")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Toggles whether to return the current state of this Analytics Property's Realtime quota. Quota is returned in [PropertyQuota](#PropertyQuota)."]
        pub return_property_quota: ::std::option::Option<::std::primitive::bool>,
    }
    impl RunRealtimeReportRequest {
        pub fn builder() -> RunRealtimeReportRequestBuilder {
            RunRealtimeReportRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum RunRealtimeReportRequestMetricAggregationsEnum {
        #[serde(rename = "METRIC_AGGREGATION_UNSPECIFIED")]
        #[doc = "Unspecified operator."]
        MetricAggregationUnspecified,
        #[serde(rename = "TOTAL")]
        #[doc = "SUM operator."]
        Total,
        #[serde(rename = "MINIMUM")]
        #[doc = "Minimum operator."]
        Minimum,
        #[serde(rename = "MAXIMUM")]
        #[doc = "Maximum operator."]
        Maximum,
        #[serde(rename = "COUNT")]
        #[doc = "Count operator."]
        Count,
    }
    impl ::std::default::Default for RunRealtimeReportRequestMetricAggregationsEnum {
        fn default() -> Self {
            Self::MetricAggregationUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response realtime report table corresponding to a request."]
    pub struct RunRealtimeReportResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes dimension columns. The number of DimensionHeaders and ordering of DimensionHeaders matches the dimensions present in rows."]
        pub dimension_headers:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionHeader>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maximums")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If requested, the maximum values of metrics."]
        pub maximums: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Row>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes metric columns. The number of MetricHeaders and ordering of MetricHeaders matches the metrics present in rows."]
        pub metric_headers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricHeader>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimums")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If requested, the minimum values of metrics."]
        pub minimums: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Row>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "propertyQuota")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This Analytics Property's Realtime quota state including this request."]
        pub property_quota: ::std::option::Option<::std::boxed::Box<PropertyQuota>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of rows in the query result, regardless of the number of rows returned in the response. For example if a query returns 175 rows and includes limit = 50 in the API request, the response will contain row_count = 175 but only 50 rows."]
        pub row_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rows of dimension value combinations and metric values in the report."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Row>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totals")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If requested, the totaled values of metrics."]
        pub totals: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Row>>>,
    }
    impl RunRealtimeReportResponse {
        pub fn builder() -> RunRealtimeReportResponseBuilder {
            RunRealtimeReportResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request to generate a report."]
    pub struct RunReportRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cohortSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cohort group associated with this request. If there is a cohort group in the request the 'cohort' dimension must be present."]
        pub cohort_spec: ::std::option::Option<::std::boxed::Box<CohortSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A currency code in ISO4217 format, such as \"AED\", \"USD\", \"JPY\". If the field is empty, the report uses the entity's default currency."]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date ranges of data to read. If multiple date ranges are requested, each response row will contain a zero based date range index. If two date ranges overlap, the event data for the overlapping days is included in the response rows for both date ranges. In a cohort request, this `dateRanges` must be unspecified."]
        pub date_ranges: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DateRange>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filter clause of dimensions. Dimensions must be requested to be used in this filter. Metrics cannot be used in this filter."]
        pub dimension_filter: ::std::option::Option<::std::boxed::Box<FilterExpression>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimensions requested and displayed."]
        pub dimensions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dimension>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A property whose events are tracked. Within a batch request, this entity should either be unspecified or consistent with the batch-level entity."]
        pub entity: ::std::option::Option<::std::boxed::Box<Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keepEmptyRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If false or unspecified, each row with all metrics equal to 0 will not be returned. If true, these rows will be returned if they are not separately removed by a filter."]
        pub keep_empty_rows: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of rows to return. If the `limit` parameter is unspecified, 10,000 rows are returned. The API returns a maximum of 100,000 rows per request, no matter how many you ask for."]
        pub limit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricAggregations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aggregation of metrics. Aggregated metric values will be shown in rows where the dimension_values are set to \"RESERVED_(MetricAggregation)\"."]
        pub metric_aggregations:
            ::std::option::Option<::std::vec::Vec<RunReportRequestMetricAggregationsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filter clause of metrics. Applied at post aggregation phase, similar to SQL having-clause. Metrics must be requested to be used in this filter. Dimensions cannot be used in this filter."]
        pub metric_filter: ::std::option::Option<::std::boxed::Box<FilterExpression>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metrics requested and displayed."]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metric>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The row count of the start row. The first row is counted as row 0."]
        pub offset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orderBys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies how rows are ordered in the response."]
        pub order_bys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderBy>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnPropertyQuota")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Toggles whether to return the current state of this Analytics Property's quota. Quota is returned in [PropertyQuota](#PropertyQuota)."]
        pub return_property_quota: ::std::option::Option<::std::primitive::bool>,
    }
    impl RunReportRequest {
        pub fn builder() -> RunReportRequestBuilder {
            RunReportRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum RunReportRequestMetricAggregationsEnum {
        #[serde(rename = "METRIC_AGGREGATION_UNSPECIFIED")]
        #[doc = "Unspecified operator."]
        MetricAggregationUnspecified,
        #[serde(rename = "TOTAL")]
        #[doc = "SUM operator."]
        Total,
        #[serde(rename = "MINIMUM")]
        #[doc = "Minimum operator."]
        Minimum,
        #[serde(rename = "MAXIMUM")]
        #[doc = "Maximum operator."]
        Maximum,
        #[serde(rename = "COUNT")]
        #[doc = "Count operator."]
        Count,
    }
    impl ::std::default::Default for RunReportRequestMetricAggregationsEnum {
        fn default() -> Self {
            Self::MetricAggregationUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response report table corresponding to a request."]
    pub struct RunReportResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes dimension columns. The number of DimensionHeaders and ordering of DimensionHeaders matches the dimensions present in rows."]
        pub dimension_headers:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionHeader>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maximums")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If requested, the maximum values of metrics."]
        pub maximums: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Row>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata for the report."]
        pub metadata: ::std::option::Option<::std::boxed::Box<ResponseMetaData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes metric columns. The number of MetricHeaders and ordering of MetricHeaders matches the metrics present in rows."]
        pub metric_headers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricHeader>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimums")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If requested, the minimum values of metrics."]
        pub minimums: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Row>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "propertyQuota")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This Analytics Property's quota state including this request."]
        pub property_quota: ::std::option::Option<::std::boxed::Box<PropertyQuota>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of rows in the query result, regardless of the number of rows returned in the response. For example if a query returns 175 rows and includes limit = 50 in the API request, the response will contain row_count = 175 but only 50 rows. To learn more about this pagination parameter, see [Pagination](https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination)."]
        pub row_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rows of dimension value combinations and metric values in the report."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Row>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totals")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If requested, the totaled values of metrics."]
        pub totals: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Row>>>,
    }
    impl RunReportResponse {
        pub fn builder() -> RunReportResponseBuilder {
            RunReportResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The filter for string"]
    pub struct StringFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "caseSensitive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, the string value is case sensitive."]
        pub case_sensitive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The match type for this filter."]
        pub match_type: ::std::option::Option<StringFilterMatchTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The string value used for the matching."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl StringFilter {
        pub fn builder() -> StringFilterBuilder {
            StringFilterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The match type for this filter."]
    pub enum StringFilterMatchTypeEnum {
        #[serde(rename = "MATCH_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified"]
        MatchTypeUnspecified,
        #[serde(rename = "EXACT")]
        #[doc = "Exact match of the string value."]
        Exact,
        #[serde(rename = "BEGINS_WITH")]
        #[doc = "Begins with the string value."]
        BeginsWith,
        #[serde(rename = "ENDS_WITH")]
        #[doc = "Ends with the string value."]
        EndsWith,
        #[serde(rename = "CONTAINS")]
        #[doc = "Contains the string value."]
        Contains,
        #[serde(rename = "FULL_REGEXP")]
        #[doc = "Full regular expression match with the string value."]
        FullRegexp,
        #[serde(rename = "PARTIAL_REGEXP")]
        #[doc = "Partial regular expression match with the string value."]
        PartialRegexp,
    }
    impl ::std::default::Default for StringFilterMatchTypeEnum {
        fn default() -> Self {
            Self::MatchTypeUnspecified
        }
    }
}
