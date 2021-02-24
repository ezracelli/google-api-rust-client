#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings on a Project/Folder/Organization related to Access Approval."]
pub struct AccessApprovalSettings {
    #[serde(rename = "enrolledAncestor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. This field is read only (not settable via UpdateAccessAccessApprovalSettings method). If the field is true, that indicates that at least one service is enrolled for Access Approval in one or more ancestors of the Project or Folder (this field will always be unset for the organization since organizations do not have ancestors)."]
    pub enrolled_ancestor: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "enrolledServices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of Google Cloud Services for which the given resource has Access Approval enrolled. Access requests for the resource given by name against any of these services contained here will be required to have explicit approval. If name refers to an organization, enrollment can be done for individual services. If name refers to a folder or project, enrollment can only be done on an all or nothing basis. If a cloud_product is repeated in this list, the first entry will be honored and all following entries will be discarded. A maximum of 10 enrolled services will be enforced, to be expanded as the set of supported services is expanded."]
    pub enrolled_services:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EnrolledService>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the settings. Format is one of: * \"projects/{project}/accessApprovalSettings\" * \"folders/{folder}/accessApprovalSettings\" * \"organizations/{organization}/accessApprovalSettings\""]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notificationEmails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of email addresses to which notifications relating to approval requests should be sent. Notifications relating to a resource will be sent to all emails in the settings of ancestor resources of that resource. A maximum of 50 email addresses are allowed."]
    pub notification_emails: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Home office and physical location of the principal."]
