#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to add a newly created and configured tenant project to a tenancy unit."]
pub struct AddTenantProjectRequest {
    #[serde(rename = "projectConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration of the new tenant project to be added to tenancy unit resources."]
    pub project_config: ::std::option::Option<::std::boxed::Box<TenantProjectConfig>>,
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Tag of the added project. Must be less than 128 characters. Required."]
    pub tag: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Api is a light-weight descriptor for an API Interface. Interfaces are also described as \"protocol buffer services\" in some contexts, such as by the \"service\" keyword in a .proto file, but they are different from API Services, which represent a concrete implementation of an interface as opposed to simply a description of methods and bindings. They are also sometimes simply referred to as \"APIs\" in other contexts, such as the name of this message itself. See https://cloud.google.com/apis/design/glossary for detailed terminology."]
pub struct Api {
    #[serde(rename = "methods")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The methods of this interface, in unspecified order."]
    pub methods: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Method>>>,
    #[serde(rename = "mixins")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Included interfaces. See Mixin."]
    pub mixins: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Mixin>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fully qualified name of this interface, including package name followed by the interface's simple name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Any metadata attached to the interface."]
    pub options: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Option>>>,
    #[serde(rename = "sourceContext")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Source context for the protocol buffer service represented by this message."]
    pub source_context: ::std::option::Option<::std::boxed::Box<SourceContext>>,
    #[serde(rename = "syntax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source syntax of the service."]
    pub syntax: ::std::option::Option<ApiSyntaxEnum>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A version string for this interface. If specified, must have the form `major-version.minor-version`, as in `1.10`. If the minor version is omitted, it defaults to zero. If the entire version field is empty, the major version is derived from the package name, as outlined below. If the field is not empty, the version in the package name will be verified to be consistent with what is provided here. The versioning schema uses [semantic versioning](http://semver.org) where the major version number indicates a breaking change and the minor version an additive, non-breaking change. Both version numbers are signals to users what to expect from different versions, and should be carefully chosen based on the product plan. The major version is also reflected in the package name of the interface, which must end in `v`, as in `google.feature.v1`. For major versions 0 and 1, the suffix can be omitted. Zero major versions must only be used for experimental, non-GA interfaces. "]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The source syntax of the service."]
pub enum ApiSyntaxEnum {
    #[serde(rename = "SYNTAX_PROTO2")]
    #[doc = "Syntax `proto2`."]
    SyntaxProto2,
    #[serde(rename = "SYNTAX_PROTO3")]
    #[doc = "Syntax `proto3`."]
    SyntaxProto3,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to apply configuration to an existing tenant project."]
pub struct ApplyTenantProjectConfigRequest {
    #[serde(rename = "projectConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration that should be applied to the existing tenant project."]
    pub project_config: ::std::option::Option<::std::boxed::Box<TenantProjectConfig>>,
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Tag of the project. Must be less than 128 characters. Required."]
    pub tag: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to attach an existing project to the tenancy unit as a new tenant resource."]
pub struct AttachTenantProjectRequest {
    #[serde(rename = "externalResource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When attaching an external project, this is in the format of `projects/{project_number}`."]
    pub external_resource: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reservedResource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When attaching a reserved project already in tenancy units, this is the tag of a tenant resource under the tenancy unit for the managed service's service producer project. The reserved tenant resource must be in an active state."]
    pub reserved_resource: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Tag of the tenant resource after attachment. Must be less than 128 characters. Required."]
    pub tag: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for an authentication provider, including support for [JSON Web Token (JWT)](https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32)."]
pub struct AuthProvider {
    #[serde(rename = "audiences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of JWT [audiences](https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.3). that are allowed to access. A JWT containing any of these audiences will be accepted. When this setting is absent, JWTs with audiences: - \"https://[service.name]/[google.protobuf.Api.name]\" - \"https://[service.name]/\" will be accepted. For example, if no audiences are in the setting, LibraryService API will accept JWTs with the following audiences: - https://library-example.googleapis.com/google.example.library.v1.LibraryService - https://library-example.googleapis.com/ Example: audiences: bookstore_android.apps.googleusercontent.com, bookstore_web.apps.googleusercontent.com"]
    pub audiences: ::std::option::Option<::std::string::String>,
    #[serde(rename = "authorizationUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Redirect URL if JWT token is required but not present or is expired. Implement authorizationUrl of securityDefinitions in OpenAPI spec."]
    pub authorization_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the auth provider. It will be referred to by `AuthRequirement.provider_id`. Example: \"bookstore_auth\"."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "issuer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies the principal that issued the JWT. See https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.1 Usually a URL or an email address. Example: https://securetoken.google.com Example: 1234567-compute@developer.gserviceaccount.com"]
    pub issuer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "jwksUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of the provider's public key set to validate signature of the JWT. See [OpenID Discovery](https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata). Optional if the key set document: - can be retrieved from [OpenID Discovery](https://openid.net/specs/openid-connect-discovery-1_0.html) of the issuer. - can be inferred from the email domain of the issuer (e.g. a Google service account). Example: https://www.googleapis.com/oauth2/v1/certs"]
    pub jwks_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "jwtLocations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines the locations to extract the JWT. JWT locations can be either from HTTP headers or URL query parameters. The rule is that the first match wins. The checking order is: checking all headers first, then URL query parameters. If not specified, default to use following 3 locations: 1) Authorization: Bearer 2) x-goog-iap-jwt-assertion 3) access_token query parameter Default locations can be specified as followings: jwt_locations: - header: Authorization value_prefix: \"Bearer \" - header: x-goog-iap-jwt-assertion - query: access_token"]
    pub jwt_locations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<JwtLocation>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "User-defined authentication requirements, including support for [JSON Web Token (JWT)](https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32)."]
pub struct AuthRequirement {
    #[serde(rename = "audiences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "NOTE: This will be deprecated soon, once AuthProvider.audiences is implemented and accepted in all the runtime components. The list of JWT [audiences](https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.3). that are allowed to access. A JWT containing any of these audiences will be accepted. When this setting is absent, only JWTs with audience \"https://Service_name/API_name\" will be accepted. For example, if no audiences are in the setting, LibraryService API will only accept JWTs with the following audience \"https://library-example.googleapis.com/google.example.library.v1.LibraryService\". Example: audiences: bookstore_android.apps.googleusercontent.com, bookstore_web.apps.googleusercontent.com"]
    pub audiences: ::std::option::Option<::std::string::String>,
    #[serde(rename = "providerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "id from authentication provider. Example: provider_id: bookstore_auth"]
    pub provider_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`Authentication` defines the authentication configuration for an API. Example for an API targeted for external use: name: calendar.googleapis.com authentication: providers: - id: google_calendar_auth jwks_uri: https://www.googleapis.com/oauth2/v1/certs issuer: https://securetoken.google.com rules: - selector: \"*\" requirements: provider_id: google_calendar_auth"]
pub struct Authentication {
    #[serde(rename = "providers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines a set of authentication providers that a service supports."]
    pub providers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuthProvider>>>,
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of authentication rules that apply to individual API methods. **NOTE:** All service configuration rules follow \"last one wins\" order."]
    pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuthenticationRule>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Authentication rules for the service. By default, if a method has any authentication requirements, every request must include a valid credential matching one of the requirements. It's an error to include more than one kind of credential in a single request. If a method doesn't have any auth requirements, request credentials will be ignored."]
pub struct AuthenticationRule {
    #[serde(rename = "allowWithoutCredential")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, the service accepts API keys without any other credential. This flag only applies to HTTP and gRPC requests."]
    pub allow_without_credential: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "oauth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The requirements for OAuth credentials."]
    pub oauth: ::std::option::Option<::std::boxed::Box<OAuthRequirements>>,
    #[serde(rename = "requirements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requirements for additional authentication providers."]
    pub requirements: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuthRequirement>>>,
    #[serde(rename = "selector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Selects the methods to which this rule applies. Refer to selector for syntax details."]
    pub selector: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`Backend` defines the backend configuration for a service."]
pub struct Backend {
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of API backend rules that apply to individual API methods. **NOTE:** All service configuration rules follow \"last one wins\" order."]
    pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BackendRule>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A backend rule provides configuration for an individual API element."]
pub struct BackendRule {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The address of the API backend. The scheme is used to determine the backend protocol and security. The following schemes are accepted: SCHEME PROTOCOL SECURITY http:// HTTP None https:// HTTP TLS grpc:// gRPC None grpcs:// gRPC TLS It is recommended to explicitly include a scheme. Leaving out the scheme may cause constrasting behaviors across platforms. If the port is unspecified, the default is: - 80 for schemes without TLS - 443 for schemes with TLS For HTTP backends, use protocol to specify the protocol version."]
    pub address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deadline")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of seconds to wait for a response from a request. The default varies based on the request protocol and deployment environment."]
    pub deadline: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "disableAuth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When disable_auth is true, a JWT ID token won't be generated and the original \"Authorization\" HTTP header will be preserved. If the header is used to carry the original token and is expected by the backend, this field must be set to true to preserve the header."]
    pub disable_auth: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "jwtAudience")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The JWT audience is used when generating a JWT ID token for the backend. This ID token will be added in the HTTP \"authorization\" header, and sent to the backend."]
    pub jwt_audience: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minDeadline")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minimum deadline in seconds needed for this method. Calls having deadline value lower than this will be rejected."]
    pub min_deadline: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "operationDeadline")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of seconds to wait for the completion of a long running operation. The default is no deadline."]
    pub operation_deadline: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "pathTranslation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub path_translation: ::std::option::Option<BackendRulePathTranslationEnum>,
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The protocol used for sending a request to the backend. The supported values are \"http/1.1\" and \"h2\". The default value is inferred from the scheme in the address field: SCHEME PROTOCOL http:// http/1.1 https:// http/1.1 grpc:// h2 grpcs:// h2 For secure HTTP backends (https://) that support HTTP/2, set this field to \"h2\" for improved performance. Configuring this field to non-default values is only supported for secure HTTP backends. This field will be ignored for all other backends. See https://www.iana.org/assignments/tls-extensiontype-values/tls-extensiontype-values.xhtml#alpn-protocol-ids for more details on the supported values."]
    pub protocol: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Selects the methods to which this rule applies. Refer to selector for syntax details."]
    pub selector: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum BackendRulePathTranslationEnum {
    #[serde(rename = "PATH_TRANSLATION_UNSPECIFIED")]
    #[doc = ""]
    PathTranslationUnspecified,
    #[serde(rename = "CONSTANT_ADDRESS")]
    #[doc = "Use the backend address as-is, with no modification to the path. If the URL pattern contains variables, the variable names and values will be appended to the query string. If a query string parameter and a URL pattern variable have the same name, this may result in duplicate keys in the query string. # Examples Given the following operation config: Method path: /api/company/{cid}/user/{uid} Backend address: https://example.cloudfunctions.net/getUser Requests to the following request paths will call the backend at the translated path: Request path: /api/company/widgetworks/user/johndoe Translated: https://example.cloudfunctions.net/getUser?cid=widgetworks&uid=johndoe Request path: /api/company/widgetworks/user/johndoe?timezone=EST Translated: https://example.cloudfunctions.net/getUser?timezone=EST&cid=widgetworks&uid=johndoe"]
    ConstantAddress,
    #[serde(rename = "APPEND_PATH_TO_ADDRESS")]
    #[doc = "The request path will be appended to the backend address. # Examples Given the following operation config: Method path: /api/company/{cid}/user/{uid} Backend address: https://example.appspot.com Requests to the following request paths will call the backend at the translated path: Request path: /api/company/widgetworks/user/johndoe Translated: https://example.appspot.com/api/company/widgetworks/user/johndoe Request path: /api/company/widgetworks/user/johndoe?timezone=EST Translated: https://example.appspot.com/api/company/widgetworks/user/johndoe?timezone=EST"]
    AppendPathToAddress,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Billing related configuration of the service. The following example shows how to configure monitored resources and metrics for billing, `consumer_destinations` is the only supported destination and the monitored resources need at least one label key `cloud.googleapis.com/location` to indicate the location of the billing usage, using different monitored resources between monitoring and billing is recommended so they can be evolved independently: monitored_resources: - type: library.googleapis.com/billing_branch labels: - key: cloud.googleapis.com/location description: | Predefined label to support billing location restriction. - key: city description: | Custom label to define the city where the library branch is located in. - key: name description: Custom label to define the name of the library branch. metrics: - name: library.googleapis.com/book/borrowed_count metric_kind: DELTA value_type: INT64 unit: \"1\" billing: consumer_destinations: - monitored_resource: library.googleapis.com/billing_branch metrics: - library.googleapis.com/book/borrowed_count"]
