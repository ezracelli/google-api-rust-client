#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message containing availability data relevant to DoubleClick Search."]
pub struct Availability {
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DS advertiser ID."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "agencyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DS agency ID."]
    pub agency_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "availabilityTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time by which all conversions have been uploaded, in epoch millis UTC."]
    pub availability_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "segmentationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The numeric segmentation identifier (for example, DoubleClick Search Floodlight activity ID)."]
    pub segmentation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "segmentationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The friendly segmentation identifier (for example, DoubleClick Search Floodlight activity name)."]
    pub segmentation_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "segmentationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The segmentation type that this availability is for (its default value is `FLOODLIGHT`)."]
    pub segmentation_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A conversion containing data relevant to DoubleClick Search."]
pub struct Conversion {
    #[serde(rename = "adGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DS ad group ID."]
    pub ad_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "adId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DS ad ID."]
    pub ad_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DS advertiser ID."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "agencyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DS agency ID."]
    pub agency_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "attributionModel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Available to advertisers only after contacting DoubleClick Search customer support."]
    pub attribution_model: ::std::option::Option<::std::string::String>,
    #[serde(rename = "campaignId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DS campaign ID."]
    pub campaign_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sales channel for the product. Acceptable values are: - \"`local`\": a physical store - \"`online`\": an online store "]
    pub channel: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clickId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DS click ID for the conversion."]
    pub click_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "conversionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For offline conversions, advertisers provide this ID. Advertisers can specify any ID that is meaningful to them. Each conversion in a request must specify a unique ID, and the combination of ID and timestamp must be unique amongst all conversions within the advertiser. For online conversions, DS copies the `dsConversionId` or `floodlightOrderId` into this property depending on the advertiser's Floodlight instructions."]
    pub conversion_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "conversionModifiedTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the conversion was last modified, in epoch millis UTC."]
    pub conversion_modified_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "conversionTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the conversion took place, in epoch millis UTC."]
    pub conversion_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "countMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Available to advertisers only after contacting DoubleClick Search customer support."]
    pub count_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "criterionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DS criterion (keyword) ID."]
    pub criterion_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The currency code for the conversion's revenue. Should be in ISO 4217 alphabetic (3-char) format."]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customDimension")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom dimensions for the conversion, which can be used to filter data in a report."]
    pub custom_dimension:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomDimension>>>,
    #[serde(rename = "customMetric")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom metrics for the conversion."]
    pub custom_metric: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomMetric>>>,
    #[serde(rename = "deviceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of device on which the conversion occurred."]
    pub device_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dsConversionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID that DoubleClick Search generates for each conversion."]
    pub ds_conversion_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "engineAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DS engine account ID."]
    pub engine_account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "floodlightOrderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Floodlight order ID provided by the advertiser for the conversion."]
    pub floodlight_order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inventoryAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID that DS generates and uses to uniquely identify the inventory account that contains the product."]
    pub inventory_account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productCountry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The country registered for the Merchant Center feed that contains the product. Use an ISO 3166 code to specify a country."]
    pub product_country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DS product group ID."]
    pub product_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The product ID (SKU)."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language registered for the Merchant Center feed that contains the product. Use an ISO 639 code to specify a language."]
    pub product_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quantityMillis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quantity of this conversion, in millis."]
    pub quantity_millis: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revenueMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The revenue amount of this `TRANSACTION` conversion, in micros (value multiplied by 1000000, no decimal). For example, to specify a revenue value of \"10\" enter \"10000000\" (10 million) in your request."]
    pub revenue_micros: ::std::option::Option<::std::string::String>,
    #[serde(rename = "segmentationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The numeric segmentation identifier (for example, DoubleClick Search Floodlight activity ID)."]
    pub segmentation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "segmentationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The friendly segmentation identifier (for example, DoubleClick Search Floodlight activity name)."]
    pub segmentation_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "segmentationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The segmentation type of this conversion (for example, `FLOODLIGHT`)."]
    pub segmentation_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of the conversion, that is, either `ACTIVE` or `REMOVED`. Note: state DELETED is deprecated."]
    pub state: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the local store for which the product was advertised. Applicable only when the channel is \"`local`\"."]
    pub store_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the conversion, that is, either `ACTION` or `TRANSACTION`. An `ACTION` conversion is an action by the user that has no monetarily quantifiable value, while a `TRANSACTION` conversion is an action that does have a monetarily quantifiable value. Examples are email list signups (`ACTION`) versus ecommerce purchases (`TRANSACTION`)."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of conversions."]
