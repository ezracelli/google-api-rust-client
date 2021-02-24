#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An `AccessLevel` is a label that can be applied to requests to Google Cloud services, along with a list of requirements necessary for the label to be applied."]
pub struct AccessLevel {
    #[serde(rename = "basic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A `BasicLevel` composed of `Conditions`."]
    pub basic: ::std::option::Option<::std::boxed::Box<BasicLevel>>,
    #[serde(rename = "custom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A `CustomLevel` written in the Common Expression Language."]
    pub custom: ::std::option::Option<::std::boxed::Box<CustomLevel>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the `AccessLevel` and its use. Does not affect behavior."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Resource name for the Access Level. The `short_name` component must begin with a letter and only include alphanumeric and '_'. Format: `accessPolicies/{policy_id}/accessLevels/{short_name}`. The maximum length of the `short_name` component is 50 characters."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human readable title. Must be unique within the Policy."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`AccessPolicy` is a container for `AccessLevels` (which define the necessary attributes to use Google Cloud services) and `ServicePerimeters` (which define regions of services able to freely pass data within a perimeter). An access policy is globally visible within an organization, and the restrictions it specifies apply to all projects within an organization."]
pub struct AccessPolicy {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. An opaque identifier for the current version of the `AccessPolicy`. This will always be a strongly validated etag, meaning that two Access Polices will be identical if and only if their etags are identical. Clients should not expect this to be in any specific format."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Resource name of the `AccessPolicy`. Format: `accessPolicies/{policy_id}`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The parent of this `AccessPolicy` in the Cloud Resource Hierarchy. Currently immutable once created. Format: `organizations/{organization_id}`"]
    pub parent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Human readable title. Does not affect behavior."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Identification for an API Operation."]
pub struct ApiOperation {
    #[serde(rename = "methodSelectors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API methods or permissions to allow. Method or permission must belong to the service specified by `service_name` field. A single MethodSelector entry with `*` specified for the `method` field will allow all methods AND permissions for the service specified in `service_name`."]
    pub method_selectors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MethodSelector>>>,
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the API whose methods or permissions the IngressPolicy or EgressPolicy want to allow. A single ApiOperation with `service_name` field set to `*` will allow all methods AND permissions for all services."]
    pub service_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`BasicLevel` is an `AccessLevel` using a set of recommended features."]
pub struct BasicLevel {
    #[serde(rename = "combiningFunction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How the `conditions` list should be combined to determine if a request is granted this `AccessLevel`. If AND is used, each `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. If OR is used, at least one `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. Default behavior is AND."]
    pub combining_function: ::std::option::Option<BasicLevelCombiningFunctionEnum>,
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A list of requirements for the `AccessLevel` to be granted."]
    pub conditions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Condition>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "How the `conditions` list should be combined to determine if a request is granted this `AccessLevel`. If AND is used, each `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. If OR is used, at least one `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. Default behavior is AND."]
pub enum BasicLevelCombiningFunctionEnum {
    #[serde(rename = "AND")]
    #[doc = "All `Conditions` must be true for the `BasicLevel` to be true."]
    And,
    #[serde(rename = "OR")]
    #[doc = "If at least one `Condition` is true, then the `BasicLevel` is true."]
    Or,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for Operations.CancelOperation."]
pub struct CancelOperationRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to commit dry-run specs in all Service Perimeters belonging to an Access Policy."]
pub struct CommitServicePerimetersRequest {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The etag for the version of the Access Policy that this commit operation is to be performed on. If, at the time of commit, the etag for the Access Policy stored in Access Context Manager is different from the specified etag, then the commit operation will not be performed and the call will fail. This field is not required. If etag is not provided, the operation will be performed as if a valid etag is provided."]
    pub etag: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response to CommitServicePerimetersRequest. This will be put inside of Operation.response field."]
pub struct CommitServicePerimetersResponse {
    #[serde(rename = "servicePerimeters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of all the Service Perimeter instances in the Access Policy."]
    pub service_perimeters:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ServicePerimeter>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A condition necessary for an `AccessLevel` to be granted. The Condition is an AND over its fields. So a Condition is true if: 1) the request IP is from one of the listed subnetworks AND 2) the originating device complies with the listed device policy AND 3) all listed access levels are granted AND 4) the request was sent at a time allowed by the DateTimeRestriction."]
