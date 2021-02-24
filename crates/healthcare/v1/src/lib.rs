#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { \"audit_configs\": [ { \"service\": \"allServices\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" }, { \"log_type\": \"ADMIN_READ\" } ] }, { \"service\": \"sampleservice.googleapis.com\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\" }, { \"log_type\": \"DATA_WRITE\", \"exempted_members\": [ \"user:aliya@example.com\" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging."]
pub struct AuditConfig {
    #[serde(rename = "auditLogConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration for logging of each type of permission."]
    pub audit_log_configs:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditLogConfig>>>,
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services."]
    pub service: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Provides the configuration for logging a type of permissions. Example: { \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging."]
pub struct AuditLogConfig {
    #[serde(rename = "exemptedMembers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members."]
    pub exempted_members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "logType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The log type that this config enables."]
    pub log_type: ::std::option::Option<AuditLogConfigLogTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The log type that this config enables."]
pub enum AuditLogConfigLogTypeEnum {
    #[serde(rename = "LOG_TYPE_UNSPECIFIED")]
    #[doc = "Default case. Should never be this."]
    LogTypeUnspecified,
    #[serde(rename = "ADMIN_READ")]
    #[doc = "Admin reads. Example: CloudIAM getIamPolicy"]
    AdminRead,
    #[serde(rename = "DATA_WRITE")]
    #[doc = "Data writes. Example: CloudSQL Users create"]
    DataWrite,
    #[serde(rename = "DATA_READ")]
    #[doc = "Data reads. Example: CloudSQL Users list"]
    DataRead,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Associates `members` with a `role`."]
pub struct Binding {
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the members in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub condition: ::std::option::Option<::std::boxed::Box<Expr>>,
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the identities requesting access for a Cloud Platform resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. "]
    pub members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Role that is assigned to `members`. For example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
    pub role: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for Operations.CancelOperation."]
pub struct CancelOperationRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Mask a string by replacing its characters with a fixed character."]
pub struct CharacterMaskConfig {
    #[serde(rename = "maskingCharacter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Character to mask the sensitive values. If not supplied, defaults to \"*\"."]
    pub masking_character: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creates a new message."]
pub struct CreateMessageRequest {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HL7v2 message."]
    pub message: ::std::option::Option<::std::boxed::Box<Message>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Pseudonymization method that generates surrogates via cryptographic hashing. Uses SHA-256. Outputs a base64-encoded representation of the hashed output (for example, `L7k0BHmF1ha5U3NfGykjro4xWi1MPVQPjhMAZbSV9mM=`)."]
pub struct CryptoHashConfig {
    #[serde(rename = "cryptoKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An AES 128/192/256 bit key. Causes the hash to be computed based on this key. A default key is generated for each Deidentify operation and is used wherever crypto_key is not specified."]
    pub crypto_key: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message representing a health dataset. A health dataset represents a collection of healthcare data pertaining to one or more patients. This may include multiple modalities of healthcare data, such as electronic medical records or medical imaging data."]
pub struct Dataset {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name of the dataset, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default timezone used by this dataset. Must be a either a valid IANA time zone name such as \"America/New_York\" or empty, which defaults to UTC. This is used for parsing times in resources, such as HL7 messages, where no explicit timezone is specified."]
    pub time_zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Shift a date forward or backward in time by a random amount which is consistent for a given patient and crypto key combination."]
pub struct DateShiftConfig {
    #[serde(rename = "cryptoKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An AES 128/192/256 bit key. Causes the shift to be computed based on this key and the patient ID. A default key is generated for each Deidentify operation and is used wherever crypto_key is not specified."]
    pub crypto_key: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configures de-id options specific to different types of content. Each submessage customizes the handling of an https://tools.ietf.org/html/rfc6838 media type or subtype. Configs are applied in a nested manner at runtime."]
pub struct DeidentifyConfig {
    #[serde(rename = "dicom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configures de-id of application/DICOM content."]
    pub dicom: ::std::option::Option<::std::boxed::Box<DicomConfig>>,
    #[serde(rename = "fhir")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configures de-id of application/FHIR content."]
    pub fhir: ::std::option::Option<::std::boxed::Box<FhirConfig>>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configures de-identification of image pixels wherever they are found in the source_dataset."]
    pub image: ::std::option::Option<::std::boxed::Box<ImageConfig>>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configures de-identification of text wherever it is found in the source_dataset."]
    pub text: ::std::option::Option<::std::boxed::Box<TextConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Redacts identifying information from the specified dataset."]
pub struct DeidentifyDatasetRequest {
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deidentify configuration."]
    pub config: ::std::option::Option<::std::boxed::Box<DeidentifyConfig>>,
    #[serde(rename = "destinationDataset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the dataset resource to create and write the redacted data to. * The destination dataset must not exist. * The destination dataset must be in the same project and location as the source dataset. De-identifying data across multiple projects or locations is not supported."]
    pub destination_dataset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creates a new DICOM store with sensitive information de-identified."]
pub struct DeidentifyDicomStoreRequest {
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "De-identify configuration."]
    pub config: ::std::option::Option<::std::boxed::Box<DeidentifyConfig>>,
    #[serde(rename = "destinationStore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the DICOM store to create and write the redacted data to. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`. * The destination dataset must exist. * The source dataset and destination dataset must both reside in the same project. De-identifying data across multiple projects is not supported. * The destination DICOM store must not exist. * The caller must have the necessary permissions to create the destination DICOM store."]
    pub destination_store: ::std::option::Option<::std::string::String>,
    #[serde(rename = "filterConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filter configuration."]
    pub filter_config: ::std::option::Option<::std::boxed::Box<DicomFilterConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Creates a new FHIR store with sensitive information de-identified."]
pub struct DeidentifyFhirStoreRequest {
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deidentify configuration."]
    pub config: ::std::option::Option<::std::boxed::Box<DeidentifyConfig>>,
    #[serde(rename = "destinationStore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the FHIR store to create and write the redacted data to. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`. * The destination dataset must exist. * The source dataset and destination dataset must both reside in the same project. De-identifying data across multiple projects is not supported. * The destination FHIR store must exist. * The caller must have the healthcare.fhirResources.update permission to write to the destination FHIR store."]
    pub destination_store: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A filter specifying the resources to include in the output. If not specified, all resources are included in the output."]
    pub resource_filter: ::std::option::Option<::std::boxed::Box<FhirFilter>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains a summary of the Deidentify operation."]
pub struct DeidentifySummary {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the parameters needed for de-identification of DICOM stores."]
pub struct DicomConfig {
    #[serde(rename = "filterProfile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tag filtering profile that determines which tags to keep/remove."]
    pub filter_profile: ::std::option::Option<DicomConfigFilterProfileEnum>,
    #[serde(rename = "keepList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of tags to keep. Remove all other tags."]
    pub keep_list: ::std::option::Option<::std::boxed::Box<TagFilterList>>,
    #[serde(rename = "removeList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of tags to remove. Keep all other tags."]
    pub remove_list: ::std::option::Option<::std::boxed::Box<TagFilterList>>,
    #[serde(rename = "skipIdRedaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, skip replacing StudyInstanceUID, SeriesInstanceUID, SOPInstanceUID, and MediaStorageSOPInstanceUID and leave them untouched. The Cloud Healthcare API regenerates these UIDs by default based on the DICOM Standard's reasoning: \"Whilst these UIDs cannot be mapped directly to an individual out of context, given access to the original images, or to a database of the original images containing the UIDs, it would be possible to recover the individual's identity.\" http://dicom.nema.org/medical/dicom/current/output/chtml/part15/sect_E.3.9.html"]
    pub skip_id_redaction: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Tag filtering profile that determines which tags to keep/remove."]
pub enum DicomConfigFilterProfileEnum {
    #[serde(rename = "TAG_FILTER_PROFILE_UNSPECIFIED")]
    #[doc = "No tag filtration profile provided. Same as KEEP_ALL_PROFILE."]
    TagFilterProfileUnspecified,
    #[serde(rename = "MINIMAL_KEEP_LIST_PROFILE")]
    #[doc = "Keep only tags required to produce valid DICOM."]
    MinimalKeepListProfile,
    #[serde(rename = "ATTRIBUTE_CONFIDENTIALITY_BASIC_PROFILE")]
    #[doc = "Remove tags based on DICOM Standard's Attribute Confidentiality Basic Profile (DICOM Standard Edition 2018e) http://dicom.nema.org/medical/dicom/2018e/output/chtml/part15/chapter_E.html."]
    AttributeConfidentialityBasicProfile,
    #[serde(rename = "KEEP_ALL_PROFILE")]
    #[doc = "Keep all tags."]
    KeepAllProfile,
    #[serde(rename = "DEIDENTIFY_TAG_CONTENTS")]
    #[doc = "Inspects within tag contents and replaces sensitive text. The process can be configured using the TextConfig. Applies to all tags with the following Value Representation names: AE, LO, LT, PN, SH, ST, UC, UT, DA, DT, AS"]
    DeidentifyTagContents,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the filter configuration for DICOM resources."]
pub struct DicomFilterConfig {
    #[serde(rename = "resourcePathsGcsUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Cloud Storage location of the filter configuration file. The `gcs_uri` must be in the format `gs://bucket/path/to/object`. The filter configuration file must contain a list of resource paths separated by newline characters (\\n or \\r\\n). Each resource path must be in the format \"/studies/{studyUID}[/series/{seriesUID}[/instances/{instanceUID}]]\" The Cloud Healthcare API service account must have the `roles/storage.objectViewer` Cloud IAM role for this Cloud Storage location."]
    pub resource_paths_gcs_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a DICOM store."]
pub struct DicomStore {
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-supplied key-value pairs used to organize DICOM stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \\p{Ll}\\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63} No more than 64 labels can be associated with a given store."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name of the DICOM store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notificationConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Notification destination for new DICOM instances. Supplied by the client."]
    pub notification_config: ::std::option::Option<::std::boxed::Box<NotificationConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Exports data from the specified DICOM store. If a given resource, such as a DICOM object with the same SOPInstance UID, already exists in the output, it is overwritten with the version in the source dataset. Exported DICOM data persists when the DICOM store from which it was exported is deleted."]
pub struct ExportDicomDataRequest {
    #[serde(rename = "bigqueryDestination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The BigQuery output destination. You can only export to a BigQuery dataset that's in the same project as the DICOM store you're exporting from. The Cloud Healthcare Service Agent requires two IAM roles on the BigQuery location: `roles/bigquery.dataEditor` and `roles/bigquery.jobUser`."]
    pub bigquery_destination:
        ::std::option::Option<::std::boxed::Box<GoogleCloudHealthcareV1DicomBigQueryDestination>>,
    #[serde(rename = "gcsDestination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Cloud Storage output destination. The Cloud Healthcare Service Agent requires the `roles/storage.objectAdmin` Cloud IAM roles on the Cloud Storage location."]
    pub gcs_destination:
        ::std::option::Option<::std::boxed::Box<GoogleCloudHealthcareV1DicomGcsDestination>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Returns additional information in regards to a completed DICOM store export."]
pub struct ExportDicomDataResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to export resources."]
pub struct ExportResourcesRequest {
    #[serde(rename = "bigqueryDestination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The BigQuery output destination. The Cloud Healthcare Service Agent requires two IAM roles on the BigQuery location: `roles/bigquery.dataEditor` and `roles/bigquery.jobUser`. The output is one BigQuery table per resource type."]
    pub bigquery_destination:
        ::std::option::Option<::std::boxed::Box<GoogleCloudHealthcareV1FhirBigQueryDestination>>,
    #[serde(rename = "gcsDestination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Cloud Storage output destination. The Healthcare Service Agent account requires the `roles/storage.objectAdmin` role on the Cloud Storage location. The exported outputs are organized by FHIR resource types. The server creates one object per resource type. Each object contains newline delimited JSON, and each line is a FHIR resource."]
    pub gcs_destination:
        ::std::option::Option<::std::boxed::Box<GoogleCloudHealthcareV1FhirGcsDestination>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response when all resources export successfully. This structure is included in the response to describe the detailed outcome after the operation finishes successfully."]
pub struct ExportResourcesResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: \"Summary size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() < 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\" expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description: \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information."]
pub struct Expr {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expression")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: ::std::option::Option<::std::string::String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies how to handle de-identification of a FHIR store."]
pub struct FhirConfig {
    #[serde(rename = "fieldMetadataList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies FHIR paths to match and how to transform them. Any field that is not matched by a FieldMetadata is passed through to the output dataset unmodified. All extensions are removed in the output."]
    pub field_metadata_list:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FieldMetadata>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Filter configuration."]
pub struct FhirFilter {
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of resources to include in the output. If this list is empty or not specified, all resources are included in the output."]
    pub resources: ::std::option::Option<::std::boxed::Box<Resources>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a FHIR store."]
pub struct FhirStore {
    #[serde(rename = "disableReferentialIntegrity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. Whether to disable referential integrity in this FHIR store. This field is immutable after FHIR store creation. The default value is false, meaning that the API enforces referential integrity and fails the requests that result in inconsistent state in the FHIR store. When this field is set to true, the API skips referential integrity checks. Consequently, operations that rely on references, such as GetPatientEverything, do not return all the results if broken references exist."]
    pub disable_referential_integrity: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "disableResourceVersioning")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. Whether to disable resource versioning for this FHIR store. This field can not be changed after the creation of FHIR store. If set to false, which is the default behavior, all write operations cause historical versions to be recorded automatically. The historical versions can be fetched through the history APIs, but cannot be updated. If set to true, no historical versions are kept. The server sends errors for attempts to read the historical versions."]
    pub disable_resource_versioning: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "enableUpdateCreate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this FHIR store has the [updateCreate capability](https://www.hl7.org/fhir/capabilitystatement-definitions.html#CapabilityStatement.rest.resource.updateCreate). This determines if the client can use an Update operation to create a new resource with a client-specified ID. If false, all IDs are server-assigned through the Create operation and attempts to update a non-existent resource return errors. It is strongly advised not to include or encode any sensitive data such as patient identifiers in client-specified resource IDs. Those IDs are part of the FHIR resource path recorded in Cloud audit logs and Cloud Pub/Sub notifications. Those IDs can also be contained in reference fields within other resources."]
    pub enable_update_create: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-supplied key-value pairs used to organize FHIR stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \\p{Ll}\\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63} No more than 64 labels can be associated with a given store."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Resource name of the FHIR store, of the form `projects/{project_id}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notificationConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If non-empty, publish all resource modifications of this FHIR store to this destination. The Cloud Pub/Sub message attributes contain a map with a string describing the action that has triggered the notification. For example, \"action\":\"CreateResource\"."]
    pub notification_config: ::std::option::Option<::std::boxed::Box<NotificationConfig>>,
    #[serde(rename = "streamConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of streaming configs that configure the destinations of streaming export for every resource mutation in this FHIR store. Each store is allowed to have up to 10 streaming configs. After a new config is added, the next resource mutation is streamed to the new location in addition to the existing ones. When a location is removed from the list, the server stops streaming to that location. Before adding a new config, you must add the required [`bigquery.dataEditor`](https://cloud.google.com/bigquery/docs/access-control#bigquery.dataEditor) role to your project's **Cloud Healthcare Service Agent** [service account](https://cloud.google.com/iam/docs/service-accounts). Some lag (typically on the order of dozens of seconds) is expected before the results show up in the streaming destination."]
    pub stream_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StreamConfig>>>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The FHIR specification version that this FHIR store supports natively. This field is immutable after store creation. Requests are rejected if they contain FHIR resources of a different version. Version is required for every FHIR store."]
    pub version: ::std::option::Option<FhirStoreVersionEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Immutable. The FHIR specification version that this FHIR store supports natively. This field is immutable after store creation. Requests are rejected if they contain FHIR resources of a different version. Version is required for every FHIR store."]
pub enum FhirStoreVersionEnum {
    #[serde(rename = "VERSION_UNSPECIFIED")]
    #[doc = "Users must specify a version on store creation or an error is returned."]
    VersionUnspecified,
    #[serde(rename = "DSTU2")]
    #[doc = "Draft Standard for Trial Use, [Release 2](https://www.hl7.org/fhir/DSTU2)"]
    Dstu2,
    #[serde(rename = "STU3")]
    #[doc = "Standard for Trial Use, [Release 3](https://www.hl7.org/fhir/STU3)"]
    Stu3,
    #[serde(rename = "R4")]
    #[doc = "[Release 4](https://www.hl7.org/fhir/R4)"]
    R4,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies FHIR paths to match, and how to handle de-identification of matching fields."]
pub struct FieldMetadata {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deidentify action for one field."]
    pub action: ::std::option::Option<FieldMetadataActionEnum>,
    #[serde(rename = "paths")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of paths to FHIR fields to be redacted. Each path is a period-separated list where each component is either a field name or FHIR type name, for example: Patient, HumanName. For \"choice\" types (those defined in the FHIR spec with the form: field[x]) we use two separate components. For example, \"deceasedAge.unit\" is matched by \"Deceased.Age.unit\". Supported types are: AdministrativeGenderCode, Code, Date, DateTime, Decimal, HumanName, Id, LanguageCode, Markdown, Oid, String, Uri, Uuid, Xhtml. Base64Binary is also supported, but may only be kept as-is or have all the content removed."]
    pub paths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Deidentify action for one field."]
pub enum FieldMetadataActionEnum {
    #[serde(rename = "ACTION_UNSPECIFIED")]
    #[doc = "No action specified."]
    ActionUnspecified,
    #[serde(rename = "TRANSFORM")]
    #[doc = "Transform the entire field."]
    Transform,
    #[serde(rename = "INSPECT_AND_TRANSFORM")]
    #[doc = "Inspect and transform any found PHI."]
    InspectAndTransform,
    #[serde(rename = "DO_NOT_TRANSFORM")]
    #[doc = "Do not transform."]
    DoNotTransform,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains a summary of the DeidentifyDicomStore operation."]
pub struct GoogleCloudHealthcareV1DeidentifyDeidentifyDicomStoreSummary {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains a summary of the DeidentifyFhirStore operation."]
pub struct GoogleCloudHealthcareV1DeidentifyDeidentifyFhirStoreSummary {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The BigQuery table where the server writes the output."]
pub struct GoogleCloudHealthcareV1DicomBigQueryDestination {
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the destination table already exists and this flag is `TRUE`, the table is overwritten by the contents of the DICOM store. If the flag is not set and the destination table already exists, the export call returns an error."]
    pub force: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "tableUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "BigQuery URI to a table, up to 2000 characters long, in the format `bq://projectId.bqDatasetId.tableId`"]
    pub table_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Cloud Storage location where the server writes the output and the export configuration."]
pub struct GoogleCloudHealthcareV1DicomGcsDestination {
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "MIME types supported by DICOM spec. Each file is written in the following format: `.../{study_id}/{series_id}/{instance_id}[/{frame_number}].{extension}` The frame_number component exists only for multi-frame instances. Supported MIME types are consistent with supported formats in DICOMweb: https://cloud.google.com/healthcare/docs/dicom#retrieve_transaction. Specifically, the following are supported: - application/dicom; transfer-syntax=1.2.840.10008.1.2.1 (uncompressed DICOM) - application/dicom; transfer-syntax=1.2.840.10008.1.2.4.50 (DICOM with embedded JPEG Baseline) - application/dicom; transfer-syntax=1.2.840.10008.1.2.4.90 (DICOM with embedded JPEG 2000 Lossless Only) - application/dicom; transfer-syntax=1.2.840.10008.1.2.4.91 (DICOM with embedded JPEG 2000) - application/dicom; transfer-syntax=* (DICOM with no transcoding) - application/octet-stream; transfer-syntax=1.2.840.10008.1.2.1 (raw uncompressed PixelData) - application/octet-stream; transfer-syntax=* (raw PixelData in whatever format it was uploaded in) - image/jpeg; transfer-syntax=1.2.840.10008.1.2.4.50 (Consumer JPEG) - image/png The following extensions are used for output files: - application/dicom -> .dcm - image/jpeg -> .jpg - image/png -> .png - application/octet-stream -> no extension If unspecified, the instances are exported in the original DICOM format they were uploaded in."]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uriPrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Cloud Storage destination to export to. URI for a Cloud Storage directory where the server writes the result files, in the format `gs://{bucket-id}/{path/to/destination/dir}`). If there is no trailing slash, the service appends one when composing the object path. The user is responsible for creating the Cloud Storage bucket referenced in `uri_prefix`."]
    pub uri_prefix: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the configuration for importing data from Cloud Storage."]
pub struct GoogleCloudHealthcareV1DicomGcsSource {
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Points to a Cloud Storage URI containing file(s) with content only. The URI must be in the following format: `gs://{bucket_id}/{object_id}`. The URI can include wildcards in `object_id` and thus identify multiple files. Supported wildcards: '*' to match 0 or more non-separator characters '**' to match 0 or more characters (including separators). Must be used at the end of a path and with no other wildcards in the path. Can also be used with a file extension (such as .dcm), which imports all files with the extension in the specified directory and its sub-directories. For example, `gs://my-bucket/my-directory/**.dcm` imports all files with .dcm extensions in `my-directory/` and its sub-directories. '?' to match 1 character All other URI formats are invalid. Files matching the wildcard are expected to contain content only, no metadata."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The configuration for exporting to BigQuery."]
pub struct GoogleCloudHealthcareV1FhirBigQueryDestination {
    #[serde(rename = "datasetUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "BigQuery URI to an existing dataset, up to 2000 characters long, in the format `bq://projectId.bqDatasetId`."]
    pub dataset_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this flag is `TRUE`, all tables are deleted from the dataset before the new exported tables are written. If the flag is not set and the destination dataset contains tables, the export call returns an error. If `write_disposition` is specified, this parameter is ignored. force=false is equivalent to write_disposition=WRITE_EMPTY and force=true is equivalent to write_disposition=WRITE_TRUNCATE."]
    pub force: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "schemaConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration for the exported BigQuery schema."]
    pub schema_config: ::std::option::Option<::std::boxed::Box<SchemaConfig>>,
    #[serde(rename = "writeDisposition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Determines whether existing tables in the destination dataset are overwritten or appended to. If a write_disposition is specified, the `force` parameter is ignored."]
    pub write_disposition:
        ::std::option::Option<GoogleCloudHealthcareV1FhirBigQueryDestinationWriteDispositionEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Determines whether existing tables in the destination dataset are overwritten or appended to. If a write_disposition is specified, the `force` parameter is ignored."]
pub enum GoogleCloudHealthcareV1FhirBigQueryDestinationWriteDispositionEnum {
    #[serde(rename = "WRITE_DISPOSITION_UNSPECIFIED")]
    #[doc = "Default behavior is the same as WRITE_EMPTY."]
    WriteDispositionUnspecified,
    #[serde(rename = "WRITE_EMPTY")]
    #[doc = "Only export data if the destination tables are empty."]
    WriteEmpty,
    #[serde(rename = "WRITE_TRUNCATE")]
    #[doc = "Erase all existing data in a tables before writing the instances."]
    WriteTruncate,
    #[serde(rename = "WRITE_APPEND")]
    #[doc = "Append data to the existing tables."]
    WriteAppend,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The configuration for exporting to Cloud Storage."]
pub struct GoogleCloudHealthcareV1FhirGcsDestination {
    #[serde(rename = "uriPrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI for a Cloud Storage directory where result files should be written, in the format of `gs://{bucket-id}/{path/to/destination/dir}`. If there is no trailing slash, the service appends one when composing the object path. The user is responsible for creating the Cloud Storage bucket referenced in `uri_prefix`."]
    pub uri_prefix: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the configuration for importing data from Cloud Storage."]
pub struct GoogleCloudHealthcareV1FhirGcsSource {
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Points to a Cloud Storage URI containing file(s) to import. The URI must be in the following format: `gs://{bucket_id}/{object_id}`. The URI can include wildcards in `object_id` and thus identify multiple files. Supported wildcards: * `*` to match 0 or more non-separator characters * `**` to match 0 or more characters (including separators). Must be used at the end of a path and with no other wildcards in the path. Can also be used with a file extension (such as .ndjson), which imports all files with the extension in the specified directory and its sub-directories. For example, `gs://my-bucket/my-directory/**.ndjson` imports all files with `.ndjson` extensions in `my-directory/` and its sub-directories. * `?` to match 1 character Files matching the wildcard are expected to contain content only, no metadata."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies where and whether to send notifications upon changes to a data store."]
pub struct Hl7V2NotificationConfig {
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricts notifications sent for messages matching a filter. If this is empty, all messages are matched. The following syntax is available: * A string field value can be written as text inside quotation marks, for example `\"query text\"`. The only valid relational operation for text fields is equality (`=`), where text is searched within the field, rather than having the field be equal to the text. For example, `\"Comment = great\"` returns messages with `great` in the comment field. * A number field value can be written as an integer, a decimal, or an exponential. The valid relational operators for number fields are the equality operator (`=`), along with the less than/greater than operators (`<`, `<=`, `>`, `>=`). Note that there is no inequality (`!=`) operator. You can prepend the `NOT` operator to an expression to negate it. * A date field value must be written in `yyyy-mm-dd` form. Fields with date and time use the RFC3339 time format. Leading zeros are required for one-digit months and days. The valid relational operators for date fields are the equality operator (`=`) , along with the less than/greater than operators (`<`, `<=`, `>`, `>=`). Note that there is no inequality (`!=`) operator. You can prepend the `NOT` operator to an expression to negate it. * Multiple field query expressions can be combined in one query by adding `AND` or `OR` operators between the expressions. If a boolean operator appears within a quoted string, it is not treated as special, it's just another part of the character string to be matched. You can prepend the `NOT` operator to an expression to negate it. The following fields and functions are available for filtering: * `message_type`, from the MSH-9.1 field. For example, `NOT message_type = \"ADT\"`. * `send_date` or `sendDate`, the YYYY-MM-DD date the message was sent in the dataset's time_zone, from the MSH-7 segment. For example, `send_date < \"2017-01-02\"`. * `send_time`, the timestamp when the message was sent, using the RFC3339 time format for comparisons, from the MSH-7 segment. For example, `send_time < \"2017-01-02T00:00:00-05:00\"`. * `send_facility`, the care center that the message came from, from the MSH-4 segment. For example, `send_facility = \"ABC\"`. * `PatientId(value, type)`, which matches if the message lists a patient having an ID of the given value and type in the PID-2, PID-3, or PID-4 segments. For example, `PatientId(\"123456\", \"MRN\")`. * `labels.x`, a string value of the label with key `x` as set using the Message.labels map. For example, `labels.\"priority\"=\"high\"`. The operator `:*` can be used to assert the existence of a label. For example, `labels.\"priority\":*`."]
    pub filter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pubsubTopic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The [Cloud Pub/Sub](https://cloud.google.com/pubsub/docs/) topic that notifications of changes are published on. Supplied by the client. The notification is a `PubsubMessage` with the following fields: * `PubsubMessage.Data` contains the resource name. * `PubsubMessage.MessageId` is the ID of this notification. It's guaranteed to be unique within the topic. * `PubsubMessage.PublishTime` is the time when the message was published. Note that notifications are only sent if the topic is non-empty. [Topic names](https://cloud.google.com/pubsub/docs/overview#names) must be scoped to a project. The Cloud Healthcare API service account, service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com, must have publisher permissions on the given Pub/Sub topic. Not having adequate permissions causes the calls that send notifications to fail. If a notification cannot be published to Cloud Pub/Sub, errors are logged to Cloud Logging. For more information, see [Viewing error logs in Cloud Logging](/healthcare/docs/how-tos/logging))."]
    pub pubsub_topic: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an HL7v2 store."]
pub struct Hl7V2Store {
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-supplied key-value pairs used to organize HL7v2 stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \\p{Ll}\\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63} No more than 64 labels can be associated with a given store."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name of the HL7v2 store, of the form `projects/{project_id}/datasets/{dataset_id}/hl7V2Stores/{hl7v2_store_id}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notificationConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of notification configs. Each configuration uses a filter to determine whether to publish a message (both Ingest & Create) on the corresponding notification destination. Only the message name is sent as part of the notification. Supplied by the client."]
    pub notification_configs:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Hl7V2NotificationConfig>>>,
    #[serde(rename = "parserConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration for the parser. It determines how the server parses the messages."]
    pub parser_config: ::std::option::Option<::std::boxed::Box<ParserConfig>>,
    #[serde(rename = "rejectDuplicateMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Determines whether to reject duplicate messages. A duplicate message is a message with the same raw bytes as a message that has already been ingested/created in this HL7v2 store. The default value is false, meaning that the store accepts the duplicate messages and it also returns the same ACK message in the IngestMessageResponse as has been returned previously. Note that only one resource is created in the store. When this field is set to true, CreateMessage/IngestMessage requests with a duplicate message will be rejected by the store, and IngestMessageErrorDetail returns a NACK message upon rejection."]
    pub reject_duplicate_message: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message that represents an arbitrary HTTP body. It should only be used for payload formats that can't be represented as JSON, such as raw binary or an HTML page. This message can be used both in streaming and non-streaming API methods in the request as well as the response. It can be used as a top-level request field, which is convenient if one wants to extract parameters from either the URL or HTTP template into the request fields and also want access to the raw HTTP body. Example: message GetResourceRequest { // A unique request id. string request_id = 1; // The raw HTTP body is bound to this field. google.api.HttpBody http_body = 2; } service ResourceService { rpc GetResource(GetResourceRequest) returns (google.api.HttpBody); rpc UpdateResource(google.api.HttpBody) returns (google.protobuf.Empty); } Example with streaming methods: service CaldavService { rpc GetCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); rpc UpdateCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); } Use of this type only changes how the request and response bodies are handled, all other features will continue to work unchanged."]
pub struct HttpBody {
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HTTP Content-Type header value specifying the content type of the body."]
    pub content_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HTTP request/response body as raw binary."]
    pub data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "extensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Application specific response metadata. Must be set in the first response for streaming APIs."]
    pub extensions: ::std::option::Option<
        ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies how to handle de-identification of image pixels."]
pub struct ImageConfig {
    #[serde(rename = "textRedactionMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Determines how to redact text from image."]
    pub text_redaction_mode: ::std::option::Option<ImageConfigTextRedactionModeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Determines how to redact text from image."]
pub enum ImageConfigTextRedactionModeEnum {
    #[serde(rename = "TEXT_REDACTION_MODE_UNSPECIFIED")]
    #[doc = "No text redaction specified. Same as REDACT_NO_TEXT."]
    TextRedactionModeUnspecified,
    #[serde(rename = "REDACT_ALL_TEXT")]
    #[doc = "Redact all text."]
    RedactAllText,
    #[serde(rename = "REDACT_SENSITIVE_TEXT")]
    #[doc = "Redact sensitive text."]
    RedactSensitiveText,
    #[serde(rename = "REDACT_NO_TEXT")]
    #[doc = "Do not redact text."]
    RedactNoText,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Imports data into the specified DICOM store. Returns an error if any of the files to import are not DICOM files. This API accepts duplicate DICOM instances by ignoring the newly-pushed instance. It does not overwrite."]
pub struct ImportDicomDataRequest {
    #[serde(rename = "gcsSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Storage source data location and import configuration. The Cloud Healthcare Service Agent requires the `roles/storage.objectViewer` Cloud IAM roles on the Cloud Storage location."]
    pub gcs_source: ::std::option::Option<::std::boxed::Box<GoogleCloudHealthcareV1DicomGcsSource>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Returns additional information in regards to a completed DICOM store import."]
pub struct ImportDicomDataResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to import resources."]
pub struct ImportResourcesRequest {
    #[serde(rename = "contentStructure")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content structure in the source location. If not specified, the server treats the input source files as BUNDLE."]
    pub content_structure: ::std::option::Option<ImportResourcesRequestContentStructureEnum>,
    #[serde(rename = "gcsSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Storage source data location and import configuration. The Healthcare Service Agent account requires the `roles/storage.objectAdmin` role on the Cloud Storage location. Each Cloud Storage object should be a text file that contains the format specified in ContentStructure."]
    pub gcs_source: ::std::option::Option<::std::boxed::Box<GoogleCloudHealthcareV1FhirGcsSource>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The content structure in the source location. If not specified, the server treats the input source files as BUNDLE."]
pub enum ImportResourcesRequestContentStructureEnum {
    #[serde(rename = "CONTENT_STRUCTURE_UNSPECIFIED")]
    #[doc = "If the content structure is not specified, the default value `BUNDLE` is used."]
    ContentStructureUnspecified,
    #[serde(rename = "BUNDLE")]
    #[doc = "The source file contains one or more lines of newline-delimited JSON (ndjson). Each line is a bundle that contains one or more resources."]
    Bundle,
    #[serde(rename = "RESOURCE")]
    #[doc = "The source file contains one or more lines of newline-delimited JSON (ndjson). Each line is a single resource."]
    Resource,
    #[serde(rename = "BUNDLE_PRETTY")]
    #[doc = "The entire file is one JSON bundle. The JSON can span multiple lines."]
    BundlePretty,
    #[serde(rename = "RESOURCE_PRETTY")]
    #[doc = "The entire file is one JSON resource. The JSON can span multiple lines."]
    ResourcePretty,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Final response of importing resources. This structure is included in the response to describe the detailed outcome after the operation finishes successfully."]
pub struct ImportResourcesResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A transformation to apply to text that is identified as a specific info_type."]
pub struct InfoTypeTransformation {
    #[serde(rename = "characterMaskConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Config for character mask."]
    pub character_mask_config: ::std::option::Option<::std::boxed::Box<CharacterMaskConfig>>,
    #[serde(rename = "cryptoHashConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Config for crypto hash."]
    pub crypto_hash_config: ::std::option::Option<::std::boxed::Box<CryptoHashConfig>>,
    #[serde(rename = "dateShiftConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Config for date shift."]
    pub date_shift_config: ::std::option::Option<::std::boxed::Box<DateShiftConfig>>,
    #[serde(rename = "infoTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "InfoTypes to apply this transformation to. If this is not specified, the transformation applies to any info_type."]
    pub info_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "redactConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Config for text redaction."]
    pub redact_config: ::std::option::Option<::std::boxed::Box<RedactConfig>>,
    #[serde(rename = "replaceWithInfoTypeConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Config for replace with InfoType."]
    pub replace_with_info_type_config:
        ::std::option::Option<::std::boxed::Box<ReplaceWithInfoTypeConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Ingests a message into the specified HL7v2 store."]
pub struct IngestMessageRequest {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HL7v2 message to ingest."]
    pub message: ::std::option::Option<::std::boxed::Box<Message>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Acknowledges that a message has been ingested into the specified HL7v2 store."]
pub struct IngestMessageResponse {
    #[serde(rename = "hl7Ack")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HL7v2 ACK message."]
    pub hl7_ack: ::std::option::Option<::std::string::String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Created message resource."]
    pub message: ::std::option::Option<::std::boxed::Box<Message>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Lists the available datasets."]
pub struct ListDatasetsResponse {
    #[serde(rename = "datasets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The first page of datasets."]
    pub datasets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dataset>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Lists the DICOM stores in the given dataset."]
pub struct ListDicomStoresResponse {
    #[serde(rename = "dicomStores")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The returned DICOM stores. Won't be more DICOM stores than the value of page_size in the request."]
    pub dicom_stores: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DicomStore>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Lists the FHIR stores in the given dataset."]
pub struct ListFhirStoresResponse {
    #[serde(rename = "fhirStores")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The returned FHIR stores. Won't be more FHIR stores than the value of page_size in the request."]
    pub fhir_stores: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FhirStore>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Lists the HL7v2 stores in the given dataset."]
pub struct ListHl7V2StoresResponse {
    #[serde(rename = "hl7V2Stores")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The returned HL7v2 stores. Won't be more HL7v2 stores than the value of page_size in the request."]
    pub hl7_v2_stores: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Hl7V2Store>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Locations.ListLocations."]
pub struct ListLocationsResponse {
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of locations that matches the specified filter in the request."]
    pub locations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Location>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The standard List next-page token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Lists the messages in the specified HL7v2 store."]
pub struct ListMessagesResponse {
    #[serde(rename = "hl7V2Messages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The returned Messages. Won't be more Messages than the value of page_size in the request. See view for populated fields."]
    pub hl7_v2_messages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Message>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results or empty if there are no more results in the list."]
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
#[doc = "A resource that represents Google Cloud Platform location."]
pub struct Location {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The friendly name for this location, typically a nearby city name. For example, \"Tokyo\"."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cross-service attributes for the location. For example {\"cloud.googleapis.com/region\": \"us-east1\"}"]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The canonical id for this location. For example: `\"us-east1\"`."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Service-specific metadata. For example the available capacity at the given location."]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name for the location, which may vary between implementations. For example: `\"projects/example-project/locations/us-east1\"`"]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A complete HL7v2 message. See [Introduction to HL7 Standards] (https://www.hl7.org/implement/standards/index.cfm?ref=common) for details on the standard."]
pub struct Message {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The datetime when the message was created. Set by the server."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Raw message bytes."]
    pub data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-supplied key-value pairs used to organize HL7v2 stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \\p{Ll}\\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63} No more than 64 labels can be associated with a given store."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "messageType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The message type for this message. MSH-9.1."]
    pub message_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name of the Message, of the form `projects/{project_id}/datasets/{dataset_id}/hl7V2Stores/{hl7_v2_store_id}/messages/{message_id}`. Assigned by the server."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parsedData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The parsed version of the raw message data."]
    pub parsed_data: ::std::option::Option<::std::boxed::Box<ParsedData>>,
    #[serde(rename = "patientIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All patient IDs listed in the PID-2, PID-3, and PID-4 segments of this message."]
    pub patient_ids: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PatientId>>>,
    #[serde(rename = "sendFacility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The hospital that this message came from. MSH-4."]
    pub send_facility: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sendTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The datetime the sending application sent this message. MSH-7."]
    pub send_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies where to send notifications upon changes to a data store."]
pub struct NotificationConfig {
    #[serde(rename = "pubsubTopic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The [Cloud Pub/Sub](https://cloud.google.com/pubsub/docs/) topic that notifications of changes are published on. Supplied by the client. PubsubMessage.Data contains the resource name. PubsubMessage.MessageId is the ID of this message. It is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message was published. Notifications are only sent if the topic is non-empty. [Topic names](https://cloud.google.com/pubsub/docs/overview#names) must be scoped to a project. Cloud Healthcare API service account must have publisher permissions on the given Cloud Pub/Sub topic. Not having adequate permissions causes the calls that send notifications to fail. If a notification can't be published to Cloud Pub/Sub, errors are logged to Cloud Logging (see [Viewing logs](/healthcare/docs/how-tos/logging)). If the number of errors exceeds a certain rate, some aren't submitted. Note that not all operations trigger notifications, see [Configuring Pub/Sub notifications](https://cloud.google.com/healthcare/docs/how-tos/pubsub) for specific details."]
    pub pubsub_topic: ::std::option::Option<::std::string::String>,
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
#[doc = "OperationMetadata provides information about the operation execution. Returned in the long-running operation's metadata field."]
pub struct OperationMetadata {
    #[serde(rename = "apiMethodName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the API method that initiated the operation."]
    pub api_method_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cancelRequested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies if cancellation was requested for the operation."]
    pub cancel_requested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "counter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub counter: ::std::option::Option<::std::boxed::Box<ProgressCounter>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the operation was created by the API."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which execution was completed."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "logsUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to audit and error logs in the log viewer. Error logs are generated only by some operations, listed at [Viewing logs](/healthcare/docs/how-tos/logging)."]
    pub logs_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The content of a HL7v2 message in a structured format."]
pub struct ParsedData {
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub segments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Segment>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The configuration for the parser. It determines how the server parses the messages."]
pub struct ParserConfig {
    #[serde(rename = "allowNullHeader")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Determines whether messages with no header are allowed."]
    pub allow_null_header: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "segmentTerminator")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Byte(s) to use as the segment terminator. If this is unset, '\\r' is used as segment terminator, matching the HL7 version 2 specification."]
    pub segment_terminator: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A patient identifier and associated type."]
pub struct PatientId {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID type. For example, MRN or NHS."]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The patient's unique identifier."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/)."]
pub struct Policy {
    #[serde(rename = "auditConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies cloud audit logging configuration for this policy."]
    pub audit_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditConfig>>>,
    #[serde(rename = "bindings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member."]
    pub bindings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Binding>>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ProgressCounter provides counters to describe an operation's progress."]
pub struct ProgressCounter {
    #[serde(rename = "failure")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of units that failed in the operation."]
    pub failure: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pending")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of units that are pending in the operation."]
    pub pending: ::std::option::Option<::std::string::String>,
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of units that succeeded in the operation."]
    pub success: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Define how to redact sensitive values. Default behaviour is erase. For example, \"My name is Jane.\" becomes \"My name is .\""]
pub struct RedactConfig {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "When using the INSPECT_AND_TRANSFORM action, each match is replaced with the name of the info_type. For example, \"My name is Jane\" becomes \"My name is [PERSON_NAME].\" The TRANSFORM action is equivalent to redacting."]
pub struct ReplaceWithInfoTypeConfig {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of FHIR resources."]
pub struct Resources {
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of resources IDs. For example, \"Patient/1234\"."]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for the FHIR BigQuery schema. Determines how the server generates the schema."]
pub struct SchemaConfig {
    #[serde(rename = "recursiveStructureDepth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The depth for all recursive structures in the output analytics schema. For example, `concept` in the CodeSystem resource is a recursive structure; when the depth is 2, the CodeSystem table will have a column called `concept.concept` but not `concept.concept.concept`. If not specified or set to 0, the server will use the default value 2. The maximum depth allowed is 5."]
    pub recursive_structure_depth: ::std::option::Option<::std::string::String>,
    #[serde(rename = "schemaType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the output schema type. Schema type is required."]
    pub schema_type: ::std::option::Option<SchemaConfigSchemaTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Specifies the output schema type. Schema type is required."]
pub enum SchemaConfigSchemaTypeEnum {
    #[serde(rename = "SCHEMA_TYPE_UNSPECIFIED")]
    #[doc = "No schema type specified. This type is unsupported."]
    SchemaTypeUnspecified,
    #[serde(rename = "ANALYTICS")]
    #[doc = "Analytics schema defined by the FHIR community. See https://github.com/FHIR/sql-on-fhir/blob/master/sql-on-fhir.md. BigQuery only allows a maximum of 10,000 columns per table. Due to this limitation, the server will not generate schemas for fields of type `Resource`, which can hold any resource type. The affected fields are `Parameters.parameter.resource`, `Bundle.entry.resource`, and `Bundle.entry.response.outcome`."]
    Analytics,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to search the resources in the specified FHIR store."]
pub struct SearchResourcesRequest {
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The FHIR resource type to search, such as Patient or Observation. For a complete list, see the FHIR Resource Index ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/resourcelist.html), [STU3](http://hl7.org/implement/standards/fhir/STU3/resourcelist.html), [R4](http://hl7.org/implement/standards/fhir/R4/resourcelist.html))."]
    pub resource_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A segment in a structured format."]
pub struct Segment {
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mapping from the positional location to the value. The key string uses zero-based indexes separated by dots to identify Fields, components and sub-components. A bracket notation is also used to identify different instances of a repeated field. Regex for key: (\\d+)(\\[\\d+\\])?(.\\d+)?(.\\d+)? Examples of (key, value) pairs: * (0.1, \"hemoglobin\") denotes that the first component of Field 0 has the value \"hemoglobin\". * (1.1.2, \"CBC\") denotes that the second sub-component of the first component of Field 1 has the value \"CBC\". * (1[0].1, \"HbA1c\") denotes that the first component of the first Instance of Field 1, which is repeated, has the value \"HbA1c\"."]
    pub fields: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "segmentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A string that indicates the type of segment. For example, EVN or PID."]
    pub segment_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "setId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set ID for segments that can be in a set. This can be empty if it's missing or isn't applicable."]
    pub set_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for `SetIamPolicy` method."]
pub struct SetIamPolicyRequest {
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them."]
    pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: \"bindings, etag\"`"]
    pub update_mask: ::std::option::Option<::std::string::String>,
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
#[doc = "Contains configuration for streaming FHIR export."]
pub struct StreamConfig {
    #[serde(rename = "bigqueryDestination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The destination BigQuery structure that contains both the dataset location and corresponding schema config. The output is organized in one table per resource type. The server reuses the existing tables (if any) that are named after the resource types. For example, \"Patient\", \"Observation\". When there is no existing table for a given resource type, the server attempts to create one. When a table schema doesn't align with the schema config, either because of existing incompatible schema or out of band incompatible modification, the server does not stream in new data. BigQuery imposes a 1 MB limit on streaming insert row size, therefore any resource mutation that generates more than 1 MB of BigQuery data is not streamed. One resolution in this case is to delete the incompatible table and let the server recreate one, though the newly created table only contains data after the table recreation. Results are appended to the corresponding BigQuery tables. Different versions of the same resource are distinguishable by the meta.versionId and meta.lastUpdated columns. The operation (CREATE/UPDATE/DELETE) that results in the new version is recorded in the meta.tag. The tables contain all historical resource versions since streaming was enabled. For query convenience, the server also creates one view per table of the same name containing only the current resource version. The streamed data in the BigQuery dataset is not guaranteed to be completely unique. The combination of the id and meta.versionId columns should ideally identify a single unique row. But in rare cases, duplicates may exist. At query time, users may use the SQL select statement to keep only one of the duplicate rows given an id and meta.versionId pair. Alternatively, the server created view mentioned above also filters out duplicates. If a resource mutation cannot be streamed to BigQuery, errors are logged to Cloud Logging. For more information, see [Viewing error logs in Cloud Logging](/healthcare/docs/how-tos/logging))."]
    pub bigquery_destination:
        ::std::option::Option<::std::boxed::Box<GoogleCloudHealthcareV1FhirBigQueryDestination>>,
    #[serde(rename = "resourceTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Supply a FHIR resource type (such as \"Patient\" or \"Observation\"). See https://www.hl7.org/fhir/valueset-resource-types.html for a list of all FHIR resource types. The server treats an empty list as an intent to stream all the supported resource types in this FHIR store."]
    pub resource_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List of tags to be filtered."]
pub struct TagFilterList {
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tags to be filtered. Tags must be DICOM Data Elements, File Meta Elements, or Directory Structuring Elements, as defined at: http://dicom.nema.org/medical/dicom/current/output/html/part06.html#table_6-1,. They may be provided by \"Keyword\" or \"Tag\". For example \"PatientID\", \"00100010\"."]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for `TestIamPermissions` method."]
pub struct TestIamPermissionsRequest {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of permissions to check for the `resource`. Permissions with wildcards (such as '*' or 'storage.*') are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions)."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for `TestIamPermissions` method."]
pub struct TestIamPermissionsResponse {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A subset of `TestPermissionsRequest.permissions` that the caller is allowed."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextConfig {
    #[serde(rename = "transformations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transformations to apply to the detected data."]
    pub transformations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InfoTypeTransformation>>>,
}