pub struct ConversionList {
    #[serde(rename = "conversion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The conversions being requested."]
    pub conversion: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Conversion>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies this as a ConversionList resource. Value: the fixed string doubleclicksearch#conversionList."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message containing the custom dimension."]
pub struct CustomDimension {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom dimension name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom dimension value."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message containing the custom metric."]
pub struct CustomMetric {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom metric name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom metric numeric value."]
    pub value: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A DoubleClick Search report. This object contains the report request, some report metadata such as currency code, and the generated report rows or report files."]
pub struct Report {
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Asynchronous report only. Contains a list of generated report files once the report has successfully completed."]
    pub files: ::std::option::Option<::std::vec::Vec<ReportFiles>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Asynchronous report only. Id of the report."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isReportReady")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Asynchronous report only. True if and only if the report has completed successfully and the report files are ready to be downloaded."]
    pub is_report_ready: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies this as a Report resource. Value: the fixed string `doubleclicksearch#report`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "request")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request that created the report. Optional fields not specified in the original request are filled with default values."]
    pub request: ::std::option::Option<::std::boxed::Box<ReportRequest>>,
    #[serde(rename = "rowCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of report rows generated by the report, not including headers."]
    pub row_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "rows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Synchronous report only. Generated report rows."]
    pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReportRow>>>,
    #[serde(rename = "statisticsCurrencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The currency code of all monetary values produced in the report, including values that are set by users (e.g., keyword bid settings) and metrics (e.g., cost and revenue). The currency code of a report is determined by the `statisticsCurrency` field of the report request."]
    pub statistics_currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statisticsTimeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If all statistics of the report are sourced from the same time zone, this would be it. Otherwise the field is unset."]
    pub statistics_time_zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportFiles {
    #[serde(rename = "byteCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size of this report file in bytes."]
    pub byte_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Use this url to download the report file."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request object used to create a DoubleClick Search report."]
pub struct ReportApiColumnSpec {
    #[serde(rename = "columnName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of a DoubleClick Search column to include in the report."]
    pub column_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customDimensionName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Segments a report by a custom dimension. The report must be scoped to an advertiser or lower, and the custom dimension must already be set up in DoubleClick Search. The custom dimension name, which appears in DoubleClick Search, is case sensitive.\\ If used in a conversion report, returns the value of the specified custom dimension for the given conversion, if set. This column does not segment the conversion report."]
    pub custom_dimension_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customMetricName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of a custom metric to include in the report. The report must be scoped to an advertiser or lower, and the custom metric must already be set up in DoubleClick Search. The custom metric name, which appears in DoubleClick Search, is case sensitive."]
    pub custom_metric_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inclusive day in YYYY-MM-DD format. When provided, this overrides the overall time range of the report for this column only. Must be provided together with `startDate`."]
    pub end_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "groupByColumn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Synchronous report only. Set to `true` to group by this column. Defaults to `false`."]
    pub group_by_column: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "headerText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Text used to identify this column in the report output; defaults to `columnName` or `savedColumnName` when not specified. This can be used to prevent collisions between DoubleClick Search columns and saved columns with the same name."]
    pub header_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "platformSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The platform that is used to provide data for the custom dimension. Acceptable values are \"floodlight\"."]
    pub platform_source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productReportPerspective")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Returns metrics only for a specific type of product activity. Accepted values are: - \"`sold`\": returns metrics only for products that were sold - \"`advertised`\": returns metrics only for products that were advertised in a Shopping campaign, and that might or might not have been sold "]
    pub product_report_perspective: ::std::option::Option<::std::string::String>,
    #[serde(rename = "savedColumnName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of a saved column to include in the report. The report must be scoped at advertiser or lower, and this saved column must already be created in the DoubleClick Search UI."]
    pub saved_column_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inclusive date in YYYY-MM-DD format. When provided, this overrides the overall time range of the report for this column only. Must be provided together with `endDate`."]
    pub start_date: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request object used to create a DoubleClick Search report."]
