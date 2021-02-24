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
        serde_json::from_str(&"\"json\"").unwrap()
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
    pub mod assets {
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
                    #[serde(rename = "assetTypes")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A list of asset types to take a snapshot for. For example: \"compute.googleapis.com/Disk\". Regular expression is also supported. For example: * \"compute.googleapis.com.*\" snapshots resources whose asset type starts with \"compute.googleapis.com\". * \".*Instance\" snapshots resources whose asset type ends with \"Instance\". * \".*Instance.*\" snapshots resources whose asset type contains \"Instance\". See [RE2](https://github.com/google/re2/wiki/Syntax) for all supported regular expression syntax. If the regular expression does not match any supported asset type, an INVALID_ARGUMENT error will be returned. If specified, only matching assets will be returned, otherwise, it will snapshot all asset types. See [Introduction to Cloud Asset Inventory](https://cloud.google.com/asset-inventory/docs/overview) for all supported asset types."]
                    pub asset_types: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "contentType")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Asset content type. If not specified, no content but the asset name will be returned."]
                    pub content_type: ::std::option::Option<QueryParametersContentTypeEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of assets to be returned in a single response. Default is 100, minimum is 1, and maximum is 1000."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The `next_page_token` returned from the previous `ListAssetsResponse`, or unspecified for the first `ListAssetsRequest`. It is a continuation of a prior `ListAssets` call, and the API should return the next page of assets."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "readTime")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Timestamp to take an asset snapshot. This can only be set to a timestamp between the current time and the current time minus 35 days (inclusive). If not specified, the current time will be used. Due to delays in resource data collection and indexing, there is a volatile window during which running the same query may get different results."]
                    pub read_time: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Asset content type. If not specified, no content but the asset name will be returned."]
                pub enum QueryParametersContentTypeEnum {
                    #[serde(rename = "CONTENT_TYPE_UNSPECIFIED")]
                    #[doc = "Unspecified content type."]
                    ContentTypeUnspecified,
                    #[serde(rename = "RESOURCE")]
                    #[doc = "Resource metadata."]
                    Resource,
                    #[serde(rename = "IAM_POLICY")]
                    #[doc = "The actual IAM policy set on a resource."]
                    IamPolicy,
                    #[serde(rename = "ORG_POLICY")]
                    #[doc = "The Cloud Organization Policy set on an asset."]
                    OrgPolicy,
                    #[serde(rename = "ACCESS_POLICY")]
                    #[doc = "The Cloud Access context mananger Policy set on an asset."]
                    AccessPolicy,
                }
                impl ::std::default::Default for QueryParametersContentTypeEnum {
                    fn default() -> Self {
                        Self::ContentTypeUnspecified
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
    #[doc = "An asset in Google Cloud. An asset can be any resource in the Google Cloud [resource hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy), a resource outside the Google Cloud resource hierarchy (such as Google Kubernetes Engine clusters and objects), or a policy (e.g. Cloud IAM policy). See [Supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types) for more information."]
    pub struct Asset {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Please also refer to the [access level user guide](https://cloud.google.com/access-context-manager/docs/overview#access-levels)."]
        pub access_level: ::std::option::Option<
            ::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1AccessLevel>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Please also refer to the [access policy user guide](https://cloud.google.com/access-context-manager/docs/overview#access-policies)."]
        pub access_policy: ::std::option::Option<
            ::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1AccessPolicy>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ancestors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ancestry path of an asset in Google Cloud [resource hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy), represented as a list of relative resource names. An ancestry path starts with the closest ancestor in the hierarchy and ends at root. If the asset is a project, folder, or organization, the ancestry path starts from the asset itself. Example: `[\"projects/123456789\", \"folders/5432\", \"organizations/1234\"]`"]
        pub ancestors: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assetType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the asset. Example: `compute.googleapis.com/Disk` See [Supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types) for more information."]
        pub asset_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iamPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A representation of the Cloud IAM policy set on a Google Cloud resource. There can be a maximum of one Cloud IAM policy set on any given resource. In addition, Cloud IAM policies inherit their granted access scope from any policies set on parent resources in the resource hierarchy. Therefore, the effectively policy is the union of both the policy set on this resource and each policy set on all of the resource's ancestry resource levels in the hierarchy. See [this topic](https://cloud.google.com/iam/docs/policies#inheritance) for more information."]
        pub iam_policy: ::std::option::Option<::std::boxed::Box<Policy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full name of the asset. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1` See [Resource names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orgPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A representation of an [organization policy](https://cloud.google.com/resource-manager/docs/organization-policy/overview#organization_policy). There can be more than one organization policy with different constraints set on a given resource."]
        pub org_policy:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudOrgpolicyV1Policy>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A representation of the resource."]
        pub resource: ::std::option::Option<::std::boxed::Box<Resource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "servicePerimeter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Please also refer to the [service perimeter user guide](https://cloud.google.com/vpc-service-controls/docs/overview)."]
        pub service_perimeter: ::std::option::Option<
            ::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1ServicePerimeter>,
        >,
    }
    impl Asset {
        pub fn builder() -> AssetBuilder {
            AssetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { \"audit_configs\": [ { \"service\": \"allServices\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" }, { \"log_type\": \"ADMIN_READ\" } ] }, { \"service\": \"sampleservice.googleapis.com\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\" }, { \"log_type\": \"DATA_WRITE\", \"exempted_members\": [ \"user:aliya@example.com\" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging."]
    pub struct AuditConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "auditLogConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The configuration for logging of each type of permission."]
        pub audit_log_configs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditLogConfig>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "service")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services."]
        pub service: ::std::option::Option<::std::string::String>,
    }
    impl AuditConfig {
        pub fn builder() -> AuditConfigBuilder {
            AuditConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides the configuration for logging a type of permissions. Example: { \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging."]
    pub struct AuditLogConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exemptedMembers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members."]
        pub exempted_members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The log type that this config enables."]
        pub log_type: ::std::option::Option<AuditLogConfigLogTypeEnum>,
    }
    impl AuditLogConfig {
        pub fn builder() -> AuditLogConfigBuilder {
            AuditLogConfigBuilder::default()
        }
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
    impl ::std::default::Default for AuditLogConfigLogTypeEnum {
        fn default() -> Self {
            Self::LogTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Associates `members` with a `role`."]
    pub struct Binding {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the members in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        pub condition: ::std::option::Option<::std::boxed::Box<Expr>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "members")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the identities requesting access for a Cloud Platform resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. "]
        pub members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Role that is assigned to `members`. For example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
        pub role: ::std::option::Option<::std::string::String>,
    }
    impl Binding {
        pub fn builder() -> BindingBuilder {
            BindingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: \"Summary size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() < 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\" expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description: \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information."]
    pub struct Expr {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expression")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Textual representation of an expression in Common Expression Language syntax."]
        pub expression: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl Expr {
        pub fn builder() -> ExprBuilder {
            ExprBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Used in `policy_type` to specify how `boolean_policy` will behave at this resource."]
    pub struct GoogleCloudOrgpolicyV1BooleanPolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enforced")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If `true`, then the `Policy` is enforced. If `false`, then any configuration is acceptable. Suppose you have a `Constraint` `constraints/compute.disableSerialPortAccess` with `constraint_default` set to `ALLOW`. A `Policy` for that `Constraint` exhibits the following behavior: - If the `Policy` at this resource has enforced set to `false`, serial port connection attempts will be allowed. - If the `Policy` at this resource has enforced set to `true`, serial port connection attempts will be refused. - If the `Policy` at this resource is `RestoreDefault`, serial port connection attempts will be allowed. - If no `Policy` is set at this resource or anywhere higher in the resource hierarchy, serial port connection attempts will be allowed. - If no `Policy` is set at this resource, but one exists higher in the resource hierarchy, the behavior is as if the`Policy` were set at this resource. The following examples demonstrate the different possible layerings: Example 1 (nearest `Constraint` wins): `organizations/foo` has a `Policy` with: {enforced: false} `projects/bar` has no `Policy` set. The constraint at `projects/bar` and `organizations/foo` will not be enforced. Example 2 (enforcement gets replaced): `organizations/foo` has a `Policy` with: {enforced: false} `projects/bar` has a `Policy` with: {enforced: true} The constraint at `organizations/foo` is not enforced. The constraint at `projects/bar` is enforced. Example 3 (RestoreDefault): `organizations/foo` has a `Policy` with: {enforced: true} `projects/bar` has a `Policy` with: {RestoreDefault: {}} The constraint at `organizations/foo` is enforced. The constraint at `projects/bar` is not enforced, because `constraint_default` for the `Constraint` is `ALLOW`."]
        pub enforced: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudOrgpolicyV1BooleanPolicy {
        pub fn builder() -> GoogleCloudOrgpolicyV1BooleanPolicyBuilder {
            GoogleCloudOrgpolicyV1BooleanPolicyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Used in `policy_type` to specify how `list_policy` behaves at this resource. `ListPolicy` can define specific values and subtrees of Cloud Resource Manager resource hierarchy (`Organizations`, `Folders`, `Projects`) that are allowed or denied by setting the `allowed_values` and `denied_values` fields. This is achieved by using the `under:` and optional `is:` prefixes. The `under:` prefix is used to denote resource subtree values. The `is:` prefix is used to denote specific values, and is required only if the value contains a \":\". Values prefixed with \"is:\" are treated the same as values with no prefix. Ancestry subtrees must be in one of the following formats: - \"projects/\", e.g. \"projects/tokyo-rain-123\" - \"folders/\", e.g. \"folders/1234\" - \"organizations/\", e.g. \"organizations/1234\" The `supports_under` field of the associated `Constraint` defines whether ancestry prefixes can be used. You can set `allowed_values` and `denied_values` in the same `Policy` if `all_values` is `ALL_VALUES_UNSPECIFIED`. `ALLOW` or `DENY` are used to allow or deny all values. If `all_values` is set to either `ALLOW` or `DENY`, `allowed_values` and `denied_values` must be unset."]
    pub struct GoogleCloudOrgpolicyV1ListPolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The policy all_values state."]
        pub all_values: ::std::option::Option<GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of values allowed at this resource. Can only be set if `all_values` is set to `ALL_VALUES_UNSPECIFIED`."]
        pub allowed_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deniedValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of values denied at this resource. Can only be set if `all_values` is set to `ALL_VALUES_UNSPECIFIED`."]
        pub denied_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inheritFromParent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines the inheritance behavior for this `Policy`. By default, a `ListPolicy` set at a resource supersedes any `Policy` set anywhere up the resource hierarchy. However, if `inherit_from_parent` is set to `true`, then the values from the effective `Policy` of the parent resource are inherited, meaning the values set in this `Policy` are added to the values inherited up the hierarchy. Setting `Policy` hierarchies that inherit both allowed values and denied values isn't recommended in most circumstances to keep the configuration simple and understandable. However, it is possible to set a `Policy` with `allowed_values` set that inherits a `Policy` with `denied_values` set. In this case, the values that are allowed must be in `allowed_values` and not present in `denied_values`. For example, suppose you have a `Constraint` `constraints/serviceuser.services`, which has a `constraint_type` of `list_constraint`, and with `constraint_default` set to `ALLOW`. Suppose that at the Organization level, a `Policy` is applied that restricts the allowed API activations to {`E1`, `E2`}. Then, if a `Policy` is applied to a project below the Organization that has `inherit_from_parent` set to `false` and field all_values set to DENY, then an attempt to activate any API will be denied. The following examples demonstrate different possible layerings for `projects/bar` parented by `organizations/foo`: Example 1 (no inherited values): `organizations/foo` has a `Policy` with values: {allowed_values: \"E1\" allowed_values:\"E2\"} `projects/bar` has `inherit_from_parent` `false` and values: {allowed_values: \"E3\" allowed_values: \"E4\"} The accepted values at `organizations/foo` are `E1`, `E2`. The accepted values at `projects/bar` are `E3`, and `E4`. Example 2 (inherited values): `organizations/foo` has a `Policy` with values: {allowed_values: \"E1\" allowed_values:\"E2\"} `projects/bar` has a `Policy` with values: {value: \"E3\" value: \"E4\" inherit_from_parent: true} The accepted values at `organizations/foo` are `E1`, `E2`. The accepted values at `projects/bar` are `E1`, `E2`, `E3`, and `E4`. Example 3 (inheriting both allowed and denied values): `organizations/foo` has a `Policy` with values: {allowed_values: \"E1\" allowed_values: \"E2\"} `projects/bar` has a `Policy` with: {denied_values: \"E1\"} The accepted values at `organizations/foo` are `E1`, `E2`. The value accepted at `projects/bar` is `E2`. Example 4 (RestoreDefault): `organizations/foo` has a `Policy` with values: {allowed_values: \"E1\" allowed_values:\"E2\"} `projects/bar` has a `Policy` with values: {RestoreDefault: {}} The accepted values at `organizations/foo` are `E1`, `E2`. The accepted values at `projects/bar` are either all or none depending on the value of `constraint_default` (if `ALLOW`, all; if `DENY`, none). Example 5 (no policy inherits parent policy): `organizations/foo` has no `Policy` set. `projects/bar` has no `Policy` set. The accepted values at both levels are either all or none depending on the value of `constraint_default` (if `ALLOW`, all; if `DENY`, none). Example 6 (ListConstraint allowing all): `organizations/foo` has a `Policy` with values: {allowed_values: \"E1\" allowed_values: \"E2\"} `projects/bar` has a `Policy` with: {all: ALLOW} The accepted values at `organizations/foo` are `E1`, E2`. Any value is accepted at `projects/bar`. Example 7 (ListConstraint allowing none): `organizations/foo` has a `Policy` with values: {allowed_values: \"E1\" allowed_values: \"E2\"} `projects/bar` has a `Policy` with: {all: DENY} The accepted values at `organizations/foo` are `E1`, E2`. No value is accepted at `projects/bar`. Example 10 (allowed and denied subtrees of Resource Manager hierarchy): Given the following resource hierarchy O1->{F1, F2}; F1->{P1}; F2->{P2, P3}, `organizations/foo` has a `Policy` with values: {allowed_values: \"under:organizations/O1\"} `projects/bar` has a `Policy` with: {allowed_values: \"under:projects/P3\"} {denied_values: \"under:folders/F2\"} The accepted values at `organizations/foo` are `organizations/O1`, `folders/F1`, `folders/F2`, `projects/P1`, `projects/P2`, `projects/P3`. The accepted values at `projects/bar` are `organizations/O1`, `folders/F1`, `projects/P1`."]
        pub inherit_from_parent: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suggestedValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The Google Cloud Console will try to default to a configuration that matches the value specified in this `Policy`. If `suggested_value` is not set, it will inherit the value specified higher in the hierarchy, unless `inherit_from_parent` is `false`."]
        pub suggested_value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudOrgpolicyV1ListPolicy {
        pub fn builder() -> GoogleCloudOrgpolicyV1ListPolicyBuilder {
            GoogleCloudOrgpolicyV1ListPolicyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The policy all_values state."]
    pub enum GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum {
        #[serde(rename = "ALL_VALUES_UNSPECIFIED")]
        #[doc = "Indicates that allowed_values or denied_values must be set."]
        AllValuesUnspecified,
        #[serde(rename = "ALLOW")]
        #[doc = "A policy with this set allows all values."]
        Allow,
        #[serde(rename = "DENY")]
        #[doc = "A policy with this set denies all values."]
        Deny,
    }
    impl ::std::default::Default for GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum {
        fn default() -> Self {
            Self::AllValuesUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines a Cloud Organization `Policy` which is used to specify `Constraints` for configurations of Cloud Platform resources."]
    pub struct GoogleCloudOrgpolicyV1Policy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "booleanPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For boolean `Constraints`, whether to enforce the `Constraint` or not."]
        pub boolean_policy:
            ::std::option::Option<::std::boxed::Box<GoogleCloudOrgpolicyV1BooleanPolicy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "constraint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the `Constraint` the `Policy` is configuring, for example, `constraints/serviceuser.services`. A [list of available constraints](/resource-manager/docs/organization-policy/org-policy-constraints) is available. Immutable after creation."]
        pub constraint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An opaque tag indicating the current version of the `Policy`, used for concurrency control. When the `Policy` is returned from either a `GetPolicy` or a `ListOrgPolicy` request, this `etag` indicates the version of the current `Policy` to use when executing a read-modify-write loop. When the `Policy` is returned from a `GetEffectivePolicy` request, the `etag` will be unset. When the `Policy` is used in a `SetOrgPolicy` method, use the `etag` value that was returned from a `GetOrgPolicy` request as part of a read-modify-write loop for concurrency control. Not setting the `etag`in a `SetOrgPolicy` request will result in an unconditional write of the `Policy`."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "listPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of values either allowed or disallowed."]
        pub list_policy: ::std::option::Option<::std::boxed::Box<GoogleCloudOrgpolicyV1ListPolicy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restoreDefault")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restores the default behavior of the constraint; independent of `Constraint` type."]
        pub restore_default:
            ::std::option::Option<::std::boxed::Box<GoogleCloudOrgpolicyV1RestoreDefault>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time stamp the `Policy` was previously updated. This is set by the server, not specified by the caller, and represents the last time a call to `SetOrgPolicy` was made for that `Policy`. Any value set by the client will be ignored."]
        pub update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Version of the `Policy`. Default version is 0;"]
        pub version: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudOrgpolicyV1Policy {
        pub fn builder() -> GoogleCloudOrgpolicyV1PolicyBuilder {
            GoogleCloudOrgpolicyV1PolicyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Ignores policies set above this resource and restores the `constraint_default` enforcement behavior of the specific `Constraint` at this resource. Suppose that `constraint_default` is set to `ALLOW` for the `Constraint` `constraints/serviceuser.services`. Suppose that organization foo.com sets a `Policy` at their Organization resource node that restricts the allowed service activations to deny all service activations. They could then set a `Policy` with the `policy_type` `restore_default` on several experimental projects, restoring the `constraint_default` enforcement of the `Constraint` for only those projects, allowing those projects to have all services activated."]
    pub struct GoogleCloudOrgpolicyV1RestoreDefault {}
    impl GoogleCloudOrgpolicyV1RestoreDefault {
        pub fn builder() -> GoogleCloudOrgpolicyV1RestoreDefaultBuilder {
            GoogleCloudOrgpolicyV1RestoreDefaultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An `AccessLevel` is a label that can be applied to requests to Google Cloud services, along with a list of requirements necessary for the label to be applied."]
    pub struct GoogleIdentityAccesscontextmanagerV1AccessLevel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A `BasicLevel` composed of `Conditions`."]
        pub basic: ::std::option::Option<
            ::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1BasicLevel>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "custom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A `CustomLevel` written in the Common Expression Language."]
        pub custom: ::std::option::Option<
            ::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1CustomLevel>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the `AccessLevel` and its use. Does not affect behavior."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Resource name for the Access Level. The `short_name` component must begin with a letter and only include alphanumeric and '_'. Format: `accessPolicies/{policy_id}/accessLevels/{short_name}`. The maximum length of the `short_name` component is 50 characters."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human readable title. Must be unique within the Policy."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleIdentityAccesscontextmanagerV1AccessLevel {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1AccessLevelBuilder {
            GoogleIdentityAccesscontextmanagerV1AccessLevelBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`AccessPolicy` is a container for `AccessLevels` (which define the necessary attributes to use Google Cloud services) and `ServicePerimeters` (which define regions of services able to freely pass data within a perimeter). An access policy is globally visible within an organization, and the restrictions it specifies apply to all projects within an organization."]
    pub struct GoogleIdentityAccesscontextmanagerV1AccessPolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. An opaque identifier for the current version of the `AccessPolicy`. This will always be a strongly validated etag, meaning that two Access Polices will be identical if and only if their etags are identical. Clients should not expect this to be in any specific format."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource name of the `AccessPolicy`. Format: `accessPolicies/{policy_id}`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The parent of this `AccessPolicy` in the Cloud Resource Hierarchy. Currently immutable once created. Format: `organizations/{organization_id}`"]
        pub parent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Human readable title. Does not affect behavior."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleIdentityAccesscontextmanagerV1AccessPolicy {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1AccessPolicyBuilder {
            GoogleIdentityAccesscontextmanagerV1AccessPolicyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identification for an API Operation."]
    pub struct GoogleIdentityAccesscontextmanagerV1ApiOperation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "methodSelectors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "API methods or permissions to allow. Method or permission must belong to the service specified by `service_name` field. A single MethodSelector entry with `*` specified for the `method` field will allow all methods AND permissions for the service specified in `service_name`."]
        pub method_selectors: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1MethodSelector>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the API whose methods or permissions the IngressPolicy or EgressPolicy want to allow. A single ApiOperation with `service_name` field set to `*` will allow all methods AND permissions for all services."]
        pub service_name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleIdentityAccesscontextmanagerV1ApiOperation {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1ApiOperationBuilder {
            GoogleIdentityAccesscontextmanagerV1ApiOperationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`BasicLevel` is an `AccessLevel` using a set of recommended features."]
    pub struct GoogleIdentityAccesscontextmanagerV1BasicLevel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "combiningFunction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How the `conditions` list should be combined to determine if a request is granted this `AccessLevel`. If AND is used, each `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. If OR is used, at least one `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. Default behavior is AND."]
        pub combining_function: ::std::option::Option<
            GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conditions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A list of requirements for the `AccessLevel` to be granted."]
        pub conditions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1Condition>>,
        >,
    }
    impl GoogleIdentityAccesscontextmanagerV1BasicLevel {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1BasicLevelBuilder {
            GoogleIdentityAccesscontextmanagerV1BasicLevelBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How the `conditions` list should be combined to determine if a request is granted this `AccessLevel`. If AND is used, each `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. If OR is used, at least one `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. Default behavior is AND."]
    pub enum GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum {
        #[serde(rename = "AND")]
        #[doc = "All `Conditions` must be true for the `BasicLevel` to be true."]
        And,
        #[serde(rename = "OR")]
        #[doc = "If at least one `Condition` is true, then the `BasicLevel` is true."]
        Or,
    }
    impl ::std::default::Default
        for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum
    {
        fn default() -> Self {
            Self::And
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A condition necessary for an `AccessLevel` to be granted. The Condition is an AND over its fields. So a Condition is true if: 1) the request IP is from one of the listed subnetworks AND 2) the originating device complies with the listed device policy AND 3) all listed access levels are granted AND 4) the request was sent at a time allowed by the DateTimeRestriction."]
    pub struct GoogleIdentityAccesscontextmanagerV1Condition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "devicePolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device specific restrictions, all restrictions must hold for the Condition to be true. If not specified, all devices are allowed."]
        pub device_policy: ::std::option::Option<
            ::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1DevicePolicy>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipSubnetworks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "CIDR block IP subnetwork specification. May be IPv4 or IPv6. Note that for a CIDR IP address block, the specified IP address portion must be properly truncated (i.e. all the host bits must be zero) or the input is considered malformed. For example, \"192.0.2.0/24\" is accepted but \"192.0.2.1/24\" is not. Similarly, for IPv6, \"2001:db8::/32\" is accepted whereas \"2001:db8::1/32\" is not. The originating IP of a request must be in one of the listed subnets in order for this Condition to be true. If empty, all IP addresses are allowed."]
        pub ip_subnetworks: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "members")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request must be made by one of the provided user or service accounts. Groups are not supported. Syntax: `user:{emailid}` `serviceAccount:{emailid}` If not specified, a request may come from any user."]
        pub members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "negate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to negate the Condition. If true, the Condition becomes a NAND over its non-empty fields, each field must be false for the Condition overall to be satisfied. Defaults to false."]
        pub negate: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request must originate from one of the provided countries/regions. Must be valid ISO 3166-1 alpha-2 codes."]
        pub regions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requiredAccessLevels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of other access levels defined in the same `Policy`, referenced by resource name. Referencing an `AccessLevel` which does not exist is an error. All access levels listed must be granted for the Condition to be true. Example: \"`accessPolicies/MY_POLICY/accessLevels/LEVEL_NAME\"`"]
        pub required_access_levels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleIdentityAccesscontextmanagerV1Condition {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1ConditionBuilder {
            GoogleIdentityAccesscontextmanagerV1ConditionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`CustomLevel` is an `AccessLevel` using the Cloud Common Expression Language to represent the necessary conditions for the level to apply to a request. See CEL spec at: https://github.com/google/cel-spec"]
    pub struct GoogleIdentityAccesscontextmanagerV1CustomLevel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expr")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A Cloud CEL expression evaluating to a boolean."]
        pub expr: ::std::option::Option<::std::boxed::Box<Expr>>,
    }
    impl GoogleIdentityAccesscontextmanagerV1CustomLevel {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1CustomLevelBuilder {
            GoogleIdentityAccesscontextmanagerV1CustomLevelBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`DevicePolicy` specifies device specific restrictions necessary to acquire a given access level. A `DevicePolicy` specifies requirements for requests from devices to be granted access levels, it does not do any enforcement on the device. `DevicePolicy` acts as an AND over all specified fields, and each repeated field is an OR over its elements. Any unset fields are ignored. For example, if the proto is { os_type : DESKTOP_WINDOWS, os_type : DESKTOP_LINUX, encryption_status: ENCRYPTED}, then the DevicePolicy will be true for requests originating from encrypted Linux desktops and encrypted Windows desktops."]
    pub struct GoogleIdentityAccesscontextmanagerV1DevicePolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedDeviceManagementLevels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allowed device management levels, an empty list allows all management levels."]
        pub allowed_device_management_levels: ::std::option::Option<
            ::std::vec::Vec<
                GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedEncryptionStatuses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allowed encryptions statuses, an empty list allows all statuses."]
        pub allowed_encryption_statuses: ::std::option::Option<
            ::std::vec::Vec<
                GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "osConstraints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allowed OS versions, an empty list allows all types and all versions."]
        pub os_constraints: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1OsConstraint>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requireAdminApproval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the device needs to be approved by the customer admin."]
        pub require_admin_approval: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requireCorpOwned")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the device needs to be corp owned."]
        pub require_corp_owned: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requireScreenlock")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not screenlock is required for the DevicePolicy to be true. Defaults to `false`."]
        pub require_screenlock: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleIdentityAccesscontextmanagerV1DevicePolicy {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1DevicePolicyBuilder {
            GoogleIdentityAccesscontextmanagerV1DevicePolicyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum {
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
    impl ::std::default::Default
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum
    {
        fn default() -> Self {
            Self::ManagementUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum {
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
    impl ::std::default::Default
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum
    {
        fn default() -> Self {
            Self::EncryptionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines the conditions under which an EgressPolicy matches a request. Conditions based on information about the source of the request. Note that if the destination of the request is protected by a ServicePerimeter, then that ServicePerimeter must have an IngressPolicy which allows access in order for this request to succeed."]
    pub struct GoogleIdentityAccesscontextmanagerV1EgressFrom {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "identities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of identities that are allowed access through this [EgressPolicy]. Should be in the format of email address. The email address should represent individual user or service account only."]
        pub identities: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "identityType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the type of identities that are allowed access to outside the perimeter. If left unspecified, then members of `identities` field will be allowed access."]
        pub identity_type:
            ::std::option::Option<GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum>,
    }
    impl GoogleIdentityAccesscontextmanagerV1EgressFrom {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1EgressFromBuilder {
            GoogleIdentityAccesscontextmanagerV1EgressFromBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies the type of identities that are allowed access to outside the perimeter. If left unspecified, then members of `identities` field will be allowed access."]
    pub enum GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum {
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
    impl ::std::default::Default for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum {
        fn default() -> Self {
            Self::IdentityTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Policy for egress from perimeter. EgressPolicies match requests based on `egress_from` and `egress_to` stanzas. For an EgressPolicy to match, both `egress_from` and `egress_to` stanzas must be matched. If an EgressPolicy matches a request, the request is allowed to span the ServicePerimeter boundary. For example, an EgressPolicy can be used to allow VMs on networks within the ServicePerimeter to access a defined set of projects outside the perimeter in certain contexts (e.g. to read data from a Cloud Storage bucket or query against a BigQuery dataset). EgressPolicies are concerned with the *resources* that a request relates as well as the API services and API actions being used. They do not related to the direction of data movement. More detailed documentation for this concept can be found in the descriptions of EgressFrom and EgressTo."]
    pub struct GoogleIdentityAccesscontextmanagerV1EgressPolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "egressFrom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines conditions on the source of a request causing this EgressPolicy to apply."]
        pub egress_from: ::std::option::Option<
            ::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1EgressFrom>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "egressTo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the conditions on the ApiOperation and destination resources that cause this EgressPolicy to apply."]
        pub egress_to:
            ::std::option::Option<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1EgressTo>>,
    }
    impl GoogleIdentityAccesscontextmanagerV1EgressPolicy {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1EgressPolicyBuilder {
            GoogleIdentityAccesscontextmanagerV1EgressPolicyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines the conditions under which an EgressPolicy matches a request. Conditions are based on information about the ApiOperation intended to be performed on the `resources` specified. Note that if the destination of the request is protected by a ServicePerimeter, then that ServicePerimeter must have an IngressPolicy which allows access in order for this request to succeed."]
    pub struct GoogleIdentityAccesscontextmanagerV1EgressTo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of ApiOperations that this egress rule applies to. A request matches if it contains an operation/service in this list."]
        pub operations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1ApiOperation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of resources, currently only projects in the form `projects/`, that match this to stanza. A request matches if it contains a resource in this list. If `*` is specified for resources, then this EgressTo rule will authorize access to all resources outside the perimeter."]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleIdentityAccesscontextmanagerV1EgressTo {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1EgressToBuilder {
            GoogleIdentityAccesscontextmanagerV1EgressToBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines the conditions under which an IngressPolicy matches a request. Conditions are based on information about the source of the request."]
    pub struct GoogleIdentityAccesscontextmanagerV1IngressFrom {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "identities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of identities that are allowed access through this ingress policy. Should be in the format of email address. The email address should represent individual user or service account only."]
        pub identities: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "identityType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the type of identities that are allowed access from outside the perimeter. If left unspecified, then members of `identities` field will be allowed access."]
        pub identity_type:
            ::std::option::Option<GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sources that this IngressPolicy authorizes access from."]
        pub sources: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1IngressSource>>,
        >,
    }
    impl GoogleIdentityAccesscontextmanagerV1IngressFrom {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1IngressFromBuilder {
            GoogleIdentityAccesscontextmanagerV1IngressFromBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies the type of identities that are allowed access from outside the perimeter. If left unspecified, then members of `identities` field will be allowed access."]
    pub enum GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum {
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
    impl ::std::default::Default for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum {
        fn default() -> Self {
            Self::IdentityTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Policy for ingress into ServicePerimeter. IngressPolicies match requests based on `ingress_from` and `ingress_to` stanzas. For an ingress policy to match, both the `ingress_from` and `ingress_to` stanzas must be matched. If an IngressPolicy matches a request, the request is allowed through the perimeter boundary from outside the perimeter. For example, access from the internet can be allowed either based on an AccessLevel or, for traffic hosted on Google Cloud, the project of the source network. For access from private networks, using the project of the hosting network is required. Individual ingress policies can be limited by restricting which services and/or actions they match using the `ingress_to` field."]
    pub struct GoogleIdentityAccesscontextmanagerV1IngressPolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ingressFrom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the conditions on the source of a request causing this IngressPolicy to apply."]
        pub ingress_from: ::std::option::Option<
            ::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1IngressFrom>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ingressTo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the conditions on the ApiOperation and request destination that cause this IngressPolicy to apply."]
        pub ingress_to:
            ::std::option::Option<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1IngressTo>>,
    }
    impl GoogleIdentityAccesscontextmanagerV1IngressPolicy {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1IngressPolicyBuilder {
            GoogleIdentityAccesscontextmanagerV1IngressPolicyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The source that IngressPolicy authorizes access from."]
    pub struct GoogleIdentityAccesscontextmanagerV1IngressSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An AccessLevel resource name that allow resources within the ServicePerimeters to be accessed from the internet. AccessLevels listed must be in the same policy as this ServicePerimeter. Referencing a nonexistent AccessLevel will cause an error. If no AccessLevel names are listed, resources within the perimeter can only be accessed via Google Cloud calls with request origins within the perimeter. Example: `accessPolicies/MY_POLICY/accessLevels/MY_LEVEL`. If `*` is specified, then all IngressSources will be allowed."]
        pub access_level: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Google Cloud resource that is allowed to ingress the perimeter. Requests from these resources will be allowed to access perimeter data. Currently only projects are allowed. Format: `projects/{project_number}` The project may be in any Google Cloud organization, not just the organization that the perimeter is defined in. `*` is not allowed, the case of allowing all Google Cloud resources only is not supported."]
        pub resource: ::std::option::Option<::std::string::String>,
    }
    impl GoogleIdentityAccesscontextmanagerV1IngressSource {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1IngressSourceBuilder {
            GoogleIdentityAccesscontextmanagerV1IngressSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines the conditions under which an IngressPolicy matches a request. Conditions are based on information about the ApiOperation intended to be performed on the destination of the request."]
    pub struct GoogleIdentityAccesscontextmanagerV1IngressTo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of ApiOperations the sources specified in corresponding IngressFrom are allowed to perform in this ServicePerimeter."]
        pub operations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1ApiOperation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of resources, currently only projects in the form `projects/`, protected by this ServicePerimeter that are allowed to be accessed by sources defined in the corresponding IngressFrom. A request matches if it contains a resource in this list. If `*` is specified for resources, then this IngressTo rule will authorize access to all resources inside the perimeter, provided that the request also matches the `operations` field."]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleIdentityAccesscontextmanagerV1IngressTo {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1IngressToBuilder {
            GoogleIdentityAccesscontextmanagerV1IngressToBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An allowed method or permission of a service specified in ApiOperation."]
    pub struct GoogleIdentityAccesscontextmanagerV1MethodSelector {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value for `method` should be a valid method name for the corresponding `service_name` in ApiOperation. If `*` used as value for `method`, then ALL methods and permissions are allowed."]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value for `permission` should be a valid Cloud IAM permission for the corresponding `service_name` in ApiOperation."]
        pub permission: ::std::option::Option<::std::string::String>,
    }
    impl GoogleIdentityAccesscontextmanagerV1MethodSelector {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1MethodSelectorBuilder {
            GoogleIdentityAccesscontextmanagerV1MethodSelectorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A restriction on the OS type and version of devices making requests."]
    pub struct GoogleIdentityAccesscontextmanagerV1OsConstraint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimumVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum allowed OS version. If not set, any version of this OS satisfies the constraint. Format: `\"major.minor.patch\"`. Examples: `\"10.5.301\"`, `\"9.2.1\"`."]
        pub minimum_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "osType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The allowed OS type."]
        pub os_type:
            ::std::option::Option<GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requireVerifiedChromeOs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only allows requests from devices with a verified Chrome OS. Verifications includes requirements that the device is enterprise-managed, conformant to domain policies, and the caller has permission to call the API targeted by the request."]
        pub require_verified_chrome_os: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleIdentityAccesscontextmanagerV1OsConstraint {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1OsConstraintBuilder {
            GoogleIdentityAccesscontextmanagerV1OsConstraintBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The allowed OS type."]
    pub enum GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum {
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
    impl ::std::default::Default for GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum {
        fn default() -> Self {
            Self::OsUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`ServicePerimeter` describes a set of Google Cloud resources which can freely import and export data amongst themselves, but not export outside of the `ServicePerimeter`. If a request with a source within this `ServicePerimeter` has a target outside of the `ServicePerimeter`, the request will be blocked. Otherwise the request is allowed. There are two types of Service Perimeter - Regular and Bridge. Regular Service Perimeters cannot overlap, a single Google Cloud project can only belong to a single regular Service Perimeter. Service Perimeter Bridges can contain only Google Cloud projects as members, a single Google Cloud project may belong to multiple Service Perimeter Bridges."]
    pub struct GoogleIdentityAccesscontextmanagerV1ServicePerimeter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the `ServicePerimeter` and its use. Does not affect behavior."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Resource name for the ServicePerimeter. The `short_name` component must begin with a letter and only include alphanumeric and '_'. Format: `accessPolicies/{policy_id}/servicePerimeters/{short_name}`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "perimeterType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Perimeter type indicator. A single project is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, the restricted service list as well as access level lists must be empty."]
        pub perimeter_type: ::std::option::Option<
            GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Proposed (or dry run) ServicePerimeter configuration. This configuration allows to specify and test ServicePerimeter configuration without enforcing actual access restrictions. Only allowed to be set when the \"use_explicit_dry_run_spec\" flag is set."]
        pub spec: ::std::option::Option<
            ::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Current ServicePerimeter configuration. Specifies sets of resources, restricted services and access levels that determine perimeter content and boundaries."]
        pub status: ::std::option::Option<
            ::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human readable title. Must be unique within the Policy."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useExplicitDryRunSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Use explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists for all Service Perimeters, and that spec is identical to the status for those Service Perimeters. When this flag is set, it inhibits the generation of the implicit spec, thereby allowing the user to explicitly provide a configuration (\"spec\") to use in a dry-run version of the Service Perimeter. This allows the user to test changes to the enforced config (\"status\") without actually enforcing them. This testing is done through analyzing the differences between currently enforced and suggested restrictions. use_explicit_dry_run_spec must bet set to True if any of the fields in the spec are set to non-default values."]
        pub use_explicit_dry_run_spec: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleIdentityAccesscontextmanagerV1ServicePerimeter {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1ServicePerimeterBuilder {
            GoogleIdentityAccesscontextmanagerV1ServicePerimeterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Perimeter type indicator. A single project is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, the restricted service list as well as access level lists must be empty."]
    pub enum GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum {
        #[serde(rename = "PERIMETER_TYPE_REGULAR")]
        #[doc = "Regular Perimeter."]
        PerimeterTypeRegular,
        #[serde(rename = "PERIMETER_TYPE_BRIDGE")]
        #[doc = "Perimeter Bridge."]
        PerimeterTypeBridge,
    }
    impl ::std::default::Default
        for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum
    {
        fn default() -> Self {
            Self::PerimeterTypeRegular
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`ServicePerimeterConfig` specifies a set of Google Cloud resources that describe specific Service Perimeter configuration."]
    pub struct GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessLevels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of `AccessLevel` resource names that allow resources within the `ServicePerimeter` to be accessed from the internet. `AccessLevels` listed must be in the same policy as this `ServicePerimeter`. Referencing a nonexistent `AccessLevel` is a syntax error. If no `AccessLevel` names are listed, resources within the perimeter can only be accessed via Google Cloud calls with request origins within the perimeter. Example: `\"accessPolicies/MY_POLICY/accessLevels/MY_LEVEL\"`. For Service Perimeter Bridge, must be empty."]
        pub access_levels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "egressPolicies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of EgressPolicies to apply to the perimeter. A perimeter may have multiple EgressPolicies, each of which is evaluated separately. Access is granted if any EgressPolicy grants it. Must be empty for a perimeter bridge."]
        pub egress_policies: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1EgressPolicy>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ingressPolicies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of IngressPolicies to apply to the perimeter. A perimeter may have multiple IngressPolicies, each of which is evaluated separately. Access is granted if any Ingress Policy grants it. Must be empty for a perimeter bridge."]
        pub ingress_policies: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1IngressPolicy>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of Google Cloud resources that are inside of the service perimeter. Currently only projects are allowed. Format: `projects/{project_number}`"]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restrictedServices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud services that are subject to the Service Perimeter restrictions. For example, if `storage.googleapis.com` is specified, access to the storage buckets inside the perimeter must meet the perimeter's access restrictions."]
        pub restricted_services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vpcAccessibleServices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for APIs allowed within Perimeter."]
        pub vpc_accessible_services: ::std::option::Option<
            ::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1VpcAccessibleServices>,
        >,
    }
    impl GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfigBuilder {
            GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies how APIs are allowed to communicate within the Service Perimeter."]
    pub struct GoogleIdentityAccesscontextmanagerV1VpcAccessibleServices {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedServices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of APIs usable within the Service Perimeter. Must be empty unless 'enable_restriction' is True. You can specify a list of individual services, as well as include the 'RESTRICTED-SERVICES' value, which automatically includes all of the services protected by the perimeter."]
        pub allowed_services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableRestriction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to restrict API calls within the Service Perimeter to the list of APIs specified in 'allowed_services'."]
        pub enable_restriction: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleIdentityAccesscontextmanagerV1VpcAccessibleServices {
        pub fn builder() -> GoogleIdentityAccesscontextmanagerV1VpcAccessibleServicesBuilder {
            GoogleIdentityAccesscontextmanagerV1VpcAccessibleServicesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "ListAssets response."]
    pub struct ListAssetsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Assets."]
        pub assets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Asset>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results. It expires 72 hours after the page token for the first page is generated. Set to empty if there are no remaining results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "readTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the snapshot was taken."]
        pub read_time: ::std::option::Option<::std::string::String>,
    }
    impl ListAssetsResponse {
        pub fn builder() -> ListAssetsResponseBuilder {
            ListAssetsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/)."]
    pub struct Policy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "auditConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies cloud audit logging configuration for this policy."]
        pub audit_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditConfig>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bindings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member."]
        pub bindings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Binding>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "`etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        pub version: ::std::option::Option<::std::primitive::i64>,
    }
    impl Policy {
        pub fn builder() -> PolicyBuilder {
            PolicyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A representation of a Google Cloud resource."]
    pub struct Resource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content of the resource, in which some sensitive fields are removed and may not be present."]
        pub data: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "discoveryDocumentUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the discovery document containing the resource's JSON schema. Example: `https://www.googleapis.com/discovery/v1/apis/compute/v1/rest` This value is unspecified for resources that do not have an API based on a discovery document, such as Cloud Bigtable."]
        pub discovery_document_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "discoveryName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The JSON schema name listed in the discovery document. Example: `Project` This value is unspecified for resources that do not have an API based on a discovery document, such as Cloud Bigtable."]
        pub discovery_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full name of the immediate parent of this resource. See [Resource Names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information. For Google Cloud assets, this value is the parent resource defined in the [Cloud IAM policy hierarchy](https://cloud.google.com/iam/docs/overview#policy_hierarchy). Example: `//cloudresourcemanager.googleapis.com/projects/my_project_123` For third-party assets, this field may be set differently."]
        pub parent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The REST URL for accessing the resource. An HTTP `GET` request using this URL returns the resource itself. Example: `https://cloudresourcemanager.googleapis.com/v1/projects/my-project-123` This value is unspecified for resources without a REST API."]
        pub resource_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The API version. Example: \"v1\"."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl Resource {
        pub fn builder() -> ResourceBuilder {
            ResourceBuilder::default()
        }
    }
}