pub struct Condition {
    #[serde(rename = "devicePolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device specific restrictions, all restrictions must hold for the Condition to be true. If not specified, all devices are allowed."]
    pub device_policy: ::std::option::Option<::std::boxed::Box<DevicePolicy>>,
    #[serde(rename = "ipSubnetworks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "CIDR block IP subnetwork specification. May be IPv4 or IPv6. Note that for a CIDR IP address block, the specified IP address portion must be properly truncated (i.e. all the host bits must be zero) or the input is considered malformed. For example, \"192.0.2.0/24\" is accepted but \"192.0.2.1/24\" is not. Similarly, for IPv6, \"2001:db8::/32\" is accepted whereas \"2001:db8::1/32\" is not. The originating IP of a request must be in one of the listed subnets in order for this Condition to be true. If empty, all IP addresses are allowed."]
    pub ip_subnetworks: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request must be made by one of the provided user or service accounts. Groups are not supported. Syntax: `user:{emailid}` `serviceAccount:{emailid}` If not specified, a request may come from any user."]
    pub members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "negate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to negate the Condition. If true, the Condition becomes a NAND over its non-empty fields, each field must be false for the Condition overall to be satisfied. Defaults to false."]
    pub negate: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request must originate from one of the provided countries/regions. Must be valid ISO 3166-1 alpha-2 codes."]
    pub regions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "requiredAccessLevels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of other access levels defined in the same `Policy`, referenced by resource name. Referencing an `AccessLevel` which does not exist is an error. All access levels listed must be granted for the Condition to be true. Example: \"`accessPolicies/MY_POLICY/accessLevels/LEVEL_NAME\"`"]
    pub required_access_levels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`CustomLevel` is an `AccessLevel` using the Cloud Common Expression Language to represent the necessary conditions for the level to apply to a request. See CEL spec at: https://github.com/google/cel-spec"]
