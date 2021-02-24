#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Result returned from `ListTraceSinks`."]
pub struct ListTraceSinksResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If there might be more results than appear in this response, then `nextPageToken` is included. To get the next set of results, call the same method again using the value of `nextPageToken` as `pageToken`."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sinks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of sinks."]
    pub sinks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TraceSink>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "OutputConfig contains a destination for writing trace data."]
pub struct OutputConfig {
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The destination for writing trace data. Currently only BigQuery is supported. E.g.: \"bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]\""]
    pub destination: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a sink used to export traces to a BigQuery dataset. The sink must be created within a project."]
pub struct TraceSink {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The canonical sink resource name, unique within the project. Must be of the form: project/[PROJECT_NUMBER]/traceSinks/[SINK_ID]. E.g.: `\"projects/12345/traceSinks/my-project-trace-sink\"`. Sink identifiers are limited to 256 characters and can include only the following characters: upper and lower-case alphanumeric characters, underscores, hyphens, and periods."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outputConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The export destination."]
    pub output_config: ::std::option::Option<::std::boxed::Box<OutputConfig>>,
    #[serde(rename = "writerIdentity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A service account name for exporting the data. This field is set by sinks.create and sinks.update. The service account will need to be granted write access to the destination specified in the output configuration, see [Granting access for a resource](/iam/docs/granting-roles-to-service-accounts#granting_access_to_a_service_account_for_a_resource). To create tables and write data this account will need the dataEditor role. Read more about roles in the [BigQuery documentation](https://cloud.google.com/bigquery/docs/access-control). E.g.: \"service-00000001@00000002.iam.gserviceaccount.com\""]
    pub writer_identity: ::std::option::Option<::std::string::String>,
}