pub struct ReportRequest {
    #[serde(rename = "columns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The columns to include in the report. This includes both DoubleClick Search columns and saved columns. For DoubleClick Search columns, only the `columnName` parameter is required. For saved columns only the `savedColumnName` parameter is required. Both `columnName` and `savedColumnName` cannot be set in the same stanza.\\ The maximum number of columns per request is 300."]
    pub columns: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReportApiColumnSpec>>>,
    #[serde(rename = "downloadFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Format that the report should be returned in. Currently `csv` or `tsv` is supported."]
    pub download_format: ::std::option::Option<::std::string::String>,
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of filters to be applied to the report.\\ The maximum number of filters per request is 300."]
    pub filters: ::std::option::Option<::std::vec::Vec<ReportRequestFilters>>,
    #[serde(rename = "includeDeletedEntities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Determines if removed entities should be included in the report. Defaults to `false`. Deprecated, please use `includeRemovedEntities` instead."]
    pub include_deleted_entities: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "includeRemovedEntities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Determines if removed entities should be included in the report. Defaults to `false`."]
    pub include_removed_entities: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "maxRowsPerFile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Asynchronous report only. The maximum number of rows per report file. A large report is split into many files based on this field. Acceptable values are `1000000` to `100000000`, inclusive."]
    pub max_rows_per_file: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "orderBy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Synchronous report only. A list of columns and directions defining sorting to be performed on the report rows.\\ The maximum number of orderings per request is 300."]
    pub order_by: ::std::option::Option<::std::vec::Vec<ReportRequestOrderBy>>,
    #[serde(rename = "reportScope")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reportScope is a set of IDs that are used to determine which subset of entities will be returned in the report. The full lineage of IDs from the lowest scoped level desired up through agency is required."]
    pub report_scope: ::std::option::Option<ReportRequestReportScope>,
    #[serde(rename = "reportType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Determines the type of rows that are returned in the report. For example, if you specify `reportType: keyword`, each row in the report will contain data about a keyword. See the [Types of Reports](/search-ads/v2/report-types/) reference for the columns that are available for each type."]
    pub report_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rowCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Synchronous report only. The maximum number of rows to return; additional rows are dropped. Acceptable values are `0` to `10000`, inclusive. Defaults to `10000`."]
    pub row_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startRow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Synchronous report only. Zero-based index of the first row to return. Acceptable values are `0` to `50000`, inclusive. Defaults to `0`."]
    pub start_row: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "statisticsCurrency")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the currency in which monetary will be returned. Possible values are: `usd`, `agency` (valid if the report is scoped to agency or lower), `advertiser` (valid if the report is scoped to * advertiser or lower), or `account` (valid if the report is scoped to engine account or lower)."]
    pub statistics_currency: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If metrics are requested in a report, this argument will be used to restrict the metrics to a specific time range."]
    pub time_range: ::std::option::Option<ReportRequestTimeRange>,
    #[serde(rename = "verifySingleTimeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If `true`, the report would only be created if all the requested stat data are sourced from a single timezone. Defaults to `false`."]
    pub verify_single_time_zone: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportRequestFilters {
    #[serde(rename = "column")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Column to perform the filter on. This can be a DoubleClick Search column or a saved column."]
    pub column: ::std::option::Option<::std::boxed::Box<ReportApiColumnSpec>>,
    #[serde(rename = "operator")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Operator to use in the filter. See the filter reference for a list of available operators."]
    pub operator: ::std::option::Option<::std::string::String>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of values to filter the column value against.\\ The maximum number of filter values per request is 300."]
    pub values: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportRequestOrderBy {
    #[serde(rename = "column")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Column to perform the sort on. This can be a DoubleClick Search-defined column or a saved column."]
    pub column: ::std::option::Option<::std::boxed::Box<ReportApiColumnSpec>>,
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sort direction, which is either `ascending` or `descending`."]
    pub sort_order: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The reportScope is a set of IDs that are used to determine which subset of entities will be returned in the report. The full lineage of IDs from the lowest scoped level desired up through agency is required."]
pub struct ReportRequestReportScope {
    #[serde(rename = "adGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DS ad group ID."]
    pub ad_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "adId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DS ad ID."]
    pub ad_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DS advertiser ID."]
    pub advertiser_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "agencyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DS agency ID."]
    pub agency_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "campaignId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DS campaign ID."]
    pub campaign_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "engineAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DS engine account ID."]
    pub engine_account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keywordId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DS keyword ID."]
    pub keyword_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "If metrics are requested in a report, this argument will be used to restrict the metrics to a specific time range."]
pub struct ReportRequestTimeRange {
    #[serde(rename = "changedAttributesSinceTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inclusive UTC timestamp in RFC format, e.g., `2013-07-16T10:16:23.555Z`. See additional references on how changed attribute reports work."]
    pub changed_attributes_since_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "changedMetricsSinceTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inclusive UTC timestamp in RFC format, e.g., `2013-07-16T10:16:23.555Z`. See additional references on how changed metrics reports work."]
    pub changed_metrics_since_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inclusive date in YYYY-MM-DD format."]
    pub end_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inclusive date in YYYY-MM-DD format."]
    pub start_date: ::std::option::Option<::std::string::String>,
}
#[doc = "A row in a DoubleClick Search report."]
pub type ReportRow = ::std::collections::BTreeMap<String, ::serde_json::Value>;
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A saved column"]
pub struct SavedColumn {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies this as a SavedColumn resource. Value: the fixed string doubleclicksearch#savedColumn."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "savedColumnName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the saved column."]
    pub saved_column_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of data this saved column will produce."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of saved columns. Advertisers create saved columns to report on Floodlight activities, Google Analytics goals, or custom KPIs. To request reports with saved columns, you'll need the saved column names that are available from this list."]
pub struct SavedColumnList {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The saved columns being requested."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SavedColumn>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies this as a SavedColumnList resource. Value: the fixed string doubleclicksearch#savedColumnList."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request to update availability."]
pub struct UpdateAvailabilityRequest {
    #[serde(rename = "availabilities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The availabilities being requested."]
    pub availabilities: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Availability>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response to a update availability request."]
pub struct UpdateAvailabilityResponse {
    #[serde(rename = "availabilities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The availabilities being returned."]
    pub availabilities: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Availability>>>,
}
