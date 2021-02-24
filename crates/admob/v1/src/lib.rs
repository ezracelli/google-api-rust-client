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
    pub mod accounts {
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
                    #[doc = "Maximum number of accounts to return."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The value returned by the last `ListPublisherAccountsResponse`; indicates that this is a continuation of a prior `ListPublisherAccounts` call, and that the system should return the next page of data."]
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
    #[doc = "Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."]
    pub struct Date {
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
    impl Date {
        pub fn builder() -> DateBuilder {
            DateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specification of a single date range. Both dates are inclusive."]
    pub struct DateRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End date of the date range, inclusive. Must be greater than or equal to the start date."]
        pub end_date: ::std::option::Option<::std::boxed::Box<Date>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start date of the date range, inclusive. Must be less than or equal to the end date."]
        pub start_date: ::std::option::Option<::std::boxed::Box<Date>>,
    }
    impl DateRange {
        pub fn builder() -> DateRangeBuilder {
            DateRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to generate an AdMob Mediation report."]
    pub struct GenerateMediationReportRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Network report specification."]
        pub report_spec: ::std::option::Option<::std::boxed::Box<MediationReportSpec>>,
    }
    impl GenerateMediationReportRequest {
        pub fn builder() -> GenerateMediationReportRequestBuilder {
            GenerateMediationReportRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The streaming response for the AdMob Mediation report where the first response contains the report header, then a stream of row responses, and finally a footer as the last response message. For example: [{ \"header\": { \"date_range\": { \"start_date\": {\"year\": 2018, \"month\": 9, \"day\": 1}, \"end_date\": {\"year\": 2018, \"month\": 9, \"day\": 1} }, \"localization_settings\": { \"currency_code\": \"USD\", \"language_code\": \"en-US\" } } }, { \"row\": { \"dimension_values\": { \"DATE\": {\"value\": \"20180918\"}, \"APP\": { \"value\": \"ca-app-pub-8123415297019784~1001342552\", \"display_label\": \"My app name!\" } }, \"metric_values\": { \"ESTIMATED_EARNINGS\": {\"decimal_value\": \"1324746\"} } } }, { \"footer\": {\"matching_row_count\": 1} }]"]
    pub struct GenerateMediationReportResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "footer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information about the generated report, such as warnings about the data."]
        pub footer: ::std::option::Option<::std::boxed::Box<ReportFooter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Report generation settings that describes the report contents, such as the report date range and localization settings."]
        pub header: ::std::option::Option<::std::boxed::Box<ReportHeader>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "row")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Actual report data."]
        pub row: ::std::option::Option<::std::boxed::Box<ReportRow>>,
    }
    impl GenerateMediationReportResponse {
        pub fn builder() -> GenerateMediationReportResponseBuilder {
            GenerateMediationReportResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to generate an AdMob Network report."]
    pub struct GenerateNetworkReportRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Network report specification."]
        pub report_spec: ::std::option::Option<::std::boxed::Box<NetworkReportSpec>>,
    }
    impl GenerateNetworkReportRequest {
        pub fn builder() -> GenerateNetworkReportRequestBuilder {
            GenerateNetworkReportRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The streaming response for the AdMob Network report where the first response contains the report header, then a stream of row responses, and finally a footer as the last response message. For example: [{ \"header\": { \"dateRange\": { \"startDate\": {\"year\": 2018, \"month\": 9, \"day\": 1}, \"endDate\": {\"year\": 2018, \"month\": 9, \"day\": 1} }, \"localizationSettings\": { \"currencyCode\": \"USD\", \"languageCode\": \"en-US\" } } }, { \"row\": { \"dimensionValues\": { \"DATE\": {\"value\": \"20180918\"}, \"APP\": { \"value\": \"ca-app-pub-8123415297019784~1001342552\", displayLabel: \"My app name!\" } }, \"metricValues\": { \"ESTIMATED_EARNINGS\": {\"microsValue\": 6500000} } } }, { \"footer\": {\"matchingRowCount\": 1} }]"]
    pub struct GenerateNetworkReportResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "footer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information about the generated report, such as warnings about the data."]
        pub footer: ::std::option::Option<::std::boxed::Box<ReportFooter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Report generation settings that describes the report contents, such as the report date range and localization settings."]
        pub header: ::std::option::Option<::std::boxed::Box<ReportHeader>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "row")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Actual report data."]
        pub row: ::std::option::Option<::std::boxed::Box<ReportRow>>,
    }
    impl GenerateNetworkReportResponse {
        pub fn builder() -> GenerateNetworkReportResponseBuilder {
            GenerateNetworkReportResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for the publisher account list request."]
    pub struct ListPublisherAccountsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "account")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Publisher that the client credentials can access."]
        pub account: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PublisherAccount>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If not empty, indicates that there might be more accounts for the request; you must pass this value in a new `ListPublisherAccountsRequest`."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListPublisherAccountsResponse {
        pub fn builder() -> ListPublisherAccountsResponseBuilder {
            ListPublisherAccountsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Localization settings for reports, such as currency and language. It affects how metrics are calculated."]
    pub struct LocalizationSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Currency code of the earning related metrics, which is the 3-letter code defined in ISO 4217. The daily average rate is used for the currency conversion. Defaults to the account currency code if unspecified."]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Language used for any localized text, such as some dimension value display labels. The language tag defined in the IETF BCP47. Defaults to 'en-US' if unspecified."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl LocalizationSettings {
        pub fn builder() -> LocalizationSettingsBuilder {
            LocalizationSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The specification for generating an AdMob Mediation report. For example, the specification to get observed ECPM sliced by ad source and app for the 'US' and 'CN' countries can look like the following example: { \"date_range\": { \"start_date\": {\"year\": 2018, \"month\": 9, \"day\": 1}, \"end_date\": {\"year\": 2018, \"month\": 9, \"day\": 30} }, \"dimensions\": [\"AD_SOURCE\", \"APP\", \"COUNTRY\"], \"metrics\": [\"OBSERVED_ECPM\"], \"dimension_filters\": [ { \"dimension\": \"COUNTRY\", \"matches_any\": {\"values\": [{\"value\": \"US\", \"value\": \"CN\"}]} } ], \"sort_conditions\": [ {\"dimension\":\"APP\", order: \"ASCENDING\"} ], \"localization_settings\": { \"currency_code\": \"USD\", \"language_code\": \"en-US\" } } For a better understanding, you can treat the preceding specification like the following pseudo SQL: SELECT AD_SOURCE, APP, COUNTRY, OBSERVED_ECPM FROM MEDIATION_REPORT WHERE DATE >= '2018-09-01' AND DATE <= '2018-09-30' AND COUNTRY IN ('US', 'CN') GROUP BY AD_SOURCE, APP, COUNTRY ORDER BY APP ASC;"]
    pub struct MediationReportSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date range for which the report is generated."]
        pub date_range: ::std::option::Option<::std::boxed::Box<DateRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionFilters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes which report rows to match based on their dimension values."]
        pub dimension_filters: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<MediationReportSpecDimensionFilter>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of dimensions of the report. The value combination of these dimensions determines the row of the report. If no dimensions are specified, the report returns a single row of requested metrics for the entire account."]
        pub dimensions: ::std::option::Option<::std::vec::Vec<MediationReportSpecDimensionsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localizationSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Localization settings of the report."]
        pub localization_settings: ::std::option::Option<::std::boxed::Box<LocalizationSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxReportRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum number of report data rows to return. If the value is not set, the API returns as many rows as possible, up to 100000. Acceptable values are 1-100000, inclusive. Values larger than 100000 return an error."]
        pub max_report_rows: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of metrics of the report. A report must specify at least one metric."]
        pub metrics: ::std::option::Option<::std::vec::Vec<MediationReportSpecMetricsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sortConditions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes the sorting of report rows. The order of the condition in the list defines its precedence; the earlier the condition, the higher its precedence. If no sort conditions are specified, the row ordering is undefined."]
        pub sort_conditions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<MediationReportSpecSortCondition>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A report time zone. Accepts an IANA TZ name values, such as \"America/Los_Angeles.\" If no time zone is defined, the account default takes effect. Check default value by the get account action. **Warning:** The \"America/Los_Angeles\" is the only supported value at the moment."]
        pub time_zone: ::std::option::Option<::std::string::String>,
    }
    impl MediationReportSpec {
        pub fn builder() -> MediationReportSpecBuilder {
            MediationReportSpecBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum MediationReportSpecDimensionsEnum {
        #[serde(rename = "DIMENSION_UNSPECIFIED")]
        #[doc = "Default value for an unset field. Do not use."]
        DimensionUnspecified,
        #[serde(rename = "DATE")]
        #[doc = "A date in the YYYY-MM-DD format (for example, \"2018-12-21\"). Requests can specify at most one time dimension."]
        Date,
        #[serde(rename = "MONTH")]
        #[doc = "A month in the YYYY-MM format (for example, \"2018-12\"). Requests can specify at most one time dimension."]
        Month,
        #[serde(rename = "WEEK")]
        #[doc = "The date of the first day of a week in the YYYY-MM-DD format (for example, \"2018-12-21\"). Requests can specify at most one time dimension."]
        Week,
        #[serde(rename = "AD_SOURCE")]
        #[doc = "The [unique ID of the ad source](/admob/api/v1/ad_sources) (for example, \"5450213213286189855\" and \"AdMob Network\" as label value)."]
        AdSource,
        #[serde(rename = "AD_SOURCE_INSTANCE")]
        #[doc = "The unique ID of the ad source instance (for example, \"ca-app-pub-1234#5678\" and \"AdMob (default)\" as label value)."]
        AdSourceInstance,
        #[serde(rename = "AD_UNIT")]
        #[doc = "The unique ID of the ad unit (for example, \"ca-app-pub-1234/8790\"). If AD_UNIT dimension is specified, then APP is included automatically."]
        AdUnit,
        #[serde(rename = "APP")]
        #[doc = "The unique ID of the mobile application (for example, \"ca-app-pub-1234~1234\")."]
        App,
        #[serde(rename = "MEDIATION_GROUP")]
        #[doc = "The unique ID of the mediation group (for example, \"ca-app-pub-1234:mg:1234\" and \"AdMob (default)\" as label value)."]
        MediationGroup,
        #[serde(rename = "COUNTRY")]
        #[doc = "CLDR country code of the place where the ad views/clicks occur (for example, \"US\" or \"FR\"). This is a geography dimension."]
        Country,
        #[serde(rename = "FORMAT")]
        #[doc = "Format of the ad unit (for example, \"banner\", \"native\"), an ad delivery dimension."]
        Format,
        #[serde(rename = "PLATFORM")]
        #[doc = "Mobile OS platform of the app (for example, \"Android\" or \"iOS\")."]
        Platform,
    }
    impl ::std::default::Default for MediationReportSpecDimensionsEnum {
        fn default() -> Self {
            Self::DimensionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum MediationReportSpecMetricsEnum {
        #[serde(rename = "METRIC_UNSPECIFIED")]
        #[doc = "Default value for an unset field. Do not use."]
        MetricUnspecified,
        #[serde(rename = "AD_REQUESTS")]
        #[doc = "The number of requests. The value is an integer."]
        AdRequests,
        #[serde(rename = "CLICKS")]
        #[doc = "The number of times a user clicks an ad. The value is an integer."]
        Clicks,
        #[serde(rename = "ESTIMATED_EARNINGS")]
        #[doc = "The estimated earnings of the AdMob publisher. The currency unit (USD, EUR, or other) of the earning metrics are determined by the localization setting for currency. The amount is in micros. For example, $6.50 would be represented as 6500000. Estimated earnings per mediation group and per ad source instance level is supported dating back to October 20, 2019. Third-party estimated earnings will show 0 for dates prior to October 20, 2019."]
        EstimatedEarnings,
        #[serde(rename = "IMPRESSIONS")]
        #[doc = "The total number of ads shown to users. The value is an integer."]
        Impressions,
        #[serde(rename = "IMPRESSION_CTR")]
        #[doc = "The ratio of clicks over impressions. The value is a double precision (approximate) decimal value."]
        ImpressionCtr,
        #[serde(rename = "MATCHED_REQUESTS")]
        #[doc = "The number of times ads are returned in response to a request. The value is an integer."]
        MatchedRequests,
        #[serde(rename = "MATCH_RATE")]
        #[doc = "The ratio of matched ad requests over the total ad requests. The value is a double precision (approximate) decimal value."]
        MatchRate,
        #[serde(rename = "OBSERVED_ECPM")]
        #[doc = "The third-party ad network's estimated average eCPM. The currency unit (USD, EUR, or other) of the earning metrics are determined by the localization setting for currency. The amount is in micros. For example, $2.30 would be represented as 2300000. The estimated average eCPM per mediation group and per ad source instance level is supported dating back to October 20, 2019. Third-party estimated average eCPM will show 0 for dates prior to October 20, 2019."]
        ObservedEcpm,
    }
    impl ::std::default::Default for MediationReportSpecMetricsEnum {
        fn default() -> Self {
            Self::MetricUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes which report rows to match based on their dimension values."]
    pub struct MediationReportSpecDimensionFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Applies the filter criterion to the specified dimension."]
        pub dimension: ::std::option::Option<MediationReportSpecDimensionFilterDimensionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchesAny")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Matches a row if its value for the specified dimension is in one of the values specified in this condition."]
        pub matches_any: ::std::option::Option<::std::boxed::Box<StringList>>,
    }
    impl MediationReportSpecDimensionFilter {
        pub fn builder() -> MediationReportSpecDimensionFilterBuilder {
            MediationReportSpecDimensionFilterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Applies the filter criterion to the specified dimension."]
    pub enum MediationReportSpecDimensionFilterDimensionEnum {
        #[serde(rename = "DIMENSION_UNSPECIFIED")]
        #[doc = "Default value for an unset field. Do not use."]
        DimensionUnspecified,
        #[serde(rename = "DATE")]
        #[doc = "A date in the YYYY-MM-DD format (for example, \"2018-12-21\"). Requests can specify at most one time dimension."]
        Date,
        #[serde(rename = "MONTH")]
        #[doc = "A month in the YYYY-MM format (for example, \"2018-12\"). Requests can specify at most one time dimension."]
        Month,
        #[serde(rename = "WEEK")]
        #[doc = "The date of the first day of a week in the YYYY-MM-DD format (for example, \"2018-12-21\"). Requests can specify at most one time dimension."]
        Week,
        #[serde(rename = "AD_SOURCE")]
        #[doc = "The [unique ID of the ad source](/admob/api/v1/ad_sources) (for example, \"5450213213286189855\" and \"AdMob Network\" as label value)."]
        AdSource,
        #[serde(rename = "AD_SOURCE_INSTANCE")]
        #[doc = "The unique ID of the ad source instance (for example, \"ca-app-pub-1234#5678\" and \"AdMob (default)\" as label value)."]
        AdSourceInstance,
        #[serde(rename = "AD_UNIT")]
        #[doc = "The unique ID of the ad unit (for example, \"ca-app-pub-1234/8790\"). If AD_UNIT dimension is specified, then APP is included automatically."]
        AdUnit,
        #[serde(rename = "APP")]
        #[doc = "The unique ID of the mobile application (for example, \"ca-app-pub-1234~1234\")."]
        App,
        #[serde(rename = "MEDIATION_GROUP")]
        #[doc = "The unique ID of the mediation group (for example, \"ca-app-pub-1234:mg:1234\" and \"AdMob (default)\" as label value)."]
        MediationGroup,
        #[serde(rename = "COUNTRY")]
        #[doc = "CLDR country code of the place where the ad views/clicks occur (for example, \"US\" or \"FR\"). This is a geography dimension."]
        Country,
        #[serde(rename = "FORMAT")]
        #[doc = "Format of the ad unit (for example, \"banner\", \"native\"), an ad delivery dimension."]
        Format,
        #[serde(rename = "PLATFORM")]
        #[doc = "Mobile OS platform of the app (for example, \"Android\" or \"iOS\")."]
        Platform,
    }
    impl ::std::default::Default for MediationReportSpecDimensionFilterDimensionEnum {
        fn default() -> Self {
            Self::DimensionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Sorting direction to be applied on a dimension or a metric."]
    pub struct MediationReportSpecSortCondition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sort by the specified dimension."]
        pub dimension: ::std::option::Option<MediationReportSpecSortConditionDimensionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metric")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sort by the specified metric."]
        pub metric: ::std::option::Option<MediationReportSpecSortConditionMetricEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "order")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sorting order of the dimension or metric."]
        pub order: ::std::option::Option<MediationReportSpecSortConditionOrderEnum>,
    }
    impl MediationReportSpecSortCondition {
        pub fn builder() -> MediationReportSpecSortConditionBuilder {
            MediationReportSpecSortConditionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Sort by the specified dimension."]
    pub enum MediationReportSpecSortConditionDimensionEnum {
        #[serde(rename = "DIMENSION_UNSPECIFIED")]
        #[doc = "Default value for an unset field. Do not use."]
        DimensionUnspecified,
        #[serde(rename = "DATE")]
        #[doc = "A date in the YYYY-MM-DD format (for example, \"2018-12-21\"). Requests can specify at most one time dimension."]
        Date,
        #[serde(rename = "MONTH")]
        #[doc = "A month in the YYYY-MM format (for example, \"2018-12\"). Requests can specify at most one time dimension."]
        Month,
        #[serde(rename = "WEEK")]
        #[doc = "The date of the first day of a week in the YYYY-MM-DD format (for example, \"2018-12-21\"). Requests can specify at most one time dimension."]
        Week,
        #[serde(rename = "AD_SOURCE")]
        #[doc = "The [unique ID of the ad source](/admob/api/v1/ad_sources) (for example, \"5450213213286189855\" and \"AdMob Network\" as label value)."]
        AdSource,
        #[serde(rename = "AD_SOURCE_INSTANCE")]
        #[doc = "The unique ID of the ad source instance (for example, \"ca-app-pub-1234#5678\" and \"AdMob (default)\" as label value)."]
        AdSourceInstance,
        #[serde(rename = "AD_UNIT")]
        #[doc = "The unique ID of the ad unit (for example, \"ca-app-pub-1234/8790\"). If AD_UNIT dimension is specified, then APP is included automatically."]
        AdUnit,
        #[serde(rename = "APP")]
        #[doc = "The unique ID of the mobile application (for example, \"ca-app-pub-1234~1234\")."]
        App,
        #[serde(rename = "MEDIATION_GROUP")]
        #[doc = "The unique ID of the mediation group (for example, \"ca-app-pub-1234:mg:1234\" and \"AdMob (default)\" as label value)."]
        MediationGroup,
        #[serde(rename = "COUNTRY")]
        #[doc = "CLDR country code of the place where the ad views/clicks occur (for example, \"US\" or \"FR\"). This is a geography dimension."]
        Country,
        #[serde(rename = "FORMAT")]
        #[doc = "Format of the ad unit (for example, \"banner\", \"native\"), an ad delivery dimension."]
        Format,
        #[serde(rename = "PLATFORM")]
        #[doc = "Mobile OS platform of the app (for example, \"Android\" or \"iOS\")."]
        Platform,
    }
    impl ::std::default::Default for MediationReportSpecSortConditionDimensionEnum {
        fn default() -> Self {
            Self::DimensionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Sort by the specified metric."]
    pub enum MediationReportSpecSortConditionMetricEnum {
        #[serde(rename = "METRIC_UNSPECIFIED")]
        #[doc = "Default value for an unset field. Do not use."]
        MetricUnspecified,
        #[serde(rename = "AD_REQUESTS")]
        #[doc = "The number of requests. The value is an integer."]
        AdRequests,
        #[serde(rename = "CLICKS")]
        #[doc = "The number of times a user clicks an ad. The value is an integer."]
        Clicks,
        #[serde(rename = "ESTIMATED_EARNINGS")]
        #[doc = "The estimated earnings of the AdMob publisher. The currency unit (USD, EUR, or other) of the earning metrics are determined by the localization setting for currency. The amount is in micros. For example, $6.50 would be represented as 6500000. Estimated earnings per mediation group and per ad source instance level is supported dating back to October 20, 2019. Third-party estimated earnings will show 0 for dates prior to October 20, 2019."]
        EstimatedEarnings,
        #[serde(rename = "IMPRESSIONS")]
        #[doc = "The total number of ads shown to users. The value is an integer."]
        Impressions,
        #[serde(rename = "IMPRESSION_CTR")]
        #[doc = "The ratio of clicks over impressions. The value is a double precision (approximate) decimal value."]
        ImpressionCtr,
        #[serde(rename = "MATCHED_REQUESTS")]
        #[doc = "The number of times ads are returned in response to a request. The value is an integer."]
        MatchedRequests,
        #[serde(rename = "MATCH_RATE")]
        #[doc = "The ratio of matched ad requests over the total ad requests. The value is a double precision (approximate) decimal value."]
        MatchRate,
        #[serde(rename = "OBSERVED_ECPM")]
        #[doc = "The third-party ad network's estimated average eCPM. The currency unit (USD, EUR, or other) of the earning metrics are determined by the localization setting for currency. The amount is in micros. For example, $2.30 would be represented as 2300000. The estimated average eCPM per mediation group and per ad source instance level is supported dating back to October 20, 2019. Third-party estimated average eCPM will show 0 for dates prior to October 20, 2019."]
        ObservedEcpm,
    }
    impl ::std::default::Default for MediationReportSpecSortConditionMetricEnum {
        fn default() -> Self {
            Self::MetricUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Sorting order of the dimension or metric."]
    pub enum MediationReportSpecSortConditionOrderEnum {
        #[serde(rename = "SORT_ORDER_UNSPECIFIED")]
        #[doc = "Default value for an unset field. Do not use."]
        SortOrderUnspecified,
        #[serde(rename = "ASCENDING")]
        #[doc = "Sort dimension value or metric value in ascending order."]
        Ascending,
        #[serde(rename = "DESCENDING")]
        #[doc = "Sort dimension value or metric value in descending order."]
        Descending,
    }
    impl ::std::default::Default for MediationReportSpecSortConditionOrderEnum {
        fn default() -> Self {
            Self::SortOrderUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The specification for generating an AdMob Network report. For example, the specification to get clicks and estimated earnings for only the 'US' and 'CN' countries can look like the following example: { 'date_range': { 'start_date': {'year': 2018, 'month': 9, 'day': 1}, 'end_date': {'year': 2018, 'month': 9, 'day': 30} }, 'dimensions': ['DATE', 'APP', 'COUNTRY'], 'metrics': ['CLICKS', 'ESTIMATED_EARNINGS'], 'dimension_filters': [ { 'dimension': 'COUNTRY', 'matches_any': {'values': [{'value': 'US', 'value': 'CN'}]} } ], 'sort_conditions': [ {'dimension':'APP', order: 'ASCENDING'}, {'metric':'CLICKS', order: 'DESCENDING'} ], 'localization_settings': { 'currency_code': 'USD', 'language_code': 'en-US' } } For a better understanding, you can treat the preceding specification like the following pseudo SQL: SELECT DATE, APP, COUNTRY, CLICKS, ESTIMATED_EARNINGS FROM NETWORK_REPORT WHERE DATE >= '2018-09-01' AND DATE <= '2018-09-30' AND COUNTRY IN ('US', 'CN') GROUP BY DATE, APP, COUNTRY ORDER BY APP ASC, CLICKS DESC;"]
    pub struct NetworkReportSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date range for which the report is generated."]
        pub date_range: ::std::option::Option<::std::boxed::Box<DateRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionFilters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes which report rows to match based on their dimension values."]
        pub dimension_filters: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<NetworkReportSpecDimensionFilter>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of dimensions of the report. The value combination of these dimensions determines the row of the report. If no dimensions are specified, the report returns a single row of requested metrics for the entire account."]
        pub dimensions: ::std::option::Option<::std::vec::Vec<NetworkReportSpecDimensionsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localizationSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Localization settings of the report."]
        pub localization_settings: ::std::option::Option<::std::boxed::Box<LocalizationSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxReportRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum number of report data rows to return. If the value is not set, the API returns as many rows as possible, up to 100000. Acceptable values are 1-100000, inclusive. Values larger than 100000 return an error."]
        pub max_report_rows: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of metrics of the report. A report must specify at least one metric."]
        pub metrics: ::std::option::Option<::std::vec::Vec<NetworkReportSpecMetricsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sortConditions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes the sorting of report rows. The order of the condition in the list defines its precedence; the earlier the condition, the higher its precedence. If no sort conditions are specified, the row ordering is undefined."]
        pub sort_conditions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<NetworkReportSpecSortCondition>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A report time zone. Accepts an IANA TZ name values, such as \"America/Los_Angeles.\" If no time zone is defined, the account default takes effect. Check default value by the get account action. **Warning:** The \"America/Los_Angeles\" is the only supported value at the moment."]
        pub time_zone: ::std::option::Option<::std::string::String>,
    }
    impl NetworkReportSpec {
        pub fn builder() -> NetworkReportSpecBuilder {
            NetworkReportSpecBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum NetworkReportSpecDimensionsEnum {
        #[serde(rename = "DIMENSION_UNSPECIFIED")]
        #[doc = "Default value for an unset field. Do not use."]
        DimensionUnspecified,
        #[serde(rename = "DATE")]
        #[doc = "A date in the YYYY-MM-DD format (for example, \"2018-12-21\"). Requests can specify at most one time dimension."]
        Date,
        #[serde(rename = "MONTH")]
        #[doc = "A month in the YYYY-MM format (for example, \"2018-12\"). Requests can specify at most one time dimension."]
        Month,
        #[serde(rename = "WEEK")]
        #[doc = "The date of the first day of a week in the YYYY-MM-DD format (for example, \"2018-12-21\"). Requests can specify at most one time dimension."]
        Week,
        #[serde(rename = "AD_UNIT")]
        #[doc = "The unique ID of the ad unit (for example, \"ca-app-pub-1234/1234\"). If AD_UNIT dimension is specified, then APP is included automatically."]
        AdUnit,
        #[serde(rename = "APP")]
        #[doc = "The unique ID of the mobile application (for example, \"ca-app-pub-1234~1234\")."]
        App,
        #[serde(rename = "AD_TYPE")]
        #[doc = "Type of the ad (for example, \"text\" or \"image\"), an ad delivery dimension. **Warning:** The dimension is incompatible with [AD_REQUESTS](#Metric.ENUM_VALUES.AD_REQUESTS), [MATCH_RATE](#Metric.ENUM_VALUES.MATCH_RATE) and [IMPRESSION_RPM](#Metric.ENUM_VALUES.IMPRESSION_RPM) metrics."]
        AdType,
        #[serde(rename = "COUNTRY")]
        #[doc = "CLDR country code of the place where the ad views/clicks occur (for example, \"US\" or \"FR\"). This is a geography dimension."]
        Country,
        #[serde(rename = "FORMAT")]
        #[doc = "Format of the ad unit (for example, \"banner\", \"native\"), an ad delivery dimension."]
        Format,
        #[serde(rename = "PLATFORM")]
        #[doc = "Mobile OS platform of the app (for example, \"Android\" or \"iOS\")."]
        Platform,
    }
    impl ::std::default::Default for NetworkReportSpecDimensionsEnum {
        fn default() -> Self {
            Self::DimensionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum NetworkReportSpecMetricsEnum {
        #[serde(rename = "METRIC_UNSPECIFIED")]
        #[doc = "Default value for an unset field. Do not use."]
        MetricUnspecified,
        #[serde(rename = "AD_REQUESTS")]
        #[doc = "The number of ad requests. The value is an integer. **Warning:** The metric is incompatible with [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension."]
        AdRequests,
        #[serde(rename = "CLICKS")]
        #[doc = "The number of times a user clicks an ad. The value is an integer."]
        Clicks,
        #[serde(rename = "ESTIMATED_EARNINGS")]
        #[doc = "The estimated earnings of the AdMob publisher. The currency unit (USD, EUR, or other) of the earning metrics are determined by the localization setting for currency. The amount is in micros. For example, $6.50 would be represented as 6500000."]
        EstimatedEarnings,
        #[serde(rename = "IMPRESSIONS")]
        #[doc = "The total number of ads shown to users. The value is an integer."]
        Impressions,
        #[serde(rename = "IMPRESSION_CTR")]
        #[doc = "The ratio of clicks over impressions. The value is a double precision (approximate) decimal value."]
        ImpressionCtr,
        #[serde(rename = "IMPRESSION_RPM")]
        #[doc = "The estimated earnings per thousand ad impressions. The value is in micros. For example, $1.03 would be represented as 1030000. Equivalent to eCPM in the AdMob UI. **Warning:** The metric is incompatible with [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension."]
        ImpressionRpm,
        #[serde(rename = "MATCHED_REQUESTS")]
        #[doc = "The number of times ads are returned in response to a request. The value is an integer."]
        MatchedRequests,
        #[serde(rename = "MATCH_RATE")]
        #[doc = "The ratio of matched ad requests over the total ad requests. The value is a double precision (approximate) decimal value. **Warning:** The metric is incompatible with [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension."]
        MatchRate,
        #[serde(rename = "SHOW_RATE")]
        #[doc = "The ratio of ads that are displayed over ads that are returned, defined as impressions / matched requests. The value is a double precision (approximate) decimal value."]
        ShowRate,
    }
    impl ::std::default::Default for NetworkReportSpecMetricsEnum {
        fn default() -> Self {
            Self::MetricUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes which report rows to match based on their dimension values."]
    pub struct NetworkReportSpecDimensionFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Applies the filter criterion to the specified dimension."]
        pub dimension: ::std::option::Option<NetworkReportSpecDimensionFilterDimensionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchesAny")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Matches a row if its value for the specified dimension is in one of the values specified in this condition."]
        pub matches_any: ::std::option::Option<::std::boxed::Box<StringList>>,
    }
    impl NetworkReportSpecDimensionFilter {
        pub fn builder() -> NetworkReportSpecDimensionFilterBuilder {
            NetworkReportSpecDimensionFilterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Applies the filter criterion to the specified dimension."]
    pub enum NetworkReportSpecDimensionFilterDimensionEnum {
        #[serde(rename = "DIMENSION_UNSPECIFIED")]
        #[doc = "Default value for an unset field. Do not use."]
        DimensionUnspecified,
        #[serde(rename = "DATE")]
        #[doc = "A date in the YYYY-MM-DD format (for example, \"2018-12-21\"). Requests can specify at most one time dimension."]
        Date,
        #[serde(rename = "MONTH")]
        #[doc = "A month in the YYYY-MM format (for example, \"2018-12\"). Requests can specify at most one time dimension."]
        Month,
        #[serde(rename = "WEEK")]
        #[doc = "The date of the first day of a week in the YYYY-MM-DD format (for example, \"2018-12-21\"). Requests can specify at most one time dimension."]
        Week,
        #[serde(rename = "AD_UNIT")]
        #[doc = "The unique ID of the ad unit (for example, \"ca-app-pub-1234/1234\"). If AD_UNIT dimension is specified, then APP is included automatically."]
        AdUnit,
        #[serde(rename = "APP")]
        #[doc = "The unique ID of the mobile application (for example, \"ca-app-pub-1234~1234\")."]
        App,
        #[serde(rename = "AD_TYPE")]
        #[doc = "Type of the ad (for example, \"text\" or \"image\"), an ad delivery dimension. **Warning:** The dimension is incompatible with [AD_REQUESTS](#Metric.ENUM_VALUES.AD_REQUESTS), [MATCH_RATE](#Metric.ENUM_VALUES.MATCH_RATE) and [IMPRESSION_RPM](#Metric.ENUM_VALUES.IMPRESSION_RPM) metrics."]
        AdType,
        #[serde(rename = "COUNTRY")]
        #[doc = "CLDR country code of the place where the ad views/clicks occur (for example, \"US\" or \"FR\"). This is a geography dimension."]
        Country,
        #[serde(rename = "FORMAT")]
        #[doc = "Format of the ad unit (for example, \"banner\", \"native\"), an ad delivery dimension."]
        Format,
        #[serde(rename = "PLATFORM")]
        #[doc = "Mobile OS platform of the app (for example, \"Android\" or \"iOS\")."]
        Platform,
    }
    impl ::std::default::Default for NetworkReportSpecDimensionFilterDimensionEnum {
        fn default() -> Self {
            Self::DimensionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Sorting direction to be applied on a dimension or a metric."]
    pub struct NetworkReportSpecSortCondition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sort by the specified dimension."]
        pub dimension: ::std::option::Option<NetworkReportSpecSortConditionDimensionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metric")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sort by the specified metric."]
        pub metric: ::std::option::Option<NetworkReportSpecSortConditionMetricEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "order")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sorting order of the dimension or metric."]
        pub order: ::std::option::Option<NetworkReportSpecSortConditionOrderEnum>,
    }
    impl NetworkReportSpecSortCondition {
        pub fn builder() -> NetworkReportSpecSortConditionBuilder {
            NetworkReportSpecSortConditionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Sort by the specified dimension."]
    pub enum NetworkReportSpecSortConditionDimensionEnum {
        #[serde(rename = "DIMENSION_UNSPECIFIED")]
        #[doc = "Default value for an unset field. Do not use."]
        DimensionUnspecified,
        #[serde(rename = "DATE")]
        #[doc = "A date in the YYYY-MM-DD format (for example, \"2018-12-21\"). Requests can specify at most one time dimension."]
        Date,
        #[serde(rename = "MONTH")]
        #[doc = "A month in the YYYY-MM format (for example, \"2018-12\"). Requests can specify at most one time dimension."]
        Month,
        #[serde(rename = "WEEK")]
        #[doc = "The date of the first day of a week in the YYYY-MM-DD format (for example, \"2018-12-21\"). Requests can specify at most one time dimension."]
        Week,
        #[serde(rename = "AD_UNIT")]
        #[doc = "The unique ID of the ad unit (for example, \"ca-app-pub-1234/1234\"). If AD_UNIT dimension is specified, then APP is included automatically."]
        AdUnit,
        #[serde(rename = "APP")]
        #[doc = "The unique ID of the mobile application (for example, \"ca-app-pub-1234~1234\")."]
        App,
        #[serde(rename = "AD_TYPE")]
        #[doc = "Type of the ad (for example, \"text\" or \"image\"), an ad delivery dimension. **Warning:** The dimension is incompatible with [AD_REQUESTS](#Metric.ENUM_VALUES.AD_REQUESTS), [MATCH_RATE](#Metric.ENUM_VALUES.MATCH_RATE) and [IMPRESSION_RPM](#Metric.ENUM_VALUES.IMPRESSION_RPM) metrics."]
        AdType,
        #[serde(rename = "COUNTRY")]
        #[doc = "CLDR country code of the place where the ad views/clicks occur (for example, \"US\" or \"FR\"). This is a geography dimension."]
        Country,
        #[serde(rename = "FORMAT")]
        #[doc = "Format of the ad unit (for example, \"banner\", \"native\"), an ad delivery dimension."]
        Format,
        #[serde(rename = "PLATFORM")]
        #[doc = "Mobile OS platform of the app (for example, \"Android\" or \"iOS\")."]
        Platform,
    }
    impl ::std::default::Default for NetworkReportSpecSortConditionDimensionEnum {
        fn default() -> Self {
            Self::DimensionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Sort by the specified metric."]
    pub enum NetworkReportSpecSortConditionMetricEnum {
        #[serde(rename = "METRIC_UNSPECIFIED")]
        #[doc = "Default value for an unset field. Do not use."]
        MetricUnspecified,
        #[serde(rename = "AD_REQUESTS")]
        #[doc = "The number of ad requests. The value is an integer. **Warning:** The metric is incompatible with [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension."]
        AdRequests,
        #[serde(rename = "CLICKS")]
        #[doc = "The number of times a user clicks an ad. The value is an integer."]
        Clicks,
        #[serde(rename = "ESTIMATED_EARNINGS")]
        #[doc = "The estimated earnings of the AdMob publisher. The currency unit (USD, EUR, or other) of the earning metrics are determined by the localization setting for currency. The amount is in micros. For example, $6.50 would be represented as 6500000."]
        EstimatedEarnings,
        #[serde(rename = "IMPRESSIONS")]
        #[doc = "The total number of ads shown to users. The value is an integer."]
        Impressions,
        #[serde(rename = "IMPRESSION_CTR")]
        #[doc = "The ratio of clicks over impressions. The value is a double precision (approximate) decimal value."]
        ImpressionCtr,
        #[serde(rename = "IMPRESSION_RPM")]
        #[doc = "The estimated earnings per thousand ad impressions. The value is in micros. For example, $1.03 would be represented as 1030000. Equivalent to eCPM in the AdMob UI. **Warning:** The metric is incompatible with [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension."]
        ImpressionRpm,
        #[serde(rename = "MATCHED_REQUESTS")]
        #[doc = "The number of times ads are returned in response to a request. The value is an integer."]
        MatchedRequests,
        #[serde(rename = "MATCH_RATE")]
        #[doc = "The ratio of matched ad requests over the total ad requests. The value is a double precision (approximate) decimal value. **Warning:** The metric is incompatible with [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension."]
        MatchRate,
        #[serde(rename = "SHOW_RATE")]
        #[doc = "The ratio of ads that are displayed over ads that are returned, defined as impressions / matched requests. The value is a double precision (approximate) decimal value."]
        ShowRate,
    }
    impl ::std::default::Default for NetworkReportSpecSortConditionMetricEnum {
        fn default() -> Self {
            Self::MetricUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Sorting order of the dimension or metric."]
    pub enum NetworkReportSpecSortConditionOrderEnum {
        #[serde(rename = "SORT_ORDER_UNSPECIFIED")]
        #[doc = "Default value for an unset field. Do not use."]
        SortOrderUnspecified,
        #[serde(rename = "ASCENDING")]
        #[doc = "Sort dimension value or metric value in ascending order."]
        Ascending,
        #[serde(rename = "DESCENDING")]
        #[doc = "Sort dimension value or metric value in descending order."]
        Descending,
    }
    impl ::std::default::Default for NetworkReportSpecSortConditionOrderEnum {
        fn default() -> Self {
            Self::SortOrderUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A publisher account contains information relevant to the use of this API, such as the time zone used for the reports."]
    pub struct PublisherAccount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Currency code of the earning-related metrics, which is the 3-letter code defined in ISO 4217. The daily average rate is used for the currency conversion."]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of this account. Format is accounts/{publisher_id}."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publisherId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID by which this publisher account can be identified in the API requests (for example, pub-1234567890)."]
        pub publisher_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportingTimeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time zone that is used in reports that are generated for this account. The value is a time-zone ID as specified by the CLDR project, for example, \"America/Los_Angeles\"."]
        pub reporting_time_zone: ::std::option::Option<::std::string::String>,
    }
    impl PublisherAccount {
        pub fn builder() -> PublisherAccountBuilder {
            PublisherAccountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Groups data available after report generation, for example, warnings and row counts. Always sent as the last message in the stream response."]
    pub struct ReportFooter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchingRowCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of rows that matched the request. Warning: This count does NOT always match the number of rows in the response. Do not make that assumption when processing the response."]
        pub matching_row_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warnings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Warnings associated with generation of the report."]
        pub warnings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReportWarning>>>,
    }
    impl ReportFooter {
        pub fn builder() -> ReportFooterBuilder {
            ReportFooterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Groups data helps to treat the generated report. Always sent as a first message in the stream response."]
    pub struct ReportHeader {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date range for which the report is generated. This is identical to the range specified in the report request."]
        pub date_range: ::std::option::Option<::std::boxed::Box<DateRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localizationSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Localization settings of the report. This is identical to the settings in the report request."]
        pub localization_settings: ::std::option::Option<::std::boxed::Box<LocalizationSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportingTimeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The report time zone. The value is a time-zone ID as specified by the CLDR project, for example, \"America/Los_Angeles\"."]
        pub reporting_time_zone: ::std::option::Option<::std::string::String>,
    }
    impl ReportHeader {
        pub fn builder() -> ReportHeaderBuilder {
            ReportHeaderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A row of the returning report."]
    pub struct ReportRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map of dimension values in a row, with keys as enum name of the dimensions."]
        pub dimension_values: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<ReportRowDimensionValue>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map of metric values in a row, with keys as enum name of the metrics. If a metric being requested has no value returned, the map will not include it."]
        pub metric_values: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<ReportRowMetricValue>>,
        >,
    }
    impl ReportRow {
        pub fn builder() -> ReportRowBuilder {
            ReportRowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Representation of a dimension value."]
    pub struct ReportRowDimensionValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The localized string representation of the value. If unspecified, the display label should be derived from the value."]
        pub display_label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dimension value in the format specified in the report's spec Dimension enum."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl ReportRowDimensionValue {
        pub fn builder() -> ReportRowDimensionValueBuilder {
            ReportRowDimensionValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Representation of a metric value."]
    pub struct ReportRowMetricValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "doubleValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Double precision (approximate) decimal values. Rates are from 0 to 1."]
        pub double_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "integerValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metric integer value."]
        pub integer_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "microsValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Amount in micros. One million is equivalent to one unit. Currency value is in the unit (USD, EUR or other) specified by the request. For example, $6.50 whould be represented as 6500000 micros."]
        pub micros_value: ::std::option::Option<::std::string::String>,
    }
    impl ReportRowMetricValue {
        pub fn builder() -> ReportRowMetricValueBuilder {
            ReportRowMetricValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Warnings associated with generation of the report."]
    pub struct ReportWarning {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes the details of the warning message, in English."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the warning."]
        pub _type: ::std::option::Option<ReportWarningTypeEnum>,
    }
    impl ReportWarning {
        pub fn builder() -> ReportWarningBuilder {
            ReportWarningBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of the warning."]
    pub enum ReportWarningTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Default value for an unset field. Do not use."]
        TypeUnspecified,
        #[serde(rename = "DATA_BEFORE_ACCOUNT_TIMEZONE_CHANGE")]
        #[doc = "Some data in this report is aggregated based on a time zone different from the requested time zone. This could happen if a local time-zone report has the start time before the last time this time zone changed. The description field will contain the date of the last time zone change."]
        DataBeforeAccountTimezoneChange,
        #[serde(rename = "DATA_DELAYED")]
        #[doc = "There is an unusual delay in processing the source data for the requested date range. The report results might be less up to date than usual. AdMob is aware of the issue and is actively working to resolve it."]
        DataDelayed,
        #[serde(rename = "OTHER")]
        #[doc = "Warnings that are exposed without a specific type. Useful when new warning types are added but the API is not changed yet."]
        Other,
        #[serde(rename = "REPORT_CURRENCY_NOT_ACCOUNT_CURRENCY")]
        #[doc = "The currency being requested is not the account currency. The earning metrics will be based on the requested currency, and thus not a good estimation of the final payment anymore, due to the currency rate fluctuation."]
        ReportCurrencyNotAccountCurrency,
    }
    impl ::std::default::Default for ReportWarningTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "List of string values."]
    pub struct StringList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The string values."]
        pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl StringList {
        pub fn builder() -> StringListBuilder {
            StringListBuilder::default()
        }
    }
}
