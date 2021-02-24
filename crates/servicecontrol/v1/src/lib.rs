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
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AllocateInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unusedArguments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of label keys that were unused by the server in processing the request. Thus, for similar requests repeated in a certain future time window, the caller can choose to ignore these labels in the requests to achieve better client-side cache hits and quota aggregation for rate quota. This field is not populated for allocation quota checks."]
        pub unused_arguments: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl AllocateInfo {
        pub fn builder() -> AllocateInfoBuilder {
            AllocateInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for the AllocateQuota method."]
    pub struct AllocateQuotaRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allocateOperation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operation that describes the quota allocation."]
        pub allocate_operation: ::std::option::Option<::std::boxed::Box<QuotaOperation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceConfigId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies which version of service configuration should be used to process the request. If unspecified or no matching version can be found, the latest one will be used."]
        pub service_config_id: ::std::option::Option<::std::string::String>,
    }
    impl AllocateQuotaRequest {
        pub fn builder() -> AllocateQuotaRequestBuilder {
            AllocateQuotaRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for the AllocateQuota method."]
    pub struct AllocateQuotaResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allocateErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates the decision of the allocate."]
        pub allocate_errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<QuotaError>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allocateInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "WARNING: DO NOT use this field until this warning message is removed."]
        pub allocate_info: ::std::option::Option<::std::boxed::Box<AllocateInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The same operation_id value used in the AllocateQuotaRequest. Used for logging and diagnostics purposes."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Quota metrics to indicate the result of allocation. Depending on the request, one or more of the following metrics will be included: 1. Per quota group or per quota metric incremental usage will be specified using the following delta metric : \"serviceruntime.googleapis.com/api/consumer/quota_used_count\" 2. The quota limit reached condition will be specified using the following boolean metric : \"serviceruntime.googleapis.com/quota/exceeded\""]
        pub quota_metrics:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricValueSet>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceConfigId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the actual config used to process the request."]
        pub service_config_id: ::std::option::Option<::std::string::String>,
    }
    impl AllocateQuotaResponse {
        pub fn builder() -> AllocateQuotaResponseBuilder {
            AllocateQuotaResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The allowed types for [VALUE] in a `[KEY]:[VALUE]` attribute."]
    pub struct AttributeValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boolValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Boolean value represented by `true` or `false`."]
        pub bool_value: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A 64-bit signed integer."]
        pub int_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A string up to 256 bytes long."]
        pub string_value: ::std::option::Option<::std::boxed::Box<TruncatableString>>,
    }
    impl AttributeValue {
        pub fn builder() -> AttributeValueBuilder {
            AttributeValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A set of attributes, each in the format `[KEY]:[VALUE]`."]
    pub struct Attributes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributeMap")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of attributes. Each attribute's key can be up to 128 bytes long. The value can be a string up to 256 bytes, a signed 64-bit integer, or the Boolean values `true` and `false`. For example: \"/instance_id\": \"my-instance\" \"/http/user_agent\": \"\" \"/http/request_bytes\": 300 \"abc.com/myattribute\": true"]
        pub attribute_map: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<AttributeValue>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "droppedAttributesCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of attributes that were discarded. Attributes can be discarded because their keys are too long or because there are too many attributes. If this value is 0 then all attributes are valid."]
        pub dropped_attributes_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl Attributes {
        pub fn builder() -> AttributesBuilder {
            AttributesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Common audit log format for Google Cloud Platform API operations. "]
    pub struct AuditLog {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authenticationInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Authentication information."]
        pub authentication_info: ::std::option::Option<::std::boxed::Box<AuthenticationInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorizationInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Authorization information. If there are multiple resources or permissions involved, then there is one AuthorizationInfo element for each {resource, permission} tuple."]
        pub authorization_info:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuthorizationInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Other service-specific data about the request, response, and other information associated with the current audited event."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "methodName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the service method or operation. For API calls, this should be the name of the API method. For example, \"google.cloud.bigquery.v2.TableService.InsertTable\" \"google.logging.v2.ConfigServiceV2.CreateSink\""]
        pub method_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numResponseItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of items returned from a List or Query API method, if applicable."]
        pub num_response_items: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "request")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operation request. This may not include all request parameters, such as those that are too large, privacy-sensitive, or duplicated elsewhere in the log record. It should never include user-generated data, such as file contents. When the JSON object represented here has a proto equivalent, the proto name will be indicated in the `@type` property."]
        pub request:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata about the operation."]
        pub request_metadata: ::std::option::Option<::std::boxed::Box<RequestMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource location information."]
        pub resource_location: ::std::option::Option<::std::boxed::Box<ResourceLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource or collection that is the target of the operation. The name is a scheme-less URI, not including the API service name. For example: \"projects/PROJECT_ID/zones/us-central1-a/instances\" \"projects/PROJECT_ID/datasets/DATASET_ID\""]
        pub resource_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceOriginalState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource's original state before mutation. Present only for operations which have successfully modified the targeted resource(s). In general, this field should contain all changed fields, except those that are already been included in `request`, `response`, `metadata` or `service_data` fields. When the JSON object represented here has a proto equivalent, the proto name will be indicated in the `@type` property."]
        pub resource_original_state:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "response")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operation response. This may not include all response elements, such as those that are too large, privacy-sensitive, or duplicated elsewhere in the log record. It should never include user-generated data, such as file contents. When the JSON object represented here has a proto equivalent, the proto name will be indicated in the `@type` property."]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Use the `metadata` field instead. Other service-specific data about the request, response, and other activities."]
        pub service_data:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the API service performing the operation. For example, `\"compute.googleapis.com\"`."]
        pub service_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the overall operation."]
        pub status: ::std::option::Option<::std::boxed::Box<Status>>,
    }
    impl AuditLog {
        pub fn builder() -> AuditLogBuilder {
            AuditLogBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This message defines request authentication attributes. Terminology is based on the JSON Web Token (JWT) standard, but the terms also correlate to concepts in other standards."]
    pub struct Auth {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessLevels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of access level resource names that allow resources to be accessed by authenticated requester. It is part of Secure GCP processing for the incoming request. An access level string has the format: \"//{api_service_name}/accessPolicies/{policy_id}/accessLevels/{short_name}\" Example: \"//accesscontextmanager.googleapis.com/accessPolicies/MY_POLICY_ID/accessLevels/MY_LEVEL\""]
        pub access_levels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audiences")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The intended audience(s) for this authentication information. Reflects the audience (`aud`) claim within a JWT. The audience value(s) depends on the `issuer`, but typically include one or more of the following pieces of information: * The services intended to receive the credential. For example, [\"https://pubsub.googleapis.com/\", \"https://storage.googleapis.com/\"]. * A set of service-based scopes. For example, [\"https://www.googleapis.com/auth/cloud-platform\"]. * The client id of an app, such as the Firebase project id for JWTs from Firebase Auth. Consult the documentation for the credential issuer to determine the information provided."]
        pub audiences: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "claims")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Structured claims presented with the credential. JWTs include `{key: value}` pairs for standard and private claims. The following is a subset of the standard required and optional claims that would typically be presented for a Google-based JWT: {'iss': 'accounts.google.com', 'sub': '113289723416554971153', 'aud': ['123456789012', 'pubsub.googleapis.com'], 'azp': '123456789012.apps.googleusercontent.com', 'email': 'jsmith@example.com', 'iat': 1353601026, 'exp': 1353604926} SAML assertions are similarly specified, but with an identity provider dependent structure."]
        pub claims:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "presenter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The authorized presenter of the credential. Reflects the optional Authorized Presenter (`azp`) claim within a JWT or the OAuth client id. For example, a Google Cloud Platform client id looks as follows: \"123456789012.apps.googleusercontent.com\"."]
        pub presenter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "principal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The authenticated principal. Reflects the issuer (`iss`) and subject (`sub`) claims within a JWT. The issuer and subject should be `/` delimited, with `/` percent-encoded within the subject fragment. For Google accounts, the principal format is: \"https://accounts.google.com/{id}\""]
        pub principal: ::std::option::Option<::std::string::String>,
    }
    impl Auth {
        pub fn builder() -> AuthBuilder {
            AuthBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Authentication information for the operation."]
    pub struct AuthenticationInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authoritySelector")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The authority selector specified by the requestor, if any. It is not guaranteed that the principal was allowed to use this authority."]
        pub authority_selector: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "principalEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of the authenticated user (or service account on behalf of third party principal) making the request. For third party identity callers, the `principal_subject` field is populated instead of this field. For privacy reasons, the principal email address is sometimes redacted. For more information, see [Caller identities in audit logs](https://cloud.google.com/logging/docs/audit#user-id)."]
        pub principal_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "principalSubject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "String representation of identity of requesting party. Populated for both first and third party identities."]
        pub principal_subject: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceAccountDelegationInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identity delegation history of an authenticated service account that makes the request. It contains information on the real authorities that try to access GCP resources by delegating on a service account. When multiple authorities present, they are guaranteed to be sorted based on the original ordering of the identity delegation events."]
        pub service_account_delegation_info:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ServiceAccountDelegationInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceAccountKeyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the service account key used to create or exchange credentials for authenticating the service account making the request. This is a scheme-less URI full resource name. For example: \"//iam.googleapis.com/projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}/keys/{key}\""]
        pub service_account_key_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thirdPartyPrincipal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The third party identification (if any) of the authenticated user making the request. When the JSON object represented here has a proto equivalent, the proto name will be indicated in the `@type` property."]
        pub third_party_principal:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl AuthenticationInfo {
        pub fn builder() -> AuthenticationInfoBuilder {
            AuthenticationInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Authorization information for the operation."]
    pub struct AuthorizationInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "granted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not authorization for `resource` and `permission` was granted."]
        pub granted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The required IAM permission."]
        pub permission: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource being accessed, as a REST-style or cloud resource string. For example: bigquery.googleapis.com/projects/PROJECTID/datasets/DATASETID or projects/PROJECTID/datasets/DATASETID"]
        pub resource: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceAttributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource attributes used in IAM condition evaluation. This field contains resource attributes like resource type and resource name. To get the whole view of the attributes used in IAM condition evaluation, the user must also look into `AuditLog.request_metadata.request_attributes`."]
        pub resource_attributes: ::std::option::Option<::std::boxed::Box<Resource>>,
    }
    impl AuthorizationInfo {
        pub fn builder() -> AuthorizationInfoBuilder {
            AuthorizationInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines the errors to be returned in google.api.servicecontrol.v1.CheckResponse.check_errors."]
    pub struct CheckError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error code."]
        pub code: ::std::option::Option<CheckErrorCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Free-form text providing details on the error cause of the error."]
        pub detail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains public information about the check error. If available, `status.code` will be non zero and client can propagate it out as public error."]
        pub status: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subject to whom this error applies. See the specific code enum for more details on this field. For example: - \"project:\" - \"folder:\" - \"organization:\""]
        pub subject: ::std::option::Option<::std::string::String>,
    }
    impl CheckError {
        pub fn builder() -> CheckErrorBuilder {
            CheckErrorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The error code."]
    pub enum CheckErrorCodeEnum {
        #[serde(rename = "ERROR_CODE_UNSPECIFIED")]
        #[doc = "This is never used in `CheckResponse`."]
        ErrorCodeUnspecified,
        #[serde(rename = "NOT_FOUND")]
        #[doc = "The consumer's project id, network container, or resource container was not found. Same as google.rpc.Code.NOT_FOUND."]
        NotFound,
        #[serde(rename = "PERMISSION_DENIED")]
        #[doc = "The consumer doesn't have access to the specified resource. Same as google.rpc.Code.PERMISSION_DENIED."]
        PermissionDenied,
        #[serde(rename = "RESOURCE_EXHAUSTED")]
        #[doc = "Quota check failed. Same as google.rpc.Code.RESOURCE_EXHAUSTED."]
        ResourceExhausted,
        #[serde(rename = "BUDGET_EXCEEDED")]
        #[doc = "Budget check failed."]
        BudgetExceeded,
        #[serde(rename = "DENIAL_OF_SERVICE_DETECTED")]
        #[doc = "The consumer's request has been flagged as a DoS attack."]
        DenialOfServiceDetected,
        #[serde(rename = "LOAD_SHEDDING")]
        #[doc = "The consumer's request should be rejected in order to protect the service from being overloaded."]
        LoadShedding,
        #[serde(rename = "ABUSER_DETECTED")]
        #[doc = "The consumer has been flagged as an abuser."]
        AbuserDetected,
        #[serde(rename = "SERVICE_NOT_ACTIVATED")]
        #[doc = "The consumer hasn't activated the service."]
        ServiceNotActivated,
        #[serde(rename = "VISIBILITY_DENIED")]
        #[doc = "The consumer cannot access the service due to visibility configuration."]
        VisibilityDenied,
        #[serde(rename = "BILLING_DISABLED")]
        #[doc = "The consumer cannot access the service because billing is disabled."]
        BillingDisabled,
        #[serde(rename = "PROJECT_DELETED")]
        #[doc = "The consumer's project has been marked as deleted (soft deletion)."]
        ProjectDeleted,
        #[serde(rename = "PROJECT_INVALID")]
        #[doc = "The consumer's project number or id does not represent a valid project."]
        ProjectInvalid,
        #[serde(rename = "CONSUMER_INVALID")]
        #[doc = "The input consumer info does not represent a valid consumer folder or organization."]
        ConsumerInvalid,
        #[serde(rename = "IP_ADDRESS_BLOCKED")]
        #[doc = "The IP address of the consumer is invalid for the specific consumer project."]
        IpAddressBlocked,
        #[serde(rename = "REFERER_BLOCKED")]
        #[doc = "The referer address of the consumer request is invalid for the specific consumer project."]
        RefererBlocked,
        #[serde(rename = "CLIENT_APP_BLOCKED")]
        #[doc = "The client application of the consumer request is invalid for the specific consumer project."]
        ClientAppBlocked,
        #[serde(rename = "API_TARGET_BLOCKED")]
        #[doc = "The API targeted by this request is invalid for the specified consumer project."]
        ApiTargetBlocked,
        #[serde(rename = "API_KEY_INVALID")]
        #[doc = "The consumer's API key is invalid."]
        ApiKeyInvalid,
        #[serde(rename = "API_KEY_EXPIRED")]
        #[doc = "The consumer's API Key has expired."]
        ApiKeyExpired,
        #[serde(rename = "API_KEY_NOT_FOUND")]
        #[doc = "The consumer's API Key was not found in config record."]
        ApiKeyNotFound,
        #[serde(rename = "SPATULA_HEADER_INVALID")]
        #[doc = "The consumer's spatula header is invalid."]
        SpatulaHeaderInvalid,
        #[serde(rename = "LOAS_ROLE_INVALID")]
        #[doc = "The consumer's LOAS role is invalid."]
        LoasRoleInvalid,
        #[serde(rename = "NO_LOAS_PROJECT")]
        #[doc = "The consumer's LOAS role has no associated project."]
        NoLoasProject,
        #[serde(rename = "LOAS_PROJECT_DISABLED")]
        #[doc = "The consumer's LOAS project is not `ACTIVE` in LoquatV2."]
        LoasProjectDisabled,
        #[serde(rename = "SECURITY_POLICY_VIOLATED")]
        #[doc = "Request is not allowed as per security policies defined in Org Policy."]
        SecurityPolicyViolated,
        #[serde(rename = "INVALID_CREDENTIAL")]
        #[doc = "The credential in the request can not be verified."]
        InvalidCredential,
        #[serde(rename = "LOCATION_POLICY_VIOLATED")]
        #[doc = "Request is not allowed as per location policies defined in Org Policy."]
        LocationPolicyViolated,
        #[serde(rename = "NAMESPACE_LOOKUP_UNAVAILABLE")]
        #[doc = "The backend server for looking up project id/number is unavailable."]
        NamespaceLookupUnavailable,
        #[serde(rename = "SERVICE_STATUS_UNAVAILABLE")]
        #[doc = "The backend server for checking service status is unavailable."]
        ServiceStatusUnavailable,
        #[serde(rename = "BILLING_STATUS_UNAVAILABLE")]
        #[doc = "The backend server for checking billing status is unavailable."]
        BillingStatusUnavailable,
        #[serde(rename = "QUOTA_CHECK_UNAVAILABLE")]
        #[doc = "The backend server for checking quota limits is unavailable."]
        QuotaCheckUnavailable,
        #[serde(rename = "LOAS_PROJECT_LOOKUP_UNAVAILABLE")]
        #[doc = "The Spanner for looking up LOAS project is unavailable."]
        LoasProjectLookupUnavailable,
        #[serde(rename = "CLOUD_RESOURCE_MANAGER_BACKEND_UNAVAILABLE")]
        #[doc = "Cloud Resource Manager backend server is unavailable."]
        CloudResourceManagerBackendUnavailable,
        #[serde(rename = "SECURITY_POLICY_BACKEND_UNAVAILABLE")]
        #[doc = "NOTE: for customers in the scope of Beta/GA of https://cloud.google.com/vpc-service-controls, this error is no longer returned. If the security backend is unavailable, rpc UNAVAILABLE status will be returned instead. It should be ignored and should not be used to reject client requests."]
        SecurityPolicyBackendUnavailable,
        #[serde(rename = "LOCATION_POLICY_BACKEND_UNAVAILABLE")]
        #[doc = "Backend server for evaluating location policy is unavailable."]
        LocationPolicyBackendUnavailable,
    }
    impl ::std::default::Default for CheckErrorCodeEnum {
        fn default() -> Self {
            Self::ErrorCodeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains additional information about the check operation."]
    pub struct CheckInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Consumer info of this check."]
        pub consumer_info: ::std::option::Option<::std::boxed::Box<ConsumerInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unusedArguments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of fields and label keys that are ignored by the server. The client doesn't need to send them for following requests to improve performance and allow better aggregation."]
        pub unused_arguments: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl CheckInfo {
        pub fn builder() -> CheckInfoBuilder {
            CheckInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for the Check method."]
    pub struct CheckRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operation to be checked."]
        pub operation: ::std::option::Option<::std::boxed::Box<Operation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestProjectSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Requests the project settings to be returned as part of the check response."]
        pub request_project_settings: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceConfigId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies which version of service configuration should be used to process the request. If unspecified or no matching version can be found, the latest one will be used."]
        pub service_config_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skipActivationCheck")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if service activation check should be skipped for this request. Default behavior is to perform the check and apply relevant quota. WARNING: Setting this flag to \"true\" will disable quota enforcement."]
        pub skip_activation_check: ::std::option::Option<::std::primitive::bool>,
    }
    impl CheckRequest {
        pub fn builder() -> CheckRequestBuilder {
            CheckRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for the Check method."]
    pub struct CheckResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "checkErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicate the decision of the check. If no check errors are present, the service should process the operation. Otherwise the service should use the list of errors to determine the appropriate action."]
        pub check_errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CheckError>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "checkInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feedback data returned from the server during processing a Check request."]
        pub check_info: ::std::option::Option<::std::boxed::Box<CheckInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The same operation_id value used in the CheckRequest. Used for logging and diagnostics purposes."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Quota information for the check request associated with this response. "]
        pub quota_info: ::std::option::Option<::std::boxed::Box<QuotaInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceConfigId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actual config id used to process the request."]
        pub service_config_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceRolloutId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current service rollout id used to process the request."]
        pub service_rollout_id: ::std::option::Option<::std::string::String>,
    }
    impl CheckResponse {
        pub fn builder() -> CheckResponseBuilder {
            CheckResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`ConsumerInfo` provides information about the consumer."]
    pub struct ConsumerInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The consumer identity number, can be Google cloud project number, folder number or organization number e.g. 1234567890. A value of 0 indicates no consumer number is found."]
        pub consumer_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google cloud project number, e.g. 1234567890. A value of 0 indicates no project number is found. NOTE: This field is deprecated after Chemist support flexible consumer id. New code should not depend on this field anymore."]
        pub project_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the consumer which should have been defined in [Google Resource Manager](https://cloud.google.com/resource-manager/)."]
        pub _type: ::std::option::Option<ConsumerInfoTypeEnum>,
    }
    impl ConsumerInfo {
        pub fn builder() -> ConsumerInfoBuilder {
            ConsumerInfoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the consumer which should have been defined in [Google Resource Manager](https://cloud.google.com/resource-manager/)."]
    pub enum ConsumerInfoTypeEnum {
        #[serde(rename = "CONSUMER_TYPE_UNSPECIFIED")]
        #[doc = "This is never used."]
        ConsumerTypeUnspecified,
        #[serde(rename = "PROJECT")]
        #[doc = "The consumer is a Google Cloud Project."]
        Project,
        #[serde(rename = "FOLDER")]
        #[doc = "The consumer is a Google Cloud Folder."]
        Folder,
        #[serde(rename = "ORGANIZATION")]
        #[doc = "The consumer is a Google Cloud Organization."]
        Organization,
        #[serde(rename = "SERVICE_SPECIFIC")]
        #[doc = "Service-specific resource container which is defined by the service producer to offer their users the ability to manage service control functionalities at a finer level of granularity than the PROJECT."]
        ServiceSpecific,
    }
    impl ::std::default::Default for ConsumerInfoTypeEnum {
        fn default() -> Self {
            Self::ConsumerTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Distribution represents a frequency distribution of double-valued sample points. It contains the size of the population of sample points plus additional optional information: - the arithmetic mean of the samples - the minimum and maximum of the samples - the sum-squared-deviation of the samples, used to compute variance - a histogram of the values of the sample points"]
    pub struct Distribution {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketCounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of samples in each histogram bucket. `bucket_counts` are optional. If present, they must sum to the `count` value. The buckets are defined below in `bucket_option`. There are N buckets. `bucket_counts[0]` is the number of samples in the underflow bucket. `bucket_counts[1]` to `bucket_counts[N-1]` are the numbers of samples in each of the finite buckets. And `bucket_counts[N] is the number of samples in the overflow bucket. See the comments of `bucket_option` below for more details. Any suffix of trailing zeros may be omitted."]
        pub bucket_counts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of samples in the distribution. Must be >= 0."]
        pub count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exemplars")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Example points. Must be in increasing order of `value` field."]
        pub exemplars: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Exemplar>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explicitBuckets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Buckets with arbitrary user-provided width."]
        pub explicit_buckets: ::std::option::Option<::std::boxed::Box<ExplicitBuckets>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exponentialBuckets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Buckets with exponentially growing width."]
        pub exponential_buckets: ::std::option::Option<::std::boxed::Box<ExponentialBuckets>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linearBuckets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Buckets with constant width."]
        pub linear_buckets: ::std::option::Option<::std::boxed::Box<LinearBuckets>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maximum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum of the population of values. Ignored if `count` is zero."]
        pub maximum: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mean")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The arithmetic mean of the samples in the distribution. If `count` is zero then this field must be zero."]
        pub mean: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum of the population of values. Ignored if `count` is zero."]
        pub minimum: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sumOfSquaredDeviation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sum of squared deviations from the mean: Sum[i=1..count]((x_i - mean)^2) where each x_i is a sample values. If `count` is zero then this field must be zero, otherwise validation of the request fails."]
        pub sum_of_squared_deviation: ::std::option::Option<::std::primitive::f64>,
    }
    impl Distribution {
        pub fn builder() -> DistributionBuilder {
            DistributionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Exemplars are example points that may be used to annotate aggregated distribution values. They are metadata that gives information about a particular value added to a Distribution bucket, such as a trace ID that was active when a value was added. They may contain further information, such as a example values and timestamps, origin, etc."]
    pub struct Exemplar {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attachments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contextual information about the example value. Examples are: Trace: type.googleapis.com/google.monitoring.v3.SpanContext Literal string: type.googleapis.com/google.protobuf.StringValue Labels dropped during aggregation: type.googleapis.com/google.monitoring.v3.DroppedLabels There may be only a single attachment of any given message type in a single exemplar, and this is enforced by the system."]
        pub attachments: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The observation (sampling) time of the above value."]
        pub timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of the exemplar point. This value determines to which bucket the exemplar belongs."]
        pub value: ::std::option::Option<::std::primitive::f64>,
    }
    impl Exemplar {
        pub fn builder() -> ExemplarBuilder {
            ExemplarBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describing buckets with arbitrary user-provided width."]
    pub struct ExplicitBuckets {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bounds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "'bound' is a list of strictly increasing boundaries between buckets. Note that a list of length N-1 defines N buckets because of fenceposting. See comments on `bucket_options` for details. The i'th finite bucket covers the interval [bound[i-1], bound[i]) where i ranges from 1 to bound_size() - 1. Note that there are no finite buckets at all if 'bound' only contains a single element; in that special case the single bound defines the boundary between the underflow and overflow buckets. bucket number lower bound upper bound i == 0 (underflow) -inf bound[i] 0 < i < bound_size() bound[i-1] bound[i] i == bound_size() (overflow) bound[i-1] +inf"]
        pub bounds: ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
    }
    impl ExplicitBuckets {
        pub fn builder() -> ExplicitBucketsBuilder {
            ExplicitBucketsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describing buckets with exponentially growing width."]
    pub struct ExponentialBuckets {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "growthFactor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The i'th exponential bucket covers the interval [scale * growth_factor^(i-1), scale * growth_factor^i) where i ranges from 1 to num_finite_buckets inclusive. Must be larger than 1.0."]
        pub growth_factor: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numFiniteBuckets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of finite buckets. With the underflow and overflow buckets, the total number of buckets is `num_finite_buckets` + 2. See comments on `bucket_options` for details."]
        pub num_finite_buckets: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The i'th exponential bucket covers the interval [scale * growth_factor^(i-1), scale * growth_factor^i) where i ranges from 1 to num_finite_buckets inclusive. Must be > 0."]
        pub scale: ::std::option::Option<::std::primitive::f64>,
    }
    impl ExponentialBuckets {
        pub fn builder() -> ExponentialBucketsBuilder {
            ExponentialBucketsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "First party identity principal."]
    pub struct FirstPartyPrincipal {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "principalEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of a Google account. ."]
        pub principal_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata about the service that uses the service account. ."]
        pub service_metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl FirstPartyPrincipal {
        pub fn builder() -> FirstPartyPrincipalBuilder {
            FirstPartyPrincipalBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A common proto for logging HTTP requests. Only contains semantics defined by the HTTP specification. Product-specific logging information MUST be defined in a separate message."]
    pub struct HttpRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cacheFillBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of HTTP response bytes inserted into cache. Set only when a cache fill was attempted."]
        pub cache_fill_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cacheHit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not an entity was served from cache (with or without validation)."]
        pub cache_hit: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cacheLookup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not a cache lookup was attempted."]
        pub cache_lookup: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cacheValidatedWithOriginServer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not the response was validated with the origin server before being served from cache. This field is only meaningful if `cache_hit` is True."]
        pub cache_validated_with_origin_server: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request processing latency on the server, from the time the request was received until the response was sent."]
        pub latency: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protocol")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Protocol used for the request. Examples: \"HTTP/1.1\", \"HTTP/2\", \"websocket\""]
        pub protocol: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The referer URL of the request, as defined in [HTTP/1.1 Header Field Definitions](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html)."]
        pub referer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "remoteIp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IP address (IPv4 or IPv6) of the client that issued the HTTP request. Examples: `\"192.168.1.1\"`, `\"FE80::0202:B3FF:FE1E:8329\"`."]
        pub remote_ip: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request method. Examples: `\"GET\"`, `\"HEAD\"`, `\"PUT\"`, `\"POST\"`."]
        pub request_method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the HTTP request message in bytes, including the request headers and the request body."]
        pub request_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The scheme (http, https), the host name, the path, and the query portion of the URL that was requested. Example: `\"http://example.com/some/info?color=red\"`."]
        pub request_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the HTTP response message sent back to the client, in bytes, including the response headers and the response body."]
        pub response_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serverIp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IP address (IPv4 or IPv6) of the origin server that the request was sent to."]
        pub server_ip: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The response code indicating the status of the response. Examples: 200, 404."]
        pub status: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userAgent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user agent sent by the client. Example: `\"Mozilla/4.0 (compatible; MSIE 6.0; Windows 98; Q312461; .NET CLR 1.0.3705)\"`."]
        pub user_agent: ::std::option::Option<::std::string::String>,
    }
    impl HttpRequest {
        pub fn builder() -> HttpRequestBuilder {
            HttpRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describing buckets with constant width."]
    pub struct LinearBuckets {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numFiniteBuckets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of finite buckets. With the underflow and overflow buckets, the total number of buckets is `num_finite_buckets` + 2. See comments on `bucket_options` for details."]
        pub num_finite_buckets: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The i'th linear bucket covers the interval [offset + (i-1) * width, offset + i * width) where i ranges from 1 to num_finite_buckets, inclusive."]
        pub offset: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The i'th linear bucket covers the interval [offset + (i-1) * width, offset + i * width) where i ranges from 1 to num_finite_buckets, inclusive. Must be strictly positive."]
        pub width: ::std::option::Option<::std::primitive::f64>,
    }
    impl LinearBuckets {
        pub fn builder() -> LinearBucketsBuilder {
            LinearBucketsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An individual log entry."]
    pub struct LogEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpRequest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Information about the HTTP request associated with this log entry, if applicable."]
        pub http_request: ::std::option::Option<::std::boxed::Box<HttpRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique ID for the log entry used for deduplication. If omitted, the implementation will generate one based on operation_id."]
        pub insert_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A set of user-defined (key, value) data that provides additional information about the log entry."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The log to which this log entry belongs. Examples: `\"syslog\"`, `\"book_log\"`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Information about an operation associated with the log entry, if applicable."]
        pub operation: ::std::option::Option<::std::boxed::Box<LogEntryOperation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protoPayload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The log entry payload, represented as a protocol buffer that is expressed as a JSON object. The only accepted type currently is AuditLog."]
        pub proto_payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The severity of the log entry. The default value is `LogSeverity.DEFAULT`."]
        pub severity: ::std::option::Option<LogEntrySeverityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Source code location information associated with the log entry, if any."]
        pub source_location: ::std::option::Option<::std::boxed::Box<LogEntrySourceLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "structPayload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The log entry payload, represented as a structure that is expressed as a JSON object."]
        pub struct_payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textPayload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The log entry payload, represented as a Unicode string (UTF-8)."]
        pub text_payload: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time the event described by the log entry occurred. If omitted, defaults to operation start time."]
        pub timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Resource name of the trace associated with the log entry, if any. If this field contains a relative resource name, you can assume the name is relative to `//tracing.googleapis.com`. Example: `projects/my-projectid/traces/06796866738c859f2f19b7cfb3214824`"]
        pub trace: ::std::option::Option<::std::string::String>,
    }
    impl LogEntry {
        pub fn builder() -> LogEntryBuilder {
            LogEntryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The severity of the log entry. The default value is `LogSeverity.DEFAULT`."]
    pub enum LogEntrySeverityEnum {
        #[serde(rename = "DEFAULT")]
        #[doc = "(0) The log entry has no assigned severity level."]
        Default,
        #[serde(rename = "DEBUG")]
        #[doc = "(100) Debug or trace information."]
        Debug,
        #[serde(rename = "INFO")]
        #[doc = "(200) Routine information, such as ongoing status or performance."]
        Info,
        #[serde(rename = "NOTICE")]
        #[doc = "(300) Normal but significant events, such as start up, shut down, or a configuration change."]
        Notice,
        #[serde(rename = "WARNING")]
        #[doc = "(400) Warning events might cause problems."]
        Warning,
        #[serde(rename = "ERROR")]
        #[doc = "(500) Error events are likely to cause problems."]
        Error,
        #[serde(rename = "CRITICAL")]
        #[doc = "(600) Critical events cause more severe problems or outages."]
        Critical,
        #[serde(rename = "ALERT")]
        #[doc = "(700) A person must take an action immediately."]
        Alert,
        #[serde(rename = "EMERGENCY")]
        #[doc = "(800) One or more systems are unusable."]
        Emergency,
    }
    impl ::std::default::Default for LogEntrySeverityEnum {
        fn default() -> Self {
            Self::Default
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional information about a potentially long-running operation with which a log entry is associated."]
    pub struct LogEntryOperation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "first")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Set this to True if this is the first log entry in the operation."]
        pub first: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An arbitrary operation identifier. Log entries with the same identifier are assumed to be part of the same operation."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "last")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Set this to True if this is the last log entry in the operation."]
        pub last: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "producer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An arbitrary producer identifier. The combination of `id` and `producer` must be globally unique. Examples for `producer`: `\"MyDivision.MyBigCompany.com\"`, `\"github.com/MyProject/MyApplication\"`."]
        pub producer: ::std::option::Option<::std::string::String>,
    }
    impl LogEntryOperation {
        pub fn builder() -> LogEntryOperationBuilder {
            LogEntryOperationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional information about the source code location that produced the log entry."]
    pub struct LogEntrySourceLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "file")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Source file name. Depending on the runtime environment, this might be a simple name or a fully-qualified name."]
        pub file: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "function")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Human-readable name of the function or method being invoked, with optional context such as the class or package name. This information may be used in contexts such as the logs viewer, where a file and line number are less meaningful. The format can vary by language. For example: `qual.if.ied.Class.method` (Java), `dir/package.func` (Go), `function` (Python)."]
        pub function: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "line")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Line within the source file. 1-based; 0 indicates no line number available."]
        pub line: ::std::option::Option<::std::string::String>,
    }
    impl LogEntrySourceLocation {
        pub fn builder() -> LogEntrySourceLocationBuilder {
            LogEntrySourceLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a single metric value."]
    pub struct MetricValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boolValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A boolean value."]
        pub bool_value: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "distributionValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A distribution value."]
        pub distribution_value: ::std::option::Option<::std::boxed::Box<Distribution>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "doubleValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A double precision floating point value."]
        pub double_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end of the time period over which this metric value's measurement applies. If not specified, google.api.servicecontrol.v1.Operation.end_time will be used."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "int64Value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A signed 64-bit integer value."]
        pub int64_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The labels describing the metric value. See comments on google.api.servicecontrol.v1.Operation.labels for the overriding relationship. Note that this map must not contain monitored resource labels."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "moneyValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A money value."]
        pub money_value: ::std::option::Option<::std::boxed::Box<Money>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start of the time period over which this metric value's measurement applies. The time period has different semantics for different metric types (cumulative, delta, and gauge). See the metric definition documentation in the service configuration for details. If not specified, google.api.servicecontrol.v1.Operation.start_time will be used."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A text string value."]
        pub string_value: ::std::option::Option<::std::string::String>,
    }
    impl MetricValue {
        pub fn builder() -> MetricValueBuilder {
            MetricValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a set of metric values in the same metric. Each metric value in the set should have a unique combination of start time, end time, and label values."]
    pub struct MetricValueSet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metric name defined in the service configuration."]
        pub metric_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values in this metric."]
        pub metric_values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricValue>>>,
    }
    impl MetricValueSet {
        pub fn builder() -> MetricValueSetBuilder {
            MetricValueSetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an amount of money with its currency type."]
    pub struct Money {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The three-letter currency code defined in ISO 4217."]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nanos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
        pub nanos: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "units")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The whole units of the amount. For example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
        pub units: ::std::option::Option<::std::string::String>,
    }
    impl Money {
        pub fn builder() -> MoneyBuilder {
            MoneyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents information regarding an operation."]
    pub struct Operation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identity of the consumer who is using the service. This field should be filled in for the operations initiated by a consumer, but not for service-initiated operations that are not related to a specific consumer. - This can be in one of the following formats: - project:PROJECT_ID, - project`_`number:PROJECT_NUMBER, - projects/PROJECT_ID or PROJECT_NUMBER, - folders/FOLDER_NUMBER, - organizations/ORGANIZATION_NUMBER, - api`_`key:API_KEY."]
        pub consumer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End time of the operation. Required when the operation is used in ServiceController.Report, but optional when the operation is used in ServiceController.Check."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "extensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unimplemented."]
        pub extensions: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DO NOT USE. This is an experimental field."]
        pub importance: ::std::option::Option<OperationImportanceEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Labels describing the operation. Only the following labels are allowed: - Labels describing monitored resources as defined in the service configuration. - Default labels of metric values. When specified, labels defined in the metric value override these default. - The following labels defined by Google Cloud Platform: - `cloud.googleapis.com/location` describing the location where the operation happened, - `servicecontrol.googleapis.com/user_agent` describing the user agent of the API request, - `servicecontrol.googleapis.com/service_agent` describing the service used to handle the API request (e.g. ESP), - `servicecontrol.googleapis.com/platform` describing the platform where the API is served, such as App Engine, Compute Engine, or Kubernetes Engine."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logEntries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents information to be logged."]
        pub log_entries: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LogEntry>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metricValueSets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents information about this operation. Each MetricValueSet corresponds to a metric defined in the service configuration. The data type used in the MetricValueSet must agree with the data type specified in the metric definition. Within a single operation, it is not allowed to have more than one MetricValue instances that have the same metric names and identical label value combinations. If a request has such duplicated MetricValue instances, the entire request is rejected with an invalid argument error."]
        pub metric_value_sets:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricValueSet>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identity of the operation. This must be unique within the scope of the service that generated the operation. If the service calls Check() and Report() on the same operation, the two calls should carry the same id. UUID version 4 is recommended, though not required. In scenarios where an operation is computed from existing information and an idempotent id is desirable for deduplication purpose, UUID version 5 is recommended. See RFC 4122 for details."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fully qualified name of the operation. Reserved for future use."]
        pub operation_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents the properties needed for quota check. Applicable only if this operation is for a quota check request. If this is not specified, no quota check will be performed."]
        pub quota_properties: ::std::option::Option<::std::boxed::Box<QuotaProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resources that are involved in the operation. The maximum supported number of entries in this field is 100."]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResourceInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Start time of the operation."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "traceSpans")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unimplemented. A list of Cloud Trace spans. The span names shall contain the id of the destination project which can be either the produce or the consumer project."]
        pub trace_spans: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TraceSpan>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Private Preview. This feature is only available for approved services. User defined labels for the resource that this operation is associated with."]
        pub user_labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl Operation {
        pub fn builder() -> OperationBuilder {
            OperationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "DO NOT USE. This is an experimental field."]
    pub enum OperationImportanceEnum {
        #[serde(rename = "LOW")]
        #[doc = "Allows data caching, batching, and aggregation. It provides higher performance with higher data loss risk."]
        Low,
        #[serde(rename = "HIGH")]
        #[doc = "Disables data aggregation to minimize data loss. It is for operations that contains significant monetary value or audit trail. This feature only applies to the client libraries."]
        High,
        #[serde(rename = "DEBUG")]
        #[doc = "Deprecated. Do not use. Disables data aggregation and enables additional validation logic. It should only be used during the onboarding process. It is only available to Google internal services, and the service must be approved by chemist-dev@google.com in order to use this level."]
        Debug,
    }
    impl ::std::default::Default for OperationImportanceEnum {
        fn default() -> Self {
            Self::Low
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This message defines attributes for a node that handles a network request. The node can be either a service or an application that sends, forwards, or receives the request. Service peers should fill in `principal` and `labels` as appropriate."]
    pub struct Peer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ip")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IP address of the peer."]
        pub ip: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The labels associated with the peer."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "port")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The network port of the peer."]
        pub port: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "principal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identity of this peer. Similar to `Request.auth.principal`, but relative to the peer instead of the request. For example, the idenity associated with a load balancer that forwared the request."]
        pub principal: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The CLDR country/region code associated with the above IP address. If the IP address is private, the `region_code` should reflect the physical location where this peer is running."]
        pub region_code: ::std::option::Option<::std::string::String>,
    }
    impl Peer {
        pub fn builder() -> PeerBuilder {
            PeerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents error information for QuotaOperation."]
    pub struct QuotaError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error code."]
        pub code: ::std::option::Option<QuotaErrorCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Free-form text that provides details on the cause of the error."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains additional information about the quota error. If available, `status.code` will be non zero."]
        pub status: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subject to whom this error applies. See the specific enum for more details on this field. For example, \"clientip:\" or \"project:\"."]
        pub subject: ::std::option::Option<::std::string::String>,
    }
    impl QuotaError {
        pub fn builder() -> QuotaErrorBuilder {
            QuotaErrorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Error code."]
    pub enum QuotaErrorCodeEnum {
        #[serde(rename = "UNSPECIFIED")]
        #[doc = "This is never used."]
        Unspecified,
        #[serde(rename = "RESOURCE_EXHAUSTED")]
        #[doc = "Quota allocation failed. Same as google.rpc.Code.RESOURCE_EXHAUSTED."]
        ResourceExhausted,
        #[serde(rename = "OUT_OF_RANGE")]
        #[doc = "Quota release failed. This error is ONLY returned on a NORMAL release. More formally: if a user requests a release of 10 tokens, but only 5 tokens were previously allocated, in a BEST_EFFORT release, this will be considered a success, 5 tokens will be released, and the result will be \"Ok\". If this is done in NORMAL mode, no tokens will be released, and an OUT_OF_RANGE error will be returned. Same as google.rpc.Code.OUT_OF_RANGE."]
        OutOfRange,
        #[serde(rename = "BILLING_NOT_ACTIVE")]
        #[doc = "Consumer cannot access the service because the service requires active billing."]
        BillingNotActive,
        #[serde(rename = "PROJECT_DELETED")]
        #[doc = "Consumer's project has been marked as deleted (soft deletion)."]
        ProjectDeleted,
        #[serde(rename = "API_KEY_INVALID")]
        #[doc = "Specified API key is invalid."]
        ApiKeyInvalid,
        #[serde(rename = "API_KEY_EXPIRED")]
        #[doc = "Specified API Key has expired."]
        ApiKeyExpired,
        #[serde(rename = "SPATULA_HEADER_INVALID")]
        #[doc = "Consumer's spatula header is invalid."]
        SpatulaHeaderInvalid,
        #[serde(rename = "LOAS_ROLE_INVALID")]
        #[doc = "The consumer's LOAS role is invalid."]
        LoasRoleInvalid,
        #[serde(rename = "NO_LOAS_PROJECT")]
        #[doc = "The consumer's LOAS role has no associated project."]
        NoLoasProject,
        #[serde(rename = "PROJECT_STATUS_UNAVAILABLE")]
        #[doc = "The backend server for looking up project id/number is unavailable."]
        ProjectStatusUnavailable,
        #[serde(rename = "SERVICE_STATUS_UNAVAILABLE")]
        #[doc = "The backend server for checking service status is unavailable."]
        ServiceStatusUnavailable,
        #[serde(rename = "BILLING_STATUS_UNAVAILABLE")]
        #[doc = "The backend server for checking billing status is unavailable."]
        BillingStatusUnavailable,
        #[serde(rename = "QUOTA_SYSTEM_UNAVAILABLE")]
        #[doc = "The backend server for checking quota limits is unavailable."]
        QuotaSystemUnavailable,
    }
    impl ::std::default::Default for QuotaErrorCodeEnum {
        fn default() -> Self {
            Self::Unspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains the quota information for a quota check response."]
    pub struct QuotaInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "limitExceeded")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Quota Metrics that have exceeded quota limits. For QuotaGroup-based quota, this is QuotaGroup.name For QuotaLimit-based quota, this is QuotaLimit.name See: google.api.Quota Deprecated: Use quota_metrics to get per quota group limit exceeded status."]
        pub limit_exceeded: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaConsumed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map of quota group name to the actual number of tokens consumed. If the quota check was not successful, then this will not be populated due to no quota consumption. We are not merging this field with 'quota_metrics' field because of the complexity of scaling in Chemist client code base. For simplicity, we will keep this field for Castor (that scales quota usage) and 'quota_metrics' for SuperQuota (that doesn't scale quota usage). "]
        pub quota_consumed:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Quota metrics to indicate the usage. Depending on the check request, one or more of the following metrics will be included: 1. For rate quota, per quota group or per quota metric incremental usage will be specified using the following delta metric: \"serviceruntime.googleapis.com/api/consumer/quota_used_count\" 2. For allocation quota, per quota metric total usage will be specified using the following gauge metric: \"serviceruntime.googleapis.com/allocation/consumer/quota_used_count\" 3. For both rate quota and allocation quota, the quota limit reached condition will be specified using the following boolean metric: \"serviceruntime.googleapis.com/quota/exceeded\""]
        pub quota_metrics:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricValueSet>>>,
    }
    impl QuotaInfo {
        pub fn builder() -> QuotaInfoBuilder {
            QuotaInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents information regarding a quota operation."]
    pub struct QuotaOperation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identity of the consumer for whom this quota operation is being performed. This can be in one of the following formats: project:, project_number:, api_key:."]
        pub consumer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Labels describing the operation."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "methodName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fully qualified name of the API method for which this quota operation is requested. This name is used for matching quota rules or metric rules and billing status rules defined in service configuration. This field should not be set if any of the following is true: (1) the quota operation is performed on non-API resources. (2) quota_metrics is set because the caller is doing quota override. Example of an RPC method name: google.example.library.v1.LibraryService.CreateShelf"]
        pub method_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identity of the operation. This is expected to be unique within the scope of the service that generated the operation, and guarantees idempotency in case of retries. In order to ensure best performance and latency in the Quota backends, operation_ids are optimally associated with time, so that related operations can be accessed fast in storage. For this reason, the recommended token for services that intend to operate at a high QPS is Unix time in nanos + UUID"]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents information about this operation. Each MetricValueSet corresponds to a metric defined in the service configuration. The data type used in the MetricValueSet must agree with the data type specified in the metric definition. Within a single operation, it is not allowed to have more than one MetricValue instances that have the same metric names and identical label value combinations. If a request has such duplicated MetricValue instances, the entire request is rejected with an invalid argument error. This field is mutually exclusive with method_name."]
        pub quota_metrics:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricValueSet>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Quota mode for this operation."]
        pub quota_mode: ::std::option::Option<QuotaOperationQuotaModeEnum>,
    }
    impl QuotaOperation {
        pub fn builder() -> QuotaOperationBuilder {
            QuotaOperationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Quota mode for this operation."]
    pub enum QuotaOperationQuotaModeEnum {
        #[serde(rename = "UNSPECIFIED")]
        #[doc = "Guard against implicit default. Must not be used."]
        Unspecified,
        #[serde(rename = "NORMAL")]
        #[doc = "For AllocateQuota request, allocates quota for the amount specified in the service configuration or specified using the quota metrics. If the amount is higher than the available quota, allocation error will be returned and no quota will be allocated. If multiple quotas are part of the request, and one fails, none of the quotas are allocated or released."]
        Normal,
        #[serde(rename = "BEST_EFFORT")]
        #[doc = "The operation allocates quota for the amount specified in the service configuration or specified using the quota metrics. If the amount is higher than the available quota, request does not fail but all available quota will be allocated. For rate quota, BEST_EFFORT will continue to deduct from other groups even if one does not have enough quota. For allocation, it will find the minimum available amount across all groups and deduct that amount from all the affected groups."]
        BestEffort,
        #[serde(rename = "CHECK_ONLY")]
        #[doc = "For AllocateQuota request, only checks if there is enough quota available and does not change the available quota. No lock is placed on the available quota either."]
        CheckOnly,
        #[serde(rename = "QUERY_ONLY")]
        #[doc = "Unimplemented. When used in AllocateQuotaRequest, this returns the effective quota limit(s) in the response, and no quota check will be performed. Not supported for other requests, and even for AllocateQuotaRequest, this is currently supported only for allowlisted services."]
        QueryOnly,
        #[serde(rename = "ADJUST_ONLY")]
        #[doc = "The operation allocates quota for the amount specified in the service configuration or specified using the quota metrics. If the requested amount is higher than the available quota, request does not fail and remaining quota would become negative (going over the limit). Not supported for Rate Quota."]
        AdjustOnly,
    }
    impl ::std::default::Default for QuotaOperationQuotaModeEnum {
        fn default() -> Self {
            Self::Unspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the properties needed for quota operations."]
    pub struct QuotaProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotaMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Quota mode for this operation."]
        pub quota_mode: ::std::option::Option<QuotaPropertiesQuotaModeEnum>,
    }
    impl QuotaProperties {
        pub fn builder() -> QuotaPropertiesBuilder {
            QuotaPropertiesBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Quota mode for this operation."]
    pub enum QuotaPropertiesQuotaModeEnum {
        #[serde(rename = "ACQUIRE")]
        #[doc = "Decreases available quota by the cost specified for the operation. If cost is higher than available quota, operation fails and returns error."]
        Acquire,
        #[serde(rename = "ACQUIRE_BEST_EFFORT")]
        #[doc = "Decreases available quota by the cost specified for the operation. If cost is higher than available quota, operation does not fail and available quota goes down to zero but it returns error."]
        AcquireBestEffort,
        #[serde(rename = "CHECK")]
        #[doc = "Does not change any available quota. Only checks if there is enough quota. No lock is placed on the checked tokens neither."]
        Check,
        #[serde(rename = "RELEASE")]
        #[doc = "Increases available quota by the operation cost specified for the operation."]
        Release,
    }
    impl ::std::default::Default for QuotaPropertiesQuotaModeEnum {
        fn default() -> Self {
            Self::Acquire
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the processing error of one Operation in the request."]
    pub struct ReportError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Operation.operation_id value from the request."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of the error when processing the Operation."]
        pub status: ::std::option::Option<::std::boxed::Box<Status>>,
    }
    impl ReportError {
        pub fn builder() -> ReportErrorBuilder {
            ReportErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for the Report method."]
    pub struct ReportRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operations to be reported. Typically the service should report one operation per request. Putting multiple operations into a single request is allowed, but should be used only when multiple operations are natually available at the time of the report. There is no limit on the number of operations in the same ReportRequest, however the ReportRequest size should be no larger than 1MB. See ReportResponse.report_errors for partial failure behavior."]
        pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceConfigId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies which version of service config should be used to process the request. If unspecified or no matching version can be found, the latest one will be used."]
        pub service_config_id: ::std::option::Option<::std::string::String>,
    }
    impl ReportRequest {
        pub fn builder() -> ReportRequestBuilder {
            ReportRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for the Report method."]
    pub struct ReportResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Partial failures, one for each `Operation` in the request that failed processing. There are three possible combinations of the RPC status: 1. The combination of a successful RPC status and an empty `report_errors` list indicates a complete success where all `Operations` in the request are processed successfully. 2. The combination of a successful RPC status and a non-empty `report_errors` list indicates a partial success where some `Operations` in the request succeeded. Each `Operation` that failed processing has a corresponding item in this list. 3. A failed RPC status indicates a general non-deterministic failure. When this happens, it's impossible to know which of the 'Operations' in the request succeeded or failed."]
        pub report_errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReportError>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceConfigId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actual config id used to process the request."]
        pub service_config_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceRolloutId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current service rollout id used to process the request."]
        pub service_rollout_id: ::std::option::Option<::std::string::String>,
    }
    impl ReportResponse {
        pub fn builder() -> ReportResponseBuilder {
            ReportResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This message defines attributes for an HTTP request. If the actual request is not an HTTP request, the runtime system should try to map the actual request to an equivalent HTTP request."]
    pub struct Request {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "auth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request authentication. May be absent for unauthenticated requests. Derived from the HTTP request `Authorization` header or equivalent."]
        pub auth: ::std::option::Option<::std::boxed::Box<Auth>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP request headers. If multiple headers share the same key, they must be merged according to the HTTP spec. All header keys must be lowercased, because HTTP header keys are case-insensitive."]
        pub headers:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "host")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP request `Host` header value."]
        pub host: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID for a request, which can be propagated to downstream systems. The ID should have low probability of collision within a single day for a specific service."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP request method, such as `GET`, `POST`."]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP URL path."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protocol")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The network protocol used with the request, such as \"http/1.1\", \"spdy/3\", \"h2\", \"h2c\", \"webrtc\", \"tcp\", \"udp\", \"quic\". See https://www.iana.org/assignments/tls-extensiontype-values/tls-extensiontype-values.xhtml#alpn-protocol-ids for details."]
        pub protocol: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP URL query in the format of `name1=value1&name2=value2`, as it appears in the first line of the HTTP request. No decoding is performed."]
        pub query: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A special parameter for request reason. It is used by security systems to associate auditing information with a request."]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheme")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP URL scheme, such as `http` and `https`."]
        pub scheme: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP request size in bytes. If unknown, it must be -1."]
        pub size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "time")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the `destination` service receives the last byte of the request."]
        pub time: ::std::option::Option<::std::string::String>,
    }
    impl Request {
        pub fn builder() -> RequestBuilder {
            RequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata about the request."]
    pub struct RequestMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "callerIp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IP address of the caller. For caller from internet, this will be public IPv4 or IPv6 address. For caller from a Compute Engine VM with external IP address, this will be the VM's external IP address. For caller from a Compute Engine VM without external IP address, if the VM is in the same organization (or project) as the accessed resource, `caller_ip` will be the VM's internal IPv4 address, otherwise the `caller_ip` will be redacted to \"gce-internal-ip\". See https://cloud.google.com/compute/docs/vpc/ for more information."]
        pub caller_ip: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "callerNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The network of the caller. Set only if the network host project is part of the same GCP organization (or project) as the accessed resource. See https://cloud.google.com/compute/docs/vpc/ for more information. This is a scheme-less URI full resource name. For example: \"//compute.googleapis.com/projects/PROJECT_ID/global/networks/NETWORK_ID\""]
        pub caller_network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "callerSuppliedUserAgent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user agent of the caller. This information is not authenticated and should be treated accordingly. For example: + `google-api-python-client/1.4.0`: The request was made by the Google API client for Python. + `Cloud SDK Command Line Tool apitools-client/1.0 gcloud/0.9.62`: The request was made by the Google Cloud SDK CLI (gcloud). + `AppEngine-Google; (+http://code.google.com/appengine; appid: s~my-project`: The request was made from the `my-project` App Engine app. NOLINT"]
        pub caller_supplied_user_agent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationAttributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The destination of a network activity, such as accepting a TCP connection. In a multi hop network activity, the destination represents the receiver of the last hop. Only two fields are used in this message, Peer.port and Peer.ip. These fields are optionally populated by those services utilizing the IAM condition feature."]
        pub destination_attributes: ::std::option::Option<::std::boxed::Box<Peer>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestAttributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Request attributes used in IAM condition evaluation. This field contains request attributes like request time and access levels associated with the request. To get the whole view of the attributes used in IAM condition evaluation, the user must also look into `AuditLog.authentication_info.resource_attributes`."]
        pub request_attributes: ::std::option::Option<::std::boxed::Box<Request>>,
    }
    impl RequestMetadata {
        pub fn builder() -> RequestMetadataBuilder {
            RequestMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This message defines core attributes for a resource. A resource is an addressable (named) entity provided by the destination service. For example, a file stored on a network storage service."]
    pub struct Resource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotations is an unstructured key-value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: https://kubernetes.io/docs/user-guide/annotations"]
        pub annotations:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The timestamp when the resource was created. This may be either the time creation was initiated or when it was completed."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The timestamp when the resource was deleted. If the resource is not deleted, this must be empty."]
        pub delete_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mutable. The display name set by clients. Must be <= 63 characters."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. An opaque value that uniquely identifies a version or generation of a resource. It can be used to confirm that the client and server agree on the ordering of a resource being written."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The labels or tags on the resource, such as AWS resource tags and Kubernetes resource labels."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. The location of the resource. The location encoding is specific to the service provider, and new encoding may be introduced as the service evolves. For Google Cloud products, the encoding is what is used by Google Cloud APIs, such as `us-east1`, `aws-us-east-1`, and `azure-eastus2`. The semantics of `location` is identical to the `cloud.googleapis.com/location` label used by some Google Cloud APIs."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stable identifier (name) of a resource on the `service`. A resource can be logically identified as \"//{resource.service}/{resource.name}\". The differences between a resource name and a URI are: * Resource name is a logical identifier, independent of network protocol and API version. For example, `//pubsub.googleapis.com/projects/123/topics/news-feed`. * URI often includes protocol and version information, so it can be used directly by applications. For example, `https://pubsub.googleapis.com/v1/projects/123/topics/news-feed`. See https://cloud.google.com/apis/design/resource_names for details."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "service")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the service that this resource belongs to, such as `pubsub.googleapis.com`. The service may be different from the DNS hostname that actually serves the request."]
        pub service: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the resource. The syntax is platform-specific because different platforms define their resources differently. For Google APIs, the type format must be \"{service}/{kind}\"."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the resource. UID is unique in the time and space for this resource within the scope of the service. It is typically generated by the server on successful creation of a resource and must not be changed. UID is used to uniquely identify resources with resource name reuses. This should be a UUID4."]
        pub uid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The timestamp when the resource was last updated. Any change to the resource made by users must refresh this value. Changes to a resource made by the service should refresh this value."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl Resource {
        pub fn builder() -> ResourceBuilder {
            ResourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a resource associated with this operation."]
    pub struct ResourceInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceContainer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of the parent of this resource instance. Must be in one of the following formats: - `projects/` - `folders/` - `organizations/`"]
        pub resource_container: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location of the resource. If not empty, the resource will be checked against location policy. The value must be a valid zone, region or multiregion. For example: \"europe-west4\" or \"northamerica-northeast1-a\""]
        pub resource_location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the resource. This is used for auditing purposes."]
        pub resource_name: ::std::option::Option<::std::string::String>,
    }
    impl ResourceInfo {
        pub fn builder() -> ResourceInfoBuilder {
            ResourceInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Location information about a resource."]
    pub struct ResourceLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentLocations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The locations of a resource after the execution of the operation. Requests to create or delete a location based resource must populate the 'current_locations' field and not the 'original_locations' field. For example: \"europe-west1-a\" \"us-east1\" \"nam3\""]
        pub current_locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalLocations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The locations of a resource prior to the execution of the operation. Requests that mutate the resource's location must populate both the 'original_locations' as well as the 'current_locations' fields. For example: \"europe-west1-a\" \"us-east1\" \"nam3\""]
        pub original_locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ResourceLocation {
        pub fn builder() -> ResourceLocationBuilder {
            ResourceLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identity delegation history of an authenticated service account."]
    pub struct ServiceAccountDelegationInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstPartyPrincipal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "First party (Google) identity as the real authority."]
        pub first_party_principal: ::std::option::Option<::std::boxed::Box<FirstPartyPrincipal>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "principalSubject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A string representing the principal_subject associated with the identity. See go/3pical for more info on how principal_subject is formatted."]
        pub principal_subject: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thirdPartyPrincipal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Third party identity as the real authority."]
        pub third_party_principal: ::std::option::Option<::std::boxed::Box<ThirdPartyPrincipal>>,
    }
    impl ServiceAccountDelegationInfo {
        pub fn builder() -> ServiceAccountDelegationInfoBuilder {
            ServiceAccountDelegationInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The context of a span, attached to Exemplars in Distribution values during aggregation. It contains the name of a span with format: projects/[PROJECT_ID_OR_NUMBER]/traces/[TRACE_ID]/spans/[SPAN_ID]"]
    pub struct SpanContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spanName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the span. The format is: projects/[PROJECT_ID_OR_NUMBER]/traces/[TRACE_ID]/spans/[SPAN_ID] `[TRACE_ID]` is a unique identifier for a trace within a project; it is a 32-character hexadecimal encoding of a 16-byte array. `[SPAN_ID]` is a unique identifier for a span within a trace; it is a 16-character hexadecimal encoding of an 8-byte array."]
        pub span_name: ::std::option::Option<::std::string::String>,
    }
    impl SpanContext {
        pub fn builder() -> SpanContextBuilder {
            SpanContextBuilder::default()
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
    #[doc = "Third party identity principal."]
    pub struct ThirdPartyPrincipal {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thirdPartyClaims")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata about third party identity."]
        pub third_party_claims:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ThirdPartyPrincipal {
        pub fn builder() -> ThirdPartyPrincipalBuilder {
            ThirdPartyPrincipalBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A span represents a single operation within a trace. Spans can be nested to form a trace tree. Often, a trace contains a root span that describes the end-to-end latency, and one or more subspans for its sub-operations. A trace can also contain multiple root spans, or none at all. Spans do not need to be contiguous—there may be gaps or overlaps between spans in a trace."]
    pub struct TraceSpan {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A set of attributes on the span. You can have up to 32 attributes per span."]
        pub attributes: ::std::option::Option<::std::boxed::Box<Attributes>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "childSpanCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional number of child spans that were generated while this span was active. If set, allows implementation to detect missing child spans."]
        pub child_span_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of the span's operation (up to 128 bytes). Stackdriver Trace displays the description in the Google Cloud Platform Console. For example, the display name can be a qualified method name or a file name and a line number where the operation is called. A best practice is to use the same display name within an application and at the same call point. This makes it easier to correlate spans in different traces."]
        pub display_name: ::std::option::Option<::std::boxed::Box<TruncatableString>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end time of the span. On the client side, this is the time kept by the local machine where the span execution ends. On the server side, this is the time when the server application handler stops running."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the span in the following format: projects/[PROJECT_ID]/traces/[TRACE_ID]/spans/SPAN_ID is a unique identifier for a trace within a project; it is a 32-character hexadecimal encoding of a 16-byte array. [SPAN_ID] is a unique identifier for a span within a trace; it is a 16-character hexadecimal encoding of an 8-byte array."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentSpanId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [SPAN_ID] of this span's parent span. If this is a root span, then this field must be empty."]
        pub parent_span_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sameProcessAsParentSpan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(Optional) Set this parameter to indicate whether this span is in the same process as its parent. If you do not set this parameter, Stackdriver Trace is unable to take advantage of this helpful information."]
        pub same_process_as_parent_span: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spanId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [SPAN_ID] portion of the span's resource name."]
        pub span_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spanKind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Distinguishes between spans generated in a particular context. For example, two spans with the same name may be distinguished using `CLIENT` (caller) and `SERVER` (callee) to identify an RPC call."]
        pub span_kind: ::std::option::Option<TraceSpanSpanKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start time of the span. On the client side, this is the time kept by the local machine where the span execution starts. On the server side, this is the time when the server's application handler starts running."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional final status for this span."]
        pub status: ::std::option::Option<::std::boxed::Box<Status>>,
    }
    impl TraceSpan {
        pub fn builder() -> TraceSpanBuilder {
            TraceSpanBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Distinguishes between spans generated in a particular context. For example, two spans with the same name may be distinguished using `CLIENT` (caller) and `SERVER` (callee) to identify an RPC call."]
    pub enum TraceSpanSpanKindEnum {
        #[serde(rename = "SPAN_KIND_UNSPECIFIED")]
        #[doc = "Unspecified. Do NOT use as default. Implementations MAY assume SpanKind.INTERNAL to be default."]
        SpanKindUnspecified,
        #[serde(rename = "INTERNAL")]
        #[doc = "Indicates that the span is used internally. Default value."]
        Internal,
        #[serde(rename = "SERVER")]
        #[doc = "Indicates that the span covers server-side handling of an RPC or other remote network request."]
        Server,
        #[serde(rename = "CLIENT")]
        #[doc = "Indicates that the span covers the client-side wrapper around an RPC or other remote request."]
        Client,
        #[serde(rename = "PRODUCER")]
        #[doc = "Indicates that the span describes producer sending a message to a broker. Unlike client and server, there is no direct critical path latency relationship between producer and consumer spans (e.g. publishing a message to a pubsub service)."]
        Producer,
        #[serde(rename = "CONSUMER")]
        #[doc = "Indicates that the span describes consumer receiving a message from a broker. Unlike client and server, there is no direct critical path latency relationship between producer and consumer spans (e.g. receiving a message from a pubsub service subscription)."]
        Consumer,
    }
    impl ::std::default::Default for TraceSpanSpanKindEnum {
        fn default() -> Self {
            Self::SpanKindUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a string that might be shortened to a specified length."]
    pub struct TruncatableString {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "truncatedByteCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of bytes removed from the original string. If this value is 0, then the string was not shortened."]
        pub truncated_byte_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The shortened string. For example, if the original string is 500 bytes long and the limit of the string is 128 bytes, then `value` contains the first 128 bytes of the 500-byte string. Truncation always happens on a UTF8 character boundary. If there are multi-byte characters in the string, then the length of the shortened string might be less than the size limit."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl TruncatableString {
        pub fn builder() -> TruncatableStringBuilder {
            TruncatableStringBuilder::default()
        }
    }
}