pub struct CustomLevel {
    #[serde(rename = "expr")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A Cloud CEL expression evaluating to a boolean."]
    pub expr: ::std::option::Option<::std::boxed::Box<Expr>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`DevicePolicy` specifies device specific restrictions necessary to acquire a given access level. A `DevicePolicy` specifies requirements for requests from devices to be granted access levels, it does not do any enforcement on the device. `DevicePolicy` acts as an AND over all specified fields, and each repeated field is an OR over its elements. Any unset fields are ignored. For example, if the proto is { os_type : DESKTOP_WINDOWS, os_type : DESKTOP_LINUX, encryption_status: ENCRYPTED}, then the DevicePolicy will be true for requests originating from encrypted Linux desktops and encrypted Windows desktops."]
pub struct DevicePolicy {
    #[serde(rename = "allowedDeviceManagementLevels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allowed device management levels, an empty list allows all management levels."]
    pub allowed_device_management_levels:
        ::std::option::Option<::std::vec::Vec<DevicePolicyAllowedDeviceManagementLevelsEnum>>,
    #[serde(rename = "allowedEncryptionStatuses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allowed encryptions statuses, an empty list allows all statuses."]
    pub allowed_encryption_statuses:
        ::std::option::Option<::std::vec::Vec<DevicePolicyAllowedEncryptionStatusesEnum>>,
    #[serde(rename = "osConstraints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allowed OS versions, an empty list allows all types and all versions."]
    pub os_constraints: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OsConstraint>>>,
    #[serde(rename = "requireAdminApproval")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the device needs to be approved by the customer admin."]
    pub require_admin_approval: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "requireCorpOwned")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the device needs to be corp owned."]
    pub require_corp_owned: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "requireScreenlock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not screenlock is required for the DevicePolicy to be true. Defaults to `false`."]
    pub require_screenlock: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum DevicePolicyAllowedDeviceManagementLevelsEnum {
    #[serde(rename = "MANAGEMENT_UNSPECIFIED")]
    #[doc = "The device's management level is not specified or not known."]
    ManagementUnspecified,
    #[serde(rename = "NONE")]
    #[doc = "The device is not managed."]
    None,
    #[serde(rename = "BASIC")]
    #[doc = "Basic management is enabled, which is generally limited to monitoring and wiping the corporate account."]
    Basic,
    #[serde(rename = "COMPLETE")]
    #[doc = "Complete device management. This includes more thorough monitoring and the ability to directly manage the device (such as remote wiping). This can be enabled through the Android Enterprise Platform."]
    Complete,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum DevicePolicyAllowedEncryptionStatusesEnum {
    #[serde(rename = "ENCRYPTION_UNSPECIFIED")]
    #[doc = "The encryption status of the device is not specified or not known."]
    EncryptionUnspecified,
    #[serde(rename = "ENCRYPTION_UNSUPPORTED")]
    #[doc = "The device does not support encryption."]
    EncryptionUnsupported,
    #[serde(rename = "UNENCRYPTED")]
    #[doc = "The device supports encryption, but is currently unencrypted."]
    Unencrypted,
    #[serde(rename = "ENCRYPTED")]
    #[doc = "The device is encrypted."]
    Encrypted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines the conditions under which an EgressPolicy matches a request. Conditions based on information about the source of the request. Note that if the destination of the request is protected by a ServicePerimeter, then that ServicePerimeter must have an IngressPolicy which allows access in order for this request to succeed."]
pub struct EgressFrom {
    #[serde(rename = "identities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of identities that are allowed access through this [EgressPolicy]. Should be in the format of email address. The email address should represent individual user or service account only."]
    pub identities: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "identityType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the type of identities that are allowed access to outside the perimeter. If left unspecified, then members of `identities` field will be allowed access."]
    pub identity_type: ::std::option::Option<EgressFromIdentityTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Specifies the type of identities that are allowed access to outside the perimeter. If left unspecified, then members of `identities` field will be allowed access."]
pub enum EgressFromIdentityTypeEnum {
    #[serde(rename = "IDENTITY_TYPE_UNSPECIFIED")]
    #[doc = "No blanket identity group specified."]
    IdentityTypeUnspecified,
    #[serde(rename = "ANY_IDENTITY")]
    #[doc = "Authorize access from all identities outside the perimeter."]
    AnyIdentity,
    #[serde(rename = "ANY_USER_ACCOUNT")]
    #[doc = "Authorize access from all human users outside the perimeter."]
    AnyUserAccount,
    #[serde(rename = "ANY_SERVICE_ACCOUNT")]
    #[doc = "Authorize access from all service accounts outside the perimeter."]
    AnyServiceAccount,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Policy for egress from perimeter. EgressPolicies match requests based on `egress_from` and `egress_to` stanzas. For an EgressPolicy to match, both `egress_from` and `egress_to` stanzas must be matched. If an EgressPolicy matches a request, the request is allowed to span the ServicePerimeter boundary. For example, an EgressPolicy can be used to allow VMs on networks within the ServicePerimeter to access a defined set of projects outside the perimeter in certain contexts (e.g. to read data from a Cloud Storage bucket or query against a BigQuery dataset). EgressPolicies are concerned with the *resources* that a request relates as well as the API services and API actions being used. They do not related to the direction of data movement. More detailed documentation for this concept can be found in the descriptions of EgressFrom and EgressTo."]
pub struct EgressPolicy {
    #[serde(rename = "egressFrom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines conditions on the source of a request causing this EgressPolicy to apply."]
    pub egress_from: ::std::option::Option<::std::boxed::Box<EgressFrom>>,
    #[serde(rename = "egressTo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines the conditions on the ApiOperation and destination resources that cause this EgressPolicy to apply."]
    pub egress_to: ::std::option::Option<::std::boxed::Box<EgressTo>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines the conditions under which an EgressPolicy matches a request. Conditions are based on information about the ApiOperation intended to be performed on the `resources` specified. Note that if the destination of the request is protected by a ServicePerimeter, then that ServicePerimeter must have an IngressPolicy which allows access in order for this request to succeed."]
pub struct EgressTo {
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of ApiOperations that this egress rule applies to. A request matches if it contains an operation/service in this list."]
    pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApiOperation>>>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of resources, currently only projects in the form `projects/`, that match this to stanza. A request matches if it contains a resource in this list. If `*` is specified for resources, then this EgressTo rule will authorize access to all resources outside the perimeter."]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
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
#[doc = "Restricts access to Cloud Console and Google Cloud APIs for a set of users using Context-Aware Access."]
pub struct GcpUserAccessBinding {
    #[serde(rename = "accessLevels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub access_levels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "groupKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. Google Group id whose members are subject to this binding's restrictions. See \"id\" in the [G Suite Directory API's Groups resource] (https://developers.google.com/admin-sdk/directory/v1/reference/groups#resource). If a group's email address/alias is changed, this resource will continue to point at the changed group. This field does not accept group email addresses or aliases. Example: \"01d520gv4vjcrht\""]
    pub group_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. Assigned by the server during creation. The last segment has an arbitrary length and has only URI unreserved characters (as defined by [RFC 3986 Section 2.3](https://tools.ietf.org/html/rfc3986#section-2.3)). Should not be specified by the client during creation. Example: \"organizations/256/gcpUserAccessBindings/b3-BhcX_Ud5N\""]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines the conditions under which an IngressPolicy matches a request. Conditions are based on information about the source of the request."]
pub struct IngressFrom {
    #[serde(rename = "identities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of identities that are allowed access through this ingress policy. Should be in the format of email address. The email address should represent individual user or service account only."]
    pub identities: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "identityType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the type of identities that are allowed access from outside the perimeter. If left unspecified, then members of `identities` field will be allowed access."]
    pub identity_type: ::std::option::Option<IngressFromIdentityTypeEnum>,
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sources that this IngressPolicy authorizes access from."]
    pub sources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IngressSource>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Specifies the type of identities that are allowed access from outside the perimeter. If left unspecified, then members of `identities` field will be allowed access."]
pub enum IngressFromIdentityTypeEnum {
    #[serde(rename = "IDENTITY_TYPE_UNSPECIFIED")]
    #[doc = "No blanket identity group specified."]
    IdentityTypeUnspecified,
    #[serde(rename = "ANY_IDENTITY")]
    #[doc = "Authorize access from all identities outside the perimeter."]
    AnyIdentity,
    #[serde(rename = "ANY_USER_ACCOUNT")]
    #[doc = "Authorize access from all human users outside the perimeter."]
    AnyUserAccount,
    #[serde(rename = "ANY_SERVICE_ACCOUNT")]
    #[doc = "Authorize access from all service accounts outside the perimeter."]
    AnyServiceAccount,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Policy for ingress into ServicePerimeter. IngressPolicies match requests based on `ingress_from` and `ingress_to` stanzas. For an ingress policy to match, both the `ingress_from` and `ingress_to` stanzas must be matched. If an IngressPolicy matches a request, the request is allowed through the perimeter boundary from outside the perimeter. For example, access from the internet can be allowed either based on an AccessLevel or, for traffic hosted on Google Cloud, the project of the source network. For access from private networks, using the project of the hosting network is required. Individual ingress policies can be limited by restricting which services and/or actions they match using the `ingress_to` field."]
pub struct IngressPolicy {
    #[serde(rename = "ingressFrom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines the conditions on the source of a request causing this IngressPolicy to apply."]
    pub ingress_from: ::std::option::Option<::std::boxed::Box<IngressFrom>>,
    #[serde(rename = "ingressTo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines the conditions on the ApiOperation and request destination that cause this IngressPolicy to apply."]
    pub ingress_to: ::std::option::Option<::std::boxed::Box<IngressTo>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The source that IngressPolicy authorizes access from."]
pub struct IngressSource {
    #[serde(rename = "accessLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An AccessLevel resource name that allow resources within the ServicePerimeters to be accessed from the internet. AccessLevels listed must be in the same policy as this ServicePerimeter. Referencing a nonexistent AccessLevel will cause an error. If no AccessLevel names are listed, resources within the perimeter can only be accessed via Google Cloud calls with request origins within the perimeter. Example: `accessPolicies/MY_POLICY/accessLevels/MY_LEVEL`. If `*` is specified, then all IngressSources will be allowed."]
    pub access_level: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Google Cloud resource that is allowed to ingress the perimeter. Requests from these resources will be allowed to access perimeter data. Currently only projects are allowed. Format: `projects/{project_number}` The project may be in any Google Cloud organization, not just the organization that the perimeter is defined in. `*` is not allowed, the case of allowing all Google Cloud resources only is not supported."]
    pub resource: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines the conditions under which an IngressPolicy matches a request. Conditions are based on information about the ApiOperation intended to be performed on the destination of the request."]
pub struct IngressTo {
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of ApiOperations the sources specified in corresponding IngressFrom are allowed to perform in this ServicePerimeter."]
    pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApiOperation>>>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of resources, currently only projects in the form `projects/`, protected by this ServicePerimeter that are allowed to be accessed by sources defined in the corresponding IngressFrom. A request matches if it contains a resource in this list. If `*` is specified for resources, then this IngressTo rule will authorize access to all resources inside the perimeter, provided that the request also matches the `operations` field."]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response to `ListAccessLevelsRequest`."]
pub struct ListAccessLevelsResponse {
    #[serde(rename = "accessLevels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of the Access Level instances."]
    pub access_levels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccessLevel>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token to retrieve the next page of results. If the value is empty, no further results remain."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response to `ListAccessPoliciesRequest`."]
pub struct ListAccessPoliciesResponse {
    #[serde(rename = "accessPolicies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of the AccessPolicy instances."]
    pub access_policies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccessPolicy>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token to retrieve the next page of results. If the value is empty, no further results remain."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response of ListGcpUserAccessBindings."]
pub struct ListGcpUserAccessBindingsResponse {
    #[serde(rename = "gcpUserAccessBindings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "GcpUserAccessBinding"]
    pub gcp_user_access_bindings:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GcpUserAccessBinding>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to get the next page of items. If blank, there are no more items."]
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
#[doc = "A response to `ListServicePerimetersRequest`."]
pub struct ListServicePerimetersResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token to retrieve the next page of results. If the value is empty, no further results remain."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "servicePerimeters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of the Service Perimeter instances."]
    pub service_perimeters:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ServicePerimeter>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An allowed method or permission of a service specified in ApiOperation."]
pub struct MethodSelector {
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value for `method` should be a valid method name for the corresponding `service_name` in ApiOperation. If `*` used as value for `method`, then ALL methods and permissions are allowed."]
    pub method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value for `permission` should be a valid Cloud IAM permission for the corresponding `service_name` in ApiOperation."]
    pub permission: ::std::option::Option<::std::string::String>,
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
#[doc = "A restriction on the OS type and version of devices making requests."]
pub struct OsConstraint {
    #[serde(rename = "minimumVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum allowed OS version. If not set, any version of this OS satisfies the constraint. Format: `\"major.minor.patch\"`. Examples: `\"10.5.301\"`, `\"9.2.1\"`."]
    pub minimum_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "osType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The allowed OS type."]
    pub os_type: ::std::option::Option<OsConstraintOsTypeEnum>,
    #[serde(rename = "requireVerifiedChromeOs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only allows requests from devices with a verified Chrome OS. Verifications includes requirements that the device is enterprise-managed, conformant to domain policies, and the caller has permission to call the API targeted by the request."]
    pub require_verified_chrome_os: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The allowed OS type."]
pub enum OsConstraintOsTypeEnum {
    #[serde(rename = "OS_UNSPECIFIED")]
    #[doc = "The operating system of the device is not specified or not known."]
    OsUnspecified,
    #[serde(rename = "DESKTOP_MAC")]
    #[doc = "A desktop Mac operating system."]
    DesktopMac,
    #[serde(rename = "DESKTOP_WINDOWS")]
    #[doc = "A desktop Windows operating system."]
    DesktopWindows,
    #[serde(rename = "DESKTOP_LINUX")]
    #[doc = "A desktop Linux operating system."]
    DesktopLinux,
    #[serde(rename = "DESKTOP_CHROME_OS")]
    #[doc = "A desktop ChromeOS operating system."]
    DesktopChromeOs,
    #[serde(rename = "ANDROID")]
    #[doc = "An Android operating system."]
    Android,
    #[serde(rename = "IOS")]
    #[doc = "An iOS operating system."]
    Ios,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to replace all existing Access Levels in an Access Policy with the Access Levels provided. This is done atomically."]
pub struct ReplaceAccessLevelsRequest {
    #[serde(rename = "accessLevels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The desired Access Levels that should replace all existing Access Levels in the Access Policy."]
    pub access_levels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccessLevel>>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The etag for the version of the Access Policy that this replace operation is to be performed on. If, at the time of replace, the etag for the Access Policy stored in Access Context Manager is different from the specified etag, then the replace operation will not be performed and the call will fail. This field is not required. If etag is not provided, the operation will be performed as if a valid etag is provided."]
    pub etag: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response to ReplaceAccessLevelsRequest. This will be put inside of Operation.response field."]
pub struct ReplaceAccessLevelsResponse {
    #[serde(rename = "accessLevels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of the Access Level instances."]
    pub access_levels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccessLevel>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to replace all existing Service Perimeters in an Access Policy with the Service Perimeters provided. This is done atomically."]
pub struct ReplaceServicePerimetersRequest {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The etag for the version of the Access Policy that this replace operation is to be performed on. If, at the time of replace, the etag for the Access Policy stored in Access Context Manager is different from the specified etag, then the replace operation will not be performed and the call will fail. This field is not required. If etag is not provided, the operation will be performed as if a valid etag is provided."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "servicePerimeters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The desired Service Perimeters that should replace all existing Service Perimeters in the Access Policy."]
    pub service_perimeters:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ServicePerimeter>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response to ReplaceServicePerimetersRequest. This will be put inside of Operation.response field."]
pub struct ReplaceServicePerimetersResponse {
    #[serde(rename = "servicePerimeters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of the Service Perimeter instances."]
    pub service_perimeters:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ServicePerimeter>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`ServicePerimeter` describes a set of Google Cloud resources which can freely import and export data amongst themselves, but not export outside of the `ServicePerimeter`. If a request with a source within this `ServicePerimeter` has a target outside of the `ServicePerimeter`, the request will be blocked. Otherwise the request is allowed. There are two types of Service Perimeter - Regular and Bridge. Regular Service Perimeters cannot overlap, a single Google Cloud project can only belong to a single regular Service Perimeter. Service Perimeter Bridges can contain only Google Cloud projects as members, a single Google Cloud project may belong to multiple Service Perimeter Bridges."]
pub struct ServicePerimeter {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the `ServicePerimeter` and its use. Does not affect behavior."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Resource name for the ServicePerimeter. The `short_name` component must begin with a letter and only include alphanumeric and '_'. Format: `accessPolicies/{policy_id}/servicePerimeters/{short_name}`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "perimeterType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Perimeter type indicator. A single project is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, the restricted service list as well as access level lists must be empty."]
    pub perimeter_type: ::std::option::Option<ServicePerimeterPerimeterTypeEnum>,
    #[serde(rename = "spec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Proposed (or dry run) ServicePerimeter configuration. This configuration allows to specify and test ServicePerimeter configuration without enforcing actual access restrictions. Only allowed to be set when the \"use_explicit_dry_run_spec\" flag is set."]
    pub spec: ::std::option::Option<::std::boxed::Box<ServicePerimeterConfig>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Current ServicePerimeter configuration. Specifies sets of resources, restricted services and access levels that determine perimeter content and boundaries."]
    pub status: ::std::option::Option<::std::boxed::Box<ServicePerimeterConfig>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human readable title. Must be unique within the Policy."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "useExplicitDryRunSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Use explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists for all Service Perimeters, and that spec is identical to the status for those Service Perimeters. When this flag is set, it inhibits the generation of the implicit spec, thereby allowing the user to explicitly provide a configuration (\"spec\") to use in a dry-run version of the Service Perimeter. This allows the user to test changes to the enforced config (\"status\") without actually enforcing them. This testing is done through analyzing the differences between currently enforced and suggested restrictions. use_explicit_dry_run_spec must bet set to True if any of the fields in the spec are set to non-default values."]
    pub use_explicit_dry_run_spec: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Perimeter type indicator. A single project is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, the restricted service list as well as access level lists must be empty."]
pub enum ServicePerimeterPerimeterTypeEnum {
    #[serde(rename = "PERIMETER_TYPE_REGULAR")]
    #[doc = "Regular Perimeter."]
    PerimeterTypeRegular,
    #[serde(rename = "PERIMETER_TYPE_BRIDGE")]
    #[doc = "Perimeter Bridge."]
    PerimeterTypeBridge,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`ServicePerimeterConfig` specifies a set of Google Cloud resources that describe specific Service Perimeter configuration."]
pub struct ServicePerimeterConfig {
    #[serde(rename = "accessLevels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of `AccessLevel` resource names that allow resources within the `ServicePerimeter` to be accessed from the internet. `AccessLevels` listed must be in the same policy as this `ServicePerimeter`. Referencing a nonexistent `AccessLevel` is a syntax error. If no `AccessLevel` names are listed, resources within the perimeter can only be accessed via Google Cloud calls with request origins within the perimeter. Example: `\"accessPolicies/MY_POLICY/accessLevels/MY_LEVEL\"`. For Service Perimeter Bridge, must be empty."]
    pub access_levels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "egressPolicies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of EgressPolicies to apply to the perimeter. A perimeter may have multiple EgressPolicies, each of which is evaluated separately. Access is granted if any EgressPolicy grants it. Must be empty for a perimeter bridge."]
    pub egress_policies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EgressPolicy>>>,
    #[serde(rename = "ingressPolicies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of IngressPolicies to apply to the perimeter. A perimeter may have multiple IngressPolicies, each of which is evaluated separately. Access is granted if any Ingress Policy grants it. Must be empty for a perimeter bridge."]
    pub ingress_policies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IngressPolicy>>>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of Google Cloud resources that are inside of the service perimeter. Currently only projects are allowed. Format: `projects/{project_number}`"]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "restrictedServices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Cloud services that are subject to the Service Perimeter restrictions. For example, if `storage.googleapis.com` is specified, access to the storage buckets inside the perimeter must meet the perimeter's access restrictions."]
    pub restricted_services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "vpcAccessibleServices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for APIs allowed within Perimeter."]
    pub vpc_accessible_services: ::std::option::Option<::std::boxed::Box<VpcAccessibleServices>>,
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
#[doc = "Specifies how APIs are allowed to communicate within the Service Perimeter."]
pub struct VpcAccessibleServices {
    #[serde(rename = "allowedServices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of APIs usable within the Service Perimeter. Must be empty unless 'enable_restriction' is True. You can specify a list of individual services, as well as include the 'RESTRICTED-SERVICES' value, which automatically includes all of the services protected by the perimeter."]
    pub allowed_services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "enableRestriction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to restrict API calls within the Service Perimeter to the list of APIs specified in 'allowed_services'."]
    pub enable_restriction: ::std::option::Option<::std::primitive::bool>,
}