pub struct Billing {
    #[serde(rename = "consumerDestinations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Billing configurations for sending metrics to the consumer project. There can be multiple consumer destinations per service, each one must have a different monitored resource type. A metric can be used in at most one consumer destination."]
    pub consumer_destinations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BillingDestination>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes the billing configuration for a new tenant project."]
pub struct BillingConfig {
    #[serde(rename = "billingAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the billing account. For example `billingAccounts/012345-567890-ABCDEF`."]
    pub billing_account: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration of a specific billing destination (Currently only support bill against consumer project)."]
pub struct BillingDestination {
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Names of the metrics to report to this billing destination. Each name must be defined in Service.metrics section."]
    pub metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "monitoredResource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The monitored resource type. The type must be defined in Service.monitored_resources section."]
    pub monitored_resource: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for Operations.CancelOperation."]
pub struct CancelOperationRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`Context` defines which contexts an API requests. Example: context: rules: - selector: \"*\" requested: - google.rpc.context.ProjectContext - google.rpc.context.OriginContext The above specifies that all methods in the API request `google.rpc.context.ProjectContext` and `google.rpc.context.OriginContext`. Available context types are defined in package `google.rpc.context`. This also provides mechanism to allowlist any protobuf message extension that can be sent in grpc metadata using “x-goog-ext--bin” and “x-goog-ext--jspb” format. For example, list any service specific protobuf types that can appear in grpc metadata as follows in your yaml file: Example: context: rules: - selector: \"google.example.library.v1.LibraryService.CreateBook\" allowed_request_extensions: - google.foo.v1.NewExtension allowed_response_extensions: - google.foo.v1.NewExtension You can also specify extension ID instead of fully qualified extension name here."]
pub struct Context {
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of RPC context rules that apply to individual API methods. **NOTE:** All service configuration rules follow \"last one wins\" order."]
    pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContextRule>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A context rule provides information about the context for an individual API element."]
pub struct ContextRule {
    #[serde(rename = "allowedRequestExtensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of full type names or extension IDs of extensions allowed in grpc side channel from client to backend."]
    pub allowed_request_extensions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "allowedResponseExtensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of full type names or extension IDs of extensions allowed in grpc side channel from backend to client."]
    pub allowed_response_extensions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "provided")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of full type names of provided contexts."]
    pub provided: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "requested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of full type names of requested contexts."]
    pub requested: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "selector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Selects the methods to which this rule applies. Refer to selector for syntax details."]
    pub selector: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Selects and configures the service controller used by the service. The service controller handles features like abuse, quota, billing, logging, monitoring, etc."]
pub struct Control {
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The service control environment to use. If empty, no control plane feature (like quota and billing) will be enabled."]
    pub environment: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to create a tenancy unit for a service consumer of a managed service."]
pub struct CreateTenancyUnitRequest {
    #[serde(rename = "tenancyUnitId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Optional service producer-provided identifier of the tenancy unit. Must be no longer than 40 characters and preferably URI friendly. If it isn't provided, a UID for the tenancy unit is automatically generated. The identifier must be unique across a managed service. If the tenancy unit already exists for the managed service and service consumer pair, calling `CreateTenancyUnit` returns the existing tenancy unit if the provided identifier is identical or empty, otherwise the call fails."]
    pub tenancy_unit_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Customize service error responses. For example, list any service specific protobuf types that can appear in error detail lists of error responses. Example: custom_error: types: - google.foo.v1.CustomError - google.foo.v1.AnotherError"]
pub struct CustomError {
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of custom error rules that apply to individual API messages. **NOTE:** All service configuration rules follow \"last one wins\" order."]
    pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomErrorRule>>>,
    #[serde(rename = "types")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of custom error detail types, e.g. 'google.foo.v1.CustomError'."]
    pub types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A custom error rule."]
pub struct CustomErrorRule {
    #[serde(rename = "isErrorType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mark this message as possible payload in error response. Otherwise, objects of this type will be filtered when they appear in error payload."]
    pub is_error_type: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "selector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Selects messages to which this rule applies. Refer to selector for syntax details."]
    pub selector: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A custom pattern is used for defining custom HTTP verb."]
pub struct CustomHttpPattern {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of this custom HTTP verb."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The path matched by this custom verb."]
    pub path: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message to delete tenant project resource from the tenancy unit."]
pub struct DeleteTenantProjectRequest {
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Tag of the resource within the tenancy unit."]
    pub tag: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`Documentation` provides the information for describing a service. Example: documentation: summary: > The Google Calendar API gives access to most calendar features. pages: - name: Overview content: (== include google/foo/overview.md ==) - name: Tutorial content: (== include google/foo/tutorial.md ==) subpages; - name: Java content: (== include google/foo/tutorial_java.md ==) rules: - selector: google.calendar.Calendar.Get description: > ... - selector: google.calendar.Calendar.Put description: > ... Documentation is provided in markdown syntax. In addition to standard markdown features, definition lists, tables and fenced code blocks are supported. Section headers can be provided and are interpreted relative to the section nesting of the context where a documentation fragment is embedded. Documentation from the IDL is merged with documentation defined via the config at normalization time, where documentation provided by config rules overrides IDL provided. A number of constructs specific to the API platform are supported in documentation text. In order to reference a proto element, the following notation can be used: [fully.qualified.proto.name][] To override the display text used for the link, this can be used: [display text][fully.qualified.proto.name] Text can be excluded from doc using the following notation: (-- internal comment --) A few directives are available in documentation. Note that directives must appear on a single line to be properly identified. The `include` directive includes a markdown file from an external source: (== include path/to/file ==) The `resource_for` directive marks a message to be the resource of a collection in REST view. If it is not specified, tools attempt to infer the resource from the operations in a collection: (== resource_for v1.shelves.books ==) The directive `suppress_warning` does not directly affect documentation and is documented together with service config validation."]
pub struct Documentation {
    #[serde(rename = "documentationRootUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL to the root of documentation."]
    pub documentation_root_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "overview")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Declares a single overview page. For example: documentation: summary: ... overview: (== include overview.md ==) This is a shortcut for the following declaration (using pages style): documentation: summary: ... pages: - name: Overview content: (== include overview.md ==) Note: you cannot specify both `overview` field and `pages` field."]
    pub overview: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The top level pages for the documentation set."]
    pub pages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Page>>>,
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of documentation rules that apply to individual API elements. **NOTE:** All service configuration rules follow \"last one wins\" order."]
    pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DocumentationRule>>>,
    #[serde(rename = "serviceRootUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the service root url if the default one (the service name from the yaml file) is not suitable. This can be seen in any fully specified service urls as well as sections that show a base that other urls are relative to."]
    pub service_root_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short summary of what the service does. Can only be provided by plain text."]
    pub summary: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A documentation rule provides information about individual API elements."]
pub struct DocumentationRule {
    #[serde(rename = "deprecationDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecation description of the selected element(s). It can be provided if an element is marked as `deprecated`."]
    pub deprecation_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the selected API(s)."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The selector is a comma-separated list of patterns. Each pattern is a qualified name of the element which may end in \"*\", indicating a wildcard. Wildcards are only allowed at the end and for a whole component of the qualified name, i.e. \"foo.*\" is ok, but not \"foo.b*\" or \"foo.*.bar\". A wildcard will match one or more components. To specify a default for all applicable elements, the whole pattern \"*\" is used."]
    pub selector: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`Endpoint` describes a network endpoint of a service that serves a set of APIs. It is commonly known as a service endpoint. A service may expose any number of service endpoints, and all service endpoints share the same service definition, such as quota limits and monitoring metrics. Example service configuration: name: library-example.googleapis.com endpoints: # Below entry makes 'google.example.library.v1.Library' # API be served from endpoint address library-example.googleapis.com. # It also allows HTTP OPTIONS calls to be passed to the backend, for # it to decide whether the subsequent cross-origin request is # allowed to proceed. - name: library-example.googleapis.com allow_cors: true"]
pub struct Endpoint {
    #[serde(rename = "aliases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DEPRECATED: This field is no longer supported. Instead of using aliases, please specify multiple google.api.Endpoint for each of the intended aliases. Additional names that this endpoint will be hosted on."]
    pub aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "allowCors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allowing [CORS](https://en.wikipedia.org/wiki/Cross-origin_resource_sharing), aka cross-domain traffic, would allow the backends served from this endpoint to receive and respond to HTTP OPTIONS requests. The response will be used by the browser to determine whether the subsequent cross-origin request is allowed to proceed."]
    pub allow_cors: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The canonical name of this endpoint."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The specification of an Internet routable address of API frontend that will handle requests to this [API Endpoint](https://cloud.google.com/apis/design/glossary). It should be either a valid IPv4 address or a fully-qualified domain name. For example, \"8.8.8.8\" or \"myservice.appspot.com\"."]
    pub target: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Enum type definition."]
pub struct Enum {
    #[serde(rename = "enumvalue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enum value definitions."]
    pub enumvalue: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EnumValue>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enum type name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Protocol buffer options."]
    pub options: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Option>>>,
    #[serde(rename = "sourceContext")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source context."]
    pub source_context: ::std::option::Option<::std::boxed::Box<SourceContext>>,
    #[serde(rename = "syntax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source syntax."]
    pub syntax: ::std::option::Option<EnumSyntaxEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The source syntax."]
pub enum EnumSyntaxEnum {
    #[serde(rename = "SYNTAX_PROTO2")]
    #[doc = "Syntax `proto2`."]
    SyntaxProto2,
    #[serde(rename = "SYNTAX_PROTO3")]
    #[doc = "Syntax `proto3`."]
    SyntaxProto3,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Enum value definition."]
pub struct EnumValue {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enum value name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "number")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enum value number."]
    pub number: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Protocol buffer options."]
    pub options: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Option>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single field of a message type."]
pub struct Field {
    #[serde(rename = "cardinality")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The field cardinality."]
    pub cardinality: ::std::option::Option<FieldCardinalityEnum>,
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The string value of the default value of this field. Proto2 syntax only."]
    pub default_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "jsonName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The field JSON name."]
    pub json_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The field type."]
    pub kind: ::std::option::Option<FieldKindEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The field name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "number")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The field number."]
    pub number: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "oneofIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The index of the field type in `Type.oneofs`, for message or enumeration types. The first type has index 1; zero means the type is not in the list."]
    pub oneof_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The protocol buffer options."]
    pub options: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Option>>>,
    #[serde(rename = "packed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to use alternative packed wire representation."]
    pub packed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "typeUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The field type URL, without the scheme, for message or enumeration types. Example: `\"type.googleapis.com/google.protobuf.Timestamp\"`."]
    pub type_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The field cardinality."]
