#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A task to execute on the completion of a job. See https://cloud.google.com/dlp/docs/concepts-actions to learn more."]
pub struct GooglePrivacyDlpV2Action {
    #[serde(rename = "jobNotificationEmails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enable email notification for project owners and editors on job's completion/failure."]
    pub job_notification_emails:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2JobNotificationEmails>>,
    #[serde(rename = "pubSub")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Publish a notification to a pubsub topic."]
    pub pub_sub: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PublishToPubSub>>,
    #[serde(rename = "publishFindingsToCloudDataCatalog")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Publish findings to Cloud Datahub."]
    pub publish_findings_to_cloud_data_catalog: ::std::option::Option<
        ::std::boxed::Box<GooglePrivacyDlpV2PublishFindingsToCloudDataCatalog>,
    >,
    #[serde(rename = "publishSummaryToCscc")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Publish summary to Cloud Security Command Center (Alpha)."]
    pub publish_summary_to_cscc:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PublishSummaryToCscc>>,
    #[serde(rename = "publishToStackdriver")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enable Stackdriver metric dlp.googleapis.com/finding_count."]
    pub publish_to_stackdriver:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PublishToStackdriver>>,
    #[serde(rename = "saveFindings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Save resulting findings in a provided location."]
    pub save_findings: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2SaveFindings>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for ActivateJobTrigger."]
pub struct GooglePrivacyDlpV2ActivateJobTriggerRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Result of a risk analysis operation request."]
pub struct GooglePrivacyDlpV2AnalyzeDataSourceRiskDetails {
    #[serde(rename = "categoricalStatsResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Categorical stats result"]
    pub categorical_stats_result:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CategoricalStatsResult>>,
    #[serde(rename = "deltaPresenceEstimationResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Delta-presence result"]
    pub delta_presence_estimation_result:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DeltaPresenceEstimationResult>>,
    #[serde(rename = "kAnonymityResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "K-anonymity result"]
    pub k_anonymity_result:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2KAnonymityResult>>,
    #[serde(rename = "kMapEstimationResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "K-map result"]
    pub k_map_estimation_result:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2KMapEstimationResult>>,
    #[serde(rename = "lDiversityResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "L-divesity result"]
    pub l_diversity_result:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2LDiversityResult>>,
    #[serde(rename = "numericalStatsResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Numerical stats result"]
    pub numerical_stats_result:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2NumericalStatsResult>>,
    #[serde(rename = "requestedOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration used for this job."]
    pub requested_options:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RequestedRiskAnalysisOptions>>,
    #[serde(rename = "requestedPrivacyMetric")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Privacy metric to compute."]
    pub requested_privacy_metric:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PrivacyMetric>>,
    #[serde(rename = "requestedSourceTable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input dataset to compute metrics over."]
    pub requested_source_table:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryTable>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An auxiliary table contains statistical information on the relative frequency of different quasi-identifiers values. It has one or several quasi-identifiers columns, and one column that indicates the relative frequency of each quasi-identifier tuple. If a tuple is present in the data but not in the auxiliary table, the corresponding relative frequency is assumed to be zero (and thus, the tuple is highly reidentifiable)."]
pub struct GooglePrivacyDlpV2AuxiliaryTable {
    #[serde(rename = "quasiIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Quasi-identifier columns."]
    pub quasi_ids:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2QuasiIdField>>>,
    #[serde(rename = "relativeFrequency")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The relative frequency column must contain a floating-point number between 0 and 1 (inclusive). Null values are assumed to be zero."]
    pub relative_frequency: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
    #[serde(rename = "table")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Auxiliary table location."]
    pub table: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryTable>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message defining a field of a BigQuery table."]
pub struct GooglePrivacyDlpV2BigQueryField {
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Designated field in the BigQuery table."]
    pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
    #[serde(rename = "table")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Source table of the field."]
    pub table: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryTable>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Row key for identifying a record in BigQuery table."]
pub struct GooglePrivacyDlpV2BigQueryKey {
    #[serde(rename = "rowNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Row number inferred at the time the table was scanned. This value is nondeterministic, cannot be queried, and may be null for inspection jobs. To locate findings within a table, specify `inspect_job.storage_config.big_query_options.identifying_fields` in `CreateDlpJobRequest`."]
    pub row_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tableReference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Complete BigQuery table reference."]
    pub table_reference: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryTable>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options defining BigQuery table and row identifiers."]
pub struct GooglePrivacyDlpV2BigQueryOptions {
    #[serde(rename = "excludedFields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "References to fields excluded from scanning. This allows you to skip inspection of entire columns which you know have no findings."]
    pub excluded_fields:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>>,
    #[serde(rename = "identifyingFields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Table fields that may uniquely identify a row within the table. When `actions.saveFindings.outputConfig.table` is specified, the values of columns specified here are available in the output table under `location.content_locations.record_location.record_key.id_values`. Nested fields such as `person.birthdate.year` are allowed."]
    pub identifying_fields:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>>,
    #[serde(rename = "rowsLimit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Max number of rows to scan. If the table has more rows than this value, the rest of the rows are omitted. If not set, or if set to 0, all rows will be scanned. Only one of rows_limit and rows_limit_percent can be specified. Cannot be used in conjunction with TimespanConfig."]
    pub rows_limit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rowsLimitPercent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Max percentage of rows to scan. The rest are omitted. The number of rows scanned is rounded down. Must be between 0 and 100, inclusively. Both 0 and 100 means no limit. Defaults to 0. Only one of rows_limit and rows_limit_percent can be specified. Cannot be used in conjunction with TimespanConfig."]
    pub rows_limit_percent: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "sampleMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sample_method: ::std::option::Option<GooglePrivacyDlpV2BigQueryOptionsSampleMethodEnum>,
    #[serde(rename = "tableReference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Complete BigQuery table reference."]
    pub table_reference: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryTable>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GooglePrivacyDlpV2BigQueryOptionsSampleMethodEnum {
    #[serde(rename = "SAMPLE_METHOD_UNSPECIFIED")]
    #[doc = ""]
    SampleMethodUnspecified,
    #[serde(rename = "TOP")]
    #[doc = "Scan groups of rows in the order BigQuery provides (default). Multiple groups of rows may be scanned in parallel, so results may not appear in the same order the rows are read."]
    Top,
    #[serde(rename = "RANDOM_START")]
    #[doc = "Randomly pick groups of rows to scan."]
    RandomStart,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message defining the location of a BigQuery table. A table is uniquely identified by its project_id, dataset_id, and table_name. Within a query a table is often referenced with a string in the format of: `:.` or `..`."]
pub struct GooglePrivacyDlpV2BigQueryTable {
    #[serde(rename = "datasetId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dataset ID of the table."]
    pub dataset_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google Cloud Platform project ID of the project containing the table. If omitted, project ID is inferred from the API call."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tableId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the table."]
    pub table_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Bounding box encompassing detected text within an image."]
pub struct GooglePrivacyDlpV2BoundingBox {
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Height of the bounding box in pixels."]
    pub height: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "left")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Left coordinate of the bounding box. (0,0) is upper left."]
    pub left: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "top")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Top coordinate of the bounding box. (0,0) is upper left."]
    pub top: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Width of the bounding box in pixels."]
    pub width: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Bucket is represented as a range, along with replacement values."]
pub struct GooglePrivacyDlpV2Bucket {
    #[serde(rename = "max")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Upper bound of the range, exclusive; type must match min."]
    pub max: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
    #[serde(rename = "min")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Lower bound of the range, inclusive. Type should be the same as max if used."]
    pub min: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
    #[serde(rename = "replacementValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Replacement value for this bucket."]
    pub replacement_value: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Generalization function that buckets values based on ranges. The ranges and replacement values are dynamically provided by the user for custom behavior, such as 1-30 -> LOW 31-65 -> MEDIUM 66-100 -> HIGH This can be used on data of type: number, long, string, timestamp. If the bound `Value` type differs from the type of data being transformed, we will first attempt converting the type of the data to be transformed to match the type of the bound before comparing. See https://cloud.google.com/dlp/docs/concepts-bucketing to learn more."]
pub struct GooglePrivacyDlpV2BucketingConfig {
    #[serde(rename = "buckets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set of buckets. Ranges must be non-overlapping."]
    pub buckets:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Bucket>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Container for bytes to inspect or redact."]
pub struct GooglePrivacyDlpV2ByteContentItem {
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content data to inspect or redact."]
    pub data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of data stored in the bytes string. Default will be TEXT_UTF8."]
    pub _type: ::std::option::Option<GooglePrivacyDlpV2ByteContentItemTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of data stored in the bytes string. Default will be TEXT_UTF8."]
pub enum GooglePrivacyDlpV2ByteContentItemTypeEnum {
    #[serde(rename = "BYTES_TYPE_UNSPECIFIED")]
    #[doc = "Unused"]
    BytesTypeUnspecified,
    #[serde(rename = "IMAGE")]
    #[doc = "Any image type."]
    Image,
    #[serde(rename = "IMAGE_JPEG")]
    #[doc = "jpeg"]
    ImageJpeg,
    #[serde(rename = "IMAGE_BMP")]
    #[doc = "bmp"]
    ImageBmp,
    #[serde(rename = "IMAGE_PNG")]
    #[doc = "png"]
    ImagePng,
    #[serde(rename = "IMAGE_SVG")]
    #[doc = "svg"]
    ImageSvg,
    #[serde(rename = "TEXT_UTF8")]
    #[doc = "plain text"]
    TextUtf8,
    #[serde(rename = "WORD_DOCUMENT")]
    #[doc = "docx, docm, dotx, dotm"]
    WordDocument,
    #[serde(rename = "PDF")]
    #[doc = "pdf"]
    Pdf,
    #[serde(rename = "AVRO")]
    #[doc = "avro"]
    Avro,
    #[serde(rename = "CSV")]
    #[doc = "csv"]
    Csv,
    #[serde(rename = "TSV")]
    #[doc = "tsv"]
    Tsv,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for canceling a DLP job."]
pub struct GooglePrivacyDlpV2CancelDlpJobRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Compute numerical stats over an individual column, including number of distinct values and value count distribution."]
pub struct GooglePrivacyDlpV2CategoricalStatsConfig {
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Field to compute categorical stats on. All column types are supported except for arrays and structs. However, it may be more informative to use NumericalStats when the field type is supported, depending on the data."]
    pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Histogram of value frequencies in the column."]
pub struct GooglePrivacyDlpV2CategoricalStatsHistogramBucket {
    #[serde(rename = "bucketSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of values in this bucket."]
    pub bucket_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bucketValueCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of distinct values in this bucket."]
    pub bucket_value_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bucketValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sample of value frequencies in this bucket. The total number of values returned per bucket is capped at 20."]
    pub bucket_values:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2ValueFrequency>>>,
    #[serde(rename = "valueFrequencyLowerBound")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Lower bound on the value frequency of the values in this bucket."]
    pub value_frequency_lower_bound: ::std::option::Option<::std::string::String>,
    #[serde(rename = "valueFrequencyUpperBound")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Upper bound on the value frequency of the values in this bucket."]
    pub value_frequency_upper_bound: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Result of the categorical stats computation."]
pub struct GooglePrivacyDlpV2CategoricalStatsResult {
    #[serde(rename = "valueFrequencyHistogramBuckets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Histogram of value frequencies in the column."]
    pub value_frequency_histogram_buckets: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2CategoricalStatsHistogramBucket>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Partially mask a string by replacing a given number of characters with a fixed character. Masking can start from the beginning or end of the string. This can be used on data of any type (numbers, longs, and so on) and when de-identifying structured data we'll attempt to preserve the original data's type. (This allows you to take a long like 123 and modify it to a string like **3."]
pub struct GooglePrivacyDlpV2CharacterMaskConfig {
    #[serde(rename = "charactersToIgnore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When masking a string, items in this list will be skipped when replacing characters. For example, if the input string is `555-555-5555` and you instruct Cloud DLP to skip `-` and mask 5 characters with `*`, Cloud DLP returns `***-**5-5555`."]
    pub characters_to_ignore:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2CharsToIgnore>>>,
    #[serde(rename = "maskingCharacter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Character to use to mask the sensitive values—for example, `*` for an alphabetic string such as a name, or `0` for a numeric string such as ZIP code or credit card number. This string must have a length of 1. If not supplied, this value defaults to `*` for strings, and `0` for digits."]
    pub masking_character: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numberToMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of characters to mask. If not set, all matching chars will be masked. Skipped characters do not count towards this tally."]
    pub number_to_mask: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "reverseOrder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mask characters in reverse order. For example, if `masking_character` is `0`, `number_to_mask` is `14`, and `reverse_order` is `false`, then the input string `1234-5678-9012-3456` is masked as `00000000000000-3456`. If `masking_character` is `*`, `number_to_mask` is `3`, and `reverse_order` is `true`, then the string `12345` is masked as `12***`."]
    pub reverse_order: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Characters to skip when doing deidentification of a value. These will be left alone and skipped."]
pub struct GooglePrivacyDlpV2CharsToIgnore {
    #[serde(rename = "charactersToSkip")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Characters to not transform when masking."]
    pub characters_to_skip: ::std::option::Option<::std::string::String>,
    #[serde(rename = "commonCharactersToIgnore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common characters to not transform when masking. Useful to avoid removing punctuation."]
    pub common_characters_to_ignore:
        ::std::option::Option<GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Common characters to not transform when masking. Useful to avoid removing punctuation."]
pub enum GooglePrivacyDlpV2CharsToIgnoreCommonCharactersToIgnoreEnum {
    #[serde(rename = "COMMON_CHARS_TO_IGNORE_UNSPECIFIED")]
    #[doc = "Unused."]
    CommonCharsToIgnoreUnspecified,
    #[serde(rename = "NUMERIC")]
    #[doc = "0-9"]
    Numeric,
    #[serde(rename = "ALPHA_UPPER_CASE")]
    #[doc = "A-Z"]
    AlphaUpperCase,
    #[serde(rename = "ALPHA_LOWER_CASE")]
    #[doc = "a-z"]
    AlphaLowerCase,
    #[serde(rename = "PUNCTUATION")]
    #[doc = "US Punctuation, one of !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~"]
    Punctuation,
    #[serde(rename = "WHITESPACE")]
    #[doc = "Whitespace character, one of [ \\t\\n\\x0B\\f\\r]"]
    Whitespace,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message representing a set of files in Cloud Storage."]
pub struct GooglePrivacyDlpV2CloudStorageFileSet {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The url, in the format `gs:///`. Trailing wildcard in the path is allowed."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options defining a file or a set of files within a Google Cloud Storage bucket."]
pub struct GooglePrivacyDlpV2CloudStorageOptions {
    #[serde(rename = "bytesLimitPerFile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Max number of bytes to scan from a file. If a scanned file's size is bigger than this value then the rest of the bytes are omitted. Only one of bytes_limit_per_file and bytes_limit_per_file_percent can be specified. Cannot be set if de-identification is requested."]
    pub bytes_limit_per_file: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bytesLimitPerFilePercent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Max percentage of bytes to scan from a file. The rest are omitted. The number of bytes scanned is rounded down. Must be between 0 and 100, inclusively. Both 0 and 100 means no limit. Defaults to 0. Only one of bytes_limit_per_file and bytes_limit_per_file_percent can be specified. Cannot be set if de-identification is requested."]
    pub bytes_limit_per_file_percent: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "fileSet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of one or more files to scan."]
    pub file_set: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FileSet>>,
    #[serde(rename = "fileTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of file type groups to include in the scan. If empty, all files are scanned and available data format processors are applied. In addition, the binary content of the selected files is always scanned as well. Images are scanned only as binary if the specified region does not support image inspection and no file_types were specified. Image inspection is restricted to 'global', 'us', 'asia', and 'europe'."]
    pub file_types:
        ::std::option::Option<::std::vec::Vec<GooglePrivacyDlpV2CloudStorageOptionsFileTypesEnum>>,
    #[serde(rename = "filesLimitPercent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Limits the number of files to scan to this percentage of the input FileSet. Number of files scanned is rounded down. Must be between 0 and 100, inclusively. Both 0 and 100 means no limit. Defaults to 0."]
    pub files_limit_percent: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "sampleMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sample_method: ::std::option::Option<GooglePrivacyDlpV2CloudStorageOptionsSampleMethodEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GooglePrivacyDlpV2CloudStorageOptionsFileTypesEnum {
    #[serde(rename = "FILE_TYPE_UNSPECIFIED")]
    #[doc = "Includes all files."]
    FileTypeUnspecified,
    #[serde(rename = "BINARY_FILE")]
    #[doc = "Includes all file extensions not covered by another entry. Binary scanning attempts to convert the content of the file to utf_8 to scan the file. If you wish to avoid this fall back, specify one or more of the other FileType's in your storage scan."]
    BinaryFile,
    #[serde(rename = "TEXT_FILE")]
    #[doc = "Included file extensions: asc,asp, aspx, brf, c, cc,cfm, cgi, cpp, csv, cxx, c++, cs, css, dart, dat, dot, eml,, epbub, ged, go, h, hh, hpp, hxx, h++, hs, html, htm, mkd, markdown, m, ml, mli, perl, pl, plist, pm, php, phtml, pht, properties, py, pyw, rb, rbw, rs, rss, rc, scala, sh, sql, swift, tex, shtml, shtm, xhtml, lhs, ics, ini, java, js, json, kix, kml, ocaml, md, txt, text, tsv, vb, vcard, vcs, wml, xcodeproj, xml, xsl, xsd, yml, yaml."]
    TextFile,
    #[serde(rename = "IMAGE")]
    #[doc = "Included file extensions: bmp, gif, jpg, jpeg, jpe, png. bytes_limit_per_file has no effect on image files. Image inspection is restricted to 'global', 'us', 'asia', and 'europe'."]
    Image,
    #[serde(rename = "WORD")]
    #[doc = "Word files >30 MB will be scanned as binary files. Included file extensions: docx, dotx, docm, dotm"]
    Word,
    #[serde(rename = "PDF")]
    #[doc = "PDF files >30 MB will be scanned as binary files. Included file extensions: pdf"]
    Pdf,
    #[serde(rename = "AVRO")]
    #[doc = "Included file extensions: avro"]
    Avro,
    #[serde(rename = "CSV")]
    #[doc = "Included file extensions: csv"]
    Csv,
    #[serde(rename = "TSV")]
    #[doc = "Included file extensions: tsv"]
    Tsv,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GooglePrivacyDlpV2CloudStorageOptionsSampleMethodEnum {
    #[serde(rename = "SAMPLE_METHOD_UNSPECIFIED")]
    #[doc = ""]
    SampleMethodUnspecified,
    #[serde(rename = "TOP")]
    #[doc = "Scan from the top (default)."]
    Top,
    #[serde(rename = "RANDOM_START")]
    #[doc = "For each file larger than bytes_limit_per_file, randomly pick the offset to start scanning. The scanned bytes are contiguous."]
    RandomStart,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message representing a single file or path in Cloud Storage."]
pub struct GooglePrivacyDlpV2CloudStoragePath {
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A url representing a file or path (no wildcards) in Cloud Storage. Example: gs://[BUCKET_NAME]/dictionary.txt"]
    pub path: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message representing a set of files in a Cloud Storage bucket. Regular expressions are used to allow fine-grained control over which files in the bucket to include. Included files are those that match at least one item in `include_regex` and do not match any items in `exclude_regex`. Note that a file that matches items from both lists will _not_ be included. For a match to occur, the entire file path (i.e., everything in the url after the bucket name) must match the regular expression. For example, given the input `{bucket_name: \"mybucket\", include_regex: [\"directory1/.*\"], exclude_regex: [\"directory1/excluded.*\"]}`: * `gs://mybucket/directory1/myfile` will be included * `gs://mybucket/directory1/directory2/myfile` will be included (`.*` matches across `/`) * `gs://mybucket/directory0/directory1/myfile` will _not_ be included (the full path doesn't match any items in `include_regex`) * `gs://mybucket/directory1/excludedfile` will _not_ be included (the path matches an item in `exclude_regex`) If `include_regex` is left empty, it will match all files by default (this is equivalent to setting `include_regex: [\".*\"]`). Some other common use cases: * `{bucket_name: \"mybucket\", exclude_regex: [\".*\\.pdf\"]}` will include all files in `mybucket` except for .pdf files * `{bucket_name: \"mybucket\", include_regex: [\"directory/[^/]+\"]}` will include all files directly under `gs://mybucket/directory/`, without matching across `/`"]
pub struct GooglePrivacyDlpV2CloudStorageRegexFileSet {
    #[serde(rename = "bucketName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of a Cloud Storage bucket. Required."]
    pub bucket_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "excludeRegex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of regular expressions matching file paths to exclude. All files in the bucket that match at least one of these regular expressions will be excluded from the scan. Regular expressions use RE2 [syntax](https://github.com/google/re2/wiki/Syntax); a guide can be found under the google/re2 repository on GitHub."]
    pub exclude_regex: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "includeRegex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of regular expressions matching file paths to include. All files in the bucket that match at least one of these regular expressions will be included in the set of files, except for those that also match an item in `exclude_regex`. Leaving this field empty will match all files by default (this is equivalent to including `.*` in the list). Regular expressions use RE2 [syntax](https://github.com/google/re2/wiki/Syntax); a guide can be found under the google/re2 repository on GitHub."]
    pub include_regex: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a color in the RGB color space."]
pub struct GooglePrivacyDlpV2Color {
    #[serde(rename = "blue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of blue in the color as a value in the interval [0, 1]."]
    pub blue: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "green")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of green in the color as a value in the interval [0, 1]."]
    pub green: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "red")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of red in the color as a value in the interval [0, 1]."]
    pub red: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The field type of `value` and `field` do not need to match to be considered equal, but not all comparisons are possible. EQUAL_TO and NOT_EQUAL_TO attempt to compare even with incompatible types, but all other comparisons are invalid with incompatible types. A `value` of type: - `string` can be compared against all other types - `boolean` can only be compared against other booleans - `integer` can be compared against doubles or a string if the string value can be parsed as an integer. - `double` can be compared against integers or a string if the string can be parsed as a double. - `Timestamp` can be compared against strings in RFC 3339 date string format. - `TimeOfDay` can be compared against timestamps and strings in the format of 'HH:mm:ss'. If we fail to compare do to type mismatch, a warning will be given and the condition will evaluate to false."]
pub struct GooglePrivacyDlpV2Condition {
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Field within the record this condition is evaluated against."]
    pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
    #[serde(rename = "operator")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Operator used to compare the field or infoType to the value."]
    pub operator: ::std::option::Option<GooglePrivacyDlpV2ConditionOperatorEnum>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value to compare against. [Mandatory, except for `EXISTS` tests.]"]
    pub value: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Operator used to compare the field or infoType to the value."]
pub enum GooglePrivacyDlpV2ConditionOperatorEnum {
    #[serde(rename = "RELATIONAL_OPERATOR_UNSPECIFIED")]
    #[doc = "Unused"]
    RelationalOperatorUnspecified,
    #[serde(rename = "EQUAL_TO")]
    #[doc = "Equal. Attempts to match even with incompatible types."]
    EqualTo,
    #[serde(rename = "NOT_EQUAL_TO")]
    #[doc = "Not equal to. Attempts to match even with incompatible types."]
    NotEqualTo,
    #[serde(rename = "GREATER_THAN")]
    #[doc = "Greater than."]
    GreaterThan,
    #[serde(rename = "LESS_THAN")]
    #[doc = "Less than."]
    LessThan,
    #[serde(rename = "GREATER_THAN_OR_EQUALS")]
    #[doc = "Greater than or equals."]
    GreaterThanOrEquals,
    #[serde(rename = "LESS_THAN_OR_EQUALS")]
    #[doc = "Less than or equals."]
    LessThanOrEquals,
    #[serde(rename = "EXISTS")]
    #[doc = "Exists"]
    Exists,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A collection of conditions."]
pub struct GooglePrivacyDlpV2Conditions {
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A collection of conditions."]
    pub conditions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Condition>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a container that may contain DLP findings. Examples of a container include a file, table, or database record."]
pub struct GooglePrivacyDlpV2Container {
    #[serde(rename = "fullPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A string representation of the full container name. Examples: - BigQuery: 'Project:DataSetId.TableId' - Google Cloud Storage: 'gs://Bucket/folders/filename.txt'"]
    pub full_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Project where the finding was found. Can be different from the project that owns the finding."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "relativePath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The rest of the path after the root. Examples: - For BigQuery table `project_id:dataset_id.table_id`, the relative path is `table_id` - Google Cloud Storage file `gs://bucket/folder/filename.txt`, the relative path is `folder/filename.txt`"]
    pub relative_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rootPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The root of the container. Examples: - For BigQuery table `project_id:dataset_id.table_id`, the root is `dataset_id` - For Google Cloud Storage file `gs://bucket/folder/filename.txt`, the root is `gs://bucket`"]
    pub root_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Container type, for example BigQuery or Google Cloud Storage."]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Findings container modification timestamp, if applicable. For Google Cloud Storage contains last file modification timestamp. For BigQuery table contains last_modified_time property. For Datastore - not populated."]
    pub update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Findings container version, if available (\"generation\" for Google Cloud Storage)."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Container structure for the content to inspect."]
pub struct GooglePrivacyDlpV2ContentItem {
    #[serde(rename = "byteItem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content data to inspect or redact. Replaces `type` and `data`."]
    pub byte_item: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ByteContentItem>>,
    #[serde(rename = "table")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Structured content for inspection. See https://cloud.google.com/dlp/docs/inspecting-text#inspecting_a_table to learn more."]
    pub table: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Table>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "String data to inspect or redact."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Precise location of the finding within a document, record, image, or metadata container."]
pub struct GooglePrivacyDlpV2ContentLocation {
    #[serde(rename = "containerName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the container where the finding is located. The top level name is the source file name or table name. Names of some common storage containers are formatted as follows: * BigQuery tables: `{project_id}:{dataset_id}.{table_id}` * Cloud Storage files: `gs://{bucket}/{path}` * Datastore namespace: {namespace} Nested names could be absent if the embedded object has no string identifier (for an example an image contained within a document)."]
    pub container_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "containerTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Findings container modification timestamp, if applicable. For Google Cloud Storage contains last file modification timestamp. For BigQuery table contains last_modified_time property. For Datastore - not populated."]
    pub container_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "containerVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Findings container version, if available (\"generation\" for Google Cloud Storage)."]
    pub container_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "documentLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location data for document files."]
    pub document_location:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DocumentLocation>>,
    #[serde(rename = "imageLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location within an image's pixels."]
    pub image_location: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ImageLocation>>,
    #[serde(rename = "metadataLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location within the metadata for inspected content."]
    pub metadata_location:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2MetadataLocation>>,
    #[serde(rename = "recordLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location within a row or record of a database table."]
    pub record_location: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RecordLocation>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for CreateDeidentifyTemplate."]
pub struct GooglePrivacyDlpV2CreateDeidentifyTemplateRequest {
    #[serde(rename = "deidentifyTemplate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The DeidentifyTemplate to create."]
    pub deidentify_template:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DeidentifyTemplate>>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. This field has no effect."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The template id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one."]
    pub template_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for CreateDlpJobRequest. Used to initiate long running jobs such as calculating risk metrics or inspecting Google Cloud Storage."]
pub struct GooglePrivacyDlpV2CreateDlpJobRequest {
    #[serde(rename = "inspectJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An inspection job scans a storage repository for InfoTypes."]
    pub inspect_job: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectJobConfig>>,
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The job id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one."]
    pub job_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. This field has no effect."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "riskJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A risk analysis job calculates re-identification risk metrics for a BigQuery table."]
    pub risk_job: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RiskAnalysisJobConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for CreateInspectTemplate."]
pub struct GooglePrivacyDlpV2CreateInspectTemplateRequest {
    #[serde(rename = "inspectTemplate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The InspectTemplate to create."]
    pub inspect_template:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectTemplate>>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. This field has no effect."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The template id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one."]
    pub template_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for CreateJobTrigger."]
pub struct GooglePrivacyDlpV2CreateJobTriggerRequest {
    #[serde(rename = "jobTrigger")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The JobTrigger to create."]
    pub job_trigger: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2JobTrigger>>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. This field has no effect."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "triggerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The trigger id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one."]
    pub trigger_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for CreateStoredInfoType."]
pub struct GooglePrivacyDlpV2CreateStoredInfoTypeRequest {
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Configuration of the storedInfoType to create."]
    pub config: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2StoredInfoTypeConfig>>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. This field has no effect."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storedInfoTypeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The storedInfoType ID can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one."]
    pub stored_info_type_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Pseudonymization method that generates deterministic encryption for the given input. Outputs a base64 encoded representation of the encrypted output. Uses AES-SIV based on the RFC https://tools.ietf.org/html/rfc5297."]
pub struct GooglePrivacyDlpV2CryptoDeterministicConfig {
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A context may be used for higher security and maintaining referential integrity such that the same identifier in two different contexts will be given a distinct surrogate. The context is appended to plaintext value being encrypted. On decryption the provided context is validated against the value used during encryption. If a context was provided during encryption, same context must be provided during decryption as well. If the context is not set, plaintext would be used as is for encryption. If the context is set but: 1. there is no record present when transforming a given value or 2. the field is not present when transforming a given value, plaintext would be used as is for encryption. Note that case (1) is expected when an `InfoTypeTransformation` is applied to both structured and non-structured `ContentItem`s."]
    pub context: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
    #[serde(rename = "cryptoKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The key used by the encryption function."]
    pub crypto_key: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CryptoKey>>,
    #[serde(rename = "surrogateInfoType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The custom info type to annotate the surrogate with. This annotation will be applied to the surrogate by prefixing it with the name of the custom info type followed by the number of characters comprising the surrogate. The following scheme defines the format: {info type name}({surrogate character count}):{surrogate} For example, if the name of custom info type is 'MY_TOKEN_INFO_TYPE' and the surrogate is 'abc', the full replacement value will be: 'MY_TOKEN_INFO_TYPE(3):abc' This annotation identifies the surrogate when inspecting content using the custom info type 'Surrogate'. This facilitates reversal of the surrogate when it occurs in free text. Note: For record transformations where the entire cell in a table is being transformed, surrogates are not mandatory. Surrogates are used to denote the location of the token and are necessary for re-identification in free form text. In order for inspection to work properly, the name of this info type must not occur naturally anywhere in your data; otherwise, inspection may either - reverse a surrogate that does not correspond to an actual identifier - be unable to parse the surrogate and result in an error Therefore, choose your custom info type name carefully after considering what your data looks like. One way to select a name that has a high chance of yielding reliable detection is to include one or more unicode characters that are highly improbable to exist in your data. For example, assuming your data is entered from a regular ASCII keyboard, the symbol with the hex code point 29DD might be used like so: ⧝MY_TOKEN_TYPE."]
    pub surrogate_info_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Pseudonymization method that generates surrogates via cryptographic hashing. Uses SHA-256. The key size must be either 32 or 64 bytes. Outputs a base64 encoded representation of the hashed output (for example, L7k0BHmF1ha5U3NfGykjro4xWi1MPVQPjhMAZbSV9mM=). Currently, only string and integer values can be hashed. See https://cloud.google.com/dlp/docs/pseudonymization to learn more."]
pub struct GooglePrivacyDlpV2CryptoHashConfig {
    #[serde(rename = "cryptoKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The key used by the hash function."]
    pub crypto_key: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CryptoKey>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This is a data encryption key (DEK) (as opposed to a key encryption key (KEK) stored by KMS). When using KMS to wrap/unwrap DEKs, be sure to set an appropriate IAM policy on the KMS CryptoKey (KEK) to ensure an attacker cannot unwrap the data crypto key."]
pub struct GooglePrivacyDlpV2CryptoKey {
    #[serde(rename = "kmsWrapped")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Kms wrapped key"]
    pub kms_wrapped:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2KmsWrappedCryptoKey>>,
    #[serde(rename = "transient")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Transient crypto key"]
    pub transient: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TransientCryptoKey>>,
    #[serde(rename = "unwrapped")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unwrapped crypto key"]
    pub unwrapped: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2UnwrappedCryptoKey>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Replaces an identifier with a surrogate using Format Preserving Encryption (FPE) with the FFX mode of operation; however when used in the `ReidentifyContent` API method, it serves the opposite function by reversing the surrogate back into the original identifier. The identifier must be encoded as ASCII. For a given crypto key and context, the same identifier will be replaced with the same surrogate. Identifiers must be at least two characters long. In the case that the identifier is the empty string, it will be skipped. See https://cloud.google.com/dlp/docs/pseudonymization to learn more. Note: We recommend using CryptoDeterministicConfig for all use cases which do not require preserving the input alphabet space and size, plus warrant referential integrity."]
pub struct GooglePrivacyDlpV2CryptoReplaceFfxFpeConfig {
    #[serde(rename = "commonAlphabet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common alphabets."]
    pub common_alphabet:
        ::std::option::Option<GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum>,
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The 'tweak', a context may be used for higher security since the same identifier in two different contexts won't be given the same surrogate. If the context is not set, a default tweak will be used. If the context is set but: 1. there is no record present when transforming a given value or 1. the field is not present when transforming a given value, a default tweak will be used. Note that case (1) is expected when an `InfoTypeTransformation` is applied to both structured and non-structured `ContentItem`s. Currently, the referenced field may be of value type integer or string. The tweak is constructed as a sequence of bytes in big endian byte order such that: - a 64 bit integer is encoded followed by a single byte of value 1 - a string is encoded in UTF-8 format followed by a single byte of value 2"]
    pub context: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
    #[serde(rename = "cryptoKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The key used by the encryption algorithm."]
    pub crypto_key: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CryptoKey>>,
    #[serde(rename = "customAlphabet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This is supported by mapping these to the alphanumeric characters that the FFX mode natively supports. This happens before/after encryption/decryption. Each character listed must appear only once. Number of characters must be in the range [2, 95]. This must be encoded as ASCII. The order of characters does not matter. The full list of allowed characters is: 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz ~`!@#$%^&*()_-+={[}]|\\:;\"'<,>.?/"]
    pub custom_alphabet: ::std::option::Option<::std::string::String>,
    #[serde(rename = "radix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The native way to select the alphabet. Must be in the range [2, 95]."]
    pub radix: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "surrogateInfoType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The custom infoType to annotate the surrogate with. This annotation will be applied to the surrogate by prefixing it with the name of the custom infoType followed by the number of characters comprising the surrogate. The following scheme defines the format: info_type_name(surrogate_character_count):surrogate For example, if the name of custom infoType is 'MY_TOKEN_INFO_TYPE' and the surrogate is 'abc', the full replacement value will be: 'MY_TOKEN_INFO_TYPE(3):abc' This annotation identifies the surrogate when inspecting content using the custom infoType [`SurrogateType`](https://cloud.google.com/dlp/docs/reference/rest/v2/InspectConfig#surrogatetype). This facilitates reversal of the surrogate when it occurs in free text. In order for inspection to work properly, the name of this infoType must not occur naturally anywhere in your data; otherwise, inspection may find a surrogate that does not correspond to an actual identifier. Therefore, choose your custom infoType name carefully after considering what your data looks like. One way to select a name that has a high chance of yielding reliable detection is to include one or more unicode characters that are highly improbable to exist in your data. For example, assuming your data is entered from a regular ASCII keyboard, the symbol with the hex code point 29DD might be used like so: ⧝MY_TOKEN_TYPE"]
    pub surrogate_info_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Common alphabets."]
pub enum GooglePrivacyDlpV2CryptoReplaceFfxFpeConfigCommonAlphabetEnum {
    #[serde(rename = "FFX_COMMON_NATIVE_ALPHABET_UNSPECIFIED")]
    #[doc = "Unused."]
    FfxCommonNativeAlphabetUnspecified,
    #[serde(rename = "NUMERIC")]
    #[doc = "`[0-9]` (radix of 10)"]
    Numeric,
    #[serde(rename = "HEXADECIMAL")]
    #[doc = "`[0-9A-F]` (radix of 16)"]
    Hexadecimal,
    #[serde(rename = "UPPER_CASE_ALPHA_NUMERIC")]
    #[doc = "`[0-9A-Z]` (radix of 36)"]
    UpperCaseAlphaNumeric,
    #[serde(rename = "ALPHA_NUMERIC")]
    #[doc = "`[0-9A-Za-z]` (radix of 62)"]
    AlphaNumeric,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Custom information type provided by the user. Used to find domain-specific sensitive information configurable to the data in question."]
pub struct GooglePrivacyDlpV2CustomInfoType {
    #[serde(rename = "detectionRules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set of detection rules to apply to all findings of this CustomInfoType. Rules are applied in order that they are specified. Not supported for the `surrogate_type` CustomInfoType."]
    pub detection_rules:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2DetectionRule>>>,
    #[serde(rename = "dictionary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of phrases to detect as a CustomInfoType."]
    pub dictionary: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Dictionary>>,
    #[serde(rename = "exclusionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set to EXCLUSION_TYPE_EXCLUDE this infoType will not cause a finding to be returned. It still can be used for rules matching."]
    pub exclusion_type: ::std::option::Option<GooglePrivacyDlpV2CustomInfoTypeExclusionTypeEnum>,
    #[serde(rename = "infoType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "CustomInfoType can either be a new infoType, or an extension of built-in infoType, when the name matches one of existing infoTypes and that infoType is specified in `InspectContent.info_types` field. Specifying the latter adds findings to the one detected by the system. If built-in info type is not specified in `InspectContent.info_types` list then the name is treated as a custom info type."]
    pub info_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
    #[serde(rename = "likelihood")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Likelihood to return for this CustomInfoType. This base value can be altered by a detection rule if the finding meets the criteria specified by the rule. Defaults to `VERY_LIKELY` if not specified."]
    pub likelihood: ::std::option::Option<GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum>,
    #[serde(rename = "regex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Regular expression based CustomInfoType."]
    pub regex: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Regex>>,
    #[serde(rename = "storedType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Load an existing `StoredInfoType` resource for use in `InspectDataSource`. Not currently supported in `InspectContent`."]
    pub stored_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2StoredType>>,
    #[serde(rename = "surrogateType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Message for detecting output from deidentification transformations that support reversing."]
    pub surrogate_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2SurrogateType>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "If set to EXCLUSION_TYPE_EXCLUDE this infoType will not cause a finding to be returned. It still can be used for rules matching."]
pub enum GooglePrivacyDlpV2CustomInfoTypeExclusionTypeEnum {
    #[serde(rename = "EXCLUSION_TYPE_UNSPECIFIED")]
    #[doc = "A finding of this custom info type will not be excluded from results."]
    ExclusionTypeUnspecified,
    #[serde(rename = "EXCLUSION_TYPE_EXCLUDE")]
    #[doc = "A finding of this custom info type will be excluded from final results, but can still affect rule execution."]
    ExclusionTypeExclude,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Likelihood to return for this CustomInfoType. This base value can be altered by a detection rule if the finding meets the criteria specified by the rule. Defaults to `VERY_LIKELY` if not specified."]
pub enum GooglePrivacyDlpV2CustomInfoTypeLikelihoodEnum {
    #[serde(rename = "LIKELIHOOD_UNSPECIFIED")]
    #[doc = "Default value; same as POSSIBLE."]
    LikelihoodUnspecified,
    #[serde(rename = "VERY_UNLIKELY")]
    #[doc = "Few matching elements."]
    VeryUnlikely,
    #[serde(rename = "UNLIKELY")]
    #[doc = ""]
    Unlikely,
    #[serde(rename = "POSSIBLE")]
    #[doc = "Some matching elements."]
    Possible,
    #[serde(rename = "LIKELY")]
    #[doc = ""]
    Likely,
    #[serde(rename = "VERY_LIKELY")]
    #[doc = "Many matching elements."]
    VeryLikely,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Record key for a finding in Cloud Datastore."]
pub struct GooglePrivacyDlpV2DatastoreKey {
    #[serde(rename = "entityKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Datastore entity key."]
    pub entity_key: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Key>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options defining a data set within Google Cloud Datastore."]
pub struct GooglePrivacyDlpV2DatastoreOptions {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind to process."]
    pub kind: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2KindExpression>>,
    #[serde(rename = "partitionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A partition ID identifies a grouping of entities. The grouping is always by project and namespace, however the namespace ID may be empty."]
    pub partition_id: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PartitionId>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Shifts dates by random number of days, with option to be consistent for the same context. See https://cloud.google.com/dlp/docs/concepts-date-shifting to learn more."]
pub struct GooglePrivacyDlpV2DateShiftConfig {
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Points to the field that contains the context, for example, an entity id. If set, must also set cryptoKey. If set, shift will be consistent for the given context."]
    pub context: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
    #[serde(rename = "cryptoKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Causes the shift to be computed based on this key and the context. This results in the same shift for the same context and crypto_key. If set, must also set context. Can only be applied to table items."]
    pub crypto_key: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CryptoKey>>,
    #[serde(rename = "lowerBoundDays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. For example, -5 means shift date to at most 5 days back in the past."]
    pub lower_bound_days: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "upperBoundDays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Range of shift in days. Actual shift will be selected at random within this range (inclusive ends). Negative means shift to earlier in time. Must not be more than 365250 days (1000 years) each direction. For example, 3 means shift date to at most 3 days into the future."]
    pub upper_bound_days: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message for a date time object. e.g. 2018-01-01, 5th August."]
pub struct GooglePrivacyDlpV2DateTime {
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "One or more of the following must be set. Must be a valid date or time value."]
    pub date: ::std::option::Option<::std::boxed::Box<GoogleTypeDate>>,
    #[serde(rename = "dayOfWeek")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Day of week"]
    pub day_of_week: ::std::option::Option<GooglePrivacyDlpV2DateTimeDayOfWeekEnum>,
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time of day"]
    pub time: ::std::option::Option<::std::boxed::Box<GoogleTypeTimeOfDay>>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time zone"]
    pub time_zone: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TimeZone>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Day of week"]
pub enum GooglePrivacyDlpV2DateTimeDayOfWeekEnum {
    #[serde(rename = "DAY_OF_WEEK_UNSPECIFIED")]
    #[doc = "The day of the week is unspecified."]
    DayOfWeekUnspecified,
    #[serde(rename = "MONDAY")]
    #[doc = "Monday"]
    Monday,
    #[serde(rename = "TUESDAY")]
    #[doc = "Tuesday"]
    Tuesday,
    #[serde(rename = "WEDNESDAY")]
    #[doc = "Wednesday"]
    Wednesday,
    #[serde(rename = "THURSDAY")]
    #[doc = "Thursday"]
    Thursday,
    #[serde(rename = "FRIDAY")]
    #[doc = "Friday"]
    Friday,
    #[serde(rename = "SATURDAY")]
    #[doc = "Saturday"]
    Saturday,
    #[serde(rename = "SUNDAY")]
    #[doc = "Sunday"]
    Sunday,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The configuration that controls how the data will change."]
pub struct GooglePrivacyDlpV2DeidentifyConfig {
    #[serde(rename = "infoTypeTransformations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Treat the dataset as free-form text and apply the same free text transformation everywhere."]
    pub info_type_transformations:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoTypeTransformations>>,
    #[serde(rename = "recordTransformations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Treat the dataset as structured. Transformations can be applied to specific locations within structured datasets, such as transforming a column within a table."]
    pub record_transformations:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RecordTransformations>>,
    #[serde(rename = "transformationErrorHandling")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mode for handling transformation errors. If left unspecified, the default mode is `TransformationErrorHandling.ThrowError`."]
    pub transformation_error_handling:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TransformationErrorHandling>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to de-identify a list of items."]
pub struct GooglePrivacyDlpV2DeidentifyContentRequest {
    #[serde(rename = "deidentifyConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for the de-identification of the content item. Items specified here will override the template referenced by the deidentify_template_name argument."]
    pub deidentify_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DeidentifyConfig>>,
    #[serde(rename = "deidentifyTemplateName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Template to use. Any configuration directly specified in deidentify_config will override those set in the template. Singular fields that are set in this request will replace their corresponding fields in the template. Repeated fields are appended. Singular sub-messages and groups are recursively merged."]
    pub deidentify_template_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inspectConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for the inspector. Items specified here will override the template referenced by the inspect_template_name argument."]
    pub inspect_config: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectConfig>>,
    #[serde(rename = "inspectTemplateName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Template to use. Any configuration directly specified in inspect_config will override those set in the template. Singular fields that are set in this request will replace their corresponding fields in the template. Repeated fields are appended. Singular sub-messages and groups are recursively merged."]
    pub inspect_template_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "item")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The item to de-identify. Will be treated as text."]
    pub item: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ContentItem>>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. This field has no effect."]
    pub location_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Results of de-identifying a ContentItem."]
pub struct GooglePrivacyDlpV2DeidentifyContentResponse {
    #[serde(rename = "item")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The de-identified item."]
    pub item: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ContentItem>>,
    #[serde(rename = "overview")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An overview of the changes that were made on the `item`."]
    pub overview:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TransformationOverview>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "DeidentifyTemplates contains instructions on how to de-identify content. See https://cloud.google.com/dlp/docs/concepts-templates to learn more."]
pub struct GooglePrivacyDlpV2DeidentifyTemplate {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The creation timestamp of an inspectTemplate."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deidentifyConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "///////////// // The core content of the template // ///////////////"]
    pub deidentify_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DeidentifyConfig>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Short description (max 256 chars)."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display name (max 256 chars)."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The template name. The template will have one of the following formats: `projects/PROJECT_ID/deidentifyTemplates/TEMPLATE_ID` OR `organizations/ORGANIZATION_ID/deidentifyTemplates/TEMPLATE_ID`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The last update timestamp of an inspectTemplate."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "δ-presence metric, used to estimate how likely it is for an attacker to figure out that one given individual appears in a de-identified dataset. Similarly to the k-map metric, we cannot compute δ-presence exactly without knowing the attack dataset, so we use a statistical model instead."]
pub struct GooglePrivacyDlpV2DeltaPresenceEstimationConfig {
    #[serde(rename = "auxiliaryTables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Several auxiliary tables can be used in the analysis. Each custom_tag used to tag a quasi-identifiers field must appear in exactly one field of one auxiliary table."]
    pub auxiliary_tables: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2StatisticalTable>>,
    >,
    #[serde(rename = "quasiIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Fields considered to be quasi-identifiers. No two fields can have the same tag."]
    pub quasi_ids:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2QuasiId>>>,
    #[serde(rename = "regionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ISO 3166-1 alpha-2 region code to use in the statistical modeling. Set if no column is tagged with a region-specific InfoType (like US_ZIP_5) or a region code."]
    pub region_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A DeltaPresenceEstimationHistogramBucket message with the following values: min_probability: 0.1 max_probability: 0.2 frequency: 42 means that there are 42 records for which δ is in [0.1, 0.2). An important particular case is when min_probability = max_probability = 1: then, every individual who shares this quasi-identifier combination is in the dataset."]
pub struct GooglePrivacyDlpV2DeltaPresenceEstimationHistogramBucket {
    #[serde(rename = "bucketSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of records within these probability bounds."]
    pub bucket_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bucketValueCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of distinct quasi-identifier tuple values in this bucket."]
    pub bucket_value_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bucketValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sample of quasi-identifier tuple values in this bucket. The total number of classes returned per bucket is capped at 20."]
    pub bucket_values: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2DeltaPresenceEstimationQuasiIdValues>>,
    >,
    #[serde(rename = "maxProbability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always greater than or equal to min_probability."]
    pub max_probability: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "minProbability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Between 0 and 1."]
    pub min_probability: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A tuple of values for the quasi-identifier columns."]
pub struct GooglePrivacyDlpV2DeltaPresenceEstimationQuasiIdValues {
    #[serde(rename = "estimatedProbability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The estimated probability that a given individual sharing these quasi-identifier values is in the dataset. This value, typically called δ, is the ratio between the number of records in the dataset with these quasi-identifier values, and the total number of individuals (inside *and* outside the dataset) with these quasi-identifier values. For example, if there are 15 individuals in the dataset who share the same quasi-identifier values, and an estimated 100 people in the entire population with these values, then δ is 0.15."]
    pub estimated_probability: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "quasiIdsValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quasi-identifier values."]
    pub quasi_ids_values:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Value>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Result of the δ-presence computation. Note that these results are an estimation, not exact values."]
pub struct GooglePrivacyDlpV2DeltaPresenceEstimationResult {
    #[serde(rename = "deltaPresenceEstimationHistogram")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The intervals [min_probability, max_probability) do not overlap. If a value doesn't correspond to any such interval, the associated frequency is zero. For example, the following records: {min_probability: 0, max_probability: 0.1, frequency: 17} {min_probability: 0.2, max_probability: 0.3, frequency: 42} {min_probability: 0.3, max_probability: 0.4, frequency: 99} mean that there are no record with an estimated probability in [0.1, 0.2) nor larger or equal to 0.4."]
    pub delta_presence_estimation_histogram: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GooglePrivacyDlpV2DeltaPresenceEstimationHistogramBucket>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deprecated; use `InspectionRuleSet` instead. Rule for modifying a `CustomInfoType` to alter behavior under certain circumstances, depending on the specific details of the rule. Not supported for the `surrogate_type` custom infoType."]
pub struct GooglePrivacyDlpV2DetectionRule {
    #[serde(rename = "hotwordRule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hotword-based detection rule."]
    pub hotword_rule: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2HotwordRule>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Custom information type based on a dictionary of words or phrases. This can be used to match sensitive information specific to the data, such as a list of employee IDs or job titles. Dictionary words are case-insensitive and all characters other than letters and digits in the unicode [Basic Multilingual Plane](https://en.wikipedia.org/wiki/Plane_%28Unicode%29#Basic_Multilingual_Plane) will be replaced with whitespace when scanning for matches, so the dictionary phrase \"Sam Johnson\" will match all three phrases \"sam johnson\", \"Sam, Johnson\", and \"Sam (Johnson)\". Additionally, the characters surrounding any match must be of a different type than the adjacent characters within the word, so letters must be next to non-letters and digits next to non-digits. For example, the dictionary word \"jen\" will match the first three letters of the text \"jen123\" but will return no matches for \"jennifer\". Dictionary words containing a large number of characters that are not letters or digits may result in unexpected findings because such characters are treated as whitespace. The [limits](https://cloud.google.com/dlp/limits) page contains details about the size limits of dictionaries. For dictionaries that do not fit within these constraints, consider using `LargeCustomDictionaryConfig` in the `StoredInfoType` API."]
pub struct GooglePrivacyDlpV2Dictionary {
    #[serde(rename = "cloudStoragePath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Newline-delimited file of words in Cloud Storage. Only a single file is accepted."]
    pub cloud_storage_path:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CloudStoragePath>>,
    #[serde(rename = "wordList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of words or phrases to search for."]
    pub word_list: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2WordList>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Combines all of the information about a DLP job."]
pub struct GooglePrivacyDlpV2DlpJob {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time when the job was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time when the job finished."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A stream of errors encountered running the job."]
    pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Error>>>,
    #[serde(rename = "inspectDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Results from inspecting a data source."]
    pub inspect_details:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectDataSourceDetails>>,
    #[serde(rename = "jobTriggerName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If created by a job trigger, the resource name of the trigger that instantiated the job."]
    pub job_trigger_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The server-assigned name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "riskDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Results from analyzing risk of a data source."]
    pub risk_details:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2AnalyzeDataSourceRiskDetails>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time when the job started."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "State of a job."]
    pub state: ::std::option::Option<GooglePrivacyDlpV2DlpJobStateEnum>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of job."]
    pub _type: ::std::option::Option<GooglePrivacyDlpV2DlpJobTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "State of a job."]
pub enum GooglePrivacyDlpV2DlpJobStateEnum {
    #[serde(rename = "JOB_STATE_UNSPECIFIED")]
    #[doc = "Unused."]
    JobStateUnspecified,
    #[serde(rename = "PENDING")]
    #[doc = "The job has not yet started."]
    Pending,
    #[serde(rename = "RUNNING")]
    #[doc = "The job is currently running. Once a job has finished it will transition to FAILED or DONE."]
    Running,
    #[serde(rename = "DONE")]
    #[doc = "The job is no longer running."]
    Done,
    #[serde(rename = "CANCELED")]
    #[doc = "The job was canceled before it could complete."]
    Canceled,
    #[serde(rename = "FAILED")]
    #[doc = "The job had an error and did not complete."]
    Failed,
    #[serde(rename = "ACTIVE")]
    #[doc = "The job is currently accepting findings via hybridInspect. A hybrid job in ACTIVE state may continue to have findings added to it through calling of hybridInspect. After the job has finished no more calls to hybridInspect may be made. ACTIVE jobs can transition to DONE."]
    Active,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of job."]
pub enum GooglePrivacyDlpV2DlpJobTypeEnum {
    #[serde(rename = "DLP_JOB_TYPE_UNSPECIFIED")]
    #[doc = "Unused"]
    DlpJobTypeUnspecified,
    #[serde(rename = "INSPECT_JOB")]
    #[doc = "The job inspected Google Cloud for sensitive data."]
    InspectJob,
    #[serde(rename = "RISK_ANALYSIS_JOB")]
    #[doc = "The job executed a Risk Analysis computation."]
    RiskAnalysisJob,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Location of a finding within a document."]
pub struct GooglePrivacyDlpV2DocumentLocation {
    #[serde(rename = "fileOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Offset of the line, from the beginning of the file, where the finding is located."]
    pub file_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An entity in a dataset is a field or set of fields that correspond to a single person. For example, in medical records the `EntityId` might be a patient identifier, or for financial records it might be an account identifier. This message is used when generalizations or analysis must take into account that multiple rows correspond to the same entity."]
pub struct GooglePrivacyDlpV2EntityId {
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Composite key indicating which field contains the entity identifier."]
    pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details information about an error encountered during job execution or the results of an unsuccessful activation of the JobTrigger."]
pub struct GooglePrivacyDlpV2Error {
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detailed error codes and messages."]
    pub details: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    #[serde(rename = "timestamps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The times the error occurred."]
    pub timestamps: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List of exclude infoTypes."]
pub struct GooglePrivacyDlpV2ExcludeInfoTypes {
    #[serde(rename = "infoTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "InfoType list in ExclusionRule rule drops a finding when it overlaps or contained within with a finding of an infoType from this list. For example, for `InspectionRuleSet.info_types` containing \"PHONE_NUMBER\"` and `exclusion_rule` containing `exclude_info_types.info_types` with \"EMAIL_ADDRESS\" the phone number findings are dropped if they overlap with EMAIL_ADDRESS finding. That leads to \"555-222-2222@example.org\" to generate only a single finding, namely email address."]
    pub info_types:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The rule that specifies conditions when findings of infoTypes specified in `InspectionRuleSet` are removed from results."]
pub struct GooglePrivacyDlpV2ExclusionRule {
    #[serde(rename = "dictionary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dictionary which defines the rule."]
    pub dictionary: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Dictionary>>,
    #[serde(rename = "excludeInfoTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set of infoTypes for which findings would affect this rule."]
    pub exclude_info_types:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ExcludeInfoTypes>>,
    #[serde(rename = "matchingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How the rule is applied, see MatchingType documentation for details."]
    pub matching_type: ::std::option::Option<GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum>,
    #[serde(rename = "regex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Regular expression which defines the rule."]
    pub regex: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Regex>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "How the rule is applied, see MatchingType documentation for details."]
pub enum GooglePrivacyDlpV2ExclusionRuleMatchingTypeEnum {
    #[serde(rename = "MATCHING_TYPE_UNSPECIFIED")]
    #[doc = "Invalid."]
    MatchingTypeUnspecified,
    #[serde(rename = "MATCHING_TYPE_FULL_MATCH")]
    #[doc = "Full match. - Dictionary: join of Dictionary results matched complete finding quote - Regex: all regex matches fill a finding quote start to end - Exclude info type: completely inside affecting info types findings"]
    MatchingTypeFullMatch,
    #[serde(rename = "MATCHING_TYPE_PARTIAL_MATCH")]
    #[doc = "Partial match. - Dictionary: at least one of the tokens in the finding matches - Regex: substring of the finding matches - Exclude info type: intersects with affecting info types findings"]
    MatchingTypePartialMatch,
    #[serde(rename = "MATCHING_TYPE_INVERSE_MATCH")]
    #[doc = "Inverse match. - Dictionary: no tokens in the finding match the dictionary - Regex: finding doesn't match the regex - Exclude info type: no intersection with affecting info types findings"]
    MatchingTypeInverseMatch,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An expression, consisting or an operator and conditions."]
pub struct GooglePrivacyDlpV2Expressions {
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Conditions to apply to the expression."]
    pub conditions: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Conditions>>,
    #[serde(rename = "logicalOperator")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The operator to apply to the result of conditions. Default and currently only supported value is `AND`."]
    pub logical_operator: ::std::option::Option<GooglePrivacyDlpV2ExpressionsLogicalOperatorEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The operator to apply to the result of conditions. Default and currently only supported value is `AND`."]
pub enum GooglePrivacyDlpV2ExpressionsLogicalOperatorEnum {
    #[serde(rename = "LOGICAL_OPERATOR_UNSPECIFIED")]
    #[doc = "Unused"]
    LogicalOperatorUnspecified,
    #[serde(rename = "AND")]
    #[doc = "Conditional AND"]
    And,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "General identifier of a data field in a storage service."]
pub struct GooglePrivacyDlpV2FieldId {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name describing the field."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The transformation to apply to the field."]
pub struct GooglePrivacyDlpV2FieldTransformation {
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only apply the transformation if the condition evaluates to true for the given `RecordCondition`. The conditions are allowed to reference fields that are not used in the actual transformation. Example Use Cases: - Apply a different bucket transformation to an age column if the zip code column for the same record is within a specific range. - Redact a field if the date of birth field is greater than 85."]
    pub condition: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RecordCondition>>,
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Input field(s) to apply the transformation to."]
    pub fields:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>>,
    #[serde(rename = "infoTypeTransformations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Treat the contents of the field as free text, and selectively transform content that matches an `InfoType`."]
    pub info_type_transformations:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoTypeTransformations>>,
    #[serde(rename = "primitiveTransformation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Apply the transformation to the entire field."]
    pub primitive_transformation:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PrimitiveTransformation>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Set of files to scan."]
pub struct GooglePrivacyDlpV2FileSet {
    #[serde(rename = "regexFileSet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The regex-filtered set of files to scan. Exactly one of `url` or `regex_file_set` must be set."]
    pub regex_file_set:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CloudStorageRegexFileSet>>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Cloud Storage url of the file(s) to scan, in the format `gs:///`. Trailing wildcard in the path is allowed. If the url ends in a trailing slash, the bucket or directory represented by the url will be scanned non-recursively (content in sub-directories will not be scanned). This means that `gs://mybucket/` is equivalent to `gs://mybucket/*`, and `gs://mybucket/directory/` is equivalent to `gs://mybucket/directory/*`. Exactly one of `url` or `regex_file_set` must be set."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a piece of potentially sensitive content."]
pub struct GooglePrivacyDlpV2Finding {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp when finding was detected."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "findingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique finding id."]
    pub finding_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "infoType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of content that might have been found. Provided if `excluded_types` is false."]
    pub info_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
    #[serde(rename = "jobCreateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time the job started that produced this finding."]
    pub job_create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "jobName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The job that stored the finding."]
    pub job_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The labels associated with this `Finding`. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?`. Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. No more than 10 labels can be associated with a given finding. Examples: * `\"environment\" : \"production\"` * `\"pipeline\" : \"etl\"`"]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "likelihood")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Confidence of how likely it is that the `info_type` is correct."]
    pub likelihood: ::std::option::Option<GooglePrivacyDlpV2FindingLikelihoodEnum>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Where the content was found."]
    pub location: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Location>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name in format projects/{project}/locations/{location}/findings/{finding} Populated only when viewing persisted findings."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quote")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content that was found. Even if the content is not textual, it may be converted to a textual representation here. Provided if `include_quote` is true and the finding is less than or equal to 4096 bytes long. If the finding exceeds 4096 bytes in length, the quote may be omitted."]
    pub quote: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quoteInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contains data parsed from quotes. Only populated if include_quote was set to true and a supported infoType was requested. Currently supported infoTypes: DATE, DATE_OF_BIRTH and TIME."]
    pub quote_info: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2QuoteInfo>>,
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The job that stored the finding."]
    pub resource_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "triggerName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Job trigger name, if applicable, for this finding."]
    pub trigger_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Confidence of how likely it is that the `info_type` is correct."]
pub enum GooglePrivacyDlpV2FindingLikelihoodEnum {
    #[serde(rename = "LIKELIHOOD_UNSPECIFIED")]
    #[doc = "Default value; same as POSSIBLE."]
    LikelihoodUnspecified,
    #[serde(rename = "VERY_UNLIKELY")]
    #[doc = "Few matching elements."]
    VeryUnlikely,
    #[serde(rename = "UNLIKELY")]
    #[doc = ""]
    Unlikely,
    #[serde(rename = "POSSIBLE")]
    #[doc = "Some matching elements."]
    Possible,
    #[serde(rename = "LIKELY")]
    #[doc = ""]
    Likely,
    #[serde(rename = "VERY_LIKELY")]
    #[doc = "Many matching elements."]
    VeryLikely,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration to control the number of findings returned. Cannot be set if de-identification is requested."]
pub struct GooglePrivacyDlpV2FindingLimits {
    #[serde(rename = "maxFindingsPerInfoType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration of findings limit given for specified infoTypes."]
    pub max_findings_per_info_type:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InfoTypeLimit>>>,
    #[serde(rename = "maxFindingsPerItem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Max number of findings that will be returned for each item scanned. When set within `InspectJobConfig`, the maximum returned is 2000 regardless if this is set higher. When set within `InspectContentRequest`, this field is ignored."]
    pub max_findings_per_item: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "maxFindingsPerRequest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Max number of findings that will be returned per request/job. When set within `InspectContentRequest`, the maximum returned is 2000 regardless if this is set higher."]
    pub max_findings_per_request: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for finishing a DLP hybrid job."]
pub struct GooglePrivacyDlpV2FinishDlpJobRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Buckets values based on fixed size ranges. The Bucketing transformation can provide all of this functionality, but requires more configuration. This message is provided as a convenience to the user for simple bucketing strategies. The transformed value will be a hyphenated string of {lower_bound}-{upper_bound}, i.e if lower_bound = 10 and upper_bound = 20 all values that are within this bucket will be replaced with \"10-20\". This can be used on data of type: double, long. If the bound Value type differs from the type of data being transformed, we will first attempt converting the type of the data to be transformed to match the type of the bound before comparing. See https://cloud.google.com/dlp/docs/concepts-bucketing to learn more."]
pub struct GooglePrivacyDlpV2FixedSizeBucketingConfig {
    #[serde(rename = "bucketSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Size of each bucket (except for minimum and maximum buckets). So if `lower_bound` = 10, `upper_bound` = 89, and `bucket_size` = 10, then the following buckets would be used: -10, 10-20, 20-30, 30-40, 40-50, 50-60, 60-70, 70-80, 80-89, 89+. Precision up to 2 decimals works."]
    pub bucket_size: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "lowerBound")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Lower bound value of buckets. All values less than `lower_bound` are grouped together into a single bucket; for example if `lower_bound` = 10, then all values less than 10 are replaced with the value \"-10\"."]
    pub lower_bound: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
    #[serde(rename = "upperBound")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Upper bound value of buckets. All values greater than upper_bound are grouped together into a single bucket; for example if `upper_bound` = 89, then all values greater than 89 are replaced with the value \"89+\"."]
    pub upper_bound: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The rule that adjusts the likelihood of findings within a certain proximity of hotwords."]
pub struct GooglePrivacyDlpV2HotwordRule {
    #[serde(rename = "hotwordRegex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Regular expression pattern defining what qualifies as a hotword."]
    pub hotword_regex: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Regex>>,
    #[serde(rename = "likelihoodAdjustment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Likelihood adjustment to apply to all matching findings."]
    pub likelihood_adjustment:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2LikelihoodAdjustment>>,
    #[serde(rename = "proximity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Proximity of the finding within which the entire hotword must reside. The total length of the window cannot exceed 1000 characters. Note that the finding itself will be included in the window, so that hotwords may be used to match substrings of the finding itself. For example, the certainty of a phone number regex \"\\(\\d{3}\\) \\d{3}-\\d{4}\" could be adjusted upwards if the area code is known to be the local area code of a company office using the hotword regex \"\\(xxx\\)\", where \"xxx\" is the area code in question."]
    pub proximity: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Proximity>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An individual hybrid item to inspect. Will be stored temporarily during processing."]
pub struct GooglePrivacyDlpV2HybridContentItem {
    #[serde(rename = "findingDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Supplementary information that will be added to each finding."]
    pub finding_details:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2HybridFindingDetails>>,
    #[serde(rename = "item")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The item to inspect."]
    pub item: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ContentItem>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Populate to associate additional data with each finding."]
pub struct GooglePrivacyDlpV2HybridFindingDetails {
    #[serde(rename = "containerDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details about the container where the content being inspected is from."]
    pub container_details: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Container>>,
    #[serde(rename = "fileOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Offset in bytes of the line, from the beginning of the file, where the finding is located. Populate if the item being scanned is only part of a bigger item, such as a shard of a file and you want to track the absolute position of the finding."]
    pub file_offset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Labels to represent user provided metadata about the data being inspected. If configured by the job, some key values may be required. The labels associated with `Finding`'s produced by hybrid inspection. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?`. Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. No more than 10 labels can be associated with a given finding. Examples: * `\"environment\" : \"production\"` * `\"pipeline\" : \"etl\"`"]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "rowOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Offset of the row for tables. Populate if the row(s) being scanned are part of a bigger dataset and you want to keep track of their absolute position."]
    pub row_offset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tableOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the container is a table, additional information to make findings meaningful such as the columns that are primary keys. If not known ahead of time, can also be set within each inspect hybrid call and the two will be merged. Note that identifying_fields will only be stored to BigQuery, and only if the BigQuery action has been included."]
    pub table_options: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TableOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to search for potentially sensitive info in a custom location."]
pub struct GooglePrivacyDlpV2HybridInspectDlpJobRequest {
    #[serde(rename = "hybridItem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The item to inspect."]
    pub hybrid_item: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2HybridContentItem>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to search for potentially sensitive info in a custom location."]
pub struct GooglePrivacyDlpV2HybridInspectJobTriggerRequest {
    #[serde(rename = "hybridItem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The item to inspect."]
    pub hybrid_item: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2HybridContentItem>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Quota exceeded errors will be thrown once quota has been met."]
pub struct GooglePrivacyDlpV2HybridInspectResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Statistics related to processing hybrid inspect requests."]
pub struct GooglePrivacyDlpV2HybridInspectStatistics {
    #[serde(rename = "abortedCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of hybrid inspection requests aborted because the job ran out of quota or was ended before they could be processed."]
    pub aborted_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pendingCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of hybrid requests currently being processed. Only populated when called via method `getDlpJob`. A burst of traffic may cause hybrid inspect requests to be enqueued. Processing will take place as quickly as possible, but resource limitations may impact how long a request is enqueued for."]
    pub pending_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "processedCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of hybrid inspection requests processed within this job."]
    pub processed_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration to control jobs where the content being inspected is outside of Google Cloud Platform."]
pub struct GooglePrivacyDlpV2HybridOptions {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short description of where the data is coming from. Will be stored once in the job. 256 max length."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "To organize findings, these labels will be added to each finding. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?`. Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. No more than 10 labels can be associated with a given finding. Examples: * `\"environment\" : \"production\"` * `\"pipeline\" : \"etl\"`"]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "requiredFindingLabelKeys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "These are labels that each inspection request must include within their 'finding_labels' map. Request may contain others, but any missing one of these will be rejected. Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?`. No more than 10 keys can be required."]
    pub required_finding_label_keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "tableOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the container is a table, additional information to make findings meaningful such as the columns that are primary keys."]
    pub table_options: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TableOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Location of the finding within an image."]
pub struct GooglePrivacyDlpV2ImageLocation {
    #[serde(rename = "boundingBoxes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Bounding boxes locating the pixels within the image containing the finding."]
    pub bounding_boxes:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2BoundingBox>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for determining how redaction of images should occur."]
pub struct GooglePrivacyDlpV2ImageRedactionConfig {
    #[serde(rename = "infoType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only one per info_type should be provided per request. If not specified, and redact_all_text is false, the DLP API will redact all text that it matches against all info_types that are found, but not specified in another ImageRedactionConfig."]
    pub info_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
    #[serde(rename = "redactAllText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, all text found in the image, regardless whether it matches an info_type, is redacted. Only one should be provided."]
    pub redact_all_text: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "redactionColor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The color to use when redacting content from an image. If not specified, the default is black."]
    pub redaction_color: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Color>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Type of information detected by the API."]
pub struct GooglePrivacyDlpV2InfoType {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed at https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type. When sending Cloud DLP results to Data Catalog, infoType names should conform to the pattern `[A-Za-z0-9$-_]{1,64}`."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "InfoType description."]
pub struct GooglePrivacyDlpV2InfoTypeDescription {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the infotype. Translated when language is provided in the request."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human readable form of the infoType name."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Internal name of the infoType."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "supportedBy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Which parts of the API supports this InfoType."]
    pub supported_by: ::std::option::Option<
        ::std::vec::Vec<GooglePrivacyDlpV2InfoTypeDescriptionSupportedByEnum>,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GooglePrivacyDlpV2InfoTypeDescriptionSupportedByEnum {
    #[serde(rename = "ENUM_TYPE_UNSPECIFIED")]
    #[doc = "Unused."]
    EnumTypeUnspecified,
    #[serde(rename = "INSPECT")]
    #[doc = "Supported by the inspect operations."]
    Inspect,
    #[serde(rename = "RISK_ANALYSIS")]
    #[doc = "Supported by the risk analysis operations."]
    RiskAnalysis,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Max findings configuration per infoType, per content item or long running DlpJob."]
pub struct GooglePrivacyDlpV2InfoTypeLimit {
    #[serde(rename = "infoType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of information the findings limit applies to. Only one limit per info_type should be provided. If InfoTypeLimit does not have an info_type, the DLP API applies the limit against all info_types that are found but not specified in another InfoTypeLimit."]
    pub info_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
    #[serde(rename = "maxFindings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Max findings limit for the given infoType."]
    pub max_findings: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Statistics regarding a specific InfoType."]
pub struct GooglePrivacyDlpV2InfoTypeStats {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of findings for this infoType."]
    pub count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "infoType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of finding this stat is for."]
    pub info_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A transformation to apply to text that is identified as a specific info_type."]
pub struct GooglePrivacyDlpV2InfoTypeTransformation {
    #[serde(rename = "infoTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "InfoTypes to apply the transformation to. An empty list will cause this transformation to apply to all findings that correspond to infoTypes that were requested in `InspectConfig`."]
    pub info_types:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>>,
    #[serde(rename = "primitiveTransformation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Primitive transformation to apply to the infoType."]
    pub primitive_transformation:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PrimitiveTransformation>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A type of transformation that will scan unstructured text and apply various `PrimitiveTransformation`s to each finding, where the transformation is applied to only values that were identified as a specific info_type."]
pub struct GooglePrivacyDlpV2InfoTypeTransformations {
    #[serde(rename = "transformations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Transformation for each infoType. Cannot specify more than one for a given infoType."]
    pub transformations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InfoTypeTransformation>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration description of the scanning process. When used with redactContent only info_types and min_likelihood are currently used."]
pub struct GooglePrivacyDlpV2InspectConfig {
    #[serde(rename = "contentOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of options defining data content to scan. If empty, text, images, and other content will be included."]
    pub content_options:
        ::std::option::Option<::std::vec::Vec<GooglePrivacyDlpV2InspectConfigContentOptionsEnum>>,
    #[serde(rename = "customInfoTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "CustomInfoTypes provided by the user. See https://cloud.google.com/dlp/docs/creating-custom-infotypes to learn more."]
    pub custom_info_types:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2CustomInfoType>>>,
    #[serde(rename = "excludeInfoTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When true, excludes type information of the findings."]
    pub exclude_info_types: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "includeQuote")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When true, a contextual quote from the data that triggered a finding is included in the response; see Finding.quote."]
    pub include_quote: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "infoTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts what info_types to look for. The values must correspond to InfoType values returned by ListInfoTypes or listed at https://cloud.google.com/dlp/docs/infotypes-reference. When no InfoTypes or CustomInfoTypes are specified in a request, the system may automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated. If you need precise control and predictability as to what detectors are run you should specify specific InfoTypes listed in the reference, otherwise a default list will be used, which may change over time."]
    pub info_types:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>>,
    #[serde(rename = "limits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration to control the number of findings returned."]
    pub limits: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FindingLimits>>,
    #[serde(rename = "minLikelihood")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only returns findings equal or above this threshold. The default is POSSIBLE. See https://cloud.google.com/dlp/docs/likelihood to learn more."]
    pub min_likelihood: ::std::option::Option<GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum>,
    #[serde(rename = "ruleSet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set of rules to apply to the findings for this InspectConfig. Exclusion rules, contained in the set are executed in the end, other rules are executed in the order they are specified for each info type."]
    pub rule_set: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InspectionRuleSet>>,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GooglePrivacyDlpV2InspectConfigContentOptionsEnum {
    #[serde(rename = "CONTENT_UNSPECIFIED")]
    #[doc = "Includes entire content of a file or a data stream."]
    ContentUnspecified,
    #[serde(rename = "CONTENT_TEXT")]
    #[doc = "Text content within the data, excluding any metadata."]
    ContentText,
    #[serde(rename = "CONTENT_IMAGE")]
    #[doc = "Images found in the data."]
    ContentImage,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Only returns findings equal or above this threshold. The default is POSSIBLE. See https://cloud.google.com/dlp/docs/likelihood to learn more."]
pub enum GooglePrivacyDlpV2InspectConfigMinLikelihoodEnum {
    #[serde(rename = "LIKELIHOOD_UNSPECIFIED")]
    #[doc = "Default value; same as POSSIBLE."]
    LikelihoodUnspecified,
    #[serde(rename = "VERY_UNLIKELY")]
    #[doc = "Few matching elements."]
    VeryUnlikely,
    #[serde(rename = "UNLIKELY")]
    #[doc = ""]
    Unlikely,
    #[serde(rename = "POSSIBLE")]
    #[doc = "Some matching elements."]
    Possible,
    #[serde(rename = "LIKELY")]
    #[doc = ""]
    Likely,
    #[serde(rename = "VERY_LIKELY")]
    #[doc = "Many matching elements."]
    VeryLikely,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to search for potentially sensitive info in a ContentItem."]
pub struct GooglePrivacyDlpV2InspectContentRequest {
    #[serde(rename = "inspectConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for the inspector. What specified here will override the template referenced by the inspect_template_name argument."]
    pub inspect_config: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectConfig>>,
    #[serde(rename = "inspectTemplateName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Template to use. Any configuration directly specified in inspect_config will override those set in the template. Singular fields that are set in this request will replace their corresponding fields in the template. Repeated fields are appended. Singular sub-messages and groups are recursively merged."]
    pub inspect_template_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "item")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The item to inspect."]
    pub item: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ContentItem>>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. This field has no effect."]
    pub location_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Results of inspecting an item."]
pub struct GooglePrivacyDlpV2InspectContentResponse {
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The findings."]
    pub result: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectResult>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The results of an inspect DataSource job."]
pub struct GooglePrivacyDlpV2InspectDataSourceDetails {
    #[serde(rename = "requestedOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration used for this job."]
    pub requested_options:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RequestedOptions>>,
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A summary of the outcome of this inspection job."]
    pub result: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Result>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Controls what and how to inspect for findings."]
pub struct GooglePrivacyDlpV2InspectJobConfig {
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Actions to execute at the completion of the job."]
    pub actions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Action>>>,
    #[serde(rename = "inspectConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How and what to scan for."]
    pub inspect_config: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectConfig>>,
    #[serde(rename = "inspectTemplateName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If provided, will be used as the default for all values in InspectConfig. `inspect_config` will be merged into the values persisted as part of the template."]
    pub inspect_template_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storageConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The data to scan."]
    pub storage_config: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2StorageConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "All the findings for a single scanned item."]
pub struct GooglePrivacyDlpV2InspectResult {
    #[serde(rename = "findings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of findings for an item."]
    pub findings:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Finding>>>,
    #[serde(rename = "findingsTruncated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, then this item might have more findings than were returned, and the findings returned are an arbitrary subset of all findings. The findings list might be truncated because the input items were too large, or because the server reached the maximum amount of resources allowed for a single API call. For best results, divide the input into smaller batches."]
    pub findings_truncated: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The inspectTemplate contains a configuration (set of types of sensitive data to be detected) to be used anywhere you otherwise would normally specify InspectConfig. See https://cloud.google.com/dlp/docs/concepts-templates to learn more."]
pub struct GooglePrivacyDlpV2InspectTemplate {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The creation timestamp of an inspectTemplate."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Short description (max 256 chars)."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display name (max 256 chars)."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inspectConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The core content of the template. Configuration of the scanning process."]
    pub inspect_config: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectConfig>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The template name. The template will have one of the following formats: `projects/PROJECT_ID/inspectTemplates/TEMPLATE_ID` OR `organizations/ORGANIZATION_ID/inspectTemplates/TEMPLATE_ID`;"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The last update timestamp of an inspectTemplate."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single inspection rule to be applied to infoTypes, specified in `InspectionRuleSet`."]
pub struct GooglePrivacyDlpV2InspectionRule {
    #[serde(rename = "exclusionRule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Exclusion rule."]
    pub exclusion_rule: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ExclusionRule>>,
    #[serde(rename = "hotwordRule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hotword-based detection rule."]
    pub hotword_rule: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2HotwordRule>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Rule set for modifying a set of infoTypes to alter behavior under certain circumstances, depending on the specific details of the rules within the set."]
pub struct GooglePrivacyDlpV2InspectionRuleSet {
    #[serde(rename = "infoTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of infoTypes this rule set is applied to."]
    pub info_types:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>>,
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set of rules to be applied to infoTypes. The rules are applied in order."]
    pub rules:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InspectionRule>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Enable email notification to project owners and editors on jobs's completion/failure."]
pub struct GooglePrivacyDlpV2JobNotificationEmails {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains a configuration to make dlp api calls on a repeating basis. See https://cloud.google.com/dlp/docs/concepts-job-triggers to learn more."]
pub struct GooglePrivacyDlpV2JobTrigger {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The creation timestamp of a triggeredJob."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User provided description (max 256 chars)"]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display name (max 100 chars)"]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A stream of errors encountered when the trigger was activated. Repeated errors may result in the JobTrigger automatically being paused. Will return the last 100 errors. Whenever the JobTrigger is modified this list will be cleared."]
    pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Error>>>,
    #[serde(rename = "inspectJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For inspect jobs, a snapshot of the configuration."]
    pub inspect_job: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectJobConfig>>,
    #[serde(rename = "lastRunTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The timestamp of the last time this trigger executed."]
    pub last_run_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique resource name for the triggeredJob, assigned by the service when the triggeredJob is created, for example `projects/dlp-test-project/jobTriggers/53234423`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A status for this trigger."]
    pub status: ::std::option::Option<GooglePrivacyDlpV2JobTriggerStatusEnum>,
    #[serde(rename = "triggers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of triggers which will be OR'ed together. Only one in the list needs to trigger for a job to be started. The list may contain only a single Schedule trigger and must have at least one object."]
    pub triggers:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Trigger>>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The last update timestamp of a triggeredJob."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. A status for this trigger."]
pub enum GooglePrivacyDlpV2JobTriggerStatusEnum {
    #[serde(rename = "STATUS_UNSPECIFIED")]
    #[doc = "Unused."]
    StatusUnspecified,
    #[serde(rename = "HEALTHY")]
    #[doc = "Trigger is healthy."]
    Healthy,
    #[serde(rename = "PAUSED")]
    #[doc = "Trigger is temporarily paused."]
    Paused,
    #[serde(rename = "CANCELLED")]
    #[doc = "Trigger is cancelled and can not be resumed."]
    Cancelled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "k-anonymity metric, used for analysis of reidentification risk."]
pub struct GooglePrivacyDlpV2KAnonymityConfig {
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Message indicating that multiple rows might be associated to a single individual. If the same entity_id is associated to multiple quasi-identifier tuples over distinct rows, we consider the entire collection of tuples as the composite quasi-identifier. This collection is a multiset: the order in which the different tuples appear in the dataset is ignored, but their frequency is taken into account. Important note: a maximum of 1000 rows can be associated to a single entity ID. If more rows are associated with the same entity ID, some might be ignored."]
    pub entity_id: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2EntityId>>,
    #[serde(rename = "quasiIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set of fields to compute k-anonymity over. When multiple fields are specified, they are considered a single composite key. Structs and repeated data types are not supported; however, nested fields are supported so long as they are not structs themselves or nested within a repeated field."]
    pub quasi_ids:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The set of columns' values that share the same ldiversity value"]
pub struct GooglePrivacyDlpV2KAnonymityEquivalenceClass {
    #[serde(rename = "equivalenceClassSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size of the equivalence class, for example number of rows with the above set of values."]
    pub equivalence_class_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quasiIdsValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set of values defining the equivalence class. One value per quasi-identifier column in the original KAnonymity metric message. The order is always the same as the original request."]
    pub quasi_ids_values:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Value>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Histogram of k-anonymity equivalence classes."]
pub struct GooglePrivacyDlpV2KAnonymityHistogramBucket {
    #[serde(rename = "bucketSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of equivalence classes in this bucket."]
    pub bucket_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bucketValueCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of distinct equivalence classes in this bucket."]
    pub bucket_value_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bucketValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sample of equivalence classes in this bucket. The total number of classes returned per bucket is capped at 20."]
    pub bucket_values: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2KAnonymityEquivalenceClass>>,
    >,
    #[serde(rename = "equivalenceClassSizeLowerBound")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Lower bound on the size of the equivalence classes in this bucket."]
    pub equivalence_class_size_lower_bound: ::std::option::Option<::std::string::String>,
    #[serde(rename = "equivalenceClassSizeUpperBound")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Upper bound on the size of the equivalence classes in this bucket."]
    pub equivalence_class_size_upper_bound: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Result of the k-anonymity computation."]
pub struct GooglePrivacyDlpV2KAnonymityResult {
    #[serde(rename = "equivalenceClassHistogramBuckets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Histogram of k-anonymity equivalence classes."]
    pub equivalence_class_histogram_buckets: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2KAnonymityHistogramBucket>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Reidentifiability metric. This corresponds to a risk model similar to what is called \"journalist risk\" in the literature, except the attack dataset is statistically modeled instead of being perfectly known. This can be done using publicly available data (like the US Census), or using a custom statistical model (indicated as one or several BigQuery tables), or by extrapolating from the distribution of values in the input dataset."]
pub struct GooglePrivacyDlpV2KMapEstimationConfig {
    #[serde(rename = "auxiliaryTables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Several auxiliary tables can be used in the analysis. Each custom_tag used to tag a quasi-identifiers column must appear in exactly one column of one auxiliary table."]
    pub auxiliary_tables:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2AuxiliaryTable>>>,
    #[serde(rename = "quasiIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Fields considered to be quasi-identifiers. No two columns can have the same tag."]
    pub quasi_ids:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2TaggedField>>>,
    #[serde(rename = "regionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ISO 3166-1 alpha-2 region code to use in the statistical modeling. Set if no column is tagged with a region-specific InfoType (like US_ZIP_5) or a region code."]
    pub region_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A KMapEstimationHistogramBucket message with the following values: min_anonymity: 3 max_anonymity: 5 frequency: 42 means that there are 42 records whose quasi-identifier values correspond to 3, 4 or 5 people in the overlying population. An important particular case is when min_anonymity = max_anonymity = 1: the frequency field then corresponds to the number of uniquely identifiable records."]
pub struct GooglePrivacyDlpV2KMapEstimationHistogramBucket {
    #[serde(rename = "bucketSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of records within these anonymity bounds."]
    pub bucket_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bucketValueCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of distinct quasi-identifier tuple values in this bucket."]
    pub bucket_value_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bucketValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sample of quasi-identifier tuple values in this bucket. The total number of classes returned per bucket is capped at 20."]
    pub bucket_values: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2KMapEstimationQuasiIdValues>>,
    >,
    #[serde(rename = "maxAnonymity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always greater than or equal to min_anonymity."]
    pub max_anonymity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minAnonymity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Always positive."]
    pub min_anonymity: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A tuple of values for the quasi-identifier columns."]
pub struct GooglePrivacyDlpV2KMapEstimationQuasiIdValues {
    #[serde(rename = "estimatedAnonymity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The estimated anonymity for these quasi-identifier values."]
    pub estimated_anonymity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quasiIdsValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quasi-identifier values."]
    pub quasi_ids_values:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Value>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Result of the reidentifiability analysis. Note that these results are an estimation, not exact values."]
pub struct GooglePrivacyDlpV2KMapEstimationResult {
    #[serde(rename = "kMapEstimationHistogram")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The intervals [min_anonymity, max_anonymity] do not overlap. If a value doesn't correspond to any such interval, the associated frequency is zero. For example, the following records: {min_anonymity: 1, max_anonymity: 1, frequency: 17} {min_anonymity: 2, max_anonymity: 3, frequency: 42} {min_anonymity: 5, max_anonymity: 10, frequency: 99} mean that there are no record with an estimated anonymity of 4, 5, or larger than 10."]
    pub k_map_estimation_histogram: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2KMapEstimationHistogramBucket>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A unique identifier for a Datastore entity. If a key's partition ID or any of its path kinds or names are reserved/read-only, the key is reserved/read-only. A reserved/read-only key is forbidden in certain documented contexts."]
pub struct GooglePrivacyDlpV2Key {
    #[serde(rename = "partitionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entities are partitioned into subsets, currently identified by a project ID and namespace ID. Queries are scoped to a single partition."]
    pub partition_id: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PartitionId>>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The entity path. An entity path consists of one or more elements composed of a kind and a string or numerical identifier, which identify entities. The first element identifies a _root entity_, the second element identifies a _child_ of the root entity, the third element identifies a child of the second entity, and so forth. The entities identified by all prefixes of the path are called the element's _ancestors_. A path can never be empty, and a path can have at most 100 elements."]
    pub path:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2PathElement>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A representation of a Datastore kind."]
pub struct GooglePrivacyDlpV2KindExpression {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the kind."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Include to use an existing data crypto key wrapped by KMS. The wrapped key must be a 128/192/256 bit key. Authorization requires the following IAM permissions when sending a request to perform a crypto transformation using a kms-wrapped crypto key: dlp.kms.encrypt"]
pub struct GooglePrivacyDlpV2KmsWrappedCryptoKey {
    #[serde(rename = "cryptoKeyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The resource name of the KMS CryptoKey to use for unwrapping."]
    pub crypto_key_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "wrappedKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The wrapped data crypto key."]
    pub wrapped_key: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "l-diversity metric, used for analysis of reidentification risk."]
pub struct GooglePrivacyDlpV2LDiversityConfig {
    #[serde(rename = "quasiIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set of quasi-identifiers indicating how equivalence classes are defined for the l-diversity computation. When multiple fields are specified, they are considered a single composite key."]
    pub quasi_ids:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>>,
    #[serde(rename = "sensitiveAttribute")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sensitive field for computing the l-value."]
    pub sensitive_attribute: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The set of columns' values that share the same ldiversity value."]
pub struct GooglePrivacyDlpV2LDiversityEquivalenceClass {
    #[serde(rename = "equivalenceClassSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size of the k-anonymity equivalence class."]
    pub equivalence_class_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numDistinctSensitiveValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of distinct sensitive values in this equivalence class."]
    pub num_distinct_sensitive_values: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quasiIdsValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Quasi-identifier values defining the k-anonymity equivalence class. The order is always the same as the original request."]
    pub quasi_ids_values:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Value>>>,
    #[serde(rename = "topSensitiveValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Estimated frequencies of top sensitive values."]
    pub top_sensitive_values:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2ValueFrequency>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Histogram of l-diversity equivalence class sensitive value frequencies."]
pub struct GooglePrivacyDlpV2LDiversityHistogramBucket {
    #[serde(rename = "bucketSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of equivalence classes in this bucket."]
    pub bucket_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bucketValueCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of distinct equivalence classes in this bucket."]
    pub bucket_value_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bucketValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sample of equivalence classes in this bucket. The total number of classes returned per bucket is capped at 20."]
    pub bucket_values: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2LDiversityEquivalenceClass>>,
    >,
    #[serde(rename = "sensitiveValueFrequencyLowerBound")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Lower bound on the sensitive value frequencies of the equivalence classes in this bucket."]
    pub sensitive_value_frequency_lower_bound: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sensitiveValueFrequencyUpperBound")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Upper bound on the sensitive value frequencies of the equivalence classes in this bucket."]
    pub sensitive_value_frequency_upper_bound: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Result of the l-diversity computation."]
pub struct GooglePrivacyDlpV2LDiversityResult {
    #[serde(rename = "sensitiveValueFrequencyHistogramBuckets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Histogram of l-diversity equivalence class sensitive value frequencies."]
    pub sensitive_value_frequency_histogram_buckets: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2LDiversityHistogramBucket>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for a custom dictionary created from a data source of any size up to the maximum size defined in the [limits](https://cloud.google.com/dlp/limits) page. The artifacts of dictionary creation are stored in the specified Google Cloud Storage location. Consider using `CustomInfoType.Dictionary` for smaller dictionaries that satisfy the size requirements."]
pub struct GooglePrivacyDlpV2LargeCustomDictionaryConfig {
    #[serde(rename = "bigQueryField")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Field in a BigQuery table where each cell represents a dictionary phrase."]
    pub big_query_field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryField>>,
    #[serde(rename = "cloudStorageFileSet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set of files containing newline-delimited lists of dictionary phrases."]
    pub cloud_storage_file_set:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CloudStorageFileSet>>,
    #[serde(rename = "outputPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location to store dictionary artifacts in Google Cloud Storage. These files will only be accessible by project owners and the DLP API. If any of these artifacts are modified, the dictionary is considered invalid and can no longer be used."]
    pub output_path: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CloudStoragePath>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Summary statistics of a custom dictionary."]
pub struct GooglePrivacyDlpV2LargeCustomDictionaryStats {
    #[serde(rename = "approxNumPhrases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Approximate number of distinct phrases in the dictionary."]
    pub approx_num_phrases: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Skips the data without modifying it if the requested transformation would cause an error. For example, if a `DateShift` transformation were applied an an IP address, this mode would leave the IP address unchanged in the response."]
pub struct GooglePrivacyDlpV2LeaveUntransformed {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message for specifying an adjustment to the likelihood of a finding as part of a detection rule."]
pub struct GooglePrivacyDlpV2LikelihoodAdjustment {
    #[serde(rename = "fixedLikelihood")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set the likelihood of a finding to a fixed value."]
    pub fixed_likelihood:
        ::std::option::Option<GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum>,
    #[serde(rename = "relativeLikelihood")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Increase or decrease the likelihood by the specified number of levels. For example, if a finding would be `POSSIBLE` without the detection rule and `relative_likelihood` is 1, then it is upgraded to `LIKELY`, while a value of -1 would downgrade it to `UNLIKELY`. Likelihood may never drop below `VERY_UNLIKELY` or exceed `VERY_LIKELY`, so applying an adjustment of 1 followed by an adjustment of -1 when base likelihood is `VERY_LIKELY` will result in a final likelihood of `LIKELY`."]
    pub relative_likelihood: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Set the likelihood of a finding to a fixed value."]
pub enum GooglePrivacyDlpV2LikelihoodAdjustmentFixedLikelihoodEnum {
    #[serde(rename = "LIKELIHOOD_UNSPECIFIED")]
    #[doc = "Default value; same as POSSIBLE."]
    LikelihoodUnspecified,
    #[serde(rename = "VERY_UNLIKELY")]
    #[doc = "Few matching elements."]
    VeryUnlikely,
    #[serde(rename = "UNLIKELY")]
    #[doc = ""]
    Unlikely,
    #[serde(rename = "POSSIBLE")]
    #[doc = "Some matching elements."]
    Possible,
    #[serde(rename = "LIKELY")]
    #[doc = ""]
    Likely,
    #[serde(rename = "VERY_LIKELY")]
    #[doc = "Many matching elements."]
    VeryLikely,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListDeidentifyTemplates."]
pub struct GooglePrivacyDlpV2ListDeidentifyTemplatesResponse {
    #[serde(rename = "deidentifyTemplates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of deidentify templates, up to page_size in ListDeidentifyTemplatesRequest."]
    pub deidentify_templates: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2DeidentifyTemplate>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the next page is available then the next page token to be used in following ListDeidentifyTemplates request."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for listing DLP jobs."]
pub struct GooglePrivacyDlpV2ListDlpJobsResponse {
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of DlpJobs that matches the specified filter in the request."]
    pub jobs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2DlpJob>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The standard List next-page token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response to the ListInfoTypes request."]
pub struct GooglePrivacyDlpV2ListInfoTypesResponse {
    #[serde(rename = "infoTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set of sensitive infoTypes."]
    pub info_types: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InfoTypeDescription>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListInspectTemplates."]
pub struct GooglePrivacyDlpV2ListInspectTemplatesResponse {
    #[serde(rename = "inspectTemplates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of inspectTemplates, up to page_size in ListInspectTemplatesRequest."]
    pub inspect_templates: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InspectTemplate>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the next page is available then the next page token to be used in following ListInspectTemplates request."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListJobTriggers."]
pub struct GooglePrivacyDlpV2ListJobTriggersResponse {
    #[serde(rename = "jobTriggers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of triggeredJobs, up to page_size in ListJobTriggersRequest."]
    pub job_triggers:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2JobTrigger>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the next page is available then the next page token to be used in following ListJobTriggers request."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListStoredInfoTypes."]
pub struct GooglePrivacyDlpV2ListStoredInfoTypesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the next page is available then the next page token to be used in following ListStoredInfoTypes request."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storedInfoTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of storedInfoTypes, up to page_size in ListStoredInfoTypesRequest."]
    pub stored_info_types:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2StoredInfoType>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the location of the finding."]
pub struct GooglePrivacyDlpV2Location {
    #[serde(rename = "byteRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Zero-based byte offsets delimiting the finding. These are relative to the finding's containing element. Note that when the content is not textual, this references the UTF-8 encoded textual representation of the content. Omitted if content is an image."]
    pub byte_range: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Range>>,
    #[serde(rename = "codepointRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unicode character offsets delimiting the finding. These are relative to the finding's containing element. Provided when the content is text."]
    pub codepoint_range: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Range>>,
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the container where this finding occurred, if available."]
    pub container: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Container>>,
    #[serde(rename = "contentLocations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of nested objects pointing to the precise location of the finding within the file or record."]
    pub content_locations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2ContentLocation>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Job trigger option for hybrid jobs. Jobs must be manually created and finished."]
pub struct GooglePrivacyDlpV2Manual {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata Location"]
pub struct GooglePrivacyDlpV2MetadataLocation {
    #[serde(rename = "storageLabel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Storage metadata."]
    pub storage_label:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2StorageMetadataLabel>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of metadata containing the finding."]
    pub _type: ::std::option::Option<GooglePrivacyDlpV2MetadataLocationTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of metadata containing the finding."]
pub enum GooglePrivacyDlpV2MetadataLocationTypeEnum {
    #[serde(rename = "METADATATYPE_UNSPECIFIED")]
    #[doc = "Unused"]
    MetadatatypeUnspecified,
    #[serde(rename = "STORAGE_METADATA")]
    #[doc = "General file metadata provided by Cloud Storage."]
    StorageMetadata,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Compute numerical stats over an individual column, including min, max, and quantiles."]
pub struct GooglePrivacyDlpV2NumericalStatsConfig {
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Field to compute numerical stats on. Supported types are integer, float, date, datetime, timestamp, time."]
    pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Result of the numerical stats computation."]
pub struct GooglePrivacyDlpV2NumericalStatsResult {
    #[serde(rename = "maxValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum value appearing in the column."]
    pub max_value: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
    #[serde(rename = "minValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minimum value appearing in the column."]
    pub min_value: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
    #[serde(rename = "quantileValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of 99 values that partition the set of field values into 100 equal sized buckets."]
    pub quantile_values:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Value>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud repository for storing output."]
pub struct GooglePrivacyDlpV2OutputStorageConfig {
    #[serde(rename = "outputSchema")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Schema used for writing the findings for Inspect jobs. This field is only used for Inspect and must be unspecified for Risk jobs. Columns are derived from the `Finding` object. If appending to an existing table, any columns from the predefined schema that are missing will be added. No columns in the existing table will be deleted. If unspecified, then all available columns will be used for a new table or an (existing) table with no schema, and no changes will be made to an existing table that has a schema. Only for use with external storage."]
    pub output_schema: ::std::option::Option<GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum>,
    #[serde(rename = "table")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Store findings in an existing table or a new table in an existing dataset. If table_id is not set a new one will be generated for you with the following format: dlp_googleapis_yyyy_mm_dd_[dlp_job_id]. Pacific timezone will be used for generating the date details. For Inspect, each column in an existing output table must have the same name, type, and mode of a field in the `Finding` object. For Risk, an existing output table should be the output of a previous Risk analysis job run on the same source table, with the same privacy metric and quasi-identifiers. Risk jobs that analyze the same table but compute a different privacy metric, or use different sets of quasi-identifiers, cannot store their results in the same table."]
    pub table: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryTable>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Schema used for writing the findings for Inspect jobs. This field is only used for Inspect and must be unspecified for Risk jobs. Columns are derived from the `Finding` object. If appending to an existing table, any columns from the predefined schema that are missing will be added. No columns in the existing table will be deleted. If unspecified, then all available columns will be used for a new table or an (existing) table with no schema, and no changes will be made to an existing table that has a schema. Only for use with external storage."]
pub enum GooglePrivacyDlpV2OutputStorageConfigOutputSchemaEnum {
    #[serde(rename = "OUTPUT_SCHEMA_UNSPECIFIED")]
    #[doc = "Unused."]
    OutputSchemaUnspecified,
    #[serde(rename = "BASIC_COLUMNS")]
    #[doc = "Basic schema including only `info_type`, `quote`, `certainty`, and `timestamp`."]
    BasicColumns,
    #[serde(rename = "GCS_COLUMNS")]
    #[doc = "Schema tailored to findings from scanning Google Cloud Storage."]
    GcsColumns,
    #[serde(rename = "DATASTORE_COLUMNS")]
    #[doc = "Schema tailored to findings from scanning Google Datastore."]
    DatastoreColumns,
    #[serde(rename = "BIG_QUERY_COLUMNS")]
    #[doc = "Schema tailored to findings from scanning Google BigQuery."]
    BigQueryColumns,
    #[serde(rename = "ALL_COLUMNS")]
    #[doc = "Schema containing all columns."]
    AllColumns,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Datastore partition ID. A partition ID identifies a grouping of entities. The grouping is always by project and namespace, however the namespace ID may be empty. A partition ID contains several dimensions: project ID and namespace ID."]
pub struct GooglePrivacyDlpV2PartitionId {
    #[serde(rename = "namespaceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If not empty, the ID of the namespace to which the entities belong."]
    pub namespace_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the project to which the entities belong."]
    pub project_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A (kind, ID/name) pair used to construct a key path. If either name or ID is set, the element is complete. If neither is set, the element is incomplete."]
pub struct GooglePrivacyDlpV2PathElement {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The auto-allocated ID of the entity. Never equal to zero. Values less than zero are discouraged and may not be supported in the future."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of the entity. A kind matching regex `__.*__` is reserved/read-only. A kind must not contain more than 1500 bytes when UTF-8 encoded. Cannot be `\"\"`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the entity. A name matching regex `__.*__` is reserved/read-only. A name must not be more than 1500 bytes when UTF-8 encoded. Cannot be `\"\"`."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A rule for transforming a value."]
pub struct GooglePrivacyDlpV2PrimitiveTransformation {
    #[serde(rename = "bucketingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Bucketing"]
    pub bucketing_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BucketingConfig>>,
    #[serde(rename = "characterMaskConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mask"]
    pub character_mask_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CharacterMaskConfig>>,
    #[serde(rename = "cryptoDeterministicConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deterministic Crypto"]
    pub crypto_deterministic_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CryptoDeterministicConfig>>,
    #[serde(rename = "cryptoHashConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Crypto"]
    pub crypto_hash_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CryptoHashConfig>>,
    #[serde(rename = "cryptoReplaceFfxFpeConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ffx-Fpe"]
    pub crypto_replace_ffx_fpe_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CryptoReplaceFfxFpeConfig>>,
    #[serde(rename = "dateShiftConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date Shift"]
    pub date_shift_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DateShiftConfig>>,
    #[serde(rename = "fixedSizeBucketingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Fixed size bucketing"]
    pub fixed_size_bucketing_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FixedSizeBucketingConfig>>,
    #[serde(rename = "redactConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Redact"]
    pub redact_config: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RedactConfig>>,
    #[serde(rename = "replaceConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Replace"]
    pub replace_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ReplaceValueConfig>>,
    #[serde(rename = "replaceWithInfoTypeConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Replace with infotype"]
    pub replace_with_info_type_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ReplaceWithInfoTypeConfig>>,
    #[serde(rename = "timePartConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time extraction"]
    pub time_part_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TimePartConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Privacy metric to compute for reidentification risk analysis."]
pub struct GooglePrivacyDlpV2PrivacyMetric {
    #[serde(rename = "categoricalStatsConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Categorical stats"]
    pub categorical_stats_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CategoricalStatsConfig>>,
    #[serde(rename = "deltaPresenceEstimationConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "delta-presence"]
    pub delta_presence_estimation_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DeltaPresenceEstimationConfig>>,
    #[serde(rename = "kAnonymityConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "K-anonymity"]
    pub k_anonymity_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2KAnonymityConfig>>,
    #[serde(rename = "kMapEstimationConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "k-map"]
    pub k_map_estimation_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2KMapEstimationConfig>>,
    #[serde(rename = "lDiversityConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "l-diversity"]
    pub l_diversity_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2LDiversityConfig>>,
    #[serde(rename = "numericalStatsConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Numerical stats"]
    pub numerical_stats_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2NumericalStatsConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message for specifying a window around a finding to apply a detection rule."]
pub struct GooglePrivacyDlpV2Proximity {
    #[serde(rename = "windowAfter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of characters after the finding to consider."]
    pub window_after: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "windowBefore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of characters before the finding to consider."]
    pub window_before: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Publish findings of a DlpJob to Cloud Data Catalog. Labels summarizing the results of the DlpJob will be applied to the entry for the resource scanned in Cloud Data Catalog. Any labels previously written by another DlpJob will be deleted. InfoType naming patterns are strictly enforced when using this feature. Note that the findings will be persisted in Cloud Data Catalog storage and are governed by Data Catalog service-specific policy, see https://cloud.google.com/terms/service-terms Only a single instance of this action can be specified and only allowed if all resources being scanned are BigQuery tables. Compatible with: Inspect"]
pub struct GooglePrivacyDlpV2PublishFindingsToCloudDataCatalog {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Publish the result summary of a DlpJob to the Cloud Security Command Center (CSCC Alpha). This action is only available for projects which are parts of an organization and whitelisted for the alpha Cloud Security Command Center. The action will publish count of finding instances and their info types. The summary of findings will be persisted in CSCC and are governed by CSCC service-specific policy, see https://cloud.google.com/terms/service-terms Only a single instance of this action can be specified. Compatible with: Inspect"]
pub struct GooglePrivacyDlpV2PublishSummaryToCscc {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Publish a message into given Pub/Sub topic when DlpJob has completed. The message contains a single field, `DlpJobName`, which is equal to the finished job's [`DlpJob.name`](https://cloud.google.com/dlp/docs/reference/rest/v2/projects.dlpJobs#DlpJob). Compatible with: Inspect, Risk"]
pub struct GooglePrivacyDlpV2PublishToPubSub {
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Pub/Sub topic to send notifications to. The topic must have given publishing access rights to the DLP API service account executing the long running DlpJob sending the notifications. Format is projects/{project}/topics/{topic}."]
    pub topic: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Enable Stackdriver metric dlp.googleapis.com/finding_count. This will publish a metric to stack driver on each infotype requested and how many findings were found for it. CustomDetectors will be bucketed as 'Custom' under the Stackdriver label 'info_type'."]
pub struct GooglePrivacyDlpV2PublishToStackdriver {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A column with a semantic tag attached."]
pub struct GooglePrivacyDlpV2QuasiId {
    #[serde(rename = "customTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A column can be tagged with a custom tag. In this case, the user must indicate an auxiliary table that contains statistical information on the possible values of this column (below)."]
    pub custom_tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Identifies the column."]
    pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
    #[serde(rename = "inferred")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If no semantic tag is indicated, we infer the statistical model from the distribution of values in the input data"]
    pub inferred: ::std::option::Option<::std::boxed::Box<GoogleProtobufEmpty>>,
    #[serde(rename = "infoType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A column can be tagged with a InfoType to use the relevant public dataset as a statistical model of population, if available. We currently support US ZIP codes, region codes, ages and genders. To programmatically obtain the list of supported InfoTypes, use ListInfoTypes with the supported_by=RISK_ANALYSIS filter."]
    pub info_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A quasi-identifier column has a custom_tag, used to know which column in the data corresponds to which column in the statistical model."]
pub struct GooglePrivacyDlpV2QuasiIdField {
    #[serde(rename = "customTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A auxiliary field."]
    pub custom_tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies the column."]
    pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A quasi-identifier column has a custom_tag, used to know which column in the data corresponds to which column in the statistical model."]
pub struct GooglePrivacyDlpV2QuasiIdentifierField {
    #[serde(rename = "customTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A column can be tagged with a custom tag. In this case, the user must indicate an auxiliary table that contains statistical information on the possible values of this column (below)."]
    pub custom_tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies the column."]
    pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message for infoType-dependent details parsed from quote."]
pub struct GooglePrivacyDlpV2QuoteInfo {
    #[serde(rename = "dateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date time indicated by the quote."]
    pub date_time: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DateTime>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Generic half-open interval [start, end)"]
pub struct GooglePrivacyDlpV2Range {
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Index of the last character of the range (exclusive)."]
    pub end: ::std::option::Option<::std::string::String>,
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Index of the first character of the range (inclusive)."]
    pub start: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A condition for determining whether a transformation should be applied to a field."]
pub struct GooglePrivacyDlpV2RecordCondition {
    #[serde(rename = "expressions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An expression."]
    pub expressions: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Expressions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message for a unique key indicating a record that contains a finding."]
pub struct GooglePrivacyDlpV2RecordKey {
    #[serde(rename = "bigQueryKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub big_query_key: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryKey>>,
    #[serde(rename = "datastoreKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub datastore_key: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DatastoreKey>>,
    #[serde(rename = "idValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Values of identifying columns in the given row. Order of values matches the order of `identifying_fields` specified in the scanning request."]
    pub id_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Location of a finding within a row or record."]
pub struct GooglePrivacyDlpV2RecordLocation {
    #[serde(rename = "fieldId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Field id of the field containing the finding."]
    pub field_id: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
    #[serde(rename = "recordKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key of the finding."]
    pub record_key: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RecordKey>>,
    #[serde(rename = "tableLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location within a `ContentItem.Table`."]
    pub table_location: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TableLocation>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration to suppress records whose suppression conditions evaluate to true."]
pub struct GooglePrivacyDlpV2RecordSuppression {
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A condition that when it evaluates to true will result in the record being evaluated to be suppressed from the transformed content."]
    pub condition: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RecordCondition>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A type of transformation that is applied over structured data such as a table."]
pub struct GooglePrivacyDlpV2RecordTransformations {
    #[serde(rename = "fieldTransformations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Transform the record by applying various field transformations."]
    pub field_transformations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2FieldTransformation>>,
    >,
    #[serde(rename = "recordSuppressions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration defining which records get suppressed entirely. Records that match any suppression rule are omitted from the output."]
    pub record_suppressions: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2RecordSuppression>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Redact a given value. For example, if used with an `InfoTypeTransformation` transforming PHONE_NUMBER, and input 'My phone number is 206-555-0123', the output would be 'My phone number is '."]
pub struct GooglePrivacyDlpV2RedactConfig {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to search for potentially sensitive info in an image and redact it by covering it with a colored rectangle."]
pub struct GooglePrivacyDlpV2RedactImageRequest {
    #[serde(rename = "byteItem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content must be PNG, JPEG, SVG or BMP."]
    pub byte_item: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ByteContentItem>>,
    #[serde(rename = "imageRedactionConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration for specifying what content to redact from images."]
    pub image_redaction_configs: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2ImageRedactionConfig>>,
    >,
    #[serde(rename = "includeFindings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the response should include findings along with the redacted image."]
    pub include_findings: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "inspectConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for the inspector."]
    pub inspect_config: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectConfig>>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. This field has no effect."]
    pub location_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Results of redacting an image."]
pub struct GooglePrivacyDlpV2RedactImageResponse {
    #[serde(rename = "extractedText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If an image was being inspected and the InspectConfig's include_quote was set to true, then this field will include all text, if any, that was found in the image."]
    pub extracted_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inspectResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The findings. Populated when include_findings in the request is true."]
    pub inspect_result: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectResult>>,
    #[serde(rename = "redactedImage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The redacted image. The type will be the same as the original image."]
    pub redacted_image: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message defining a custom regular expression."]
pub struct GooglePrivacyDlpV2Regex {
    #[serde(rename = "groupIndexes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The index of the submatch to extract as findings. When not specified, the entire match is returned. No more than 3 may be included."]
    pub group_indexes: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "pattern")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pattern defining the regular expression. Its syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to re-identify an item."]
pub struct GooglePrivacyDlpV2ReidentifyContentRequest {
    #[serde(rename = "inspectConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for the inspector."]
    pub inspect_config: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectConfig>>,
    #[serde(rename = "inspectTemplateName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Template to use. Any configuration directly specified in `inspect_config` will override those set in the template. Singular fields that are set in this request will replace their corresponding fields in the template. Repeated fields are appended. Singular sub-messages and groups are recursively merged."]
    pub inspect_template_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "item")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The item to re-identify. Will be treated as text."]
    pub item: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ContentItem>>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. This field has no effect."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reidentifyConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for the re-identification of the content item. This field shares the same proto message type that is used for de-identification, however its usage here is for the reversal of the previous de-identification. Re-identification is performed by examining the transformations used to de-identify the items and executing the reverse. This requires that only reversible transformations be provided here. The reversible transformations are: - `CryptoDeterministicConfig` - `CryptoReplaceFfxFpeConfig`"]
    pub reidentify_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DeidentifyConfig>>,
    #[serde(rename = "reidentifyTemplateName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Template to use. References an instance of `DeidentifyTemplate`. Any configuration directly specified in `reidentify_config` or `inspect_config` will override those set in the template. The `DeidentifyTemplate` used must include only reversible transformations. Singular fields that are set in this request will replace their corresponding fields in the template. Repeated fields are appended. Singular sub-messages and groups are recursively merged."]
    pub reidentify_template_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Results of re-identifying a item."]
pub struct GooglePrivacyDlpV2ReidentifyContentResponse {
    #[serde(rename = "item")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The re-identified item."]
    pub item: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ContentItem>>,
    #[serde(rename = "overview")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An overview of the changes that were made to the `item`."]
    pub overview:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TransformationOverview>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Replace each input value with a given `Value`."]
pub struct GooglePrivacyDlpV2ReplaceValueConfig {
    #[serde(rename = "newValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value to replace it with."]
    pub new_value: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Replace each matching finding with the name of the info_type."]
pub struct GooglePrivacyDlpV2ReplaceWithInfoTypeConfig {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Snapshot of the inspection configuration."]
pub struct GooglePrivacyDlpV2RequestedOptions {
    #[serde(rename = "jobConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inspect config."]
    pub job_config: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectJobConfig>>,
    #[serde(rename = "snapshotInspectTemplate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If run with an InspectTemplate, a snapshot of its state at the time of this run."]
    pub snapshot_inspect_template:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectTemplate>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Risk analysis options."]
pub struct GooglePrivacyDlpV2RequestedRiskAnalysisOptions {
    #[serde(rename = "jobConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The job config for the risk job."]
    pub job_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RiskAnalysisJobConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "All result fields mentioned below are updated while the job is processing."]
pub struct GooglePrivacyDlpV2Result {
    #[serde(rename = "hybridStats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Statistics related to the processing of hybrid inspect."]
    pub hybrid_stats:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2HybridInspectStatistics>>,
    #[serde(rename = "infoTypeStats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Statistics of how many instances of each info type were found during inspect job."]
    pub info_type_stats:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2InfoTypeStats>>>,
    #[serde(rename = "processedBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total size in bytes that were processed."]
    pub processed_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalEstimatedBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Estimate of the number of bytes to process."]
    pub total_estimated_bytes: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for a risk analysis job. See https://cloud.google.com/dlp/docs/concepts-risk-analysis to learn more."]
pub struct GooglePrivacyDlpV2RiskAnalysisJobConfig {
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Actions to execute at the completion of the job. Are executed in the order provided."]
    pub actions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Action>>>,
    #[serde(rename = "privacyMetric")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Privacy metric to compute."]
    pub privacy_metric: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PrivacyMetric>>,
    #[serde(rename = "sourceTable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input dataset to compute metrics over."]
    pub source_table: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryTable>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Values of the row."]
pub struct GooglePrivacyDlpV2Row {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Individual cells."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Value>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "If set, the detailed findings will be persisted to the specified OutputStorageConfig. Only a single instance of this action can be specified. Compatible with: Inspect, Risk"]
pub struct GooglePrivacyDlpV2SaveFindings {
    #[serde(rename = "outputConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location to store findings outside of DLP."]
    pub output_config:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2OutputStorageConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Schedule for inspect job triggers."]
pub struct GooglePrivacyDlpV2Schedule {
    #[serde(rename = "recurrencePeriodDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "With this option a job is started a regular periodic basis. For example: every day (86400 seconds). A scheduled start time will be skipped if the previous execution has not ended when its scheduled time occurs. This value must be set to a time duration greater than or equal to 1 day and can be no longer than 60 days."]
    pub recurrence_period_duration: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An auxiliary table containing statistical information on the relative frequency of different quasi-identifiers values. It has one or several quasi-identifiers columns, and one column that indicates the relative frequency of each quasi-identifier tuple. If a tuple is present in the data but not in the auxiliary table, the corresponding relative frequency is assumed to be zero (and thus, the tuple is highly reidentifiable)."]
pub struct GooglePrivacyDlpV2StatisticalTable {
    #[serde(rename = "quasiIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Quasi-identifier columns."]
    pub quasi_ids: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2QuasiIdentifierField>>,
    >,
    #[serde(rename = "relativeFrequency")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The relative frequency column must contain a floating-point number between 0 and 1 (inclusive). Null values are assumed to be zero."]
    pub relative_frequency: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
    #[serde(rename = "table")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Auxiliary table location."]
    pub table: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryTable>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Shared message indicating Cloud storage type."]
pub struct GooglePrivacyDlpV2StorageConfig {
    #[serde(rename = "bigQueryOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "BigQuery options."]
    pub big_query_options:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2BigQueryOptions>>,
    #[serde(rename = "cloudStorageOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Cloud Storage options."]
    pub cloud_storage_options:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2CloudStorageOptions>>,
    #[serde(rename = "datastoreOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Cloud Datastore options."]
    pub datastore_options:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DatastoreOptions>>,
    #[serde(rename = "hybridOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hybrid inspection options."]
    pub hybrid_options: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2HybridOptions>>,
    #[serde(rename = "timespanConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timespan_config: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2TimespanConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Storage metadata label to indicate which metadata entry contains findings."]
pub struct GooglePrivacyDlpV2StorageMetadataLabel {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub key: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "StoredInfoType resource message that contains information about the current version and any pending updates."]
pub struct GooglePrivacyDlpV2StoredInfoType {
    #[serde(rename = "currentVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Current version of the stored info type."]
    pub current_version:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2StoredInfoTypeVersion>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pendingVersions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pending versions of the stored info type. Empty if no versions are pending."]
    pub pending_versions: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2StoredInfoTypeVersion>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for stored infoTypes. All fields and subfield are provided by the user. For more information, see https://cloud.google.com/dlp/docs/creating-custom-infotypes."]
pub struct GooglePrivacyDlpV2StoredInfoTypeConfig {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the StoredInfoType (max 256 characters)."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dictionary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Store dictionary-based CustomInfoType."]
    pub dictionary: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Dictionary>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display name of the StoredInfoType (max 256 characters)."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "largeCustomDictionary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "StoredInfoType where findings are defined by a dictionary of phrases."]
    pub large_custom_dictionary:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2LargeCustomDictionaryConfig>>,
    #[serde(rename = "regex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Store regular expression-based StoredInfoType."]
    pub regex: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Regex>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Statistics for a StoredInfoType."]
pub struct GooglePrivacyDlpV2StoredInfoTypeStats {
    #[serde(rename = "largeCustomDictionary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "StoredInfoType where findings are defined by a dictionary of phrases."]
    pub large_custom_dictionary:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2LargeCustomDictionaryStats>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Version of a StoredInfoType, including the configuration used to build it, create timestamp, and current state."]
pub struct GooglePrivacyDlpV2StoredInfoTypeVersion {
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "StoredInfoType configuration."]
    pub config: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2StoredInfoTypeConfig>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Create timestamp of the version. Read-only, determined by the system when the version is created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Errors that occurred when creating this storedInfoType version, or anomalies detected in the storedInfoType data that render it unusable. Only the five most recent errors will be displayed, with the most recent error appearing first. For example, some of the data for stored custom dictionaries is put in the user's Google Cloud Storage bucket, and if this data is modified or deleted by the user or another system, the dictionary becomes invalid. If any errors occur, fix the problem indicated by the error message and use the UpdateStoredInfoType API method to create another version of the storedInfoType to continue using it, reusing the same `config` if it was not the source of the error."]
    pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Error>>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Stored info type version state. Read-only, updated by the system during dictionary creation."]
    pub state: ::std::option::Option<GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum>,
    #[serde(rename = "stats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Statistics about this storedInfoType version."]
    pub stats: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2StoredInfoTypeStats>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Stored info type version state. Read-only, updated by the system during dictionary creation."]
pub enum GooglePrivacyDlpV2StoredInfoTypeVersionStateEnum {
    #[serde(rename = "STORED_INFO_TYPE_STATE_UNSPECIFIED")]
    #[doc = "Unused"]
    StoredInfoTypeStateUnspecified,
    #[serde(rename = "PENDING")]
    #[doc = "StoredInfoType version is being created."]
    Pending,
    #[serde(rename = "READY")]
    #[doc = "StoredInfoType version is ready for use."]
    Ready,
    #[serde(rename = "FAILED")]
    #[doc = "StoredInfoType creation failed. All relevant error messages are returned in the `StoredInfoTypeVersion` message."]
    Failed,
    #[serde(rename = "INVALID")]
    #[doc = "StoredInfoType is no longer valid because artifacts stored in user-controlled storage were modified. To fix an invalid StoredInfoType, use the `UpdateStoredInfoType` method to create a new version."]
    Invalid,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A reference to a StoredInfoType to use with scanning."]
pub struct GooglePrivacyDlpV2StoredType {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp indicating when the version of the `StoredInfoType` used for inspection was created. Output-only field, populated by the system."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name of the requested `StoredInfoType`, for example `organizations/433245324/storedInfoTypes/432452342` or `projects/project-id/storedInfoTypes/432452342`."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A collection that informs the user the number of times a particular `TransformationResultCode` and error details occurred."]
pub struct GooglePrivacyDlpV2SummaryResult {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Outcome of the transformation."]
    pub code: ::std::option::Option<GooglePrivacyDlpV2SummaryResultCodeEnum>,
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of transformations counted by this result."]
    pub count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A place for warnings or errors to show up if a transformation didn't work as expected."]
    pub details: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Outcome of the transformation."]
pub enum GooglePrivacyDlpV2SummaryResultCodeEnum {
    #[serde(rename = "TRANSFORMATION_RESULT_CODE_UNSPECIFIED")]
    #[doc = "Unused"]
    TransformationResultCodeUnspecified,
    #[serde(rename = "SUCCESS")]
    #[doc = "Transformation completed without an error."]
    Success,
    #[serde(rename = "ERROR")]
    #[doc = "Transformation had an error."]
    Error,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message for detecting output from deidentification transformations such as [`CryptoReplaceFfxFpeConfig`](https://cloud.google.com/dlp/docs/reference/rest/v2/organizations.deidentifyTemplates#cryptoreplaceffxfpeconfig). These types of transformations are those that perform pseudonymization, thereby producing a \"surrogate\" as output. This should be used in conjunction with a field on the transformation such as `surrogate_info_type`. This CustomInfoType does not support the use of `detection_rules`."]
pub struct GooglePrivacyDlpV2SurrogateType {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Structured content to inspect. Up to 50,000 `Value`s per request allowed. See https://cloud.google.com/dlp/docs/inspecting-text#inspecting_a_table to learn more."]
pub struct GooglePrivacyDlpV2Table {
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Headers of the table."]
    pub headers:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>>,
    #[serde(rename = "rows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rows of the table."]
    pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2Row>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Location of a finding within a table."]
pub struct GooglePrivacyDlpV2TableLocation {
    #[serde(rename = "rowIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The zero-based index of the row where the finding is located. Only populated for resources that have a natural ordering, not BigQuery. In BigQuery, to identify the row a finding came from, populate BigQueryOptions.identifying_fields with your primary key column names and when you store the findings the value of those columns will be stored inside of Finding."]
    pub row_index: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Instructions regarding the table content being inspected."]
pub struct GooglePrivacyDlpV2TableOptions {
    #[serde(rename = "identifyingFields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The columns that are the primary keys for table objects included in ContentItem. A copy of this cell's value will stored alongside alongside each finding so that the finding can be traced to the specific row it came from. No more than 3 may be provided."]
    pub identifying_fields:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A column with a semantic tag attached."]
pub struct GooglePrivacyDlpV2TaggedField {
    #[serde(rename = "customTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A column can be tagged with a custom tag. In this case, the user must indicate an auxiliary table that contains statistical information on the possible values of this column (below)."]
    pub custom_tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Identifies the column."]
    pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
    #[serde(rename = "inferred")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If no semantic tag is indicated, we infer the statistical model from the distribution of values in the input data"]
    pub inferred: ::std::option::Option<::std::boxed::Box<GoogleProtobufEmpty>>,
    #[serde(rename = "infoType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A column can be tagged with a InfoType to use the relevant public dataset as a statistical model of population, if available. We currently support US ZIP codes, region codes, ages and genders. To programmatically obtain the list of supported InfoTypes, use ListInfoTypes with the supported_by=RISK_ANALYSIS filter."]
    pub info_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Throw an error and fail the request when a transformation error occurs."]
pub struct GooglePrivacyDlpV2ThrowError {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "For use with `Date`, `Timestamp`, and `TimeOfDay`, extract or preserve a portion of the value."]
pub struct GooglePrivacyDlpV2TimePartConfig {
    #[serde(rename = "partToExtract")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The part of the time to keep."]
    pub part_to_extract: ::std::option::Option<GooglePrivacyDlpV2TimePartConfigPartToExtractEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The part of the time to keep."]
pub enum GooglePrivacyDlpV2TimePartConfigPartToExtractEnum {
    #[serde(rename = "TIME_PART_UNSPECIFIED")]
    #[doc = "Unused"]
    TimePartUnspecified,
    #[serde(rename = "YEAR")]
    #[doc = "[0-9999]"]
    Year,
    #[serde(rename = "MONTH")]
    #[doc = "[1-12]"]
    Month,
    #[serde(rename = "DAY_OF_MONTH")]
    #[doc = "[1-31]"]
    DayOfMonth,
    #[serde(rename = "DAY_OF_WEEK")]
    #[doc = "[1-7]"]
    DayOfWeek,
    #[serde(rename = "WEEK_OF_YEAR")]
    #[doc = "[1-53]"]
    WeekOfYear,
    #[serde(rename = "HOUR_OF_DAY")]
    #[doc = "[0-23]"]
    HourOfDay,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Time zone of the date time object."]
pub struct GooglePrivacyDlpV2TimeZone {
    #[serde(rename = "offsetMinutes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set only if the offset can be determined. Positive for time ahead of UTC. E.g. For \"UTC-9\", this value is -540."]
    pub offset_minutes: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration of the timespan of the items to include in scanning. Currently only supported when inspecting Google Cloud Storage and BigQuery."]
pub struct GooglePrivacyDlpV2TimespanConfig {
    #[serde(rename = "enableAutoPopulationOfTimespanConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When the job is started by a JobTrigger we will automatically figure out a valid start_time to avoid scanning files that have not been modified since the last time the JobTrigger executed. This will be based on the time of the execution of the last run of the JobTrigger."]
    pub enable_auto_population_of_timespan_config: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Exclude files, tables, or rows newer than this value. If not set, no upper time limit is applied."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Exclude files, tables, or rows older than this value. If not set, no lower time limit is applied."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestampField")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specification of the field containing the timestamp of scanned items. Used for data sources like Datastore and BigQuery. For BigQuery: If this value is not specified and the table was modified between the given start and end times, the entire table will be scanned. If this value is specified, then rows are filtered based on the given start and end times. Rows with a `NULL` value in the provided BigQuery column are skipped. Valid data types of the provided BigQuery column are: `INTEGER`, `DATE`, `TIMESTAMP`, and `DATETIME`. For Datastore: If this value is specified, then entities are filtered based on the given start and end times. If an entity does not contain the provided timestamp property or contains empty or invalid values, then it is included. Valid data types of the provided timestamp property are: `TIMESTAMP`."]
    pub timestamp_field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "How to handle transformation errors during de-identification. A transformation error occurs when the requested transformation is incompatible with the data. For example, trying to de-identify an IP address using a `DateShift` transformation would result in a transformation error, since date info cannot be extracted from an IP address. Information about any incompatible transformations, and how they were handled, is returned in the response as part of the `TransformationOverviews`."]
pub struct GooglePrivacyDlpV2TransformationErrorHandling {
    #[serde(rename = "leaveUntransformed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ignore errors"]
    pub leave_untransformed:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2LeaveUntransformed>>,
    #[serde(rename = "throwError")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Throw an error"]
    pub throw_error: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2ThrowError>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Overview of the modifications that occurred."]
pub struct GooglePrivacyDlpV2TransformationOverview {
    #[serde(rename = "transformationSummaries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Transformations applied to the dataset."]
    pub transformation_summaries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2TransformationSummary>>,
    >,
    #[serde(rename = "transformedBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total size in bytes that were transformed in some way."]
    pub transformed_bytes: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Summary of a single transformation. Only one of 'transformation', 'field_transformation', or 'record_suppress' will be set."]
pub struct GooglePrivacyDlpV2TransformationSummary {
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set if the transformation was limited to a specific FieldId."]
    pub field: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2FieldId>>,
    #[serde(rename = "fieldTransformations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The field transformation that was applied. If multiple field transformations are requested for a single field, this list will contain all of them; otherwise, only one is supplied."]
    pub field_transformations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2FieldTransformation>>,
    >,
    #[serde(rename = "infoType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set if the transformation was limited to a specific InfoType."]
    pub info_type: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InfoType>>,
    #[serde(rename = "recordSuppress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The specific suppression option these stats apply to."]
    pub record_suppress:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2RecordSuppression>>,
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Collection of all transformations that took place or had an error."]
    pub results:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GooglePrivacyDlpV2SummaryResult>>>,
    #[serde(rename = "transformation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The specific transformation these stats apply to."]
    pub transformation:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2PrimitiveTransformation>>,
    #[serde(rename = "transformedBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total size in bytes that were transformed in some way."]
    pub transformed_bytes: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Use this to have a random data crypto key generated. It will be discarded after the request finishes."]
pub struct GooglePrivacyDlpV2TransientCryptoKey {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Name of the key. This is an arbitrary string used to differentiate different keys. A unique key is generated per name: two separate `TransientCryptoKey` protos share the same generated key if their names are the same. When the data crypto key is generated, this name is not used in any way (repeating the api call will result in a different key being generated)."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "What event needs to occur for a new job to be started."]
pub struct GooglePrivacyDlpV2Trigger {
    #[serde(rename = "manual")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For use with hybrid jobs. Jobs must be manually created and finished."]
    pub manual: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Manual>>,
    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Create a job on a repeating basis based on the elapse of time."]
    pub schedule: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Schedule>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Using raw keys is prone to security risks due to accidentally leaking the key. Choose another type of key if possible."]
pub struct GooglePrivacyDlpV2UnwrappedCryptoKey {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A 128/192/256 bit key."]
    pub key: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for UpdateDeidentifyTemplate."]
pub struct GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest {
    #[serde(rename = "deidentifyTemplate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "New DeidentifyTemplate value."]
    pub deidentify_template:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2DeidentifyTemplate>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mask to control which fields get updated."]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for UpdateInspectTemplate."]
pub struct GooglePrivacyDlpV2UpdateInspectTemplateRequest {
    #[serde(rename = "inspectTemplate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "New InspectTemplate value."]
    pub inspect_template:
        ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2InspectTemplate>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mask to control which fields get updated."]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for UpdateJobTrigger."]
pub struct GooglePrivacyDlpV2UpdateJobTriggerRequest {
    #[serde(rename = "jobTrigger")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "New JobTrigger value."]
    pub job_trigger: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2JobTrigger>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mask to control which fields get updated."]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for UpdateStoredInfoType."]
pub struct GooglePrivacyDlpV2UpdateStoredInfoTypeRequest {
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Updated configuration for the storedInfoType. If not provided, a new version of the storedInfoType will be created with the existing configuration."]
    pub config: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2StoredInfoTypeConfig>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mask to control which fields get updated."]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Set of primitive values supported by the system. Note that for the purposes of inspection or transformation, the number of bytes considered to comprise a 'Value' is based on its representation as a UTF-8 encoded string. For example, if 'integer_value' is set to 123456789, the number of bytes would be counted as 9, even though an int64 only holds up to 8 bytes of data."]
pub struct GooglePrivacyDlpV2Value {
    #[serde(rename = "booleanValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "boolean"]
    pub boolean_value: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "dateValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "date"]
    pub date_value: ::std::option::Option<::std::boxed::Box<GoogleTypeDate>>,
    #[serde(rename = "dayOfWeekValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "day of week"]
    pub day_of_week_value: ::std::option::Option<GooglePrivacyDlpV2ValueDayOfWeekValueEnum>,
    #[serde(rename = "floatValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "float"]
    pub float_value: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "integerValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "integer"]
    pub integer_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "string"]
    pub string_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "time of day"]
    pub time_value: ::std::option::Option<::std::boxed::Box<GoogleTypeTimeOfDay>>,
    #[serde(rename = "timestampValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "timestamp"]
    pub timestamp_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "day of week"]
pub enum GooglePrivacyDlpV2ValueDayOfWeekValueEnum {
    #[serde(rename = "DAY_OF_WEEK_UNSPECIFIED")]
    #[doc = "The day of the week is unspecified."]
    DayOfWeekUnspecified,
    #[serde(rename = "MONDAY")]
    #[doc = "Monday"]
    Monday,
    #[serde(rename = "TUESDAY")]
    #[doc = "Tuesday"]
    Tuesday,
    #[serde(rename = "WEDNESDAY")]
    #[doc = "Wednesday"]
    Wednesday,
    #[serde(rename = "THURSDAY")]
    #[doc = "Thursday"]
    Thursday,
    #[serde(rename = "FRIDAY")]
    #[doc = "Friday"]
    Friday,
    #[serde(rename = "SATURDAY")]
    #[doc = "Saturday"]
    Saturday,
    #[serde(rename = "SUNDAY")]
    #[doc = "Sunday"]
    Sunday,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A value of a field, including its frequency."]
pub struct GooglePrivacyDlpV2ValueFrequency {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How many times the value is contained in the field."]
    pub count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A value contained in the field in question."]
    pub value: ::std::option::Option<::std::boxed::Box<GooglePrivacyDlpV2Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message defining a list of words or phrases to search for in the data."]
pub struct GooglePrivacyDlpV2WordList {
    #[serde(rename = "words")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Words or phrases defining the dictionary. The dictionary must contain at least one phrase and every phrase must contain at least 2 characters that are letters or digits. [required]"]
    pub words: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct GoogleProtobufEmpty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
pub struct GoogleRpcStatus {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status code, which should be an enum value of google.rpc.Code."]
    pub code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
    pub details: ::std::option::Option<
        ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    >,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."]
pub struct GoogleTypeDate {
    #[serde(rename = "day")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub day: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "month")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub month: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "year")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub year: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`."]
pub struct GoogleTypeTimeOfDay {
    #[serde(rename = "hours")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub hours: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "minutes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minutes of hour of day. Must be from 0 to 59."]
    pub minutes: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "nanos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub nanos: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "seconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub seconds: ::std::option::Option<::std::primitive::i64>,
}
