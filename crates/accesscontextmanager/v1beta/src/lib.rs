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
    #[doc = "Required. Resource name for the Access Level. The `short_name` component must begin with a letter and only include alphanumeric and '_'. Format: `accessPolicies/{policy_id}/accessLevels/{short_name}`. The maximum length // of the `short_name` component is 50 characters."]
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
    #[doc = "Perimeter type indicator. A single project is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, restricted/unrestricted service lists as well as access lists must be empty."]
    pub perimeter_type: ::std::option::Option<ServicePerimeterPerimeterTypeEnum>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Current ServicePerimeter configuration. Specifies sets of resources, restricted/unrestricted services and access levels that determine perimeter content and boundaries."]
    pub status: ::std::option::Option<::std::boxed::Box<ServicePerimeterConfig>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human readable title. Must be unique within the Policy."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Perimeter type indicator. A single project is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, restricted/unrestricted service lists as well as access lists must be empty."]
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
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of Google Cloud resources that are inside of the service perimeter. Currently only projects are allowed. Format: `projects/{project_number}`"]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "restrictedServices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Cloud services that are subject to the Service Perimeter restrictions. Must contain a list of services. For example, if `storage.googleapis.com` is specified, access to the storage buckets inside the perimeter must meet the perimeter's access restrictions."]
    pub restricted_services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "unrestrictedServices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Cloud services that are not subject to the Service Perimeter restrictions. Deprecated. Must be set to a single wildcard \"*\". The wildcard means that unless explicitly specified by \"restricted_services\" list, any service is treated as unrestricted."]
    pub unrestricted_services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "vpcAccessibleServices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Beta. Configuration for APIs allowed within Perimeter."]
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