pub enum FieldCardinalityEnum {
    #[serde(rename = "CARDINALITY_UNKNOWN")]
    #[doc = "For fields with unknown cardinality."]
    CardinalityUnknown,
    #[serde(rename = "CARDINALITY_OPTIONAL")]
    #[doc = "For optional fields."]
    CardinalityOptional,
    #[serde(rename = "CARDINALITY_REQUIRED")]
    #[doc = "For required fields. Proto2 syntax only."]
    CardinalityRequired,
    #[serde(rename = "CARDINALITY_REPEATED")]
    #[doc = "For repeated fields."]
    CardinalityRepeated,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The field type."]
pub enum FieldKindEnum {
    #[serde(rename = "TYPE_UNKNOWN")]
    #[doc = "Field type unknown."]
    TypeUnknown,
    #[serde(rename = "TYPE_DOUBLE")]
    #[doc = "Field type double."]
    TypeDouble,
    #[serde(rename = "TYPE_FLOAT")]
    #[doc = "Field type float."]
    TypeFloat,
    #[serde(rename = "TYPE_INT64")]
    #[doc = "Field type int64."]
    TypeInt64,
    #[serde(rename = "TYPE_UINT64")]
    #[doc = "Field type uint64."]
    TypeUint64,
    #[serde(rename = "TYPE_INT32")]
    #[doc = "Field type int32."]
    TypeInt32,
    #[serde(rename = "TYPE_FIXED64")]
    #[doc = "Field type fixed64."]
    TypeFixed64,
    #[serde(rename = "TYPE_FIXED32")]
    #[doc = "Field type fixed32."]
    TypeFixed32,
    #[serde(rename = "TYPE_BOOL")]
    #[doc = "Field type bool."]
    TypeBool,
    #[serde(rename = "TYPE_STRING")]
    #[doc = "Field type string."]
    TypeString,
    #[serde(rename = "TYPE_GROUP")]
    #[doc = "Field type group. Proto2 syntax only, and deprecated."]
    TypeGroup,
    #[serde(rename = "TYPE_MESSAGE")]
    #[doc = "Field type message."]
    TypeMessage,
    #[serde(rename = "TYPE_BYTES")]
    #[doc = "Field type bytes."]
    TypeBytes,
    #[serde(rename = "TYPE_UINT32")]
    #[doc = "Field type uint32."]
    TypeUint32,
    #[serde(rename = "TYPE_ENUM")]
    #[doc = "Field type enum."]
    TypeEnum,
    #[serde(rename = "TYPE_SFIXED32")]
    #[doc = "Field type sfixed32."]
    TypeSfixed32,
    #[serde(rename = "TYPE_SFIXED64")]
    #[doc = "Field type sfixed64."]
    TypeSfixed64,
    #[serde(rename = "TYPE_SINT32")]
    #[doc = "Field type sint32."]
    TypeSint32,
    #[serde(rename = "TYPE_SINT64")]
    #[doc = "Field type sint64."]
    TypeSint64,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines the HTTP configuration for an API service. It contains a list of HttpRule, each specifying the mapping of an RPC method to one or more HTTP REST API methods."]
pub struct Http {
    #[serde(rename = "fullyDecodeReservedExpansion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When set to true, URL path parameters will be fully URI-decoded except in cases of single segment matches in reserved expansion, where \"%2F\" will be left encoded. The default behavior is to not decode RFC 6570 reserved characters in multi segment matches."]
    pub fully_decode_reserved_expansion: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of HTTP configuration rules that apply to individual API methods. **NOTE:** All service configuration rules follow \"last one wins\" order."]
    pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HttpRule>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "# gRPC Transcoding gRPC Transcoding is a feature for mapping between a gRPC method and one or more HTTP REST endpoints. It allows developers to build a single API service that supports both gRPC APIs and REST APIs. Many systems, including [Google APIs](https://github.com/googleapis/googleapis), [Cloud Endpoints](https://cloud.google.com/endpoints), [gRPC Gateway](https://github.com/grpc-ecosystem/grpc-gateway), and [Envoy](https://github.com/envoyproxy/envoy) proxy support this feature and use it for large scale production services. `HttpRule` defines the schema of the gRPC/REST mapping. The mapping specifies how different portions of the gRPC request message are mapped to the URL path, URL query parameters, and HTTP request body. It also controls how the gRPC response message is mapped to the HTTP response body. `HttpRule` is typically specified as an `google.api.http` annotation on the gRPC method. Each mapping specifies a URL path template and an HTTP method. The path template may refer to one or more fields in the gRPC request message, as long as each field is a non-repeated field with a primitive (non-message) type. The path template controls how fields of the request message are mapped to the URL path. Example: service Messaging { rpc GetMessage(GetMessageRequest) returns (Message) { option (google.api.http) = { get: \"/v1/{name=messages/*}\" }; } } message GetMessageRequest { string name = 1; // Mapped to URL path. } message Message { string text = 1; // The resource content. } This enables an HTTP REST to gRPC mapping as below: HTTP | gRPC -----|----- `GET /v1/messages/123456` | `GetMessage(name: \"messages/123456\")` Any fields in the request message which are not bound by the path template automatically become HTTP query parameters if there is no HTTP request body. For example: service Messaging { rpc GetMessage(GetMessageRequest) returns (Message) { option (google.api.http) = { get:\"/v1/messages/{message_id}\" }; } } message GetMessageRequest { message SubMessage { string subfield = 1; } string message_id = 1; // Mapped to URL path. int64 revision = 2; // Mapped to URL query parameter `revision`. SubMessage sub = 3; // Mapped to URL query parameter `sub.subfield`. } This enables a HTTP JSON to RPC mapping as below: HTTP | gRPC -----|----- `GET /v1/messages/123456?revision=2&sub.subfield=foo` | `GetMessage(message_id: \"123456\" revision: 2 sub: SubMessage(subfield: \"foo\"))` Note that fields which are mapped to URL query parameters must have a primitive type or a repeated primitive type or a non-repeated message type. In the case of a repeated type, the parameter can be repeated in the URL as `...?param=A&param=B`. In the case of a message type, each field of the message is mapped to a separate parameter, such as `...?foo.a=A&foo.b=B&foo.c=C`. For HTTP methods that allow a request body, the `body` field specifies the mapping. Consider a REST update method on the message resource collection: service Messaging { rpc UpdateMessage(UpdateMessageRequest) returns (Message) { option (google.api.http) = { patch: \"/v1/messages/{message_id}\" body: \"message\" }; } } message UpdateMessageRequest { string message_id = 1; // mapped to the URL Message message = 2; // mapped to the body } The following HTTP JSON to RPC mapping is enabled, where the representation of the JSON in the request body is determined by protos JSON encoding: HTTP | gRPC -----|----- `PATCH /v1/messages/123456 { \"text\": \"Hi!\" }` | `UpdateMessage(message_id: \"123456\" message { text: \"Hi!\" })` The special name `*` can be used in the body mapping to define that every field not bound by the path template should be mapped to the request body. This enables the following alternative definition of the update method: service Messaging { rpc UpdateMessage(Message) returns (Message) { option (google.api.http) = { patch: \"/v1/messages/{message_id}\" body: \"*\" }; } } message Message { string message_id = 1; string text = 2; } The following HTTP JSON to RPC mapping is enabled: HTTP | gRPC -----|----- `PATCH /v1/messages/123456 { \"text\": \"Hi!\" }` | `UpdateMessage(message_id: \"123456\" text: \"Hi!\")` Note that when using `*` in the body mapping, it is not possible to have HTTP parameters, as all fields not bound by the path end in the body. This makes this option more rarely used in practice when defining REST APIs. The common usage of `*` is in custom methods which don't use the URL at all for transferring data. It is possible to define multiple HTTP methods for one RPC by using the `additional_bindings` option. Example: service Messaging { rpc GetMessage(GetMessageRequest) returns (Message) { option (google.api.http) = { get: \"/v1/messages/{message_id}\" additional_bindings { get: \"/v1/users/{user_id}/messages/{message_id}\" } }; } } message GetMessageRequest { string message_id = 1; string user_id = 2; } This enables the following two alternative HTTP JSON to RPC mappings: HTTP | gRPC -----|----- `GET /v1/messages/123456` | `GetMessage(message_id: \"123456\")` `GET /v1/users/me/messages/123456` | `GetMessage(user_id: \"me\" message_id: \"123456\")` ## Rules for HTTP mapping 1. Leaf request fields (recursive expansion nested messages in the request message) are classified into three categories: - Fields referred by the path template. They are passed via the URL path. - Fields referred by the HttpRule.body. They are passed via the HTTP request body. - All other fields are passed via the URL query parameters, and the parameter name is the field path in the request message. A repeated field can be represented as multiple query parameters under the same name. 2. If HttpRule.body is \"*\", there is no URL query parameter, all fields are passed via URL path and HTTP request body. 3. If HttpRule.body is omitted, there is no HTTP request body, all fields are passed via URL path and URL query parameters. ### Path template syntax Template = \"/\" Segments [ Verb ] ; Segments = Segment { \"/\" Segment } ; Segment = \"*\" | \"**\" | LITERAL | Variable ; Variable = \"{\" FieldPath [ \"=\" Segments ] \"}\" ; FieldPath = IDENT { \".\" IDENT } ; Verb = \":\" LITERAL ; The syntax `*` matches a single URL path segment. The syntax `**` matches zero or more URL path segments, which must be the last part of the URL path except the `Verb`. The syntax `Variable` matches part of the URL path as specified by its template. A variable template must not contain other variables. If a variable matches a single path segment, its template may be omitted, e.g. `{var}` is equivalent to `{var=*}`. The syntax `LITERAL` matches literal text in the URL path. If the `LITERAL` contains any reserved character, such characters should be percent-encoded before the matching. If a variable contains exactly one path segment, such as `\"{var}\"` or `\"{var=*}\"`, when such a variable is expanded into a URL path on the client side, all characters except `[-_.~0-9a-zA-Z]` are percent-encoded. The server side does the reverse decoding. Such variables show up in the [Discovery Document](https://developers.google.com/discovery/v1/reference/apis) as `{var}`. If a variable contains multiple path segments, such as `\"{var=foo/*}\"` or `\"{var=**}\"`, when such a variable is expanded into a URL path on the client side, all characters except `[-_.~/0-9a-zA-Z]` are percent-encoded. The server side does the reverse decoding, except \"%2F\" and \"%2f\" are left unchanged. Such variables show up in the [Discovery Document](https://developers.google.com/discovery/v1/reference/apis) as `{+var}`. ## Using gRPC API Service Configuration gRPC API Service Configuration (service config) is a configuration language for configuring a gRPC service to become a user-facing product. The service config is simply the YAML representation of the `google.api.Service` proto message. As an alternative to annotating your proto file, you can configure gRPC transcoding in your service config YAML files. You do this by specifying a `HttpRule` that maps the gRPC method to a REST endpoint, achieving the same effect as the proto annotation. This can be particularly useful if you have a proto that is reused in multiple services. Note that any transcoding specified in the service config will override any matching transcoding configuration in the proto. Example: http: rules: # Selects a gRPC method and applies HttpRule to it. - selector: example.v1.Messaging.GetMessage get: /v1/messages/{message_id}/{sub.subfield} ## Special notes When gRPC Transcoding is used to map a gRPC to JSON REST endpoints, the proto to JSON conversion must follow the [proto3 specification](https://developers.google.com/protocol-buffers/docs/proto3#json). While the single segment variable follows the semantics of [RFC 6570](https://tools.ietf.org/html/rfc6570) Section 3.2.2 Simple String Expansion, the multi segment variable **does not** follow RFC 6570 Section 3.2.3 Reserved Expansion. The reason is that the Reserved Expansion does not expand special characters like `?` and `#`, which would lead to invalid URLs. As the result, gRPC Transcoding uses a custom encoding for multi segment variables. The path variables **must not** refer to any repeated or mapped field, because client libraries are not capable of handling such variable expansion. The path variables **must not** capture the leading \"/\" character. The reason is that the most common use case \"{var}\" does not capture the leading \"/\" character. For consistency, all path variables must share the same behavior. Repeated message fields must not be mapped to URL query parameters, because no client library can support such complicated mapping. If an API needs to use a JSON array for request or response body, it can map the request or response body to a repeated field. However, some gRPC Transcoding implementations may not support this feature."]
pub struct HttpRule {
    #[serde(rename = "additionalBindings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional HTTP bindings for the selector. Nested bindings must not contain an `additional_bindings` field themselves (that is, the nesting may only be one level deep)."]
    pub additional_bindings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HttpRule>>>,
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the request field whose value is mapped to the HTTP request body, or `*` for mapping all request fields not captured by the path pattern to the HTTP body, or omitted for not having any HTTP request body. NOTE: the referred field must be present at the top-level of the request message type."]
    pub body: ::std::option::Option<::std::string::String>,
    #[serde(rename = "custom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The custom pattern is used for specifying an HTTP method that is not included in the `pattern` field, such as HEAD, or \"*\" to leave the HTTP method unspecified for this rule. The wild-card rule is useful for services that provide content to Web (HTML) clients."]
    pub custom: ::std::option::Option<::std::boxed::Box<CustomHttpPattern>>,
    #[serde(rename = "delete")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maps to HTTP DELETE. Used for deleting a resource."]
    pub delete: ::std::option::Option<::std::string::String>,
    #[serde(rename = "get")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maps to HTTP GET. Used for listing and getting information about resources."]
    pub get: ::std::option::Option<::std::string::String>,
    #[serde(rename = "patch")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maps to HTTP PATCH. Used for updating a resource."]
    pub patch: ::std::option::Option<::std::string::String>,
    #[serde(rename = "post")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maps to HTTP POST. Used for creating a resource or performing an action."]
    pub post: ::std::option::Option<::std::string::String>,
    #[serde(rename = "put")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maps to HTTP PUT. Used for replacing a resource."]
    pub put: ::std::option::Option<::std::string::String>,
    #[serde(rename = "responseBody")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The name of the response field whose value is mapped to the HTTP response body. When omitted, the entire response message will be used as the HTTP response body. NOTE: The referred field must be present at the top-level of the response message type."]
    pub response_body: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Selects a method to which this rule applies. Refer to selector for syntax details."]
    pub selector: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies a location to extract JWT from an API request."]
pub struct JwtLocation {
    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies HTTP header name to extract JWT token."]
    pub header: ::std::option::Option<::std::string::String>,
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies URL query parameter name to extract JWT token."]
    pub query: ::std::option::Option<::std::string::String>,
    #[serde(rename = "valuePrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value prefix. The value format is \"value_prefix{token}\" Only applies to \"in\" header type. Must be empty for \"in\" query type. If not empty, the header value has to match (case sensitive) this prefix. If not matched, JWT will not be extracted. If matched, JWT will be extracted after the prefix is removed. For example, for \"Authorization: Bearer {JWT}\", value_prefix=\"Bearer \" with a space at the end."]
    pub value_prefix: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A description of a label."]
pub struct LabelDescriptor {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A human-readable description for the label."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The label key."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "valueType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of data that can be assigned to the label."]
    pub value_type: ::std::option::Option<LabelDescriptorValueTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of data that can be assigned to the label."]
pub enum LabelDescriptorValueTypeEnum {
    #[serde(rename = "STRING")]
    #[doc = "A variable-length string. This is the default."]
    String,
    #[serde(rename = "BOOL")]
    #[doc = "Boolean; true or false."]
    Bool,
    #[serde(rename = "INT64")]
    #[doc = "A 64-bit signed integer."]
    Int64,
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
#[doc = "Response for the list request."]
pub struct ListTenancyUnitsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token for large results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tenancyUnits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tenancy units matching the request."]
    pub tenancy_units: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TenancyUnit>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A description of a log type. Example in YAML format: - name: library.googleapis.com/activity_history description: The history of borrowing and returning library items. display_name: Activity labels: - key: /customer_id description: Identifier of a library customer"]
pub struct LogDescriptor {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A human-readable description of this log. This information appears in the documentation and can contain details."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human-readable name for this log. This information appears on the user interface and should be concise."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of labels that are available to describe a specific log entry. Runtime requests that contain labels not specified here are considered invalid."]
    pub labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LabelDescriptor>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the log. It must be less than 512 characters long and can include the following characters: upper- and lower-case alphanumeric characters [A-Za-z0-9], and punctuation characters including slash, underscore, hyphen, period [/_-.]."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Logging configuration of the service. The following example shows how to configure logs to be sent to the producer and consumer projects. In the example, the `activity_history` log is sent to both the producer and consumer projects, whereas the `purchase_history` log is only sent to the producer project. monitored_resources: - type: library.googleapis.com/branch labels: - key: /city description: The city where the library branch is located in. - key: /name description: The name of the branch. logs: - name: activity_history labels: - key: /customer_id - name: purchase_history logging: producer_destinations: - monitored_resource: library.googleapis.com/branch logs: - activity_history - purchase_history consumer_destinations: - monitored_resource: library.googleapis.com/branch logs: - activity_history"]
pub struct Logging {
    #[serde(rename = "consumerDestinations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Logging configurations for sending logs to the consumer project. There can be multiple consumer destinations, each one must have a different monitored resource type. A log can be used in at most one consumer destination."]
    pub consumer_destinations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LoggingDestination>>>,
    #[serde(rename = "producerDestinations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Logging configurations for sending logs to the producer project. There can be multiple producer destinations, each one must have a different monitored resource type. A log can be used in at most one producer destination."]
    pub producer_destinations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LoggingDestination>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration of a specific logging destination (the producer project or the consumer project)."]
pub struct LoggingDestination {
    #[serde(rename = "logs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Names of the logs to be sent to this destination. Each name must be defined in the Service.logs section. If the log name is not a domain scoped name, it will be automatically prefixed with the service name followed by \"/\"."]
    pub logs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "monitoredResource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The monitored resource type. The type must be defined in the Service.monitored_resources section."]
    pub monitored_resource: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Method represents a method of an API interface."]
pub struct Method {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The simple name of this method."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Any metadata attached to the method."]
    pub options: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Option>>>,
    #[serde(rename = "requestStreaming")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, the request is streamed."]
    pub request_streaming: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "requestTypeUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL of the input message type."]
    pub request_type_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "responseStreaming")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, the response is streamed."]
    pub response_streaming: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "responseTypeUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the output message type."]
    pub response_type_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "syntax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source syntax of this method."]
    pub syntax: ::std::option::Option<MethodSyntaxEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The source syntax of this method."]
pub enum MethodSyntaxEnum {
    #[serde(rename = "SYNTAX_PROTO2")]
    #[doc = "Syntax `proto2`."]
    SyntaxProto2,
    #[serde(rename = "SYNTAX_PROTO3")]
    #[doc = "Syntax `proto3`."]
    SyntaxProto3,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines a metric type and its schema. Once a metric descriptor is created, deleting or altering it stops data collection and makes the metric type's existing data unusable. "]
pub struct MetricDescriptor {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A detailed description of the metric, which can be used in documentation."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A concise name for the metric, which can be displayed in user interfaces. Use sentence case without an ending period, for example \"Request count\". This field is optional but it is recommended to be set for any metrics associated with user-visible concepts, such as Quota."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of labels that can be used to describe a specific instance of this metric type. For example, the `appengine.googleapis.com/http/server/response_latencies` metric type has a label for the HTTP response code, `response_code`, so you can look at latencies for successful responses or just for responses that failed."]
    pub labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LabelDescriptor>>>,
    #[serde(rename = "launchStage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The launch stage of the metric definition."]
    pub launch_stage: ::std::option::Option<MetricDescriptorLaunchStageEnum>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Metadata which can be used to guide usage of the metric."]
    pub metadata: ::std::option::Option<::std::boxed::Box<MetricDescriptorMetadata>>,
    #[serde(rename = "metricKind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the metric records instantaneous values, changes to a value, etc. Some combinations of `metric_kind` and `value_type` might not be supported."]
    pub metric_kind: ::std::option::Option<MetricDescriptorMetricKindEnum>,
    #[serde(rename = "monitoredResourceTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Read-only. If present, then a time series, which is identified partially by a metric type and a MonitoredResourceDescriptor, that is associated with this metric type can only be associated with one of the monitored resource types listed here."]
    pub monitored_resource_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the metric descriptor."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The metric type, including its DNS name prefix. The type is not URL-encoded. All user-defined metric types have the DNS name `custom.googleapis.com` or `external.googleapis.com`. Metric types should use a natural hierarchical grouping. For example: \"custom.googleapis.com/invoice/paid/amount\" \"external.googleapis.com/prometheus/up\" \"appengine.googleapis.com/http/server/response_latencies\""]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The units in which the metric value is reported. It is only applicable if the `value_type` is `INT64`, `DOUBLE`, or `DISTRIBUTION`. The `unit` defines the representation of the stored metric values. Different systems might scale the values to be more easily displayed (so a value of `0.02kBy` _might_ be displayed as `20By`, and a value of `3523kBy` _might_ be displayed as `3.5MBy`). However, if the `unit` is `kBy`, then the value of the metric is always in thousands of bytes, no matter how it might be displayed. If you want a custom metric to record the exact number of CPU-seconds used by a job, you can create an `INT64 CUMULATIVE` metric whose `unit` is `s{CPU}` (or equivalently `1s{CPU}` or just `s`). If the job uses 12,005 CPU-seconds, then the value is written as `12005`. Alternatively, if you want a custom metric to record data in a more granular way, you can create a `DOUBLE CUMULATIVE` metric whose `unit` is `ks{CPU}`, and then write the value `12.005` (which is `12005/1000`), or use `Kis{CPU}` and write `11.723` (which is `12005/1024`). The supported units are a subset of [The Unified Code for Units of Measure](https://unitsofmeasure.org/ucum.html) standard: **Basic units (UNIT)** * `bit` bit * `By` byte * `s` second * `min` minute * `h` hour * `d` day * `1` dimensionless **Prefixes (PREFIX)** * `k` kilo (10^3) * `M` mega (10^6) * `G` giga (10^9) * `T` tera (10^12) * `P` peta (10^15) * `E` exa (10^18) * `Z` zetta (10^21) * `Y` yotta (10^24) * `m` milli (10^-3) * `u` micro (10^-6) * `n` nano (10^-9) * `p` pico (10^-12) * `f` femto (10^-15) * `a` atto (10^-18) * `z` zepto (10^-21) * `y` yocto (10^-24) * `Ki` kibi (2^10) * `Mi` mebi (2^20) * `Gi` gibi (2^30) * `Ti` tebi (2^40) * `Pi` pebi (2^50) **Grammar** The grammar also includes these connectors: * `/` division or ratio (as an infix operator). For examples, `kBy/{email}` or `MiBy/10ms` (although you should almost never have `/s` in a metric `unit`; rates should always be computed at query time from the underlying cumulative or delta value). * `.` multiplication or composition (as an infix operator). For examples, `GBy.d` or `k{watt}.h`. The grammar for a unit is as follows: Expression = Component { \".\" Component } { \"/\" Component } ; Component = ( [ PREFIX ] UNIT | \"%\" ) [ Annotation ] | Annotation | \"1\" ; Annotation = \"{\" NAME \"}\" ; Notes: * `Annotation` is just a comment if it follows a `UNIT`. If the annotation is used alone, then the unit is equivalent to `1`. For examples, `{request}/s == 1/s`, `By{transmitted}/s == By/s`. * `NAME` is a sequence of non-blank printable ASCII characters not containing `{` or `}`. * `1` represents a unitary [dimensionless unit](https://en.wikipedia.org/wiki/Dimensionless_quantity) of 1, such as in `1/s`. It is typically used when none of the basic units are appropriate. For example, \"new users per day\" can be represented as `1/d` or `{new-users}/d` (and a metric value `5` would mean \"5 new users). Alternatively, \"thousands of page views per day\" would be represented as `1000/d` or `k1/d` or `k{page_views}/d` (and a metric value of `5.3` would mean \"5300 page views per day\"). * `%` represents dimensionless value of 1/100, and annotates values giving a percentage (so the metric values are typically in the range of 0..100, and a metric value `3` means \"3 percent\"). * `10^2.%` indicates a metric contains a ratio, typically in the range 0..1, that will be multiplied by 100 and displayed as a percentage (so a metric value `0.03` means \"3 percent\")."]
    pub unit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "valueType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the measurement is an integer, a floating-point number, etc. Some combinations of `metric_kind` and `value_type` might not be supported."]
    pub value_type: ::std::option::Option<MetricDescriptorValueTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. The launch stage of the metric definition."]
pub enum MetricDescriptorLaunchStageEnum {
    #[serde(rename = "LAUNCH_STAGE_UNSPECIFIED")]
    #[doc = "Do not use this default value."]
    LaunchStageUnspecified,
    #[serde(rename = "UNIMPLEMENTED")]
    #[doc = "The feature is not yet implemented. Users can not use it."]
    Unimplemented,
    #[serde(rename = "PRELAUNCH")]
    #[doc = "Prelaunch features are hidden from users and are only visible internally."]
    Prelaunch,
    #[serde(rename = "EARLY_ACCESS")]
    #[doc = "Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released."]
    EarlyAccess,
    #[serde(rename = "ALPHA")]
    #[doc = "Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don’t have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases."]
    Alpha,
    #[serde(rename = "BETA")]
    #[doc = "Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases."]
    Beta,
    #[serde(rename = "GA")]
    #[doc = "GA features are open to all developers and are considered stable and fully qualified for production use."]
    Ga,
    #[serde(rename = "DEPRECATED")]
    #[doc = "Deprecated features are scheduled to be shut down and removed. For more information, see the “Deprecation Policy” section of our [Terms of Service](https://cloud.google.com/terms/) and the [Google Cloud Platform Subject to the Deprecation Policy](https://cloud.google.com/terms/deprecation) documentation."]
    Deprecated,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Whether the metric records instantaneous values, changes to a value, etc. Some combinations of `metric_kind` and `value_type` might not be supported."]
pub enum MetricDescriptorMetricKindEnum {
    #[serde(rename = "METRIC_KIND_UNSPECIFIED")]
    #[doc = "Do not use this default value."]
    MetricKindUnspecified,
    #[serde(rename = "GAUGE")]
    #[doc = "An instantaneous measurement of a value."]
    Gauge,
    #[serde(rename = "DELTA")]
    #[doc = "The change in a value during a time interval."]
    Delta,
    #[serde(rename = "CUMULATIVE")]
    #[doc = "A value accumulated over a time interval. Cumulative measurements in a time series should have the same start time and increasing end times, until an event resets the cumulative value to zero and sets a new start time for the following points."]
    Cumulative,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Whether the measurement is an integer, a floating-point number, etc. Some combinations of `metric_kind` and `value_type` might not be supported."]
pub enum MetricDescriptorValueTypeEnum {
    #[serde(rename = "VALUE_TYPE_UNSPECIFIED")]
    #[doc = "Do not use this default value."]
    ValueTypeUnspecified,
    #[serde(rename = "BOOL")]
    #[doc = "The value is a boolean. This value type can be used only if the metric kind is `GAUGE`."]
    Bool,
    #[serde(rename = "INT64")]
    #[doc = "The value is a signed 64-bit integer."]
    Int64,
    #[serde(rename = "DOUBLE")]
    #[doc = "The value is a double precision floating point number."]
    Double,
    #[serde(rename = "STRING")]
    #[doc = "The value is a text string. This value type can be used only if the metric kind is `GAUGE`."]
    String,
    #[serde(rename = "DISTRIBUTION")]
    #[doc = "The value is a `Distribution`."]
    Distribution,
    #[serde(rename = "MONEY")]
    #[doc = "The value is money."]
    Money,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional annotations that can be used to guide the usage of a metric."]
pub struct MetricDescriptorMetadata {
    #[serde(rename = "ingestDelay")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The delay of data points caused by ingestion. Data points older than this age are guaranteed to be ingested and available to be read, excluding data loss due to errors."]
    pub ingest_delay: ::std::option::Option<::std::string::String>,
    #[serde(rename = "launchStage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Must use the MetricDescriptor.launch_stage instead."]
    pub launch_stage: ::std::option::Option<MetricDescriptorMetadataLaunchStageEnum>,
    #[serde(rename = "samplePeriod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sampling period of metric data points. For metrics which are written periodically, consecutive data points are stored at this time interval, excluding data loss due to errors. Metrics with a higher granularity have a smaller sampling period."]
    pub sample_period: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Deprecated. Must use the MetricDescriptor.launch_stage instead."]
pub enum MetricDescriptorMetadataLaunchStageEnum {
    #[serde(rename = "LAUNCH_STAGE_UNSPECIFIED")]
    #[doc = "Do not use this default value."]
    LaunchStageUnspecified,
    #[serde(rename = "UNIMPLEMENTED")]
    #[doc = "The feature is not yet implemented. Users can not use it."]
    Unimplemented,
    #[serde(rename = "PRELAUNCH")]
    #[doc = "Prelaunch features are hidden from users and are only visible internally."]
    Prelaunch,
    #[serde(rename = "EARLY_ACCESS")]
    #[doc = "Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released."]
    EarlyAccess,
    #[serde(rename = "ALPHA")]
    #[doc = "Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don’t have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases."]
    Alpha,
    #[serde(rename = "BETA")]
    #[doc = "Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases."]
    Beta,
    #[serde(rename = "GA")]
    #[doc = "GA features are open to all developers and are considered stable and fully qualified for production use."]
    Ga,
    #[serde(rename = "DEPRECATED")]
    #[doc = "Deprecated features are scheduled to be shut down and removed. For more information, see the “Deprecation Policy” section of our [Terms of Service](https://cloud.google.com/terms/) and the [Google Cloud Platform Subject to the Deprecation Policy](https://cloud.google.com/terms/deprecation) documentation."]
    Deprecated,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Bind API methods to metrics. Binding a method to a metric causes that metric's configured quota behaviors to apply to the method call."]
pub struct MetricRule {
    #[serde(rename = "metricCosts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metrics to update when the selected methods are called, and the associated cost applied to each metric. The key of the map is the metric name, and the values are the amount increased for the metric against which the quota limits are defined. The value must not be negative."]
    pub metric_costs:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "selector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Selects the methods to which this rule applies. Refer to selector for syntax details."]
    pub selector: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Declares an API Interface to be included in this interface. The including interface must redeclare all the methods from the included interface, but documentation and options are inherited as follows: - If after comment and whitespace stripping, the documentation string of the redeclared method is empty, it will be inherited from the original method. - Each annotation belonging to the service config (http, visibility) which is not set in the redeclared method will be inherited. - If an http annotation is inherited, the path pattern will be modified as follows. Any version prefix will be replaced by the version of the including interface plus the root path if specified. Example of a simple mixin: package google.acl.v1; service AccessControl { // Get the underlying ACL object. rpc GetAcl(GetAclRequest) returns (Acl) { option (google.api.http).get = \"/v1/{resource=**}:getAcl\"; } } package google.storage.v2; service Storage { // rpc GetAcl(GetAclRequest) returns (Acl); // Get a data record. rpc GetData(GetDataRequest) returns (Data) { option (google.api.http).get = \"/v2/{resource=**}\"; } } Example of a mixin configuration: apis: - name: google.storage.v2.Storage mixins: - name: google.acl.v1.AccessControl The mixin construct implies that all methods in `AccessControl` are also declared with same name and request/response types in `Storage`. A documentation generator or annotation processor will see the effective `Storage.GetAcl` method after inheriting documentation and annotations as follows: service Storage { // Get the underlying ACL object. rpc GetAcl(GetAclRequest) returns (Acl) { option (google.api.http).get = \"/v2/{resource=**}:getAcl\"; } ... } Note how the version in the path pattern changed from `v1` to `v2`. If the `root` field in the mixin is specified, it should be a relative path under which inherited HTTP paths are placed. Example: apis: - name: google.storage.v2.Storage mixins: - name: google.acl.v1.AccessControl root: acls This implies the following inherited HTTP annotation: service Storage { // Get the underlying ACL object. rpc GetAcl(GetAclRequest) returns (Acl) { option (google.api.http).get = \"/v2/acls/{resource=**}:getAcl\"; } ... }"]
pub struct Mixin {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fully qualified name of the interface which is included."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "root")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If non-empty specifies a path under which inherited HTTP paths are rooted."]
    pub root: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An object that describes the schema of a MonitoredResource object using a type name and a set of labels. For example, the monitored resource descriptor for Google Compute Engine VM instances has a type of `\"gce_instance\"` and specifies the use of the labels `\"instance_id\"` and `\"zone\"` to identify particular VM instances. Different APIs can support different monitored resource types. APIs generally provide a `list` method that returns the monitored resource descriptors used by the API. "]
pub struct MonitoredResourceDescriptor {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A detailed description of the monitored resource type that might be used in documentation."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A concise name for the monitored resource type that might be displayed in user interfaces. It should be a Title Cased Noun Phrase, without any article or other determiners. For example, `\"Google Cloud SQL Database\"`."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A set of labels used to describe instances of this monitored resource type. For example, an individual Google Cloud SQL database is identified by values for the labels `\"database_id\"` and `\"zone\"`."]
    pub labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LabelDescriptor>>>,
    #[serde(rename = "launchStage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The launch stage of the monitored resource definition."]
    pub launch_stage: ::std::option::Option<MonitoredResourceDescriptorLaunchStageEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The resource name of the monitored resource descriptor: `\"projects/{project_id}/monitoredResourceDescriptors/{type}\"` where {type} is the value of the `type` field in this object and {project_id} is a project ID that provides API-specific context for accessing the type. APIs that do not use project information can use the resource name format `\"monitoredResourceDescriptors/{type}\"`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The monitored resource type. For example, the type `\"cloudsql_database\"` represents databases in Google Cloud SQL."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. The launch stage of the monitored resource definition."]
pub enum MonitoredResourceDescriptorLaunchStageEnum {
    #[serde(rename = "LAUNCH_STAGE_UNSPECIFIED")]
    #[doc = "Do not use this default value."]
    LaunchStageUnspecified,
    #[serde(rename = "UNIMPLEMENTED")]
    #[doc = "The feature is not yet implemented. Users can not use it."]
    Unimplemented,
    #[serde(rename = "PRELAUNCH")]
    #[doc = "Prelaunch features are hidden from users and are only visible internally."]
    Prelaunch,
    #[serde(rename = "EARLY_ACCESS")]
    #[doc = "Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released."]
    EarlyAccess,
    #[serde(rename = "ALPHA")]
    #[doc = "Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don’t have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases."]
    Alpha,
    #[serde(rename = "BETA")]
    #[doc = "Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases."]
    Beta,
    #[serde(rename = "GA")]
    #[doc = "GA features are open to all developers and are considered stable and fully qualified for production use."]
    Ga,
    #[serde(rename = "DEPRECATED")]
    #[doc = "Deprecated features are scheduled to be shut down and removed. For more information, see the “Deprecation Policy” section of our [Terms of Service](https://cloud.google.com/terms/) and the [Google Cloud Platform Subject to the Deprecation Policy](https://cloud.google.com/terms/deprecation) documentation."]
    Deprecated,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Monitoring configuration of the service. The example below shows how to configure monitored resources and metrics for monitoring. In the example, a monitored resource and two metrics are defined. The `library.googleapis.com/book/returned_count` metric is sent to both producer and consumer projects, whereas the `library.googleapis.com/book/num_overdue` metric is only sent to the consumer project. monitored_resources: - type: library.googleapis.com/Branch display_name: \"Library Branch\" description: \"A branch of a library.\" launch_stage: GA labels: - key: resource_container description: \"The Cloud container (ie. project id) for the Branch.\" - key: location description: \"The location of the library branch.\" - key: branch_id description: \"The id of the branch.\" metrics: - name: library.googleapis.com/book/returned_count display_name: \"Books Returned\" description: \"The count of books that have been returned.\" launch_stage: GA metric_kind: DELTA value_type: INT64 unit: \"1\" labels: - key: customer_id description: \"The id of the customer.\" - name: library.googleapis.com/book/num_overdue display_name: \"Books Overdue\" description: \"The current number of overdue books.\" launch_stage: GA metric_kind: GAUGE value_type: INT64 unit: \"1\" labels: - key: customer_id description: \"The id of the customer.\" monitoring: producer_destinations: - monitored_resource: library.googleapis.com/Branch metrics: - library.googleapis.com/book/returned_count consumer_destinations: - monitored_resource: library.googleapis.com/Branch metrics: - library.googleapis.com/book/returned_count - library.googleapis.com/book/num_overdue"]
pub struct Monitoring {
    #[serde(rename = "consumerDestinations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Monitoring configurations for sending metrics to the consumer project. There can be multiple consumer destinations. A monitored resource type may appear in multiple monitoring destinations if different aggregations are needed for different sets of metrics associated with that monitored resource type. A monitored resource and metric pair may only be used once in the Monitoring configuration."]
    pub consumer_destinations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MonitoringDestination>>>,
    #[serde(rename = "producerDestinations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Monitoring configurations for sending metrics to the producer project. There can be multiple producer destinations. A monitored resource type may appear in multiple monitoring destinations if different aggregations are needed for different sets of metrics associated with that monitored resource type. A monitored resource and metric pair may only be used once in the Monitoring configuration."]
    pub producer_destinations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MonitoringDestination>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration of a specific monitoring destination (the producer project or the consumer project)."]
pub struct MonitoringDestination {
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Types of the metrics to report to this monitoring destination. Each type must be defined in Service.metrics section."]
    pub metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "monitoredResource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The monitored resource type. The type must be defined in Service.monitored_resources section."]
    pub monitored_resource: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "OAuth scopes are a way to define data and permissions on data. For example, there are scopes defined for \"Read-only access to Google Calendar\" and \"Access to Cloud Platform\". Users can consent to a scope for an application, giving it permission to access that data on their behalf. OAuth scope specifications should be fairly coarse grained; a user will need to see and understand the text description of what your scope means. In most cases: use one or at most two OAuth scopes for an entire family of products. If your product has multiple APIs, you should probably be sharing the OAuth scope across all of those APIs. When you need finer grained OAuth consent screens: talk with your product management about how developers will use them in practice. Please note that even though each of the canonical scopes is enough for a request to be accepted and passed to the backend, a request can still fail due to the backend requiring additional scopes or permissions."]
pub struct OAuthRequirements {
    #[serde(rename = "canonicalScopes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of publicly documented OAuth scopes that are allowed access. An OAuth token containing any of these scopes will be accepted. Example: canonical_scopes: https://www.googleapis.com/auth/calendar, https://www.googleapis.com/auth/calendar.read"]
    pub canonical_scopes: ::std::option::Option<::std::string::String>,
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
#[doc = "A protocol buffer option, which can be attached to a message, field, enumeration, etc."]
pub struct Option {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The option's name. For protobuf built-in options (options defined in descriptor.proto), this is the short name. For example, `\"map_entry\"`. For custom options, it should be the fully-qualified name. For example, `\"google.api.http\"`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The option's value packed in an Any message. If the value is a primitive, the corresponding wrapper type defined in google/protobuf/wrappers.proto should be used. If the value is an enum, it should be stored as an int32 value using the google.protobuf.Int32Value type."]
    pub value: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a documentation page. A page can contain subpages to represent nested documentation set structure."]
pub struct Page {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Markdown content of the page. You can use (== include {path} ==) to include content from a Markdown file."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the page. It will be used as an identity of the page to generate URI of the page, text of the link to this page in navigation, etc. The full page name (start from the root page name to this page concatenated with `.`) can be used as reference to the page in your documentation. For example: pages: - name: Tutorial content: (== include tutorial.md ==) subpages: - name: Java content: (== include tutorial_java.md ==) You can reference `Java` page using Markdown reference link syntax: `Java`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subpages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subpages of this page. The order of subpages specified here will be honored in the generated docset."]
    pub subpages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Page>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Translates to IAM Policy bindings (without auditing at this level)"]
pub struct PolicyBinding {
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uses the same format as in IAM policy. `member` must include both a prefix and ID. For example, `user:{emailId}`, `serviceAccount:{emailId}`, `group:{emailId}`."]
    pub members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Role. (https://cloud.google.com/iam/docs/understanding-roles) For example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
    pub role: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Quota configuration helps to achieve fairness and budgeting in service usage. The metric based quota configuration works this way: - The service configuration defines a set of metrics. - For API calls, the quota.metric_rules maps methods to metrics with corresponding costs. - The quota.limits defines limits on the metrics, which will be used for quota checks at runtime. An example quota configuration in yaml format: quota: limits: - name: apiWriteQpsPerProject metric: library.googleapis.com/write_calls unit: \"1/min/{project}\" # rate limit for consumer projects values: STANDARD: 10000 # The metric rules bind all methods to the read_calls metric, # except for the UpdateBook and DeleteBook methods. These two methods # are mapped to the write_calls metric, with the UpdateBook method # consuming at twice rate as the DeleteBook method. metric_rules: - selector: \"*\" metric_costs: library.googleapis.com/read_calls: 1 - selector: google.example.library.v1.LibraryService.UpdateBook metric_costs: library.googleapis.com/write_calls: 2 - selector: google.example.library.v1.LibraryService.DeleteBook metric_costs: library.googleapis.com/write_calls: 1 Corresponding Metric definition: metrics: - name: library.googleapis.com/read_calls display_name: Read requests metric_kind: DELTA value_type: INT64 - name: library.googleapis.com/write_calls display_name: Write requests metric_kind: DELTA value_type: INT64 "]
pub struct Quota {
    #[serde(rename = "limits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of `QuotaLimit` definitions for the service."]
    pub limits: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<QuotaLimit>>>,
    #[serde(rename = "metricRules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of `MetricRule` definitions, each one mapping a selected method to one or more metrics."]
    pub metric_rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricRule>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`QuotaLimit` defines a specific limit that applies over a specified duration for a limit type. There can be at most one limit for a duration and limit type combination defined within a `QuotaGroup`."]
pub struct QuotaLimit {
    #[serde(rename = "defaultLimit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Default number of tokens that can be consumed during the specified duration. This is the number of tokens assigned when a client application developer activates the service for his/her project. Specifying a value of 0 will block all requests. This can be used if you are provisioning quota to selected consumers and blocking others. Similarly, a value of -1 will indicate an unlimited quota. No other negative values are allowed. Used by group-based quotas only."]
    pub default_limit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. User-visible, extended description for this quota limit. Should be used only when more context is needed to understand this limit than provided by the limit's display name (see: `display_name`)."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-visible display name for this limit. Optional. If not set, the UI will provide a default display name based on the quota configuration. This field can be used to override the default display name generated from the configuration."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Duration of this limit in textual notation. Must be \"100s\" or \"1d\". Used by group-based quotas only."]
    pub duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "freeTier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Free tier value displayed in the Developers Console for this limit. The free tier is the number of tokens that will be subtracted from the billed amount when billing is enabled. This field can only be set on a limit with duration \"1d\", in a billable group; it is invalid on any other limit. If this field is not set, it defaults to 0, indicating that there is no free tier for this service. Used by group-based quotas only."]
    pub free_tier: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxLimit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum number of tokens that can be consumed during the specified duration. Client application developers can override the default limit up to this maximum. If specified, this value cannot be set to a value less than the default limit. If not specified, it is set to the default limit. To allow clients to apply overrides with no upper bound, set this to -1, indicating unlimited maximum quota. Used by group-based quotas only."]
    pub max_limit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metric")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the metric this quota limit applies to. The quota limits with the same metric will be checked together during runtime. The metric must be defined within the service config."]
    pub metric: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the quota limit. The name must be provided, and it must be unique within the service. The name can only include alphanumeric characters as well as '-'. The maximum length of the limit name is 64 characters."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specify the unit of the quota limit. It uses the same syntax as Metric.unit. The supported unit kinds are determined by the quota backend system. Here are some examples: * \"1/min/{project}\" for quota per minute per project. Note: the order of unit components is insignificant. The \"1\" at the beginning is required to follow the metric unit syntax."]
    pub unit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tiered limit values. You must specify this as a key:value pair, with an integer value that is the maximum number of requests allowed for the specified unit. Currently only STANDARD is supported."]
    pub values: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message to remove a tenant project resource from the tenancy unit."]
pub struct RemoveTenantProjectRequest {
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Tag of the resource within the tenancy unit."]
    pub tag: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the search query."]
pub struct SearchTenancyUnitsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pagination token for large results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tenancyUnits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tenancy Units matching the request."]
    pub tenancy_units: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TenancyUnit>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`Service` is the root object of Google service configuration schema. It describes basic information about a service, such as the name and the title, and delegates other aspects to sub-sections. Each sub-section is either a proto message or a repeated proto message that configures a specific aspect, such as auth. See each proto message definition for details. Example: type: google.api.Service name: calendar.googleapis.com title: Google Calendar API apis: - name: google.calendar.v3.Calendar authentication: providers: - id: google_calendar_auth jwks_uri: https://www.googleapis.com/oauth2/v1/certs issuer: https://securetoken.google.com rules: - selector: \"*\" requirements: provider_id: google_calendar_auth"]
pub struct Service {
    #[serde(rename = "apis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of API interfaces exported by this service. Only the `name` field of the google.protobuf.Api needs to be provided by the configuration author, as the remaining fields will be derived from the IDL during the normalization process. It is an error to specify an API interface here which cannot be resolved against the associated IDL files."]
    pub apis: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Api>>>,
    #[serde(rename = "authentication")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Auth configuration."]
    pub authentication: ::std::option::Option<::std::boxed::Box<Authentication>>,
    #[serde(rename = "backend")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API backend configuration."]
    pub backend: ::std::option::Option<::std::boxed::Box<Backend>>,
    #[serde(rename = "billing")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Billing configuration."]
    pub billing: ::std::option::Option<::std::boxed::Box<Billing>>,
    #[serde(rename = "configVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. The service config compiler always sets this field to `3`."]
    pub config_version: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Context configuration."]
    pub context: ::std::option::Option<::std::boxed::Box<Context>>,
    #[serde(rename = "control")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for the service control plane."]
    pub control: ::std::option::Option<::std::boxed::Box<Control>>,
    #[serde(rename = "customError")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom error configuration."]
    pub custom_error: ::std::option::Option<::std::boxed::Box<CustomError>>,
    #[serde(rename = "documentation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional API documentation."]
    pub documentation: ::std::option::Option<::std::boxed::Box<Documentation>>,
    #[serde(rename = "endpoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for network endpoints. If this is empty, then an endpoint with the same name as the service is automatically generated to service all defined APIs."]
    pub endpoints: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Endpoint>>>,
    #[serde(rename = "enums")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of all enum types included in this API service. Enums referenced directly or indirectly by the `apis` are automatically included. Enums which are not referenced but shall be included should be listed here by name. Example: enums: - name: google.someapi.v1.SomeEnum"]
    pub enums: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Enum>>>,
    #[serde(rename = "http")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTTP configuration."]
    pub http: ::std::option::Option<::std::boxed::Box<Http>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique ID for a specific instance of this message, typically assigned by the client for tracking purpose. Must be no longer than 63 characters and only lower case letters, digits, '.', '_' and '-' are allowed. If empty, the server may choose to generate one instead."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "logging")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Logging configuration."]
    pub logging: ::std::option::Option<::std::boxed::Box<Logging>>,
    #[serde(rename = "logs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines the logs used by this service."]
    pub logs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LogDescriptor>>>,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines the metrics used by this service."]
    pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricDescriptor>>>,
    #[serde(rename = "monitoredResources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines the monitored resources used by this service. This is required by the Service.monitoring and Service.logging configurations."]
    pub monitored_resources:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MonitoredResourceDescriptor>>>,
    #[serde(rename = "monitoring")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Monitoring configuration."]
    pub monitoring: ::std::option::Option<::std::boxed::Box<Monitoring>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The service name, which is a DNS-like logical identifier for the service, such as `calendar.googleapis.com`. The service name typically goes through DNS verification to make sure the owner of the service also owns the DNS name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "producerProjectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google project that owns this service."]
    pub producer_project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quota")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Quota configuration."]
    pub quota: ::std::option::Option<::std::boxed::Box<Quota>>,
    #[serde(rename = "sourceInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The source information for this configuration if available."]
    pub source_info: ::std::option::Option<::std::boxed::Box<SourceInfo>>,
    #[serde(rename = "systemParameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "System parameter configuration."]
    pub system_parameters: ::std::option::Option<::std::boxed::Box<SystemParameters>>,
    #[serde(rename = "systemTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of all proto message types included in this API service. It serves similar purpose as [google.api.Service.types], except that these types are not needed by user-defined APIs. Therefore, they will not show up in the generated discovery doc. This field should only be used to define system APIs in ESF."]
    pub system_types: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Type>>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The product title for this service."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "types")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of all proto message types included in this API service. Types referenced directly or indirectly by the `apis` are automatically included. Messages which are not referenced but shall be included, such as types used by the `google.protobuf.Any` type, should be listed here by name. Example: types: - name: google.protobuf.Int32"]
    pub types: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Type>>>,
    #[serde(rename = "usage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration controlling usage of this service."]
    pub usage: ::std::option::Option<::std::boxed::Box<Usage>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes the service account configuration for the tenant project."]
pub struct ServiceAccountConfig {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the IAM service account to be created in tenant project. The email format of the service account is \"@.iam.gserviceaccount.com\". This account ID must be unique within tenant project and service producers have to guarantee it. The ID must be 6-30 characters long, and match the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])`."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tenantProjectRoles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Roles for the associated service account for the tenant project."]
    pub tenant_project_roles: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`SourceContext` represents information about the source of a protobuf element, like the file in which it is defined."]
pub struct SourceContext {
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The path-qualified name of the .proto file that contained the associated protobuf element. For example: `\"google/protobuf/source_context.proto\"`."]
    pub file_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Source information used to create a Service Config"]
pub struct SourceInfo {
    #[serde(rename = "sourceFiles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All files used during config generation."]
    pub source_files: ::std::option::Option<
        ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    >,
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
#[doc = "Define a parameter's name and location. The parameter may be passed as either an HTTP header or a URL query parameter, and if both are passed the behavior is implementation-dependent."]
pub struct SystemParameter {
    #[serde(rename = "httpHeader")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Define the HTTP header name to use for the parameter. It is case insensitive."]
    pub http_header: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Define the name of the parameter, such as \"api_key\" . It is case sensitive."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "urlQueryParameter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Define the URL query parameter name to use for the parameter. It is case sensitive."]
    pub url_query_parameter: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Define a system parameter rule mapping system parameter definitions to methods."]
pub struct SystemParameterRule {
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Define parameters. Multiple names may be defined for a parameter. For a given method call, only one of them should be used. If multiple names are used the behavior is implementation-dependent. If none of the specified names are present the behavior is parameter-dependent."]
    pub parameters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SystemParameter>>>,
    #[serde(rename = "selector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Selects the methods to which this rule applies. Use '*' to indicate all methods in all APIs. Refer to selector for syntax details."]
    pub selector: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "### System parameter configuration A system parameter is a special kind of parameter defined by the API system, not by an individual API. It is typically mapped to an HTTP header and/or a URL query parameter. This configuration specifies which methods change the names of the system parameters."]
pub struct SystemParameters {
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Define system parameters. The parameters defined here will override the default parameters implemented by the system. If this field is missing from the service config, default system parameters will be used. Default system parameters and names is implementation-dependent. Example: define api key for all methods system_parameters rules: - selector: \"*\" parameters: - name: api_key url_query_parameter: api_key Example: define 2 api key names for a specific method. system_parameters rules: - selector: \"/ListShelves\" parameters: - name: api_key http_header: Api-Key1 - name: api_key http_header: Api-Key2 **NOTE:** All service configuration rules follow \"last one wins\" order."]
    pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SystemParameterRule>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Representation of a tenancy unit."]
pub struct TenancyUnit {
    #[serde(rename = "consumer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. @OutputOnly Cloud resource name of the consumer of this service. For example 'projects/123456'."]
    pub consumer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. @OutputOnly The time this tenancy unit was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Globally unique identifier of this tenancy unit \"services/{service}/{collection id}/{resource id}/tenancyUnits/{unit}\""]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Google Cloud API name of the managed service owning this tenancy unit. For example 'serviceconsumermanagement.googleapis.com'."]
    pub service: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tenantResources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resources constituting the tenancy unit. There can be at most 512 tenant resources in a tenancy unit."]
    pub tenant_resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TenantResource>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This structure defines a tenant project to be added to the specified tenancy unit and its initial configuration and properties. A project lien is created for the tenant project to prevent the tenant project from being deleted accidentally. The lien is deleted as part of tenant project removal."]
pub struct TenantProjectConfig {
    #[serde(rename = "billingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Billing account properties. The billing account must be specified."]
    pub billing_config: ::std::option::Option<::std::boxed::Box<BillingConfig>>,
    #[serde(rename = "folder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Folder where project in this tenancy unit must be located This folder must have been previously created with the required permissions for the caller to create and configure a project in it. Valid folder resource names have the format `folders/{folder_number}` (for example, `folders/123456`)."]
    pub folder: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Labels that are applied to this project."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "serviceAccountConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for the IAM service account on the tenant project."]
    pub service_account_config: ::std::option::Option<::std::boxed::Box<ServiceAccountConfig>>,
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Cloud API names of services that are activated on this project during provisioning. If any of these services can't be activated, the request fails. For example: 'compute.googleapis.com','cloudfunctions.googleapis.com'"]
    pub services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "tenantProjectPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes ownership and policies for the new tenant project. Required."]
    pub tenant_project_policy: ::std::option::Option<::std::boxed::Box<TenantProjectPolicy>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes policy settings that need to be applied to a newly created tenant project."]
pub struct TenantProjectPolicy {
    #[serde(rename = "policyBindings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Policy bindings to be applied to the tenant project, in addition to the 'roles/owner' role granted to the Service Consumer Management service account. At least one binding must have the role `roles/owner`."]
    pub policy_bindings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PolicyBinding>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Resource constituting the TenancyUnit."]
pub struct TenantResource {
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. @OutputOnly Identifier of the tenant resource. For cloud projects, it is in the form 'projects/{number}'. For example 'projects/123456'."]
    pub resource: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of tenant resource."]
    pub status: ::std::option::Option<TenantResourceStatusEnum>,
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique per single tenancy unit."]
    pub tag: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Status of tenant resource."]
pub enum TenantResourceStatusEnum {
    #[serde(rename = "STATUS_UNSPECIFIED")]
    #[doc = "Unspecified status is the default unset value."]
    StatusUnspecified,
    #[serde(rename = "PENDING_CREATE")]
    #[doc = "Creation of the tenant resource is ongoing."]
    PendingCreate,
    #[serde(rename = "ACTIVE")]
    #[doc = "Active resource."]
    Active,
    #[serde(rename = "PENDING_DELETE")]
    #[doc = "Deletion of the resource is ongoing."]
    PendingDelete,
    #[serde(rename = "FAILED")]
    #[doc = "Tenant resource creation or deletion has failed."]
    Failed,
    #[serde(rename = "DELETED")]
    #[doc = "Tenant resource has been deleted."]
    Deleted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A protocol buffer message type."]
pub struct Type {
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of fields."]
    pub fields: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Field>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fully qualified message name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "oneofs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of types appearing in `oneof` definitions in this type."]
    pub oneofs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The protocol buffer options."]
    pub options: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Option>>>,
    #[serde(rename = "sourceContext")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source context."]
    pub source_context: ::std::option::Option<::std::boxed::Box<SourceContext>>,
    #[serde(rename = "syntax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source syntax."]
    pub syntax: ::std::option::Option<TypeSyntaxEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The source syntax."]
pub enum TypeSyntaxEnum {
    #[serde(rename = "SYNTAX_PROTO2")]
    #[doc = "Syntax `proto2`."]
    SyntaxProto2,
    #[serde(rename = "SYNTAX_PROTO3")]
    #[doc = "Syntax `proto3`."]
    SyntaxProto3,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message to undelete tenant project resource previously deleted from the tenancy unit."]
pub struct UndeleteTenantProjectRequest {
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Tag of the resource within the tenancy unit."]
    pub tag: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration controlling usage of a service."]
pub struct Usage {
    #[serde(rename = "producerNotificationChannel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full resource name of a channel used for sending notifications to the service producer. Google Service Management currently only supports [Google Cloud Pub/Sub](https://cloud.google.com/pubsub) as a notification channel. To use Google Cloud Pub/Sub as the channel, this must be the name of a Cloud Pub/Sub topic that uses the Cloud Pub/Sub topic name format documented in https://cloud.google.com/pubsub/docs/overview."]
    pub producer_notification_channel: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requirements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requirements that must be satisfied before a consumer project can use the service. Each requirement is of the form /; for example 'serviceusage.googleapis.com/billing-enabled'. For Google APIs, a Terms of Service requirement must be included here. Google Cloud APIs must include \"serviceusage.googleapis.com/tos/cloud\". Other Google APIs should include \"serviceusage.googleapis.com/tos/universal\". Additional ToS can be included based on the business needs."]
    pub requirements: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of usage rules that apply to individual API methods. **NOTE:** All service configuration rules follow \"last one wins\" order."]
    pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UsageRule>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Usage configuration rules for the service. NOTE: Under development. Use this rule to configure unregistered calls for the service. Unregistered calls are calls that do not contain consumer project identity. (Example: calls that do not contain an API key). By default, API methods do not allow unregistered calls, and each method call must be identified by a consumer project identity. Use this rule to allow/disallow unregistered calls. Example of an API that wants to allow unregistered calls for entire service. usage: rules: - selector: \"*\" allow_unregistered_calls: true Example of a method that wants to allow unregistered calls. usage: rules: - selector: \"google.example.library.v1.LibraryService.CreateBook\" allow_unregistered_calls: true"]
pub struct UsageRule {
    #[serde(rename = "allowUnregisteredCalls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, the selected method allows unregistered calls, e.g. calls that don't identify any user or application."]
    pub allow_unregistered_calls: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "selector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Selects the methods to which this rule applies. Use '*' to indicate all methods in all APIs. Refer to selector for syntax details."]
    pub selector: ::std::option::Option<::std::string::String>,
    #[serde(rename = "skipServiceControl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, the selected method should skip service control and the control plane features, such as quota and billing, will not be available. This flag is used by Google Cloud Endpoints to bypass checks for internal methods, such as service health check methods."]
    pub skip_service_control: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the `AddVisibilityLabels` method. This response message is assigned to the `response` field of the returned Operation when that operation is done."]
pub struct V1AddVisibilityLabelsResponse {
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The updated set of visibility labels for this consumer on this service."]
    pub labels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for BatchCreateProducerOverrides"]
pub struct V1Beta1BatchCreateProducerOverridesResponse {
    #[serde(rename = "overrides")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The overrides that were created."]
    pub overrides: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<V1Beta1QuotaOverride>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the `DisableConsumer` method. This response message is assigned to the `response` field of the returned Operation when that operation is done."]
pub struct V1Beta1DisableConsumerResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the `EnableConsumer` method. This response message is assigned to the `response` field of the returned Operation when that operation is done."]
pub struct V1Beta1EnableConsumerResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the `GenerateServiceIdentity` method. This response message is assigned to the `response` field of the returned Operation when that operation is done."]
pub struct V1Beta1GenerateServiceIdentityResponse {
    #[serde(rename = "identity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ServiceIdentity that was created or retrieved."]
    pub identity: ::std::option::Option<::std::boxed::Box<V1Beta1ServiceIdentity>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ImportProducerOverrides"]
pub struct V1Beta1ImportProducerOverridesResponse {
    #[serde(rename = "overrides")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The overrides that were created from the imported data."]
    pub overrides: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<V1Beta1QuotaOverride>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ImportProducerQuotaPolicies"]
pub struct V1Beta1ImportProducerQuotaPoliciesResponse {
    #[serde(rename = "policies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The policies that were created from the imported data."]
    pub policies:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<V1Beta1ProducerQuotaPolicy>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Quota policy created by service producer."]
pub struct V1Beta1ProducerQuotaPolicy {
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cloud resource container at which the quota policy is created. The format is {container_type}/{container_number}"]
    pub container: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = " If this map is nonempty, then this policy applies only to specific values for dimensions defined in the limit unit. For example, an policy on a limit with the unit 1/{project}/{region} could contain an entry with the key \"region\" and the value \"us-east-1\"; the policy is only applied to quota consumed in that region. This map has the following restrictions: * Keys that are not defined in the limit's unit are not valid keys. Any string appearing in {brackets} in the unit (besides {project} or {user}) is a defined key. * \"project\" is not a valid key; the project is already specified in the parent resource name. * \"user\" is not a valid key; the API does not support quota polcies that apply only to a specific user. * If \"region\" appears as a key, its value must be a valid Cloud region. * If \"zone\" appears as a key, its value must be a valid Cloud zone. * If any valid key other than \"region\" or \"zone\" appears in the map, then all valid keys other than \"region\" or \"zone\" must also appear in the map."]
    pub dimensions:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "metric")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the metric to which this policy applies. An example name would be: `compute.googleapis.com/cpus`"]
    pub metric: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the producer policy. An example name would be: `services/compute.googleapis.com/organizations/123/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/producerQuotaPolicies/4a3f2c1d`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "policyValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quota policy value. Can be any nonnegative integer, or -1 (unlimited quota)."]
    pub policy_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The limit unit of the limit to which this policy applies. An example unit would be: `1/{project}/{region}` Note that `{project}` and `{region}` are not placeholders in this example; the literal characters `{` and `}` occur in the string."]
    pub unit: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A quota override"]
pub struct V1Beta1QuotaOverride {
    #[serde(rename = "adminOverrideAncestor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the ancestor that requested the override. For example: \"organizations/12345\" or \"folders/67890\". Used by admin overrides only."]
    pub admin_override_ancestor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = " If this map is nonempty, then this override applies only to specific values for dimensions defined in the limit unit. For example, an override on a limit with the unit 1/{project}/{region} could contain an entry with the key \"region\" and the value \"us-east-1\"; the override is only applied to quota consumed in that region. This map has the following restrictions: * Keys that are not defined in the limit's unit are not valid keys. Any string appearing in {brackets} in the unit (besides {project} or {user}) is a defined key. * \"project\" is not a valid key; the project is already specified in the parent resource name. * \"user\" is not a valid key; the API does not support quota overrides that apply only to a specific user. * If \"region\" appears as a key, its value must be a valid Cloud region. * If \"zone\" appears as a key, its value must be a valid Cloud zone. * If any valid key other than \"region\" or \"zone\" appears in the map, then all valid keys other than \"region\" or \"zone\" must also appear in the map."]
    pub dimensions:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "metric")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the metric to which this override applies. An example name would be: `compute.googleapis.com/cpus`"]
    pub metric: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the producer override. An example name would be: `services/compute.googleapis.com/projects/123/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/producerOverrides/4a3f2c1d`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "overrideValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The overriding quota limit value. Can be any nonnegative integer, or -1 (unlimited quota)."]
    pub override_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The limit unit of the limit to which this override applies. An example unit would be: `1/{project}/{region}` Note that `{project}` and `{region}` are not placeholders in this example; the literal characters `{` and `}` occur in the string."]
    pub unit: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the `RefreshConsumer` method. This response message is assigned to the `response` field of the returned Operation when that operation is done."]
pub struct V1Beta1RefreshConsumerResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A service identity in the Identity and Access Management API."]
pub struct V1Beta1ServiceIdentity {
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address of the service identity."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "P4 service identity resource name. An example name would be: `services/serviceconsumermanagement.googleapis.com/projects/123/serviceIdentities/default`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The P4 service identity configuration tag. This must be defined in activation_grants. If not specified when creating the account, the tag is set to \"default\"."]
    pub tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uniqueId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique and stable id of the service identity."]
    pub unique_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A default identity in the Identity and Access Management API."]
pub struct V1DefaultIdentity {
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address of the default identity."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Default identity resource name. An example name would be: `services/serviceconsumermanagement.googleapis.com/projects/123/defaultIdentity`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Default Identity tag. If specified when creating the account, the tag must be present in activation_grants. If not specified when creating the account, the tag is set to the tag specified in activation_grants."]
    pub tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uniqueId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique and stable id of the default identity."]
    pub unique_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the `DisableConsumer` method. This response message is assigned to the `response` field of the returned Operation when that operation is done."]
pub struct V1DisableConsumerResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the `EnableConsumer` method. This response message is assigned to the `response` field of the returned Operation when that operation is done."]
pub struct V1EnableConsumerResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the `GenerateDefaultIdentity` method. This response message is assigned to the `response` field of the returned Operation when that operation is done."]
pub struct V1GenerateDefaultIdentityResponse {
    #[serde(rename = "attachStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the role attachment. Under development (go/si-attach-role), currently always return ATTACH_STATUS_UNSPECIFIED)"]
    pub attach_status: ::std::option::Option<V1GenerateDefaultIdentityResponseAttachStatusEnum>,
    #[serde(rename = "identity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DefaultIdentity that was created or retrieved."]
    pub identity: ::std::option::Option<::std::boxed::Box<V1DefaultIdentity>>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Role attached to consumer project. Empty if not attached in this request. (Under development, currently always return empty.)"]
    pub role: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Status of the role attachment. Under development (go/si-attach-role), currently always return ATTACH_STATUS_UNSPECIFIED)"]
pub enum V1GenerateDefaultIdentityResponseAttachStatusEnum {
    #[serde(rename = "ATTACH_STATUS_UNSPECIFIED")]
    #[doc = "Indicates that the AttachStatus was not set."]
    AttachStatusUnspecified,
    #[serde(rename = "ATTACHED")]
    #[doc = "The default identity was attached to a role successfully in this request."]
    Attached,
    #[serde(rename = "ATTACH_SKIPPED")]
    #[doc = "The request specified that no attempt should be made to attach the role."]
    AttachSkipped,
    #[serde(rename = "PREVIOUSLY_ATTACHED")]
    #[doc = "Role was attached to the consumer project at some point in time. Tenant manager doesn't make assertion about the current state of the identity with respect to the consumer. Role attachment should happen only once after activation and cannot be reattached after customer removes it. (go/si-attach-role)"]
    PreviouslyAttached,
    #[serde(rename = "ATTACH_DENIED_BY_ORG_POLICY")]
    #[doc = "Role attachment was denied in this request by customer set org policy. (go/si-attach-role)"]
    AttachDeniedByOrgPolicy,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the `GenerateServiceAccount` method. This response message is assigned to the `response` field of the returned Operation when that operation is done."]
pub struct V1GenerateServiceAccountResponse {
    #[serde(rename = "account")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ServiceAccount that was created or retrieved."]
    pub account: ::std::option::Option<::std::boxed::Box<V1ServiceAccount>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the `RefreshConsumer` method. This response message is assigned to the `response` field of the returned Operation when that operation is done."]
pub struct V1RefreshConsumerResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the `RemoveVisibilityLabels` method. This response message is assigned to the `response` field of the returned Operation when that operation is done."]
pub struct V1RemoveVisibilityLabelsResponse {
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The updated set of visibility labels for this consumer on this service."]
    pub labels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A service account in the Identity and Access Management API."]
pub struct V1ServiceAccount {
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address of the service account."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "iamAccountName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. See b/136209818."]
    pub iam_account_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "P4 SA resource name. An example name would be: `services/serviceconsumermanagement.googleapis.com/projects/123/serviceAccounts/default`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The P4 SA configuration tag. This must be defined in activation_grants. If not specified when creating the account, the tag is set to \"default\"."]
    pub tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uniqueId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique and stable id of the service account."]
    pub unique_id: ::std::option::Option<::std::string::String>,
}
