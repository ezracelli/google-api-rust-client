#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Count number for each account."]
pub struct AccountCount {
    #[serde(rename = "account")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account owner."]
    pub account: ::std::option::Option<::std::boxed::Box<UserInfo>>,
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of artifacts found for this account."]
    pub count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An error that occurred when querying a specific account"]
pub struct AccountCountError {
    #[serde(rename = "account")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account owner."]
    pub account: ::std::option::Option<::std::boxed::Box<UserInfo>>,
    #[serde(rename = "errorType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account query error."]
    pub error_type: ::std::option::Option<AccountCountErrorErrorTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Account query error."]
pub enum AccountCountErrorErrorTypeEnum {
    #[serde(rename = "ERROR_TYPE_UNSPECIFIED")]
    #[doc = "Default."]
    ErrorTypeUnspecified,
    #[serde(rename = "WILDCARD_TOO_BROAD")]
    #[doc = "Permanent - prefix terms expanded to too many query terms."]
    WildcardTooBroad,
    #[serde(rename = "TOO_MANY_TERMS")]
    #[doc = "Permanent - query contains too many terms."]
    TooManyTerms,
    #[serde(rename = "LOCATION_UNAVAILABLE")]
    #[doc = "Transient - data in transit between storage replicas, temporarily unavailable."]
    LocationUnavailable,
    #[serde(rename = "UNKNOWN")]
    #[doc = "Unrecognized error."]
    Unknown,
    #[serde(rename = "DEADLINE_EXCEEDED")]
    #[doc = "Deadline exceeded when querying the account."]
    DeadlineExceeded,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Accounts to search"]
pub struct AccountInfo {
    #[serde(rename = "emails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A set of accounts to search."]
    pub emails: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A status detailing the status of each account creation, and the HeldAccount, if successful."]
pub struct AddHeldAccountResult {
    #[serde(rename = "account")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If present, this account was successfully created."]
    pub account: ::std::option::Option<::std::boxed::Box<HeldAccount>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This represents the success status. If failed, check message."]
    pub status: ::std::option::Option<::std::boxed::Box<Status>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Add a list of accounts to a hold."]
pub struct AddHeldAccountsRequest {
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account IDs to identify which accounts to add. Only account_ids or only emails should be specified, but not both."]
    pub account_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "emails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Emails to identify which accounts to add. Only emails or only account_ids should be specified, but not both."]
    pub emails: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for batch create held accounts."]
pub struct AddHeldAccountsResponse {
    #[serde(rename = "responses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of responses, in the same order as the batch request."]
    pub responses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AddHeldAccountResult>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Add an account with the permission specified. The role cannot be owner. If an account already has a role in the matter, it will be overwritten."]
pub struct AddMatterPermissionsRequest {
    #[serde(rename = "ccMe")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only relevant if send_emails is true. True to CC requestor in the email message. False to not CC requestor."]
    pub cc_me: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "matterPermission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MatterPermission to add."]
    pub matter_permission: ::std::option::Option<::std::boxed::Box<MatterPermission>>,
    #[serde(rename = "sendEmails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True to send notification email to the added account. False to not send notification email."]
    pub send_emails: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for Operations.CancelOperation."]
pub struct CancelOperationRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Close a matter by ID."]
pub struct CloseMatterRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response to a CloseMatterRequest."]
pub struct CloseMatterResponse {
    #[serde(rename = "matter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The updated matter, with state CLOSED."]
    pub matter: ::std::option::Option<::std::boxed::Box<Matter>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An export file on cloud storage"]
pub struct CloudStorageFile {
    #[serde(rename = "bucketName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cloud storage bucket name of this export file. Can be used in cloud storage JSON/XML API, but not to list the bucket contents. Instead, you can get individual export files by object name."]
    pub bucket_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "md5Hash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The md5 hash of the file."]
    pub md5_hash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "objectName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cloud storage object name of this export file. Can be used in cloud storage JSON/XML API."]
    pub object_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size of the export file."]
    pub size: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Export sink for cloud storage files."]
pub struct CloudStorageSink {
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The exported files on cloud storage."]
    pub files: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CloudStorageFile>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Corpus specific queries."]
pub struct CorpusQuery {
    #[serde(rename = "driveQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details pertaining to Drive holds. If set, corpus must be Drive."]
    pub drive_query: ::std::option::Option<::std::boxed::Box<HeldDriveQuery>>,
    #[serde(rename = "groupsQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details pertaining to Groups holds. If set, corpus must be Groups."]
    pub groups_query: ::std::option::Option<::std::boxed::Box<HeldGroupsQuery>>,
    #[serde(rename = "hangoutsChatQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details pertaining to Hangouts Chat holds. If set, corpus must be Hangouts Chat."]
    pub hangouts_chat_query: ::std::option::Option<::std::boxed::Box<HeldHangoutsChatQuery>>,
    #[serde(rename = "mailQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details pertaining to mail holds. If set, corpus must be mail."]
    pub mail_query: ::std::option::Option<::std::boxed::Box<HeldMailQuery>>,
    #[serde(rename = "voiceQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details pertaining to Voice holds. If set, corpus must be Voice."]
    pub voice_query: ::std::option::Option<::std::boxed::Box<HeldVoiceQuery>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Long running operation metadata for CountArtifacts."]
pub struct CountArtifactsMetadata {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "End time of count operation. Available when operation is done."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "matterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The matter ID of the associated matter."]
    pub matter_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The search query from the request."]
    pub query: ::std::option::Option<::std::boxed::Box<Query>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creation time of count operation."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Count artifacts request."]
pub struct CountArtifactsRequest {
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The search query."]
    pub query: ::std::option::Option<::std::boxed::Box<Query>>,
    #[serde(rename = "view")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the granularity of the count result returned in response."]
    pub view: ::std::option::Option<CountArtifactsRequestViewEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Specifies the granularity of the count result returned in response."]
pub enum CountArtifactsRequestViewEnum {
    #[serde(rename = "COUNT_RESULT_VIEW_UNSPECIFIED")]
    #[doc = "Default. It works the same as TOTAL_COUNT."]
    CountResultViewUnspecified,
    #[serde(rename = "TOTAL_COUNT")]
    #[doc = "Response includes: total count, queried accounts count, matching accounts count, non-queryable accounts, queried account errors."]
    TotalCount,
    #[serde(rename = "ALL")]
    #[doc = "Response includes additional breakdown of account count."]
    All,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Definition of the response for method CountArtifacts."]
pub struct CountArtifactsResponse {
    #[serde(rename = "groupsCountResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Count metrics of Groups."]
    pub groups_count_result: ::std::option::Option<::std::boxed::Box<GroupsCountResult>>,
    #[serde(rename = "mailCountResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Count metrics of Mail."]
    pub mail_count_result: ::std::option::Option<::std::boxed::Box<MailCountResult>>,
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total count of artifacts. For mail and groups, artifacts refers to messages."]
    pub total_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The options for Drive export."]
pub struct DriveExportOptions {
    #[serde(rename = "includeAccessInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set to true to include access level information for users with indirect access to files."]
    pub include_access_info: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Drive search advanced options"]
pub struct DriveOptions {
    #[serde(rename = "includeSharedDrives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set to true to include shared drive."]
    pub include_shared_drives: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "includeTeamDrives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set to true to include Team Drive."]
    pub include_team_drives: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "versionDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Search the versions of the Drive file as of the reference date. These timestamps are in GMT and rounded down to the given date."]
    pub version_date: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An export"]
pub struct Export {
    #[serde(rename = "cloudStorageSink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Export sink for cloud storage files."]
    pub cloud_storage_sink: ::std::option::Option<::std::boxed::Box<CloudStorageSink>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the export was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "exportOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advanced options of the export."]
    pub export_options: ::std::option::Option<::std::boxed::Box<ExportOptions>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The generated export ID."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "matterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The matter ID."]
    pub matter_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The export name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The search query being exported."]
    pub query: ::std::option::Option<::std::boxed::Box<Query>>,
    #[serde(rename = "requester")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The requester of the export."]
    pub requester: ::std::option::Option<::std::boxed::Box<UserInfo>>,
    #[serde(rename = "stats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Export statistics."]
    pub stats: ::std::option::Option<::std::boxed::Box<ExportStats>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The export status."]
    pub status: ::std::option::Option<ExportStatusEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The export status."]
pub enum ExportStatusEnum {
    #[serde(rename = "EXPORT_STATUS_UNSPECIFIED")]
    #[doc = "The status is unspecified."]
    ExportStatusUnspecified,
    #[serde(rename = "COMPLETED")]
    #[doc = "The export completed."]
    Completed,
    #[serde(rename = "FAILED")]
    #[doc = "The export failed."]
    Failed,
    #[serde(rename = "IN_PROGRESS")]
    #[doc = "The export is still being executed."]
    InProgress,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Export advanced options"]
pub struct ExportOptions {
    #[serde(rename = "driveOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Option available for Drive export."]
    pub drive_options: ::std::option::Option<::std::boxed::Box<DriveExportOptions>>,
    #[serde(rename = "groupsOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Option available for groups export."]
    pub groups_options: ::std::option::Option<::std::boxed::Box<GroupsExportOptions>>,
    #[serde(rename = "hangoutsChatOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Option available for hangouts chat export."]
    pub hangouts_chat_options: ::std::option::Option<::std::boxed::Box<HangoutsChatExportOptions>>,
    #[serde(rename = "mailOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Option available for mail export."]
    pub mail_options: ::std::option::Option<::std::boxed::Box<MailExportOptions>>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The requested export location."]
    pub region: ::std::option::Option<ExportOptionsRegionEnum>,
    #[serde(rename = "voiceOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Option available for voice export."]
    pub voice_options: ::std::option::Option<::std::boxed::Box<VoiceExportOptions>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The requested export location."]
pub enum ExportOptionsRegionEnum {
    #[serde(rename = "EXPORT_REGION_UNSPECIFIED")]
    #[doc = "The region is unspecified. Will be treated the same as ANY."]
    ExportRegionUnspecified,
    #[serde(rename = "ANY")]
    #[doc = "Any region."]
    Any,
    #[serde(rename = "US")]
    #[doc = "US region."]
    Us,
    #[serde(rename = "EUROPE")]
    #[doc = "Europe region."]
    Europe,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Stats of an export."]
pub struct ExportStats {
    #[serde(rename = "exportedArtifactCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of documents already processed by the export."]
    pub exported_artifact_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sizeInBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size of export in bytes."]
    pub size_in_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalArtifactCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of documents to be exported."]
    pub total_artifact_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Groups specific count metrics."]
pub struct GroupsCountResult {
    #[serde(rename = "accountCountErrors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error occurred when querying these accounts."]
    pub account_count_errors:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountCountError>>>,
    #[serde(rename = "accountCounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subtotal count per matching account that have more than zero messages."]
    pub account_counts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountCount>>>,
    #[serde(rename = "matchingAccountsCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of accounts that can be queried and have more than zero messages."]
    pub matching_accounts_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nonQueryableAccounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When data scope is HELD_DATA in the request Query, these accounts in the request are not queried because they are not on hold. For other data scope, this field is not set."]
    pub non_queryable_accounts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "queriedAccountsCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of accounts involved in this count operation."]
    pub queried_accounts_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The options for groups export."]
pub struct GroupsExportOptions {
    #[serde(rename = "exportFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The export format for groups export."]
    pub export_format: ::std::option::Option<GroupsExportOptionsExportFormatEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The export format for groups export."]
pub enum GroupsExportOptionsExportFormatEnum {
    #[serde(rename = "EXPORT_FORMAT_UNSPECIFIED")]
    #[doc = "No export format specified."]
    ExportFormatUnspecified,
    #[serde(rename = "MBOX")]
    #[doc = "MBOX as export format."]
    Mbox,
    #[serde(rename = "PST")]
    #[doc = "PST as export format"]
    Pst,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The options for hangouts chat export."]
pub struct HangoutsChatExportOptions {
    #[serde(rename = "exportFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The export format for hangouts chat export."]
    pub export_format: ::std::option::Option<HangoutsChatExportOptionsExportFormatEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The export format for hangouts chat export."]
pub enum HangoutsChatExportOptionsExportFormatEnum {
    #[serde(rename = "EXPORT_FORMAT_UNSPECIFIED")]
    #[doc = "No export format specified."]
    ExportFormatUnspecified,
    #[serde(rename = "MBOX")]
    #[doc = "MBOX as export format."]
    Mbox,
    #[serde(rename = "PST")]
    #[doc = "PST as export format"]
    Pst,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Accounts to search"]
pub struct HangoutsChatInfo {
    #[serde(rename = "roomId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A set of rooms to search."]
    pub room_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Hangouts chat search advanced options"]
pub struct HangoutsChatOptions {
    #[serde(rename = "includeRooms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set to true to include rooms."]
    pub include_rooms: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An account being held in a particular hold. This structure is immutable. This can be either a single user or a google group, depending on the corpus."]
pub struct HeldAccount {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The account's ID as provided by the Admin SDK."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The primary email address of the account. If used as an input, this takes precedence over account ID."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The first name of the account holder."]
    pub first_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "holdTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. When the account was put on hold."]
    pub hold_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The last name of the account holder."]
    pub last_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Query options for Drive holds."]
pub struct HeldDriveQuery {
    #[serde(rename = "includeSharedDriveFiles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, include files in shared drives in the hold."]
    pub include_shared_drive_files: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "includeTeamDriveFiles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, include files in Team Drives in the hold."]
    pub include_team_drive_files: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Query options for group holds."]
pub struct HeldGroupsQuery {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The end time range for the search query. These timestamps are in GMT and rounded down to the start of the given date."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The start time range for the search query. These timestamps are in GMT and rounded down to the start of the given date."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "terms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The search terms for the hold."]
    pub terms: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Query options for hangouts chat holds."]
pub struct HeldHangoutsChatQuery {
    #[serde(rename = "includeRooms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, include rooms the user has participated in."]
    pub include_rooms: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Query options for mail holds."]
pub struct HeldMailQuery {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The end time range for the search query. These timestamps are in GMT and rounded down to the start of the given date."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The start time range for the search query. These timestamps are in GMT and rounded down to the start of the given date."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "terms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The search terms for the hold."]
    pub terms: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A organizational unit being held in a particular hold. This structure is immutable."]
pub struct HeldOrgUnit {
    #[serde(rename = "holdTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When the org unit was put on hold. This property is immutable."]
    pub hold_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orgUnitId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The org unit's immutable ID as provided by the Admin SDK."]
    pub org_unit_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Query options for Voice holds."]
pub struct HeldVoiceQuery {
    #[serde(rename = "coveredData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Data covered by this rule. Should be non-empty. Order does not matter and duplicates will be ignored."]
    pub covered_data: ::std::option::Option<::std::vec::Vec<HeldVoiceQueryCoveredDataEnum>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum HeldVoiceQueryCoveredDataEnum {
    #[serde(rename = "COVERED_DATA_UNSPECIFIED")]
    #[doc = "Covered data unspecified."]
    CoveredDataUnspecified,
    #[serde(rename = "TEXT_MESSAGES")]
    #[doc = "Voice text message will be covered."]
    TextMessages,
    #[serde(rename = "VOICEMAILS")]
    #[doc = "Voicemail will be covered."]
    Voicemails,
    #[serde(rename = "CALL_LOGS")]
    #[doc = "Call logs will be covered."]
    CallLogs,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a hold within Vault. A hold restricts purging of artifacts based on the combination of the query and accounts restrictions. A hold can be configured to either apply to an explicitly configured set of accounts, or can be applied to all members of an organizational unit."]
pub struct Hold {
    #[serde(rename = "accounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, the hold applies to the enumerated accounts and org_unit must be empty."]
    pub accounts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HeldAccount>>>,
    #[serde(rename = "corpus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The corpus to be searched."]
    pub corpus: ::std::option::Option<HoldCorpusEnum>,
    #[serde(rename = "holdId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique immutable ID of the hold. Assigned during creation."]
    pub hold_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the hold."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orgUnit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, the hold applies to all members of the organizational unit and accounts must be empty. This property is mutable. For groups holds, set the accounts field."]
    pub org_unit: ::std::option::Option<::std::boxed::Box<HeldOrgUnit>>,
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The corpus-specific query. If set, the corpusQuery must match corpus type."]
    pub query: ::std::option::Option<::std::boxed::Box<CorpusQuery>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last time this hold was modified."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The corpus to be searched."]
pub enum HoldCorpusEnum {
    #[serde(rename = "CORPUS_TYPE_UNSPECIFIED")]
    #[doc = "No corpus specified."]
    CorpusTypeUnspecified,
    #[serde(rename = "DRIVE")]
    #[doc = "Drive."]
    Drive,
    #[serde(rename = "MAIL")]
    #[doc = "Mail."]
    Mail,
    #[serde(rename = "GROUPS")]
    #[doc = "Groups."]
    Groups,
    #[serde(rename = "HANGOUTS_CHAT")]
    #[doc = "Hangouts Chat."]
    HangoutsChat,
    #[serde(rename = "VOICE")]
    #[doc = "Google Voice."]
    Voice,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The holds for a matter."]
pub struct ListExportsResponse {
    #[serde(rename = "exports")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of exports."]
    pub exports: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Export>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Page token to retrieve the next page of results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Returns a list of held accounts for a hold."]
pub struct ListHeldAccountsResponse {
    #[serde(rename = "accounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The held accounts on a hold."]
    pub accounts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HeldAccount>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The holds for a matter."]
pub struct ListHoldsResponse {
    #[serde(rename = "holds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of holds."]
    pub holds: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Hold>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Page token to retrieve the next page of results in the list. If this is empty, then there are no more holds to list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Provides the list of matters."]
pub struct ListMattersResponse {
    #[serde(rename = "matters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of matters."]
    pub matters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Matter>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Page token to retrieve the next page of results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Operations.ListOperations."]
pub struct ListOperationsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The standard List next-page token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of operations that matches the specified filter in the request."]
    pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Definition of the response for method ListSaveQuery."]
pub struct ListSavedQueriesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Page token to retrieve the next page of results in the list. If this is empty, then there are no more saved queries to list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "savedQueries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of output saved queries."]
    pub saved_queries: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SavedQuery>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Mail specific count metrics."]
pub struct MailCountResult {
    #[serde(rename = "accountCountErrors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error occurred when querying these accounts."]
    pub account_count_errors:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountCountError>>>,
    #[serde(rename = "accountCounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subtotal count per matching account that have more than zero messages."]
    pub account_counts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountCount>>>,
    #[serde(rename = "matchingAccountsCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of accounts that can be queried and have more than zero messages."]
    pub matching_accounts_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nonQueryableAccounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When data scope is HELD_DATA in the request Query, these accounts in the request are not queried because they are not on hold. For other data scope, this field is not set."]
    pub non_queryable_accounts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "queriedAccountsCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of accounts involved in this count operation."]
    pub queried_accounts_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The options for mail export."]
pub struct MailExportOptions {
    #[serde(rename = "exportFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The export file format."]
    pub export_format: ::std::option::Option<MailExportOptionsExportFormatEnum>,
    #[serde(rename = "showConfidentialModeContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set to true to export confidential mode content."]
    pub show_confidential_mode_content: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The export file format."]
pub enum MailExportOptionsExportFormatEnum {
    #[serde(rename = "EXPORT_FORMAT_UNSPECIFIED")]
    #[doc = "No export format specified."]
    ExportFormatUnspecified,
    #[serde(rename = "MBOX")]
    #[doc = "MBOX as export format."]
    Mbox,
    #[serde(rename = "PST")]
    #[doc = "PST as export format"]
    Pst,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Mail search advanced options"]
pub struct MailOptions {
    #[serde(rename = "excludeDrafts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set to true to exclude drafts."]
    pub exclude_drafts: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a matter."]
pub struct Matter {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of the matter."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "matterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The matter ID which is generated by the server. Should be blank when creating a new matter."]
    pub matter_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "matterPermissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of users and access to the matter. Currently there is no programmer defined limit on the number of permissions a matter can have."]
    pub matter_permissions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MatterPermission>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the matter."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of the matter."]
    pub state: ::std::option::Option<MatterStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of the matter."]
pub enum MatterStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The matter has no specified state."]
    StateUnspecified,
    #[serde(rename = "OPEN")]
    #[doc = "This matter is open."]
    Open,
    #[serde(rename = "CLOSED")]
    #[doc = "This matter is closed."]
    Closed,
    #[serde(rename = "DELETED")]
    #[doc = "This matter is deleted."]
    Deleted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Currently each matter only has one owner, and all others are collaborators. When an account is purged, its corresponding MatterPermission resources cease to exist."]
pub struct MatterPermission {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The account ID, as provided by Admin SDK."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's role in this matter."]
    pub role: ::std::option::Option<MatterPermissionRoleEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The user's role in this matter."]
pub enum MatterPermissionRoleEnum {
    #[serde(rename = "ROLE_UNSPECIFIED")]
    #[doc = "No role assigned."]
    RoleUnspecified,
    #[serde(rename = "COLLABORATOR")]
    #[doc = "A collaborator to the matter."]
    Collaborator,
    #[serde(rename = "OWNER")]
    #[doc = "The owner of the matter."]
    Owner,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This resource represents a long-running operation that is the result of a network API call."]
pub struct Operation {
    #[serde(rename = "done")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
    pub done: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error result of the operation in case of failure or cancellation."]
    pub error: ::std::option::Option<::std::boxed::Box<Status>>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "response")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
    pub response: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Org Unit to search"]
pub struct OrgUnitInfo {
    #[serde(rename = "orgUnitId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Org unit to search, as provided by the Admin SDK Directory API."]
    pub org_unit_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A query definition relevant for search & export."]
pub struct Query {
    #[serde(rename = "accountInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When 'ACCOUNT' is chosen as search method, account_info needs to be specified."]
    pub account_info: ::std::option::Option<::std::boxed::Box<AccountInfo>>,
    #[serde(rename = "corpus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The corpus to search."]
    pub corpus: ::std::option::Option<QueryCorpusEnum>,
    #[serde(rename = "dataScope")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The data source to search from."]
    pub data_scope: ::std::option::Option<QueryDataScopeEnum>,
    #[serde(rename = "driveOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For Drive search, specify more options in this field."]
    pub drive_options: ::std::option::Option<::std::boxed::Box<DriveOptions>>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The end time range for the search query. These timestamps are in GMT and rounded down to the start of the given date."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hangoutsChatInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When 'ROOM' is chosen as search method, hangout_chats_info needs to be specified. (read-only)"]
    pub hangouts_chat_info: ::std::option::Option<::std::boxed::Box<HangoutsChatInfo>>,
    #[serde(rename = "hangoutsChatOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For hangouts chat search, specify more options in this field. (read-only)"]
    pub hangouts_chat_options: ::std::option::Option<::std::boxed::Box<HangoutsChatOptions>>,
    #[serde(rename = "mailOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For mail search, specify more options in this field."]
    pub mail_options: ::std::option::Option<::std::boxed::Box<MailOptions>>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The search method to use. This field is similar to the search_method field but is introduced to support shared drives. It supports all search method types. In case the search_method is TEAM_DRIVE the response of this field will be SHARED_DRIVE only."]
    pub method: ::std::option::Option<QueryMethodEnum>,
    #[serde(rename = "orgUnitInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When 'ORG_UNIT' is chosen as as search method, org_unit_info needs to be specified."]
    pub org_unit_info: ::std::option::Option<::std::boxed::Box<OrgUnitInfo>>,
    #[serde(rename = "searchMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The search method to use."]
    pub search_method: ::std::option::Option<QuerySearchMethodEnum>,
    #[serde(rename = "sharedDriveInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When 'SHARED_DRIVE' is chosen as search method, shared_drive_info needs to be specified."]
    pub shared_drive_info: ::std::option::Option<::std::boxed::Box<SharedDriveInfo>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The start time range for the search query. These timestamps are in GMT and rounded down to the start of the given date."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "teamDriveInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When 'TEAM_DRIVE' is chosen as search method, team_drive_info needs to be specified."]
    pub team_drive_info: ::std::option::Option<::std::boxed::Box<TeamDriveInfo>>,
    #[serde(rename = "terms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The corpus-specific search operators used to generate search results."]
    pub terms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time zone name. It should be an IANA TZ name, such as \"America/Los_Angeles\". For more information, see Time Zone."]
    pub time_zone: ::std::option::Option<::std::string::String>,
    #[serde(rename = "voiceOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For voice search, specify more options in this field."]
    pub voice_options: ::std::option::Option<::std::boxed::Box<VoiceOptions>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The corpus to search."]
pub enum QueryCorpusEnum {
    #[serde(rename = "CORPUS_TYPE_UNSPECIFIED")]
    #[doc = "No corpus specified."]
    CorpusTypeUnspecified,
    #[serde(rename = "DRIVE")]
    #[doc = "Drive."]
    Drive,
    #[serde(rename = "MAIL")]
    #[doc = "Mail."]
    Mail,
    #[serde(rename = "GROUPS")]
    #[doc = "Groups."]
    Groups,
    #[serde(rename = "HANGOUTS_CHAT")]
    #[doc = "Hangouts Chat."]
    HangoutsChat,
    #[serde(rename = "VOICE")]
    #[doc = "Google Voice."]
    Voice,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The data source to search from."]
pub enum QueryDataScopeEnum {
    #[serde(rename = "DATA_SCOPE_UNSPECIFIED")]
    #[doc = "No data scope specified."]
    DataScopeUnspecified,
    #[serde(rename = "ALL_DATA")]
    #[doc = "All available data."]
    AllData,
    #[serde(rename = "HELD_DATA")]
    #[doc = "Data on hold."]
    HeldData,
    #[serde(rename = "UNPROCESSED_DATA")]
    #[doc = "Data not processed."]
    UnprocessedData,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The search method to use. This field is similar to the search_method field but is introduced to support shared drives. It supports all search method types. In case the search_method is TEAM_DRIVE the response of this field will be SHARED_DRIVE only."]
pub enum QueryMethodEnum {
    #[serde(rename = "SEARCH_METHOD_UNSPECIFIED")]
    #[doc = "A search method must be specified. If a request does not specify a search method, it will be rejected."]
    SearchMethodUnspecified,
    #[serde(rename = "ACCOUNT")]
    #[doc = "Will search all accounts provided in account_info."]
    Account,
    #[serde(rename = "ORG_UNIT")]
    #[doc = "Will search all accounts in the OU specified in org_unit_info."]
    OrgUnit,
    #[serde(rename = "TEAM_DRIVE")]
    #[doc = "Will search for all accounts in the Team Drive specified in team_drive_info."]
    TeamDrive,
    #[serde(rename = "ENTIRE_ORG")]
    #[doc = "Will search for all accounts in the organization. No need to set account_info or org_unit_info. Not all CORPUS_TYPE support this scope. Supported by MAIL."]
    EntireOrg,
    #[serde(rename = "ROOM")]
    #[doc = "Will search in the Room specified in hangout_chats_info. (read-only)"]
    Room,
    #[serde(rename = "SHARED_DRIVE")]
    #[doc = "Will search for all accounts in the shared drive specified in shared_drive_info."]
    SharedDrive,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The search method to use."]
pub enum QuerySearchMethodEnum {
    #[serde(rename = "SEARCH_METHOD_UNSPECIFIED")]
    #[doc = "A search method must be specified. If a request does not specify a search method, it will be rejected."]
    SearchMethodUnspecified,
    #[serde(rename = "ACCOUNT")]
    #[doc = "Will search all accounts provided in account_info."]
    Account,
    #[serde(rename = "ORG_UNIT")]
    #[doc = "Will search all accounts in the OU specified in org_unit_info."]
    OrgUnit,
    #[serde(rename = "TEAM_DRIVE")]
    #[doc = "Will search for all accounts in the Team Drive specified in team_drive_info."]
    TeamDrive,
    #[serde(rename = "ENTIRE_ORG")]
    #[doc = "Will search for all accounts in the organization. No need to set account_info or org_unit_info. Not all CORPUS_TYPE support this scope. Supported by MAIL."]
    EntireOrg,
    #[serde(rename = "ROOM")]
    #[doc = "Will search in the Room specified in hangout_chats_info. (read-only)"]
    Room,
    #[serde(rename = "SHARED_DRIVE")]
    #[doc = "Will search for all accounts in the shared drive specified in shared_drive_info."]
    SharedDrive,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Remove a list of accounts from a hold."]
pub struct RemoveHeldAccountsRequest {
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account IDs to identify HeldAccounts to remove."]
    pub account_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for batch delete held accounts."]
pub struct RemoveHeldAccountsResponse {
    #[serde(rename = "statuses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of statuses for deleted accounts. Results have the same order as the request."]
    pub statuses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Status>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Remove an account as a matter collaborator."]
pub struct RemoveMatterPermissionsRequest {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The account ID."]
    pub account_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Reopen a matter by ID."]
pub struct ReopenMatterRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response to a ReopenMatterRequest."]
pub struct ReopenMatterResponse {
    #[serde(rename = "matter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The updated matter, with state OPEN."]
    pub matter: ::std::option::Option<::std::boxed::Box<Matter>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Definition of the saved query."]
pub struct SavedQuery {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The server generated timestamp at which saved query was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the saved query."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "matterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The matter ID of the associated matter. The server does not look at this field during create and always uses matter id in the URL."]
    pub matter_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The underlying Query object which contains all the information of the saved query."]
    pub query: ::std::option::Option<::std::boxed::Box<Query>>,
    #[serde(rename = "savedQueryId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique identifier for the saved query."]
    pub saved_query_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Shared drives to search"]
pub struct SharedDriveInfo {
    #[serde(rename = "sharedDriveIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of Shared drive IDs, as provided by Drive API."]
    pub shared_drive_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
pub struct Status {
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
#[doc = "Team Drives to search"]
pub struct TeamDriveInfo {
    #[serde(rename = "teamDriveIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of Team Drive IDs, as provided by Drive API."]
    pub team_drive_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Undelete a matter by ID."]
pub struct UndeleteMatterRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "User's information."]
pub struct UserInfo {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The displayed name of the user."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address of the user."]
    pub email: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The options for voice export."]
pub struct VoiceExportOptions {
    #[serde(rename = "exportFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The export format for voice export."]
    pub export_format: ::std::option::Option<VoiceExportOptionsExportFormatEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The export format for voice export."]
pub enum VoiceExportOptionsExportFormatEnum {
    #[serde(rename = "EXPORT_FORMAT_UNSPECIFIED")]
    #[doc = "No export format specified."]
    ExportFormatUnspecified,
    #[serde(rename = "MBOX")]
    #[doc = "MBOX as export format."]
    Mbox,
    #[serde(rename = "PST")]
    #[doc = "PST as export format"]
    Pst,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Voice search options"]
pub struct VoiceOptions {
    #[serde(rename = "coveredData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Datatypes to search"]
    pub covered_data: ::std::option::Option<::std::vec::Vec<VoiceOptionsCoveredDataEnum>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum VoiceOptionsCoveredDataEnum {
    #[serde(rename = "COVERED_DATA_UNSPECIFIED")]
    #[doc = "Covered data unspecified."]
    CoveredDataUnspecified,
    #[serde(rename = "TEXT_MESSAGES")]
    #[doc = "Voice text message will be covered."]
    TextMessages,
    #[serde(rename = "VOICEMAILS")]
    #[doc = "Voicemail will be covered."]
    Voicemails,
    #[serde(rename = "CALL_LOGS")]
    #[doc = "Call logs will be covered."]
    CallLogs,
}
