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
    pub mod transfer_jobs {
        pub mod methods {
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projectId")]
                    #[doc = "Required. The ID of the Google Cloud Platform Console project that owns the job."]
                    pub project_id: ::std::string::String,
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
                    #[doc = "Required. A list of query parameters specified as JSON text in the form of: `{\"projectId\":\"my_project_id\", \"jobNames\":[\"jobid1\",\"jobid2\",...], \"jobStatuses\":[\"status1\",\"status2\",...]}` Since `jobNames` and `jobStatuses` support multiple values, their values must be specified with array notation. `projectId` is required. `jobNames` and `jobStatuses` are optional. The valid values for `jobStatuses` are case-insensitive: ENABLED, DISABLED, and DELETED."]
                    pub filter: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The list page size. The max allowed value is 256."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The list page token."]
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
    pub mod transfer_operations {
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
                    #[serde(rename = "filter")]
                    #[doc = "Required. A list of query parameters specified as JSON text in the form of: `{\"projectId\":\"my_project_id\", \"jobNames\":[\"jobid1\",\"jobid2\",...], \"operationNames\":[\"opid1\",\"opid2\",...], \"transferStatuses\":[\"status1\",\"status2\",...]}` Since `jobNames`, `operationNames`, and `transferStatuses` support multiple values, they must be specified with array notation. `projectId` is required. `jobNames`, `operationNames`, and `transferStatuses` are optional. The valid values for `transferStatuses` are case-insensitive: IN_PROGRESS, PAUSED, SUCCESS, FAILED, and ABORTED."]
                    pub filter: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The list page size. The max allowed value is 256."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The list page token."]
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
    #[doc = "AWS access key (see [AWS Security Credentials](https://docs.aws.amazon.com/general/latest/gr/aws-security-credentials.html)). For information on our data retention policy for user credentials, see [User credentials](/storage-transfer/docs/data-retention#user-credentials)."]
    pub struct AwsAccessKey {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessKeyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. AWS access key ID."]
        pub access_key_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secretAccessKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. AWS secret access key. This field is not returned in RPC responses."]
        pub secret_access_key: ::std::option::Option<::std::string::String>,
    }
    impl AwsAccessKey {
        pub fn builder() -> AwsAccessKeyBuilder {
            AwsAccessKeyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An AwsS3Data resource can be a data source, but not a data sink. In an AwsS3Data resource, an object's name is the S3 object's key name."]
    pub struct AwsS3Data {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "awsAccessKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. AWS access key used to sign the API requests to the AWS S3 bucket. Permissions on the bucket must be granted to the access ID of the AWS access key. For information on our data retention policy for user credentials, see [User credentials](/storage-transfer/docs/data-retention#user-credentials)."]
        pub aws_access_key: ::std::option::Option<::std::boxed::Box<AwsAccessKey>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. S3 Bucket name (see [Creating a bucket](https://docs.aws.amazon.com/AmazonS3/latest/dev/create-bucket-get-location-example.html))."]
        pub bucket_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Root path to transfer objects. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should generally not begin with a '/'."]
        pub path: ::std::option::Option<::std::string::String>,
    }
    impl AwsS3Data {
        pub fn builder() -> AwsS3DataBuilder {
            AwsS3DataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An AzureBlobStorageData resource can be a data source, but not a data sink. An AzureBlobStorageData resource represents one Azure container. The storage account determines the [Azure endpoint](https://docs.microsoft.com/en-us/azure/storage/common/storage-create-storage-account#storage-account-endpoints). In an AzureBlobStorageData resource, a blobs's name is the [Azure Blob Storage blob's key name](https://docs.microsoft.com/en-us/rest/api/storageservices/naming-and-referencing-containers--blobs--and-metadata#blob-names)."]
    pub struct AzureBlobStorageData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "azureCredentials")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. Credentials used to authenticate API requests to Azure. For information on our data retention policy for user credentials, see [User credentials](/storage-transfer/docs/data-retention#user-credentials)."]
        pub azure_credentials: ::std::option::Option<::std::boxed::Box<AzureCredentials>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "container")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The container to transfer from the Azure Storage account."]
        pub container: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Root path to transfer objects. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should generally not begin with a '/'."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storageAccount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the Azure Storage account."]
        pub storage_account: ::std::option::Option<::std::string::String>,
    }
    impl AzureBlobStorageData {
        pub fn builder() -> AzureBlobStorageDataBuilder {
            AzureBlobStorageDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Azure credentials For information on our data retention policy for user credentials, see [User credentials](/storage-transfer/docs/data-retention#user-credentials)."]
    pub struct AzureCredentials {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sasToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Azure shared access signature. (see [Grant limited access to Azure Storage resources using shared access signatures (SAS)](https://docs.microsoft.com/en-us/azure/storage/common/storage-sas-overview))."]
        pub sas_token: ::std::option::Option<::std::string::String>,
    }
    impl AzureCredentials {
        pub fn builder() -> AzureCredentialsBuilder {
            AzureCredentialsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for Operations.CancelOperation."]
    pub struct CancelOperationRequest {}
    impl CancelOperationRequest {
        pub fn builder() -> CancelOperationRequestBuilder {
            CancelOperationRequestBuilder::default()
        }
    }
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
    #[doc = "An entry describing an error that has occurred."]
    pub struct ErrorLogEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of messages that carry the error details."]
        pub error_details: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A URL that refers to the target (a data source, a data sink, or an object) with which the error is associated."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl ErrorLogEntry {
        pub fn builder() -> ErrorLogEntryBuilder {
            ErrorLogEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A summary of errors by error code, plus a count and sample error log entries."]
    pub struct ErrorSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required."]
        pub error_code: ::std::option::Option<ErrorSummaryErrorCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Count of this type of error."]
        pub error_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorLogEntries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error samples. At most 5 error log entries will be recorded for a given error code for a single transfer operation."]
        pub error_log_entries:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ErrorLogEntry>>>,
    }
    impl ErrorSummary {
        pub fn builder() -> ErrorSummaryBuilder {
            ErrorSummaryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required."]
    pub enum ErrorSummaryErrorCodeEnum {
        #[serde(rename = "OK")]
        #[doc = "Not an error; returned on success HTTP Mapping: 200 OK"]
        Ok,
        #[serde(rename = "CANCELLED")]
        #[doc = "The operation was cancelled, typically by the caller. HTTP Mapping: 499 Client Closed Request"]
        Cancelled,
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown error. For example, this error may be returned when a `Status` value received from another address space belongs to an error space that is not known in this address space. Also errors raised by APIs that do not return enough error information may be converted to this error. HTTP Mapping: 500 Internal Server Error"]
        Unknown,
        #[serde(rename = "INVALID_ARGUMENT")]
        #[doc = "The client specified an invalid argument. Note that this differs from `FAILED_PRECONDITION`. `INVALID_ARGUMENT` indicates arguments that are problematic regardless of the state of the system (e.g., a malformed file name). HTTP Mapping: 400 Bad Request"]
        InvalidArgument,
        #[serde(rename = "DEADLINE_EXCEEDED")]
        #[doc = "The deadline expired before the operation could complete. For operations that change the state of the system, this error may be returned even if the operation has completed successfully. For example, a successful response from a server could have been delayed long enough for the deadline to expire. HTTP Mapping: 504 Gateway Timeout"]
        DeadlineExceeded,
        #[serde(rename = "NOT_FOUND")]
        #[doc = "Some requested entity (e.g., file or directory) was not found. Note to server developers: if a request is denied for an entire class of users, such as gradual feature rollout or undocumented allowlist, `NOT_FOUND` may be used. If a request is denied for some users within a class of users, such as user-based access control, `PERMISSION_DENIED` must be used. HTTP Mapping: 404 Not Found"]
        NotFound,
        #[serde(rename = "ALREADY_EXISTS")]
        #[doc = "The entity that a client attempted to create (e.g., file or directory) already exists. HTTP Mapping: 409 Conflict"]
        AlreadyExists,
        #[serde(rename = "PERMISSION_DENIED")]
        #[doc = "The caller does not have permission to execute the specified operation. `PERMISSION_DENIED` must not be used for rejections caused by exhausting some resource (use `RESOURCE_EXHAUSTED` instead for those errors). `PERMISSION_DENIED` must not be used if the caller can not be identified (use `UNAUTHENTICATED` instead for those errors). This error code does not imply the request is valid or the requested entity exists or satisfies other pre-conditions. HTTP Mapping: 403 Forbidden"]
        PermissionDenied,
        #[serde(rename = "UNAUTHENTICATED")]
        #[doc = "The request does not have valid authentication credentials for the operation. HTTP Mapping: 401 Unauthorized"]
        Unauthenticated,
        #[serde(rename = "RESOURCE_EXHAUSTED")]
        #[doc = "Some resource has been exhausted, perhaps a per-user quota, or perhaps the entire file system is out of space. HTTP Mapping: 429 Too Many Requests"]
        ResourceExhausted,
        #[serde(rename = "FAILED_PRECONDITION")]
        #[doc = "The operation was rejected because the system is not in a state required for the operation's execution. For example, the directory to be deleted is non-empty, an rmdir operation is applied to a non-directory, etc. Service implementors can use the following guidelines to decide between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`: (a) Use `UNAVAILABLE` if the client can retry just the failing call. (b) Use `ABORTED` if the client should retry at a higher level (e.g., when a client-specified test-and-set fails, indicating the client should restart a read-modify-write sequence). (c) Use `FAILED_PRECONDITION` if the client should not retry until the system state has been explicitly fixed. E.g., if an \"rmdir\" fails because the directory is non-empty, `FAILED_PRECONDITION` should be returned since the client should not retry unless the files are deleted from the directory. HTTP Mapping: 400 Bad Request"]
        FailedPrecondition,
        #[serde(rename = "ABORTED")]
        #[doc = "The operation was aborted, typically due to a concurrency issue such as a sequencer check failure or transaction abort. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 409 Conflict"]
        Aborted,
        #[serde(rename = "OUT_OF_RANGE")]
        #[doc = "The operation was attempted past the valid range. E.g., seeking or reading past end-of-file. Unlike `INVALID_ARGUMENT`, this error indicates a problem that may be fixed if the system state changes. For example, a 32-bit file system will generate `INVALID_ARGUMENT` if asked to read at an offset that is not in the range [0,2^32-1], but it will generate `OUT_OF_RANGE` if asked to read from an offset past the current file size. There is a fair bit of overlap between `FAILED_PRECONDITION` and `OUT_OF_RANGE`. We recommend using `OUT_OF_RANGE` (the more specific error) when it applies so that callers who are iterating through a space can easily look for an `OUT_OF_RANGE` error to detect when they are done. HTTP Mapping: 400 Bad Request"]
        OutOfRange,
        #[serde(rename = "UNIMPLEMENTED")]
        #[doc = "The operation is not implemented or is not supported/enabled in this service. HTTP Mapping: 501 Not Implemented"]
        Unimplemented,
        #[serde(rename = "INTERNAL")]
        #[doc = "Internal errors. This means that some invariants expected by the underlying system have been broken. This error code is reserved for serious errors. HTTP Mapping: 500 Internal Server Error"]
        Internal,
        #[serde(rename = "UNAVAILABLE")]
        #[doc = "The service is currently unavailable. This is most likely a transient condition, which can be corrected by retrying with a backoff. Note that it is not always safe to retry non-idempotent operations. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 503 Service Unavailable"]
        Unavailable,
        #[serde(rename = "DATA_LOSS")]
        #[doc = "Unrecoverable data loss or corruption. HTTP Mapping: 500 Internal Server Error"]
        DataLoss,
    }
    impl ::std::default::Default for ErrorSummaryErrorCodeEnum {
        fn default() -> Self {
            Self::Ok
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "In a GcsData resource, an object's name is the Cloud Storage object's name and its \"last modification time\" refers to the object's `updated` property of Cloud Storage objects, which changes when the content or the metadata of the object is updated."]
    pub struct GcsData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Cloud Storage bucket name (see [Bucket Name Requirements](https://cloud.google.com/storage/docs/naming#requirements))."]
        pub bucket_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Root path to transfer objects. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should generally not begin with a '/'. (must meet Object Name Requirements](https://cloud.google.com/storage/docs/naming#objectnames))."]
        pub path: ::std::option::Option<::std::string::String>,
    }
    impl GcsData {
        pub fn builder() -> GcsDataBuilder {
            GcsDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Google service account"]
    pub struct GoogleServiceAccount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email address of the service account."]
        pub account_email: ::std::option::Option<::std::string::String>,
    }
    impl GoogleServiceAccount {
        pub fn builder() -> GoogleServiceAccountBuilder {
            GoogleServiceAccountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An HttpData resource specifies a list of objects on the web to be transferred over HTTP. The information of the objects to be transferred is contained in a file referenced by a URL. The first line in the file must be `\"TsvHttpData-1.0\"`, which specifies the format of the file. Subsequent lines specify the information of the list of objects, one object per list entry. Each entry has the following tab-delimited fields: * **HTTP URL** — The location of the object. * **Length** — The size of the object in bytes. * **MD5** — The base64-encoded MD5 hash of the object. For an example of a valid TSV file, see [Transferring data from URLs](https://cloud.google.com/storage-transfer/docs/create-url-list). When transferring data based on a URL list, keep the following in mind: * When an object located at `http(s)://hostname:port/` is transferred to a data sink, the name of the object at the data sink is `/`. * If the specified size of an object does not match the actual size of the object fetched, the object will not be transferred. * If the specified MD5 does not match the MD5 computed from the transferred bytes, the object transfer will fail. * Ensure that each URL you specify is publicly accessible. For example, in Cloud Storage you can [share an object publicly] (https://cloud.google.com/storage/docs/cloud-console#_sharingdata) and get a link to it. * Storage Transfer Service obeys `robots.txt` rules and requires the source HTTP server to support `Range` requests and to return a `Content-Length` header in each response. * ObjectConditions have no effect when filtering objects to transfer."]
    pub struct HttpData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "listUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The URL that points to the file that stores the object list entries. This file must allow public access. Currently, only URLs with HTTP and HTTPS schemes are supported."]
        pub list_url: ::std::option::Option<::std::string::String>,
    }
    impl HttpData {
        pub fn builder() -> HttpDataBuilder {
            HttpDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Operations.ListOperations."]
    pub struct ListOperationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of operations that matches the specified filter in the request."]
        pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
    }
    impl ListOperationsResponse {
        pub fn builder() -> ListOperationsResponseBuilder {
            ListOperationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response from ListTransferJobs."]
    pub struct ListTransferJobsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list next page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transferJobs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of transfer jobs."]
        pub transfer_jobs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TransferJob>>>,
    }
    impl ListTransferJobsResponse {
        pub fn builder() -> ListTransferJobsResponseBuilder {
            ListTransferJobsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specification to configure notifications published to Cloud Pub/Sub. Notifications will be published to the customer-provided topic using the following `PubsubMessage.attributes`: * `\"eventType\"`: one of the EventType values * `\"payloadFormat\"`: one of the PayloadFormat values * `\"projectId\"`: the project_id of the `TransferOperation` * `\"transferJobName\"`: the transfer_job_name of the `TransferOperation` * `\"transferOperationName\"`: the name of the `TransferOperation` The `PubsubMessage.data` will contain a TransferOperation resource formatted according to the specified `PayloadFormat`."]
    pub struct NotificationConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Event types for which a notification is desired. If empty, send notifications for all event types."]
        pub event_types: ::std::option::Option<::std::vec::Vec<NotificationConfigEventTypesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payloadFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The desired format of the notification message payloads."]
        pub payload_format: ::std::option::Option<NotificationConfigPayloadFormatEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pubsubTopic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The `Topic.name` of the Cloud Pub/Sub topic to which to publish notifications. Must be of the format: `projects/{project}/topics/{topic}`. Not matching this format will result in an INVALID_ARGUMENT error."]
        pub pubsub_topic: ::std::option::Option<::std::string::String>,
    }
    impl NotificationConfig {
        pub fn builder() -> NotificationConfigBuilder {
            NotificationConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum NotificationConfigEventTypesEnum {
        #[serde(rename = "EVENT_TYPE_UNSPECIFIED")]
        #[doc = "Illegal value, to avoid allowing a default."]
        EventTypeUnspecified,
        #[serde(rename = "TRANSFER_OPERATION_SUCCESS")]
        #[doc = "`TransferOperation` completed with status SUCCESS."]
        TransferOperationSuccess,
        #[serde(rename = "TRANSFER_OPERATION_FAILED")]
        #[doc = "`TransferOperation` completed with status FAILED."]
        TransferOperationFailed,
        #[serde(rename = "TRANSFER_OPERATION_ABORTED")]
        #[doc = "`TransferOperation` completed with status ABORTED."]
        TransferOperationAborted,
    }
    impl ::std::default::Default for NotificationConfigEventTypesEnum {
        fn default() -> Self {
            Self::EventTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The desired format of the notification message payloads."]
    pub enum NotificationConfigPayloadFormatEnum {
        #[serde(rename = "PAYLOAD_FORMAT_UNSPECIFIED")]
        #[doc = "Illegal value, to avoid allowing a default."]
        PayloadFormatUnspecified,
        #[serde(rename = "NONE")]
        #[doc = "No payload is included with the notification."]
        None,
        #[serde(rename = "JSON")]
        #[doc = "`TransferOperation` is [formatted as a JSON response](https://developers.google.com/protocol-buffers/docs/proto3#json), in application/json."]
        Json,
    }
    impl ::std::default::Default for NotificationConfigPayloadFormatEnum {
        fn default() -> Self {
            Self::PayloadFormatUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Conditions that determine which objects will be transferred. Applies only to Cloud Data Sources such as S3, Azure, and Cloud Storage. The \"last modification time\" refers to the time of the last change to the object's content or metadata — specifically, this is the `updated` property of Cloud Storage objects, the `LastModified` field of S3 objects, and the `Last-Modified` header of Azure blobs."]
    pub struct ObjectConditions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludePrefixes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If you specify `exclude_prefixes`, Storage Transfer Service uses the items in the `exclude_prefixes` array to determine which objects to exclude from a transfer. Objects must not start with one of the matching `exclude_prefixes` for inclusion in a transfer. The following are requirements of `exclude_prefixes`: * Each exclude-prefix can contain any sequence of Unicode characters, to a max length of 1024 bytes when UTF8-encoded, and must not contain Carriage Return or Line Feed characters. Wildcard matching and regular expression matching are not supported. * Each exclude-prefix must omit the leading slash. For example, to exclude the object `s3://my-aws-bucket/logs/y=2015/requests.gz`, specify the exclude-prefix as `logs/y=2015/requests.gz`. * None of the exclude-prefix values can be empty, if specified. * Each exclude-prefix must exclude a distinct portion of the object namespace. No exclude-prefix may be a prefix of another exclude-prefix. * If include_prefixes is specified, then each exclude-prefix must start with the value of a path explicitly included by `include_prefixes`. The max size of `exclude_prefixes` is 1000. For more information, see [Filtering objects from transfers](/storage-transfer/docs/filtering-objects-from-transfers)."]
        pub exclude_prefixes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includePrefixes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If you specify `include_prefixes`, Storage Transfer Service uses the items in the `include_prefixes` array to determine which objects to include in a transfer. Objects must start with one of the matching `include_prefixes` for inclusion in the transfer. If exclude_prefixes is specified, objects must not start with any of the `exclude_prefixes` specified for inclusion in the transfer. The following are requirements of `include_prefixes`: * Each include-prefix can contain any sequence of Unicode characters, to a max length of 1024 bytes when UTF8-encoded, and must not contain Carriage Return or Line Feed characters. Wildcard matching and regular expression matching are not supported. * Each include-prefix must omit the leading slash. For example, to include the object `s3://my-aws-bucket/logs/y=2015/requests.gz`, specify the include-prefix as `logs/y=2015/requests.gz`. * None of the include-prefix values can be empty, if specified. * Each include-prefix must include a distinct portion of the object namespace. No include-prefix may be a prefix of another include-prefix. The max size of `include_prefixes` is 1000. For more information, see [Filtering objects from transfers](/storage-transfer/docs/filtering-objects-from-transfers)."]
        pub include_prefixes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedBefore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, only objects with a \"last modification time\" before this timestamp and objects that don't have a \"last modification time\" will be transferred."]
        pub last_modified_before: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedSince")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, only objects with a \"last modification time\" on or after this timestamp and objects that don't have a \"last modification time\" are transferred. The `last_modified_since` and `last_modified_before` fields can be used together for chunked data processing. For example, consider a script that processes each day's worth of data at a time. For that you'd set each of the fields as follows: * `last_modified_since` to the start of the day * `last_modified_before` to the end of the day"]
        pub last_modified_since: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxTimeElapsedSinceLastModification")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, only objects with a \"last modification time\" on or after `NOW` - `max_time_elapsed_since_last_modification` and objects that don't have a \"last modification time\" are transferred. For each TransferOperation started by this TransferJob, `NOW` refers to the start_time of the `TransferOperation`."]
        pub max_time_elapsed_since_last_modification: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minTimeElapsedSinceLastModification")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, only objects with a \"last modification time\" before `NOW` - `min_time_elapsed_since_last_modification` and objects that don't have a \"last modification time\" are transferred. For each TransferOperation started by this TransferJob, `NOW` refers to the start_time of the `TransferOperation`."]
        pub min_time_elapsed_since_last_modification: ::std::option::Option<::std::string::String>,
    }
    impl ObjectConditions {
        pub fn builder() -> ObjectConditionsBuilder {
            ObjectConditionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This resource represents a long-running operation that is the result of a network API call."]
    pub struct Operation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "done")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
        pub done: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error result of the operation in case of failure or cancellation."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents the transfer operation object. To request a TransferOperation object, use transferOperations.get."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The server-assigned unique name. The format of `name` is `transferOperations/some/unique/name`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "response")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl Operation {
        pub fn builder() -> OperationBuilder {
            OperationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request passed to PauseTransferOperation."]
    pub struct PauseTransferOperationRequest {}
    impl PauseTransferOperationRequest {
        pub fn builder() -> PauseTransferOperationRequestBuilder {
            PauseTransferOperationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request passed to ResumeTransferOperation."]
    pub struct ResumeTransferOperationRequest {}
    impl ResumeTransferOperationRequest {
        pub fn builder() -> ResumeTransferOperationRequestBuilder {
            ResumeTransferOperationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request passed to RunTransferJob."]
    pub struct RunTransferJobRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The ID of the Google Cloud Platform Console project that owns the transfer job."]
        pub project_id: ::std::option::Option<::std::string::String>,
    }
    impl RunTransferJobRequest {
        pub fn builder() -> RunTransferJobRequestBuilder {
            RunTransferJobRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Transfers can be scheduled to recur or to run just once."]
    pub struct Schedule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTimeOfDay")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time in UTC that no further transfer operations are scheduled. Combined with schedule_end_date, `end_time_of_day` specifies the end date and time for starting new transfer operations. This field must be greater than or equal to the timestamp corresponding to the combintation of schedule_start_date and start_time_of_day, and is subject to the following: * If `end_time_of_day` is not set and `schedule_end_date` is set, then a default value of `23:59:59` is used for `end_time_of_day`. * If `end_time_of_day` is set and `schedule_end_date` is not set, then INVALID_ARGUMENT is returned."]
        pub end_time_of_day: ::std::option::Option<::std::boxed::Box<TimeOfDay>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "repeatInterval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Interval between the start of each scheduled TransferOperation. If unspecified, the default value is 24 hours. This value may not be less than 1 hour."]
        pub repeat_interval: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduleEndDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last day a transfer runs. Date boundaries are determined relative to UTC time. A job will run once per 24 hours within the following guidelines: * If `schedule_end_date` and schedule_start_date are the same and in the future relative to UTC, the transfer is executed only one time. * If `schedule_end_date` is later than `schedule_start_date` and `schedule_end_date` is in the future relative to UTC, the job will run each day at start_time_of_day through `schedule_end_date`."]
        pub schedule_end_date: ::std::option::Option<::std::boxed::Box<Date>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduleStartDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The start date of a transfer. Date boundaries are determined relative to UTC time. If `schedule_start_date` and start_time_of_day are in the past relative to the job's creation time, the transfer starts the day after you schedule the transfer request. **Note:** When starting jobs at or near midnight UTC it is possible that a job will start later than expected. For example, if you send an outbound request on June 1 one millisecond prior to midnight UTC and the Storage Transfer Service server receives the request on June 2, then it will create a TransferJob with `schedule_start_date` set to June 2 and a `start_time_of_day` set to midnight UTC. The first scheduled TransferOperation will take place on June 3 at midnight UTC."]
        pub schedule_start_date: ::std::option::Option<::std::boxed::Box<Date>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTimeOfDay")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time in UTC that a transfer job is scheduled to run. Transfers may start later than this time. If `start_time_of_day` is not specified: * One-time transfers run immediately. * Recurring transfers run immediately, and each day at midnight UTC, through schedule_end_date. If `start_time_of_day` is specified: * One-time transfers run at the specified time. * Recurring transfers run at the specified time each day, through `schedule_end_date`."]
        pub start_time_of_day: ::std::option::Option<::std::boxed::Box<TimeOfDay>>,
    }
    impl Schedule {
        pub fn builder() -> ScheduleBuilder {
            ScheduleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
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
    #[doc = "A collection of counters that report the progress of a transfer operation."]
    pub struct TransferCounters {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bytesCopiedToSink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bytes that are copied to the data sink."]
        pub bytes_copied_to_sink: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bytesDeletedFromSink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bytes that are deleted from the data sink."]
        pub bytes_deleted_from_sink: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bytesDeletedFromSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bytes that are deleted from the data source."]
        pub bytes_deleted_from_source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bytesFailedToDeleteFromSink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bytes that failed to be deleted from the data sink."]
        pub bytes_failed_to_delete_from_sink: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bytesFoundFromSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bytes found in the data source that are scheduled to be transferred, excluding any that are filtered based on object conditions or skipped due to sync."]
        pub bytes_found_from_source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bytesFoundOnlyFromSink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bytes found only in the data sink that are scheduled to be deleted."]
        pub bytes_found_only_from_sink: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bytesFromSourceFailed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bytes in the data source that failed to be transferred or that failed to be deleted after being transferred."]
        pub bytes_from_source_failed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bytesFromSourceSkippedBySync")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bytes in the data source that are not transferred because they already exist in the data sink."]
        pub bytes_from_source_skipped_by_sync: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectsCopiedToSink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Objects that are copied to the data sink."]
        pub objects_copied_to_sink: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectsDeletedFromSink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Objects that are deleted from the data sink."]
        pub objects_deleted_from_sink: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectsDeletedFromSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Objects that are deleted from the data source."]
        pub objects_deleted_from_source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectsFailedToDeleteFromSink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Objects that failed to be deleted from the data sink."]
        pub objects_failed_to_delete_from_sink: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectsFoundFromSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Objects found in the data source that are scheduled to be transferred, excluding any that are filtered based on object conditions or skipped due to sync."]
        pub objects_found_from_source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectsFoundOnlyFromSink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Objects found only in the data sink that are scheduled to be deleted."]
        pub objects_found_only_from_sink: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectsFromSourceFailed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Objects in the data source that failed to be transferred or that failed to be deleted after being transferred."]
        pub objects_from_source_failed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectsFromSourceSkippedBySync")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Objects in the data source that are not transferred because they already exist in the data sink."]
        pub objects_from_source_skipped_by_sync: ::std::option::Option<::std::string::String>,
    }
    impl TransferCounters {
        pub fn builder() -> TransferCountersBuilder {
            TransferCountersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This resource represents the configuration of a transfer job that runs periodically."]
    pub struct TransferJob {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time that the transfer job was created."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deletionTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time that the transfer job was deleted."]
        pub deletion_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description provided by the user for the job. Its max length is 1024 bytes when Unicode-encoded."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModificationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time that the transfer job was last modified."]
        pub last_modification_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latestOperationName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the most recently started TransferOperation of this JobConfig. Present if and only if at least one TransferOperation has been created for this JobConfig."]
        pub latest_operation_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique name (within the transfer project) assigned when the job is created. If this field is empty in a CreateTransferJobRequest, Storage Transfer Service will assign a unique name. Otherwise, the specified name is used as the unique name for this job. If the specified name is in use by a job, the creation request fails with an ALREADY_EXISTS error. This name must start with `\"transferJobs/\"` prefix and end with a letter or a number, and should be no more than 128 characters. Example: `\"transferJobs/[A-Za-z0-9-._~]*[A-Za-z0-9]$\"` Invalid job names will fail with an INVALID_ARGUMENT error."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notificationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Notification configuration."]
        pub notification_config: ::std::option::Option<::std::boxed::Box<NotificationConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the Google Cloud Platform Project that owns the job."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schedule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Schedule specification."]
        pub schedule: ::std::option::Option<::std::boxed::Box<Schedule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the job. This value MUST be specified for `CreateTransferJobRequests`. **Note:** The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation."]
        pub status: ::std::option::Option<TransferJobStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transferSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transfer specification."]
        pub transfer_spec: ::std::option::Option<::std::boxed::Box<TransferSpec>>,
    }
    impl TransferJob {
        pub fn builder() -> TransferJobBuilder {
            TransferJobBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Status of the job. This value MUST be specified for `CreateTransferJobRequests`. **Note:** The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation."]
    pub enum TransferJobStatusEnum {
        #[serde(rename = "STATUS_UNSPECIFIED")]
        #[doc = "Zero is an illegal value."]
        StatusUnspecified,
        #[serde(rename = "ENABLED")]
        #[doc = "New transfers will be performed based on the schedule."]
        Enabled,
        #[serde(rename = "DISABLED")]
        #[doc = "New transfers will not be scheduled."]
        Disabled,
        #[serde(rename = "DELETED")]
        #[doc = "This is a soft delete state. After a transfer job is set to this state, the job and all the transfer executions are subject to garbage collection. Transfer jobs become eligible for garbage collection 30 days after their status is set to `DELETED`."]
        Deleted,
    }
    impl ::std::default::Default for TransferJobStatusEnum {
        fn default() -> Self {
            Self::StatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A description of the execution of a transfer."]
    pub struct TransferOperation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "counters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the progress of the transfer operation."]
        pub counters: ::std::option::Option<::std::boxed::Box<TransferCounters>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End time of this transfer execution."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorBreakdowns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Summarizes errors encountered with sample error log entries."]
        pub error_breakdowns:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ErrorSummary>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A globally unique ID assigned by the system."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notificationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Notification configuration."]
        pub notification_config: ::std::option::Option<::std::boxed::Box<NotificationConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the Google Cloud Platform Project that owns the operation."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start time of this transfer execution."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the transfer operation."]
        pub status: ::std::option::Option<TransferOperationStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transferJobName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the transfer job that triggers this transfer operation."]
        pub transfer_job_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transferSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transfer specification."]
        pub transfer_spec: ::std::option::Option<::std::boxed::Box<TransferSpec>>,
    }
    impl TransferOperation {
        pub fn builder() -> TransferOperationBuilder {
            TransferOperationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Status of the transfer operation."]
    pub enum TransferOperationStatusEnum {
        #[serde(rename = "STATUS_UNSPECIFIED")]
        #[doc = "Zero is an illegal value."]
        StatusUnspecified,
        #[serde(rename = "IN_PROGRESS")]
        #[doc = "In progress."]
        InProgress,
        #[serde(rename = "PAUSED")]
        #[doc = "Paused."]
        Paused,
        #[serde(rename = "SUCCESS")]
        #[doc = "Completed successfully."]
        Success,
        #[serde(rename = "FAILED")]
        #[doc = "Terminated due to an unrecoverable failure."]
        Failed,
        #[serde(rename = "ABORTED")]
        #[doc = "Aborted by the user."]
        Aborted,
        #[serde(rename = "QUEUED")]
        #[doc = "Temporarily delayed by the system. No user action is required."]
        Queued,
    }
    impl ::std::default::Default for TransferOperationStatusEnum {
        fn default() -> Self {
            Self::StatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "TransferOptions define the actions to be performed on objects in a transfer."]
    pub struct TransferOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteObjectsFromSourceAfterTransfer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether objects should be deleted from the source after they are transferred to the sink. **Note:** This option and delete_objects_unique_in_sink are mutually exclusive."]
        pub delete_objects_from_source_after_transfer:
            ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteObjectsUniqueInSink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether objects that exist only in the sink should be deleted. **Note:** This option and delete_objects_from_source_after_transfer are mutually exclusive."]
        pub delete_objects_unique_in_sink: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "overwriteObjectsAlreadyExistingInSink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When to overwrite objects that already exist in the sink. The default is that only objects that are different from the source are ovewritten. If true, all objects in the sink whose name matches an object in the source will be overwritten with the source object."]
        pub overwrite_objects_already_existing_in_sink:
            ::std::option::Option<::std::primitive::bool>,
    }
    impl TransferOptions {
        pub fn builder() -> TransferOptionsBuilder {
            TransferOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for running a transfer."]
    pub struct TransferSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "awsS3DataSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An AWS S3 data source."]
        pub aws_s3_data_source: ::std::option::Option<::std::boxed::Box<AwsS3Data>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "azureBlobStorageDataSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An Azure Blob Storage data source."]
        pub azure_blob_storage_data_source:
            ::std::option::Option<::std::boxed::Box<AzureBlobStorageData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsDataSink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Cloud Storage data sink."]
        pub gcs_data_sink: ::std::option::Option<::std::boxed::Box<GcsData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsDataSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Cloud Storage data source."]
        pub gcs_data_source: ::std::option::Option<::std::boxed::Box<GcsData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpDataSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An HTTP URL data source."]
        pub http_data_source: ::std::option::Option<::std::boxed::Box<HttpData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectConditions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only objects that satisfy these object conditions are included in the set of data source and data sink objects. Object conditions based on objects' \"last modification time\" do not exclude objects in a data sink."]
        pub object_conditions: ::std::option::Option<::std::boxed::Box<ObjectConditions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transferOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the option delete_objects_unique_in_sink is `true` and time-based object conditions such as 'last modification time' are specified, the request fails with an INVALID_ARGUMENT error."]
        pub transfer_options: ::std::option::Option<::std::boxed::Box<TransferOptions>>,
    }
    impl TransferSpec {
        pub fn builder() -> TransferSpecBuilder {
            TransferSpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request passed to UpdateTransferJob."]
    pub struct UpdateTransferJobRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The ID of the Google Cloud Platform Console project that owns the job."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transferJob")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The job to update. `transferJob` is expected to specify only four fields: description, transfer_spec, notification_config, and status. An `UpdateTransferJobRequest` that specifies other fields will be rejected with the error INVALID_ARGUMENT. Updating a job satus to DELETED requires `storagetransfer.jobs.delete` permissions."]
        pub transfer_job: ::std::option::Option<::std::boxed::Box<TransferJob>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTransferJobFieldMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The field mask of the fields in `transferJob` that are to be updated in this request. Fields in `transferJob` that can be updated are: description, transfer_spec, notification_config, and status. To update the `transfer_spec` of the job, a complete transfer specification must be provided. An incomplete specification missing any required fields will be rejected with the error INVALID_ARGUMENT."]
        pub update_transfer_job_field_mask: ::std::option::Option<::std::string::String>,
    }
    impl UpdateTransferJobRequest {
        pub fn builder() -> UpdateTransferJobRequestBuilder {
            UpdateTransferJobRequestBuilder::default()
        }
    }
}