pub struct AccessLocations {
    #[serde(rename = "principalOfficeCountry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The \"home office\" location of the principal. A two-letter country code (ISO 3166-1 alpha-2), such as \"US\", \"DE\" or \"GB\" or a region code. In some limited situations Google systems may refer refer to a region code instead of a country code. Possible Region Codes: * ASI: Asia * EUR: Europe * OCE: Oceania * AFR: Africa * NAM: North America * SAM: South America * ANT: Antarctica * ANY: Any location"]
    pub principal_office_country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "principalPhysicalLocationCountry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Physical location of the principal at the time of the access. A two-letter country code (ISO 3166-1 alpha-2), such as \"US\", \"DE\" or \"GB\" or a region code. In some limited situations Google systems may refer refer to a region code instead of a country code. Possible Region Codes: * ASI: Asia * EUR: Europe * OCE: Oceania * AFR: Africa * NAM: North America * SAM: South America * ANT: Antarctica * ANY: Any location"]
    pub principal_physical_location_country: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessReason {
    #[serde(rename = "detail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "More detail about certain reason types. See comments for each type above."]
    pub detail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of access justification."]
    pub _type: ::std::option::Option<AccessReasonTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of access justification."]
pub enum AccessReasonTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Default value for proto, shouldn't be used."]
    TypeUnspecified,
    #[serde(rename = "CUSTOMER_INITIATED_SUPPORT")]
    #[doc = "Customer made a request or raised an issue that required the principal to access customer data. `detail` is of the form (\"#####\" is the issue ID): * \"Feedback Report: #####\" * \"Case Number: #####\" * \"Case ID: #####\" * \"E-PIN Reference: #####\" * \"Google-#####\" * \"T-#####\""]
    CustomerInitiatedSupport,
    #[serde(rename = "GOOGLE_INITIATED_SERVICE")]
    #[doc = "The principal accessed customer data in order to diagnose or resolve a suspected issue in services or a known outage. Often this access is used to confirm that customers are not affected by a suspected service issue or to remediate a reversible system issue."]
    GoogleInitiatedService,
    #[serde(rename = "GOOGLE_INITIATED_REVIEW")]
    #[doc = "Google initiated service for security, fraud, abuse, or compliance purposes."]
    GoogleInitiatedReview,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request for the customer to approve access to a resource."]
pub struct ApprovalRequest {
    #[serde(rename = "approve")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Access was approved."]
    pub approve: ::std::option::Option<::std::boxed::Box<ApproveDecision>>,
    #[serde(rename = "dismiss")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request was dismissed."]
    pub dismiss: ::std::option::Option<::std::boxed::Box<DismissDecision>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the request. Format is \"{projects|folders|organizations}/{id}/approvalRequests/{approval_request}\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which approval was requested."]
    pub request_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestedExpiration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The requested expiration for the approval. If the request is approved, access will be granted from the time of approval until the expiration time."]
    pub requested_expiration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestedLocations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The locations for which approval is being requested."]
    pub requested_locations: ::std::option::Option<::std::boxed::Box<AccessLocations>>,
    #[serde(rename = "requestedReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The justification for which approval is being requested."]
    pub requested_reason: ::std::option::Option<::std::boxed::Box<AccessReason>>,
    #[serde(rename = "requestedResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource for which approval is being requested. The format of the resource name is defined at https://cloud.google.com/apis/design/resource_names. The resource name here may either be a \"full\" resource name (e.g. \"//library.googleapis.com/shelves/shelf1/books/book2\") or a \"relative\" resource name (e.g. \"shelves/shelf1/books/book2\") as described in the resource name specification."]
    pub requested_resource_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestedResourceProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Properties related to the resource represented by requested_resource_name."]
    pub requested_resource_properties: ::std::option::Option<::std::boxed::Box<ResourceProperties>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to approve an ApprovalRequest."]
pub struct ApproveApprovalRequestMessage {
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The expiration time of this approval."]
    pub expire_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A decision that has been made to approve access to a resource."]
pub struct ApproveDecision {
    #[serde(rename = "approveTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which approval was granted."]
    pub approve_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the approval expires."]
    pub expire_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to dismiss an approval request."]
pub struct DismissApprovalRequestMessage {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A decision that has been made to dismiss an approval request."]
pub struct DismissDecision {
    #[serde(rename = "dismissTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the approval request was dismissed."]
    pub dismiss_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "implicit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field will be true if the ApprovalRequest was implcitly dismissed due to inaction by the access approval approvers (the request is not acted on by the approvers before the exiration time)."]
    pub implicit: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the enrollment of a cloud resource into a specific service."]
pub struct EnrolledService {
    #[serde(rename = "cloudProduct")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The product for which Access Approval will be enrolled. Allowed values are listed below (case-sensitive): * all * App Engine * BigQuery * Cloud Bigtable * Cloud Key Management Service * Compute Engine * Cloud Dataflow * Cloud Identity and Access Management * Cloud Pub/Sub * Cloud Storage * Persistent Disk Note: These values are supported as input for legacy purposes, but will not be returned from the API. * all * appengine.googleapis.com * bigquery.googleapis.com * bigtable.googleapis.com * cloudkms.googleapis.com * compute.googleapis.com * dataflow.googleapis.com * iam.googleapis.com * pubsub.googleapis.com * storage.googleapis.com Calls to UpdateAccessApprovalSettings using 'all' or any of the XXX.googleapis.com will be translated to the associated product name ('all', 'App Engine', etc.). Note: 'all' will enroll the resource in all products supported at both 'GA' and 'Preview' levels. More information about levels of support is available at https://cloud.google.com/access-approval/docs/supported-services"]
    pub cloud_product: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enrollmentLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The enrollment level of the service."]
    pub enrollment_level: ::std::option::Option<EnrolledServiceEnrollmentLevelEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The enrollment level of the service."]
pub enum EnrolledServiceEnrollmentLevelEnum {
    #[serde(rename = "ENROLLMENT_LEVEL_UNSPECIFIED")]
    #[doc = "Default value for proto, shouldn't be used."]
    EnrollmentLevelUnspecified,
    #[serde(rename = "BLOCK_ALL")]
    #[doc = "Service is enrolled in Access Approval for all requests"]
    BlockAll,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response to listing of ApprovalRequest objects."]
pub struct ListApprovalRequestsResponse {
    #[serde(rename = "approvalRequests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Approval request details."]
    pub approval_requests:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApprovalRequest>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The properties associated with the resource of the request."]
pub struct ResourceProperties {
    #[serde(rename = "excludesDescendants")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether an approval will exclude the descendants of the resource being requested."]
    pub excludes_descendants: ::std::option::Option<::std::primitive::bool>,
}
