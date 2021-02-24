#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "gdata"]
pub struct GdataBlobstore2Info {
    #[serde(rename = "blobGeneration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub blob_generation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "blobId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub blob_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "downloadReadHandle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub download_read_handle: ::std::option::Option<::std::string::String>,
    #[serde(rename = "readToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub read_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uploadMetadataContainer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub upload_metadata_container: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "gdata"]
pub struct GdataCompositeMedia {
    #[serde(rename = "blobRef")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub blob_ref: ::std::option::Option<::std::string::String>,
    #[serde(rename = "blobstore2Info")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub blobstore2_info: ::std::option::Option<::std::boxed::Box<GdataBlobstore2Info>>,
    #[serde(rename = "cosmoBinaryReference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub cosmo_binary_reference: ::std::option::Option<::std::string::String>,
    #[serde(rename = "crc32cHash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub crc32c_hash: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "inline")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub inline: ::std::option::Option<::std::string::String>,
    #[serde(rename = "length")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub length: ::std::option::Option<::std::string::String>,
    #[serde(rename = "md5Hash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub md5_hash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub object_id: ::std::option::Option<::std::boxed::Box<GdataObjectId>>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "referenceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub reference_type: ::std::option::Option<GdataCompositeMediaReferenceTypeEnum>,
    #[serde(rename = "sha1Hash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub sha1_hash: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "gdata"]
pub enum GdataCompositeMediaReferenceTypeEnum {
    #[serde(rename = "PATH")]
    #[doc = "gdata"]
    Path,
    #[serde(rename = "BLOB_REF")]
    #[doc = "gdata"]
    BlobRef,
    #[serde(rename = "INLINE")]
    #[doc = "gdata"]
    Inline,
    #[serde(rename = "BIGSTORE_REF")]
    #[doc = "gdata"]
    BigstoreRef,
    #[serde(rename = "COSMO_BINARY_REFERENCE")]
    #[doc = "gdata"]
    CosmoBinaryReference,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "gdata"]
pub struct GdataContentTypeInfo {
    #[serde(rename = "bestGuess")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub best_guess: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fromBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub from_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fromFileName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub from_file_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fromHeader")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub from_header: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fromUrlPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub from_url_path: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "gdata"]
pub struct GdataDiffChecksumsResponse {
    #[serde(rename = "checksumsLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub checksums_location: ::std::option::Option<::std::boxed::Box<GdataCompositeMedia>>,
    #[serde(rename = "chunkSizeBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub chunk_size_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub object_location: ::std::option::Option<::std::boxed::Box<GdataCompositeMedia>>,
    #[serde(rename = "objectSizeBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub object_size_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub object_version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "gdata"]
pub struct GdataDiffDownloadResponse {
    #[serde(rename = "objectLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub object_location: ::std::option::Option<::std::boxed::Box<GdataCompositeMedia>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "gdata"]
pub struct GdataDiffUploadRequest {
    #[serde(rename = "checksumsInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub checksums_info: ::std::option::Option<::std::boxed::Box<GdataCompositeMedia>>,
    #[serde(rename = "objectInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub object_info: ::std::option::Option<::std::boxed::Box<GdataCompositeMedia>>,
    #[serde(rename = "objectVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub object_version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "gdata"]
pub struct GdataDiffUploadResponse {
    #[serde(rename = "objectVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub object_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "originalObject")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub original_object: ::std::option::Option<::std::boxed::Box<GdataCompositeMedia>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "gdata"]
pub struct GdataDiffVersionResponse {
    #[serde(rename = "objectSizeBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub object_size_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub object_version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "gdata"]
pub struct GdataDownloadParameters {
    #[serde(rename = "allowGzipCompression")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub allow_gzip_compression: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "ignoreRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub ignore_range: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "gdata"]
pub struct GdataMedia {
    #[serde(rename = "algorithm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub algorithm: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bigstoreObjectRef")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub bigstore_object_ref: ::std::option::Option<::std::string::String>,
    #[serde(rename = "blobRef")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub blob_ref: ::std::option::Option<::std::string::String>,
    #[serde(rename = "blobstore2Info")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub blobstore2_info: ::std::option::Option<::std::boxed::Box<GdataBlobstore2Info>>,
    #[serde(rename = "compositeMedia")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub composite_media:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GdataCompositeMedia>>>,
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub content_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contentTypeInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub content_type_info: ::std::option::Option<::std::boxed::Box<GdataContentTypeInfo>>,
    #[serde(rename = "cosmoBinaryReference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub cosmo_binary_reference: ::std::option::Option<::std::string::String>,
    #[serde(rename = "crc32cHash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub crc32c_hash: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "diffChecksumsResponse")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub diff_checksums_response:
        ::std::option::Option<::std::boxed::Box<GdataDiffChecksumsResponse>>,
    #[serde(rename = "diffDownloadResponse")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub diff_download_response: ::std::option::Option<::std::boxed::Box<GdataDiffDownloadResponse>>,
    #[serde(rename = "diffUploadRequest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub diff_upload_request: ::std::option::Option<::std::boxed::Box<GdataDiffUploadRequest>>,
    #[serde(rename = "diffUploadResponse")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub diff_upload_response: ::std::option::Option<::std::boxed::Box<GdataDiffUploadResponse>>,
    #[serde(rename = "diffVersionResponse")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub diff_version_response: ::std::option::Option<::std::boxed::Box<GdataDiffVersionResponse>>,
    #[serde(rename = "downloadParameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub download_parameters: ::std::option::Option<::std::boxed::Box<GdataDownloadParameters>>,
    #[serde(rename = "filename")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub filename: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub hash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hashVerified")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub hash_verified: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "inline")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub inline: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isPotentialRetry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub is_potential_retry: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "length")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub length: ::std::option::Option<::std::string::String>,
    #[serde(rename = "md5Hash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub md5_hash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mediaId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub media_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub object_id: ::std::option::Option<::std::boxed::Box<GdataObjectId>>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "referenceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub reference_type: ::std::option::Option<GdataMediaReferenceTypeEnum>,
    #[serde(rename = "sha1Hash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub sha1_hash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sha256Hash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub sha256_hash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "gdata"]
pub enum GdataMediaReferenceTypeEnum {
    #[serde(rename = "PATH")]
    #[doc = "gdata"]
    Path,
    #[serde(rename = "BLOB_REF")]
    #[doc = "gdata"]
    BlobRef,
    #[serde(rename = "INLINE")]
    #[doc = "gdata"]
    Inline,
    #[serde(rename = "GET_MEDIA")]
    #[doc = "gdata"]
    GetMedia,
    #[serde(rename = "COMPOSITE_MEDIA")]
    #[doc = "gdata"]
    CompositeMedia,
    #[serde(rename = "BIGSTORE_REF")]
    #[doc = "gdata"]
    BigstoreRef,
    #[serde(rename = "DIFF_VERSION_RESPONSE")]
    #[doc = "gdata"]
    DiffVersionResponse,
    #[serde(rename = "DIFF_CHECKSUMS_RESPONSE")]
    #[doc = "gdata"]
    DiffChecksumsResponse,
    #[serde(rename = "DIFF_DOWNLOAD_RESPONSE")]
    #[doc = "gdata"]
    DiffDownloadResponse,
    #[serde(rename = "DIFF_UPLOAD_REQUEST")]
    #[doc = "gdata"]
    DiffUploadRequest,
    #[serde(rename = "DIFF_UPLOAD_RESPONSE")]
    #[doc = "gdata"]
    DiffUploadResponse,
    #[serde(rename = "COSMO_BINARY_REFERENCE")]
    #[doc = "gdata"]
    CosmoBinaryReference,
    #[serde(rename = "ARBITRARY_BYTES")]
    #[doc = "gdata"]
    ArbitraryBytes,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "gdata"]
pub struct GdataObjectId {
    #[serde(rename = "bucketName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub bucket_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "generation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub generation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "gdata"]
    pub object_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A job creating reports of a specific type."]
pub struct Job {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creation date/time of the job."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date/time when this job will expire/expired. After a job expired, no new reports are generated."]
    pub expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The server-generated ID of the job (max. 40 characters)."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the job (max. 100 characters)."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reportTypeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of reports this job creates. Corresponds to the ID of a ReportType."]
    pub report_type_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "systemManaged")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if this a system-managed job that cannot be modified by the user; otherwise false."]
    pub system_managed: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ReportingService.ListJobs."]
pub struct ListJobsResponse {
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of jobs."]
    pub jobs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Job>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve next page of results. Pass this value in the ListJobsRequest.page_token field in the subsequent call to `ListJobs` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ReportingService.ListReportTypes."]
pub struct ListReportTypesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve next page of results. Pass this value in the ListReportTypesRequest.page_token field in the subsequent call to `ListReportTypes` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reportTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of report types."]
    pub report_types: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReportType>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ReportingService.ListReports."]
pub struct ListReportsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve next page of results. Pass this value in the ListReportsRequest.page_token field in the subsequent call to `ListReports` method to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reports")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of report types."]
    pub reports: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Report>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A report's metadata including the URL from which the report itself can be downloaded."]
pub struct Report {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date/time when this report was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "downloadUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL from which the report can be downloaded (max. 1000 characters)."]
    pub download_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The end of the time period that the report instance covers. The value is exclusive."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The server-generated ID of the report."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "jobExpireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date/time when the job this report belongs to will expire/expired."]
    pub job_expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the job that created this report."]
    pub job_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The start of the time period that the report instance covers. The value is inclusive."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A report type."]
pub struct ReportType {
    #[serde(rename = "deprecateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date/time when this report type was/will be deprecated."]
    pub deprecate_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the report type (max. 100 characters)."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the report type (max. 100 characters)."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "systemManaged")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if this a system-managed report type; otherwise false. Reporting jobs for system-managed report types are created automatically and can thus not be used in the `CreateJob` method."]
    pub system_managed: ::std::option::Option<::std::primitive::bool>,
}
