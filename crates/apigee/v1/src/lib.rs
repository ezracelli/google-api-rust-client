#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message that represents an arbitrary HTTP body. It should only be used for payload formats that can't be represented as JSON, such as raw binary or an HTML page. This message can be used both in streaming and non-streaming API methods in the request as well as the response. It can be used as a top-level request field, which is convenient if one wants to extract parameters from either the URL or HTTP template into the request fields and also want access to the raw HTTP body. Example: message GetResourceRequest { // A unique request id. string request_id = 1; // The raw HTTP body is bound to this field. google.api.HttpBody http_body = 2; } service ResourceService { rpc GetResource(GetResourceRequest) returns (google.api.HttpBody); rpc UpdateResource(google.api.HttpBody) returns (google.protobuf.Empty); } Example with streaming methods: service CaldavService { rpc GetCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); rpc UpdateCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); } Use of this type only changes how the request and response bodies are handled, all other features will continue to work unchanged."]
pub struct GoogleApiHttpBody {
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
pub struct GoogleCloudApigeeV1Access {
    #[serde(rename = "Get")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub get: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1AccessGet>>,
    #[serde(rename = "Remove")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub remove: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1AccessRemove>>,
    #[serde(rename = "Set")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub set: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1AccessSet>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Get action. For example, \"Get\" : { \"name\" : \"target.name\", \"value\" : \"default\" }"]
pub struct GoogleCloudApigeeV1AccessGet {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Remove action. For example, \"Remove\" : { \"name\" : \"target.name\", \"success\" : true }"]
pub struct GoogleCloudApigeeV1AccessRemove {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub success: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Set action. For example, \"Set\" : { \"name\" : \"target.name\", \"success\" : true, \"value\" : \"default\" }"]
pub struct GoogleCloudApigeeV1AccessSet {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub success: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for ActivateNatAddressRequest. Activate the nat address request."]
pub struct GoogleCloudApigeeV1ActivateNatAddressRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Reference to a certificate or key/certificate pair."]
pub struct GoogleCloudApigeeV1Alias {
    #[serde(rename = "alias")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource ID for this alias. Values must match the regular expression `[^/]{1,255}`."]
    pub alias: ::std::option::Option<::std::string::String>,
    #[serde(rename = "certsInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Chain of certificates under this alias."]
    pub certs_info: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1Certificate>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of alias."]
    pub _type: ::std::option::Option<GoogleCloudApigeeV1AliasTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of alias."]
pub enum GoogleCloudApigeeV1AliasTypeEnum {
    #[serde(rename = "ALIAS_TYPE_UNSPECIFIED")]
    #[doc = "Alias type is not specified."]
    AliasTypeUnspecified,
    #[serde(rename = "CERT")]
    #[doc = "Certificate."]
    Cert,
    #[serde(rename = "KEY_CERT")]
    #[doc = "Key/certificate pair."]
    KeyCert,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1AliasRevisionConfig {
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location of the alias file. For example, a Google Cloud Storage URI."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the alias revision included in the keystore in the following format: `organizations/{org}/environments/{env}/keystores/{keystore}/aliases/{alias}/revisions/{rev}`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub _type: ::std::option::Option<GoogleCloudApigeeV1AliasRevisionConfigTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GoogleCloudApigeeV1AliasRevisionConfigTypeEnum {
    #[serde(rename = "ALIAS_TYPE_UNSPECIFIED")]
    #[doc = "Alias type is not specified."]
    AliasTypeUnspecified,
    #[serde(rename = "CERT")]
    #[doc = "Certificate."]
    Cert,
    #[serde(rename = "KEY_CERT")]
    #[doc = "Key/certificate pair."]
    KeyCert,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "the Api category resource wrapped with response status, error_code etc."]
pub struct GoogleCloudApigeeV1ApiCategory {
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details of category."]
    pub data: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1ApiCategoryData>>,
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID that can be used to find errors in the log files."]
    pub error_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the operation."]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID that can be used to find request details in the log files."]
    pub request_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the operation."]
    pub status: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "the Api category resource."]
pub struct GoogleCloudApigeeV1ApiCategoryData {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the category (a UUID)."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the category."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "siteId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the portal."]
    pub site_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time the category was last modified in milliseconds since epoch."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1ApiProduct {
    #[serde(rename = "apiResources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "approvalType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flag that specifies how API keys are approved to access the APIs defined by the API product. If set to `manual`, the consumer key is generated and returned in \"pending\" state. In this case, the API keys won't work until they have been explicitly approved. If set to `auto`, the consumer key is generated and returned in \"approved\" state and can be used immediately. **Note:** Typically, `auto` is used to provide access to free or trial API products that provide limited quota or capabilities."]
    pub approval_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Array of attributes that may be used to extend the default API product profile with customer-specific metadata. You can specify a maximum of 18 attributes. Use this property to specify the access level of the API product as either `public`, `private`, or `internal`. Only products marked `public` are available to developers in the Apigee developer portal. For example, you can set a product to `internal` while it is in development and then change access to `public` when it is ready to release on the portal. API products marked as `private` do not appear on the portal, but can be accessed by external developers."]
    pub attributes:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Attribute>>>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Response only. Creation time of this environment as milliseconds since epoch."]
    pub created_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the API product. Include key information about the API product that is not captured by other fields. Comma-separated list of API resources to be bundled in the API product. By default, the resource paths are mapped from the `proxy.pathsuffix` variable. The proxy path suffix is defined as the URI fragment following the ProxyEndpoint base path. For example, if the `apiResources` element is defined to be `/forecastrss` and the base path defined for the API proxy is `/weather`, then only requests to `/weather/forecastrss` are permitted by the API product. You can select a specific path, or you can select all subpaths with the following wildcard: - `/**`: Indicates that all sub-URIs are included. - `/*` : Indicates that only URIs one level down are included. By default, / supports the same resources as /** as well as the base path defined by the API proxy. For example, if the base path of the API proxy is `/v1/weatherapikey`, then the API product supports requests to `/v1/weatherapikey` and to any sub-URIs, such as `/v1/weatherapikey/forecastrss`, `/v1/weatherapikey/region/CA`, and so on. For more information, see Managing API products."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name displayed in the UI or developer portal to developers registering for API access."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "environments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Comma-separated list of environment names to which the API product is bound. Requests to environments that are not listed are rejected. By specifying one or more environments, you can bind the resources listed in the API product to a specific environment, preventing developers from accessing those resources through API proxies deployed in another environment. This setting is used, for example, to prevent resources associated with API proxies in `prod` from being accessed by API proxies deployed in `test`."]
    pub environments: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Response only. Modified time of this environment as milliseconds since epoch."]
    pub last_modified_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Internal name of the API product. Characters you can use in the name are restricted to: `A-Z0-9._\\-$ %`. **Note:** The internal name cannot be edited when updating the API product."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration used to group Apigee proxies or remote services with resources, method types, and quotas. The resource refers to the resource URI (excluding the base path). With this grouping, the API product creator is able to fine-tune and give precise control over which REST methods have access to specific resources and how many calls can be made (using the `quota` setting). **Note:** The `api_resources` setting cannot be specified for both the API product and operation group; otherwise the call will fail."]
    pub operation_group:
        ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1OperationGroup>>,
    #[serde(rename = "proxies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Comma-separated list of API proxy names to which this API product is bound. By specifying API proxies, you can associate resources in the API product with specific API proxies, preventing developers from accessing those resources through other API proxies. Apigee rejects requests to API proxies that are not listed. **Note:** The API proxy names must already exist in the specified environment as they will be validated upon creation."]
    pub proxies: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "quota")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of request messages permitted per app by this API product for the specified `quotaInterval` and `quotaTimeUnit`. For example, a `quota` of 50, for a `quotaInterval` of 12 and a `quotaTimeUnit` of hours means 50 requests are allowed every 12 hours."]
    pub quota: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quotaInterval")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time interval over which the number of request messages is calculated."]
    pub quota_interval: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quotaTimeUnit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time unit defined for the `quotaInterval`. Valid values include `minute`, `hour`, `day`, or `month`."]
    pub quota_time_unit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Comma-separated list of OAuth scopes that are validated at runtime. Apigee validates that the scopes in any access token presented match the scopes defined in the OAuth policy associated with the API product."]
    pub scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1ApiProductRef {
    #[serde(rename = "apiproduct")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the API product."]
    pub apiproduct: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the API product."]
    pub status: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata describing the API proxy"]
pub struct GoogleCloudApigeeV1ApiProxy {
    #[serde(rename = "latestRevisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of the most recently created revision for this api proxy."]
    pub latest_revision_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metaData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata describing the API proxy."]
    pub meta_data: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1EntityMetadata>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the API proxy."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of revisons defined for the API proxy."]
    pub revision: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "API proxy revision."]
pub struct GoogleCloudApigeeV1ApiProxyRevision {
    #[serde(rename = "basepaths")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Base URL of the API proxy."]
    pub basepaths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "configurationVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Version of the API proxy configuration schema to which the API proxy conforms. Currently, the only supported value is 4.0 (`majorVersion.minorVersion`). This setting may be used in the future to track the evolution of the API proxy format."]
    pub configuration_version:
        ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1ConfigVersion>>,
    #[serde(rename = "contextInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Revision number, app name, and organization for the API proxy."]
    pub context_info: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time that the API proxy revision was created in milliseconds since epoch."]
    pub created_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the API proxy revision."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human-readable name of the API proxy."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityMetaDataAsProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata describing the API proxy revision as a key-value map."]
    pub entity_meta_data_as_properties:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time that the API proxy revision was last modified in milliseconds since epoch."]
    pub last_modified_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the API proxy."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "policies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of policy names included in the API proxy revision.."]
    pub policies: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "proxies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of proxy names included in the API proxy revision."]
    pub proxies: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "proxyEndpoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of ProxyEndpoints in the `/proxies` directory of the API proxy. Typically, this element is included only when the API proxy was created using the Edge UI. This is a 'manifest' setting designed to provide visibility into the contents of the API proxy."]
    pub proxy_endpoints: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "resourceFiles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of resource files included in the API proxy revision."]
    pub resource_files: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1ResourceFiles>>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of the resources included in the API proxy revision formatted as \"{type}://{name}\"."]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API proxy revision."]
    pub revision: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sharedFlows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of the shared flows included in the API proxy revision."]
    pub shared_flows: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "spec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OpenAPI Specification that is associated with the API proxy. The value is set to a URL or to a path in the specification store."]
    pub spec: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetEndpoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of TargetEndpoints in the `/targets` directory of the API proxy. Typically, this element is included only when the API proxy was created using the Edge UI. This is a 'manifest' setting designed to provide visibility into the contents of the API proxy."]
    pub target_endpoints: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "targetServers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of TargetServers referenced in any TargetEndpoint in the API proxy. Typically, you will see this element only when the API proxy was created using the Edge UI. This is a 'manifest' setting designed to provide visibility into the contents of the API proxy."]
    pub target_servers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "targets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of the targets included in the API proxy revision."]
    pub targets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of the teams included in the API proxy revision."]
    pub teams: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type. Set to `Application`. Maintained for compatibility with the Apigee Edge API."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1ApiResponseWrapper {
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID that can be used to find errors in the log files."]
    pub error_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the operation."]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID that can be used to find request details in the log files."]
    pub request_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the operation."]
    pub status: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1App {
    #[serde(rename = "apiProducts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of API products associated with the app."]
    pub api_products:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ApiProductRef>>>,
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the app."]
    pub app_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of attributes."]
    pub attributes:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Attribute>>>,
    #[serde(rename = "callbackUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Callback URL used by OAuth 2.0 authorization servers to communicate authorization codes back to apps."]
    pub callback_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "companyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the company that owns the app."]
    pub company_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Unix time when the app was created."]
    pub created_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "credentials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Set of credentials for the app. Credentials are API key/secret pairs associated with API products."]
    pub credentials:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Credential>>>,
    #[serde(rename = "developerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the developer."]
    pub developer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keyExpiresIn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Duration, in milliseconds, of the consumer key that will be generated for the app. The default value, -1, indicates an infinite validity period. Once set, the expiration can't be updated. json key: keyExpiresIn"]
    pub key_expires_in: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Last modified time as milliseconds since epoch."]
    pub last_modified_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the app."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Scopes to apply to the app. The specified scope names must already exist on the API product that you associate with the app."]
    pub scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the credential."]
    pub status: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1AsyncQuery {
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creation time of the query."]
    pub created: ::std::option::Option<::std::string::String>,
    #[serde(rename = "envgroupHostname")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hostname is available only when query is executed at host level."]
    pub envgroup_hostname: ::std::option::Option<::std::string::String>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error is set when query fails."]
    pub error: ::std::option::Option<::std::string::String>,
    #[serde(rename = "executionTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ExecutionTime is available only after the query is completed."]
    pub execution_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Asynchronous Query Name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "queryParams")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contains information like metrics, dimenstions etc of the AsyncQuery."]
    pub query_params: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1QueryMetadata>>,
    #[serde(rename = "reportDefinitionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Asynchronous Report ID."]
    pub report_definition_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Result is available only after the query is completed."]
    pub result: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1AsyncQueryResult>>,
    #[serde(rename = "resultFileSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ResultFileSize is available only after the query is completed."]
    pub result_file_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resultRows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ResultRows is available only after the query is completed."]
    pub result_rows: ::std::option::Option<::std::string::String>,
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Self link of the query. Example: `/organizations/myorg/environments/myenv/queries/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd` or following format if query is running at host level: `/organizations/myorg/hostQueries/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd`"]
    pub _self: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Query state could be \"enqueued\", \"running\", \"completed\", \"failed\"."]
    pub state: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Last updated timestamp for the query."]
    pub updated: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1AsyncQueryResult {
    #[serde(rename = "expires")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Query result will be unaccessable after this time."]
    pub expires: ::std::option::Option<::std::string::String>,
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Self link of the query results. Example: `/organizations/myorg/environments/myenv/queries/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd/result` or following format if query is running at host level: `/organizations/myorg/hostQueries/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd/result`"]
    pub _self: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1AsyncQueryResultView {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error code when there is a failure."]
    pub code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error message when there is a failure."]
    pub error: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata contains information like metrics, dimenstions etc of the AsyncQuery."]
    pub metadata: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1QueryMetadata>>,
    #[serde(rename = "rows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rows of query result. Each row is a JSON object. Example: {sum(message_count): 1, developer_app: \"(not set)\",…}"]
    pub rows: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "State of retrieving ResultView."]
    pub state: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Key-value pair to store extra metadata."]
pub struct GoogleCloudApigeeV1Attribute {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API key of the attribute."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value of the attribute."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1Attributes {
    #[serde(rename = "attribute")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of attributes."]
    pub attribute:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Attribute>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "CanaryEvaluation represents the canary analysis between two versions of the runtime that is serving requests."]
pub struct GoogleCloudApigeeV1CanaryEvaluation {
    #[serde(rename = "control")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The stable version that is serving requests."]
    pub control: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Create time of the canary evaluation."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. End time for the evaluation's analysis."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metricLabels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Labels used to filter the metrics used for a canary evaluation."]
    pub metric_labels:
        ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1CanaryEvaluationMetricLabels>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Name of the canary evalution."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Start time for the canary evaluation's analysis."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current state of the canary evaluation."]
    pub state: ::std::option::Option<GoogleCloudApigeeV1CanaryEvaluationStateEnum>,
    #[serde(rename = "treatment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The newer version that is serving requests."]
    pub treatment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "verdict")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resulting verdict of the canary evaluations: NONE, PASS, or FAIL."]
    pub verdict: ::std::option::Option<GoogleCloudApigeeV1CanaryEvaluationVerdictEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The current state of the canary evaluation."]
pub enum GoogleCloudApigeeV1CanaryEvaluationStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "No state has been specified."]
    StateUnspecified,
    #[serde(rename = "RUNNING")]
    #[doc = "The canary evaluation is still in progress."]
    Running,
    #[serde(rename = "SUCCEEDED")]
    #[doc = "The canary evaluation has finished."]
    Succeeded,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The resulting verdict of the canary evaluations: NONE, PASS, or FAIL."]
pub enum GoogleCloudApigeeV1CanaryEvaluationVerdictEnum {
    #[serde(rename = "VERDICT_UNSPECIFIED")]
    #[doc = "Verdict is not available yet."]
    VerdictUnspecified,
    #[serde(rename = "NONE")]
    #[doc = "No verdict reached."]
    None,
    #[serde(rename = "FAIL")]
    #[doc = "Evaluation is not good."]
    Fail,
    #[serde(rename = "PASS")]
    #[doc = "Evaluation is good."]
    Pass,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Labels that can be used to filter Apigee metrics."]
pub struct GoogleCloudApigeeV1CanaryEvaluationMetricLabels {
    #[serde(rename = "env")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The environment ID associated with the metrics."]
    pub env: ::std::option::Option<::std::string::String>,
    #[serde(rename = "instance_id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The instance ID associated with the metrics. In Apigee Hybrid, the value is configured during installation."]
    pub instance_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The location associated with the metrics."]
    pub location: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "X.509 certificate as defined in RFC 5280."]
pub struct GoogleCloudApigeeV1CertInfo {
    #[serde(rename = "basicConstraints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "X.509 basic constraints extension."]
    pub basic_constraints: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expiryDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "X.509 `notAfter` validity period in milliseconds since epoch."]
    pub expiry_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isValid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flag that specifies whether the certificate is valid. Flag is set to `Yes` if the certificate is valid, `No` if expired, or `Not yet` if not yet valid."]
    pub is_valid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "issuer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "X.509 issuer."]
    pub issuer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publicKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Public key component of the X.509 subject public key info."]
    pub public_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "X.509 serial number."]
    pub serial_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sigAlgName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "X.509 signatureAlgorithm."]
    pub sig_alg_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subject")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "X.509 subject."]
    pub subject: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subjectAlternativeNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "X.509 subject alternative names (SANs) extension."]
    pub subject_alternative_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "validFrom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "X.509 `notBefore` validity period in milliseconds since epoch."]
    pub valid_from: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "X.509 version."]
    pub version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1Certificate {
    #[serde(rename = "certInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Chain of certificates under this name."]
    pub cert_info:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1CertInfo>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1CommonNameConfig {
    #[serde(rename = "matchWildCards")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub match_wild_cards: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Version of the API proxy configuration schema. Currently, only 4.0 is supported."]
pub struct GoogleCloudApigeeV1ConfigVersion {
    #[serde(rename = "majorVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Major version of the API proxy configuration schema."]
    pub major_version: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "minorVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minor version of the API proxy configuration schema."]
    pub minor_version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1Credential {
    #[serde(rename = "apiProducts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of API products this credential can be used for."]
    pub api_products:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ApiProductRef>>>,
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of attributes associated with this credential."]
    pub attributes:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Attribute>>>,
    #[serde(rename = "consumerKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Consumer key."]
    pub consumer_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "consumerSecret")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Secret key."]
    pub consumer_secret: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time the credential will expire in milliseconds since epoch."]
    pub expires_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "issuedAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time the credential was issued in milliseconds since epoch."]
    pub issued_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of scopes to apply to the app. Specified scopes must already exist on the API product that you associate with the app."]
    pub scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the credential."]
    pub status: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1CustomReport {
    #[serde(rename = "chartType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains the chart type for the report"]
    pub chart_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "comments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Legacy field: not used. This field contains a list of comments associated with custom report"]
    pub comments: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Unix time when the app was created json key: createdAt"]
    pub created_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This contains the list of dimensions for the report"]
    pub dimensions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This is the display name for the report"]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Environment name"]
    pub environment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains the filter expression"]
    pub filter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fromTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Legacy field: not used. Contains the from time for the report"]
    pub from_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Modified time of this entity as milliseconds since epoch. json key: lastModifiedAt"]
    pub last_modified_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastViewedAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Last viewed time of this entity as milliseconds since epoch"]
    pub last_viewed_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Legacy field: not used This field contains the limit for the result retrieved"]
    pub limit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. This contains the list of metrics"]
    pub metrics: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1CustomReportMetric>>,
    >,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Unique identifier for the report T his is a legacy field used to encode custom report unique id"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Legacy field: not used. This field contains the offset for the data"]
    pub offset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "organization")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Organization name"]
    pub organization: ::std::option::Option<::std::string::String>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains report properties such as ui metadata etc."]
    pub properties: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ReportProperty>>,
    >,
    #[serde(rename = "sortByCols")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Legacy field: not used much. Contains the list of sort by columns"]
    pub sort_by_cols: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Legacy field: not used much. Contains the sort order for the sort columns"]
    pub sort_order: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Legacy field: not used. This field contains a list of tags associated with custom report"]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains the time unit of aggregation for the report"]
    pub time_unit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "toTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Legacy field: not used. Contains the end time for the report"]
    pub to_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "topk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Legacy field: not used. This field contains the top k parameter value for restricting the result"]
    pub topk: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This encapsulates a metric property of the form sum(message_count) where name is message_count and function is sum"]
pub struct GoogleCloudApigeeV1CustomReportMetric {
    #[serde(rename = "function")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "aggregate function"]
    pub function: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "name of the metric"]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Data collector configuration."]
pub struct GoogleCloudApigeeV1DataCollector {
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which the data collector was created in milliseconds since the epoch."]
    pub created_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of the data collector."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which the Data Collector was last updated in milliseconds since the epoch."]
    pub last_modified_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the data collector. Must begin with `dc_`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The type of data this data collector will collect."]
    pub _type: ::std::option::Option<GoogleCloudApigeeV1DataCollectorTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Immutable. The type of data this data collector will collect."]
pub enum GoogleCloudApigeeV1DataCollectorTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "For future compatibility."]
    TypeUnspecified,
    #[serde(rename = "INTEGER")]
    #[doc = "For integer values."]
    Integer,
    #[serde(rename = "FLOAT")]
    #[doc = "For float values."]
    Float,
    #[serde(rename = "STRING")]
    #[doc = "For string values."]
    String,
    #[serde(rename = "BOOLEAN")]
    #[doc = "For boolean values."]
    Boolean,
    #[serde(rename = "DATETIME")]
    #[doc = "For datetime values."]
    Datetime,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Data collector and its configuration."]
pub struct GoogleCloudApigeeV1DataCollectorConfig {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the data collector in the following format: `organizations/{org}/datacollectors/{datacollector}`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Data type accepted by the data collector."]
    pub _type: ::std::option::Option<GoogleCloudApigeeV1DataCollectorConfigTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Data type accepted by the data collector."]
pub enum GoogleCloudApigeeV1DataCollectorConfigTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "For future compatibility."]
    TypeUnspecified,
    #[serde(rename = "INTEGER")]
    #[doc = "For integer values."]
    Integer,
    #[serde(rename = "FLOAT")]
    #[doc = "For float values."]
    Float,
    #[serde(rename = "STRING")]
    #[doc = "For string values."]
    String,
    #[serde(rename = "BOOLEAN")]
    #[doc = "For boolean values."]
    Boolean,
    #[serde(rename = "DATETIME")]
    #[doc = "For datetime values."]
    Datetime,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The data store defines the connection to export data repository (Cloud Storage, BigQuery), including the credentials used to access the data repository."]
pub struct GoogleCloudApigeeV1Datastore {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Datastore create time, in milliseconds since the epoch of 1970-01-01T00:00:00Z"]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "datastoreConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Datastore Configurations."]
    pub datastore_config:
        ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1DatastoreConfig>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Display name in UI"]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Datastore last update time, in milliseconds since the epoch of 1970-01-01T00:00:00Z"]
    pub last_update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "org")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Organization that the datastore belongs to"]
    pub org: ::std::option::Option<::std::string::String>,
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Resource link of Datastore. Example: `/organizations/{org}/analytics/datastores/{uuid}`"]
    pub _self: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Destination storage type. Supported types `gcs` or `bigquery`."]
    pub target_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration detail for datastore"]
pub struct GoogleCloudApigeeV1DatastoreConfig {
    #[serde(rename = "bucketName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the Cloud Storage bucket. Required for `gcs` target_type."]
    pub bucket_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "datasetName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "BigQuery dataset name Required for `bigquery` target_type."]
    pub dataset_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Path of Cloud Storage bucket Required for `gcs` target_type."]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. GCP project in which the datastore exists"]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tablePrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Prefix of BigQuery table Required for `bigquery` target_type."]
    pub table_prefix: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Date range of the data to export."]
pub struct GoogleCloudApigeeV1DateRange {
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. End date (exclusive) of the data to export in the format `yyyy-mm-dd`. The date range ends at 00:00:00 UTC on the end date- which will not be in the output."]
    pub end: ::std::option::Option<::std::string::String>,
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Start date of the data to export in the format `yyyy-mm-dd`. The date range begins at 00:00:00 UTC on the start date."]
    pub start: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1DebugMask {
    #[serde(rename = "faultJSONPaths")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of JSON paths that specify the JSON elements to be filtered from JSON payloads in error flows."]
    pub fault_json_paths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "faultXPaths")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of XPaths that specify the XML elements to be filtered from XML payloads in error flows."]
    pub fault_x_paths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the debug mask."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "namespaces")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Map of namespaces to URIs."]
    pub namespaces:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "requestJSONPaths")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of JSON paths that specify the JSON elements to be filtered from JSON request message payloads."]
    pub request_json_paths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "requestXPaths")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of XPaths that specify the XML elements to be filtered from XML request message payloads."]
    pub request_x_paths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "responseJSONPaths")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of JSON paths that specify the JSON elements to be filtered from JSON response message payloads."]
    pub response_json_paths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "responseXPaths")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of XPaths that specify the XML elements to be filtered from XML response message payloads."]
    pub response_x_paths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "variables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of variables that should be masked from the debug output."]
    pub variables: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1DebugSession {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The number of request to be traced. Min = 1, Max = 15, Default = 10."]
    pub count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A conditional statement which is evaluated against the request message to determine if it should be traced. Syntax matches that of on API Proxy bundle flow Condition."]
    pub filter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique ID for this DebugSession."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The time in seconds after which this DebugSession should end. This value will override the value in query param, if both are provided."]
    pub timeout: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tracesize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The maximum number of bytes captured from the response payload. Min = 0, Max = 5120, Default = 5120."]
    pub tracesize: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "validity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The length of time, in seconds, that this debug session is valid, starting from when it's received in the control plane. Min = 1, Max = 15, Default = 10."]
    pub validity: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A transaction contains all of the debug information of the entire message flow of an API call processed by the runtime plane. The information is collected and recorded at critical points of the message flow in the runtime apiproxy."]
pub struct GoogleCloudApigeeV1DebugSessionTransaction {
    #[serde(rename = "completed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flag indicating whether a transaction is completed or not"]
    pub completed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "point")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of debug data collected by runtime plane at various defined points in the flow."]
    pub point: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Point>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1DeleteCustomReportResponse {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The response contains only a message field."]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1Deployment {
    #[serde(rename = "apiProxy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API proxy."]
    pub api_proxy: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deployStartTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time the API proxy was marked `deployed` in the control plane in millisconds since epoch."]
    pub deploy_start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Environment."]
    pub environment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Errors reported for this deployment. Populated only when state == ERROR. This field is not populated in List APIs."]
    pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status reported by each runtime instance. This field is not populated in List APIs."]
    pub instances: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1InstanceDeploymentStatus>>,
    >,
    #[serde(rename = "pods")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status reported by runtime pods. This field is not populated for List APIs."]
    pub pods:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1PodStatus>>>,
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API proxy revision."]
    pub revision: ::std::option::Option<::std::string::String>,
    #[serde(rename = "routeConflicts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Conflicts in the desired state routing configuration. The presence of conflicts does not cause the state to be ERROR, but it will mean that some of the deployments basepaths are not routed to its environment. If the conflicts change, the state will transition to PROGRESSING until the latest configuration is rolled out to all instances. This field is not populated in List APIs."]
    pub route_conflicts: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudApigeeV1DeploymentChangeReportRoutingConflict>,
        >,
    >,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Current state of the deployment. This field is not populated in List APIs."]
    pub state: ::std::option::Option<GoogleCloudApigeeV1DeploymentStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Current state of the deployment. This field is not populated in List APIs."]
pub enum GoogleCloudApigeeV1DeploymentStateEnum {
    #[serde(rename = "RUNTIME_STATE_UNSPECIFIED")]
    #[doc = "This value should never be returned."]
    RuntimeStateUnspecified,
    #[serde(rename = "READY")]
    #[doc = "The runtime has loaded the deployment."]
    Ready,
    #[serde(rename = "PROGRESSING")]
    #[doc = "The deployment is not fully ready in the runtime."]
    Progressing,
    #[serde(rename = "ERROR")]
    #[doc = "There is an error with the deployment that requires intervention."]
    Error,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for GenerateDeployChangeReport and GenerateUndeployChangeReport. This report contains any validation failures that would cause the deployment to be rejected, as well changes and conflicts in routing that may occur due to the new deployment. The existence of a routing warning does not necessarily imply that the deployment request is bad, if the desired state of the deployment request is to effect a routing change. The primary purposes of the routing messages are: 1) To inform users of routing changes that may have an effect on traffic currently being routed to other existing deployments. 2) To warn users if some basepath in the proxy will not receive traffic due to an existing deployment having already claimed that basepath. The presence of routing conflicts/changes will not cause non-dry-run DeployApiProxy/UndeployApiProxy requests to be rejected."]
pub struct GoogleCloudApigeeV1DeploymentChangeReport {
    #[serde(rename = "routingChanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All routing changes that may result from a deployment request."]
    pub routing_changes: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1DeploymentChangeReportRoutingChange>>,
    >,
    #[serde(rename = "routingConflicts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All basepath conflicts detected for a deployment request."]
    pub routing_conflicts: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudApigeeV1DeploymentChangeReportRoutingConflict>,
        >,
    >,
    #[serde(rename = "validationErrors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Validation errors that would cause the deployment change request to be rejected."]
    pub validation_errors: ::std::option::Option<::std::boxed::Box<GoogleRpcPreconditionFailure>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a potential routing change that may occur as a result of some deployment operation."]
pub struct GoogleCloudApigeeV1DeploymentChangeReportRoutingChange {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A human-readable description of this routing change."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "environmentGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the environment group affected by this routing change."]
    pub environment_group: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fromDeployment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The basepath/deployment that may stop receiving some traffic."]
    pub from_deployment: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudApigeeV1DeploymentChangeReportRoutingDeployment>,
    >,
    #[serde(rename = "shouldSequenceRollout")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if using sequenced rollout would make this routing change safer. Note: this does not necessarily imply that automated sequenced rollout mode is supported for the operation."]
    pub should_sequence_rollout: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "toDeployment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The basepath/deployment that may start receiving that traffic. May be null if no deployment is able to receive the traffic."]
    pub to_deployment: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudApigeeV1DeploymentChangeReportRoutingDeployment>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a routing conflict that may cause a deployment not to receive traffic at some basepath."]
pub struct GoogleCloudApigeeV1DeploymentChangeReportRoutingConflict {
    #[serde(rename = "conflictingDeployment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The existing basepath/deployment causing the conflict."]
    pub conflicting_deployment: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudApigeeV1DeploymentChangeReportRoutingDeployment>,
    >,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A human-readable description of this conflict."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "environmentGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the environment group in which this conflict exists."]
    pub environment_group: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A tuple representing a basepath and the deployment containing it."]
pub struct GoogleCloudApigeeV1DeploymentChangeReportRoutingDeployment {
    #[serde(rename = "apiProxy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the deployed proxy revision containing the basepath."]
    pub api_proxy: ::std::option::Option<::std::string::String>,
    #[serde(rename = "basepath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The basepath receiving traffic."]
    pub basepath: ::std::option::Option<::std::string::String>,
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the environment in which the proxy is deployed."]
    pub environment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the deployed proxy revision containing the basepath."]
    pub revision: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1DeploymentConfig {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional key-value metadata for the deployment."]
    pub attributes:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "basePath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Base path where the application will be hosted. Defaults to \"/\"."]
    pub base_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location of the API proxy bundle as a URI."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the API or shared flow revision to be deployed in the following format: `organizations/{org}/apis/{api}/revisions/{rev}` or `organizations/{org}/sharedflows/{sharedflow}/revisions/{rev}`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "proxyUid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique ID of the API proxy revision."]
    pub proxy_uid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique ID. The ID will only change if the deployment is deleted and recreated."]
    pub uid: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1Developer {
    #[serde(rename = "accessType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Access type."]
    pub access_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "appFamily")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Developer app family."]
    pub app_family: ::std::option::Option<::std::string::String>,
    #[serde(rename = "apps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of apps associated with the developer."]
    pub apps: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Developer attributes (name/value pairs). The custom attribute limit is 18."]
    pub attributes:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Attribute>>>,
    #[serde(rename = "companies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of companies associated with the developer."]
    pub companies: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time at which the developer was created in milliseconds since epoch."]
    pub created_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "developerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the developer. **Note**: IDs are generated internally by Apigee and are not guaranteed to stay the same over time."]
    pub developer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Email address of the developer. This value is used to uniquely identify the developer in Apigee hybrid. Note that the email address has to be in lowercase only."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. First name of the developer."]
    pub first_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time at which the developer was last modified in milliseconds since epoch."]
    pub last_modified_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Last name of the developer."]
    pub last_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "organizationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Name of the Apigee organization in which the developer resides."]
    pub organization_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Status of the developer. Valid values are `active` and `inactive`."]
    pub status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. User name of the developer. Not used by Apigee hybrid."]
    pub user_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1DeveloperApp {
    #[serde(rename = "apiProducts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of API products associated with the developer app."]
    pub api_products: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "appFamily")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Developer app family."]
    pub app_family: ::std::option::Option<::std::string::String>,
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the developer app."]
    pub app_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of attributes for the developer app."]
    pub attributes:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Attribute>>>,
    #[serde(rename = "callbackUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Callback URL used by OAuth 2.0 authorization servers to communicate authorization codes back to developer apps."]
    pub callback_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time the developer app was created in milliseconds since epoch."]
    pub created_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "credentials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Set of credentials for the developer app consisting of the consumer key/secret pairs associated with the API products."]
    pub credentials:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Credential>>>,
    #[serde(rename = "developerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the developer."]
    pub developer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keyExpiresIn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Expiration time, in milliseconds, for the consumer key that is generated for the developer app. If not set or left to the default value of `-1`, the API key never expires. The expiration time can't be updated after it is set."]
    pub key_expires_in: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time the developer app was modified in milliseconds since epoch."]
    pub last_modified_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the developer app."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Scopes to apply to the developer app. The specified scopes must already exist for the API product that you associate with the developer app."]
    pub scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the credential. Valid values include `approved` or `revoked`."]
    pub status: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1DeveloperAppKey {
    #[serde(rename = "apiProducts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of API products for which the credential can be used. **Note**: Do not specify the list of API products when creating a consumer key and secret for a developer app. Instead, use the UpdateDeveloperAppKey API to make the association after the consumer key and secret are created."]
    pub api_products: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of attributes associated with the credential."]
    pub attributes:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Attribute>>>,
    #[serde(rename = "consumerKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Consumer key."]
    pub consumer_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "consumerSecret")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Secret key."]
    pub consumer_secret: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time the developer app expires in milliseconds since epoch."]
    pub expires_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expiresInSeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input only. Expiration time, in seconds, for the consumer key. If not set or left to the default value of `-1`, the API key never expires. The expiration time can't be updated after it is set."]
    pub expires_in_seconds: ::std::option::Option<::std::string::String>,
    #[serde(rename = "issuedAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time the developer app was created in milliseconds since epoch."]
    pub issued_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Scopes to apply to the app. The specified scope names must already be defined for the API product that you associate with the app."]
    pub scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the credential. Valid values include `approved` or `revoked`."]
    pub status: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This message type encapsulates a metric grouped by dimension."]
pub struct GoogleCloudApigeeV1DimensionMetric {
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains a list of metrics."]
    pub metrics:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Metric>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains the name of the dimension."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata common to many entities in this API."]
pub struct GoogleCloudApigeeV1EntityMetadata {
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time at which the API proxy was created, in milliseconds since epoch."]
    pub created_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time at which the API proxy was most recently modified, in milliseconds since epoch."]
    pub last_modified_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of entity described"]
    pub sub_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1Environment {
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Creation time of this environment as milliseconds since epoch."]
    pub created_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Description of the environment."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Display name for this environment."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Last modification time of this environment as milliseconds since epoch."]
    pub last_modified_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Name of the environment. Values must match the regular expression `^[.\\\\p{Alnum}-_]{1,255}$`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Key-value pairs that may be used for customizing the environment."]
    pub properties: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1Properties>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. State of the environment. Values other than ACTIVE means the resource is not ready to use."]
    pub state: ::std::option::Option<GoogleCloudApigeeV1EnvironmentStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. State of the environment. Values other than ACTIVE means the resource is not ready to use."]
pub enum GoogleCloudApigeeV1EnvironmentStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Resource is in an unspecified state."]
    StateUnspecified,
    #[serde(rename = "CREATING")]
    #[doc = "Resource is being created."]
    Creating,
    #[serde(rename = "ACTIVE")]
    #[doc = "Resource is provisioned and ready to use."]
    Active,
    #[serde(rename = "DELETING")]
    #[doc = "The resource is being deleted."]
    Deleting,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1EnvironmentConfig {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time that the environment configuration was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dataCollectors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of data collectors used by the deployments in the environment."]
    pub data_collectors: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1DataCollectorConfig>>,
    >,
    #[serde(rename = "debugMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Debug mask that applies to all deployments in the environment."]
    pub debug_mask: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1DebugMask>>,
    #[serde(rename = "deployments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of deployments in the environment."]
    pub deployments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1DeploymentConfig>>,
    >,
    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature flags inherited from the organization and environment."]
    pub feature_flags:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "flowhooks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of flow hooks in the environment."]
    pub flowhooks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1FlowHookConfig>>,
    >,
    #[serde(rename = "keystores")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of keystores in the environment."]
    pub keystores: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1KeystoreConfig>>,
    >,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the environment configuration in the following format: `organizations/{org}/environments/{env}/configs/{config}`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "provider")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used by the Control plane to add context information to help detect the source of the document during diagnostics and debugging."]
    pub provider: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pubsubTopic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the PubSub topic for the environment."]
    pub pubsub_topic: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceReferences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of resource references in the environment."]
    pub resource_references: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ReferenceConfig>>,
    >,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of resource versions in the environment."]
    pub resources: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ResourceConfig>>,
    >,
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Revision ID of the environment configuration. The higher the value, the more recently the configuration was deployed."]
    pub revision_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sequenceNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DEPRECATED: Use revision_id."]
    pub sequence_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of target servers in the environment. Disabled target servers are not displayed."]
    pub targets: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1TargetServerConfig>>,
    >,
    #[serde(rename = "traceConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Trace configurations. Contains config for the environment and config overrides for specific API proxies."]
    pub trace_config:
        ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1RuntimeTraceConfig>>,
    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique ID for the environment configuration. The ID will only change if the environment is deleted and recreated."]
    pub uid: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "EnvironmentGroup configuration. An environment group is used to group one or more Apigee environments under a single host name."]
pub struct GoogleCloudApigeeV1EnvironmentGroup {
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which the environment group was created as milliseconds since epoch."]
    pub created_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hostnames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Host names for this environment group."]
    pub hostnames: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which the environment group was last updated as milliseconds since epoch."]
    pub last_modified_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the environment group."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. State of the environment group. Values other than ACTIVE means the resource is not ready to use."]
    pub state: ::std::option::Option<GoogleCloudApigeeV1EnvironmentGroupStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. State of the environment group. Values other than ACTIVE means the resource is not ready to use."]
pub enum GoogleCloudApigeeV1EnvironmentGroupStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Resource is in an unspecified state."]
    StateUnspecified,
    #[serde(rename = "CREATING")]
    #[doc = "Resource is being created."]
    Creating,
    #[serde(rename = "ACTIVE")]
    #[doc = "Resource is provisioned and ready to use."]
    Active,
    #[serde(rename = "DELETING")]
    #[doc = "The resource is being deleted."]
    Deleting,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "EnvironmentGroupAttachment is a resource which defines an attachment of an environment to an environment group."]
pub struct GoogleCloudApigeeV1EnvironmentGroupAttachment {
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which the environment group attachment was created as milliseconds since epoch."]
    pub created_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. ID of the attached environment."]
    pub environment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the environment group attachment."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "EnvironmentGroupConfig is a revisioned snapshot of an EnvironmentGroup and its associated routing rules."]
pub struct GoogleCloudApigeeV1EnvironmentGroupConfig {
    #[serde(rename = "hostnames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Host names for the environment group."]
    pub hostnames: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the environment group in the following format: `organizations/{org}/envgroups/{envgroup}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Revision id that defines the ordering of the EnvironmentGroupConfig resource. The higher the revision, the more recently the configuration was deployed."]
    pub revision_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "routingRules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ordered list of routing rules defining how traffic to this environment group's hostnames should be routed to different environments."]
    pub routing_rules:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1RoutingRule>>>,
    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique id for the environment group config that will only change if the environment group is deleted and recreated."]
    pub uid: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of an export job."]
pub struct GoogleCloudApigeeV1Export {
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time the export job was created."]
    pub created: ::std::option::Option<::std::string::String>,
    #[serde(rename = "datastoreName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the datastore that is the destination of the export job [datastore]"]
    pub datastore_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the export job."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Error is set when export fails"]
    pub error: ::std::option::Option<::std::string::String>,
    #[serde(rename = "executionTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Execution time for this export job. If the job is still in progress, it will be set to the amount of time that has elapsed since`created`, in seconds. Else, it will set to (`updated` - `created`), in seconds."]
    pub execution_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display name of the export job."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Self link of the export job. A URI that can be used to retrieve the status of an export job. Example: `/organizations/myorg/environments/myenv/analytics/exports/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd`"]
    pub _self: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Status of the export job. Valid values include `enqueued`, `running`, `completed`, and `failed`."]
    pub state: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time the export job was last updated."]
    pub updated: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request body for [CreateExportRequest]"]
pub struct GoogleCloudApigeeV1ExportRequest {
    #[serde(rename = "csvDelimiter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Delimiter used in the CSV file, if `outputFormat` is set to `csv`. Defaults to the `,` (comma) character. Supported delimiter characters include comma (`,`), pipe (`|`), and tab (`\\t`)."]
    pub csv_delimiter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "datastoreName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Name of the preconfigured datastore."]
    pub datastore_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dateRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Date range of the data to export."]
    pub date_range: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1DateRange>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Description of the export job."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Display name of the export job."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outputFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Output format of the export. Valid values include: `csv` or `json`. Defaults to `json`. Note: Configure the delimiter for CSV output using the `csvDelimiter` property."]
    pub output_format: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1FlowHook {
    #[serde(rename = "continueOnError")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Flag that specifies whether execution should continue if the flow hook throws an exception. Set to `true` to continue execution. Set to `false` to stop execution if the flow hook throws an exception.Defaults to `true`."]
    pub continue_on_error: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the flow hook."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "flowHookPoint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Where in the API call flow the flow hook is invoked. Must be one of `PreProxyFlowHook`, `PostProxyFlowHook`, `PreTargetFlowHook`, or `PostTargetFlowHook`."]
    pub flow_hook_point: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sharedFlow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shared flow attached to this flow hook, or empty if there is none attached."]
    pub shared_flow: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1FlowHookConfig {
    #[serde(rename = "continueOnError")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flag that specifies whether the flow should abort after an error in the flow hook. Defaults to `true` (continue on error)."]
    pub continue_on_error: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the flow hook in the following format: `organizations/{org}/environments/{env}/flowhooks/{point}`. Valid `point` values include: `PreProxyFlowHook`, `PostProxyFlowHook`, `PreTargetFlowHook`, and `PostTargetFlowHook`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sharedFlowName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the shared flow to invoke in the following format: `organizations/{org}/sharedflows/{sharedflow}`"]
    pub shared_flow_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for GetSyncAuthorization."]
pub struct GoogleCloudApigeeV1GetSyncAuthorizationRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1IngressConfig {
    #[serde(rename = "environmentGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of environment groups in the organization."]
    pub environment_groups: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1EnvironmentGroupConfig>>,
    >,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the resource in the following format: `organizations/{org}/deployedIngressConfig`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revisionCreateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time at which the IngressConfig revision was created."]
    pub revision_create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Revision id that defines the ordering on IngressConfig resources. The higher the revision, the more recently the configuration was deployed."]
    pub revision_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique id for the ingress config that will only change if the organization is deleted and recreated."]
    pub uid: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Apigee runtime instance."]
pub struct GoogleCloudApigeeV1Instance {
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time the instance was created in milliseconds since epoch."]
    pub created_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Description of the instance."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "diskEncryptionKeyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Customer Managed Encryption Key (CMEK) used for disk and volume encryption. Required for Apigee paid subscriptions only. Use the following format: `projects/([^/]+)/locations/([^/]+)/keyRings/([^/]+)/cryptoKeys/([^/]+)`"]
    pub disk_encryption_key_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Display name for the instance."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Hostname or IP address of the exposed Apigee endpoint used by clients to connect to the service."]
    pub host: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time the instance was last modified in milliseconds since epoch."]
    pub last_modified_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Compute Engine location where the instance resides."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Resource ID of the instance. Values must match the regular expression `^a-z{0,30}[a-z\\d]$`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "peeringCidrRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The size of the CIDR block range that will be reserved by the instance. If not specified, default to SLASH_16."]
    pub peering_cidr_range: ::std::option::Option<GoogleCloudApigeeV1InstancePeeringCidrRangeEnum>,
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Port number of the exposed Apigee endpoint."]
    pub port: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. State of the instance. Values other than ACTIVE means the resource is not ready to use."]
    pub state: ::std::option::Option<GoogleCloudApigeeV1InstanceStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. The size of the CIDR block range that will be reserved by the instance. If not specified, default to SLASH_16."]
pub enum GoogleCloudApigeeV1InstancePeeringCidrRangeEnum {
    #[serde(rename = "CIDR_RANGE_UNSPECIFIED")]
    #[doc = "Range not specified."]
    CidrRangeUnspecified,
    #[serde(rename = "SLASH_16")]
    #[doc = "The \"/16\" CIDR range."]
    Slash16,
    #[serde(rename = "SLASH_20")]
    #[doc = "The \"/20\" CIDR range."]
    Slash20,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. State of the instance. Values other than ACTIVE means the resource is not ready to use."]
pub enum GoogleCloudApigeeV1InstanceStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Resource is in an unspecified state."]
    StateUnspecified,
    #[serde(rename = "CREATING")]
    #[doc = "Resource is being created."]
    Creating,
    #[serde(rename = "ACTIVE")]
    #[doc = "Resource is provisioned and ready to use."]
    Active,
    #[serde(rename = "DELETING")]
    #[doc = "The resource is being deleted."]
    Deleting,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "InstanceAttachment represents the installation of an environment onto an instance."]
pub struct GoogleCloudApigeeV1InstanceAttachment {
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time the attachment was created in milliseconds since epoch."]
    pub created_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the attached environment."]
    pub environment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. ID of the attachment."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The status of a deployment as reported by a single instance."]
pub struct GoogleCloudApigeeV1InstanceDeploymentStatus {
    #[serde(rename = "deployedRevisions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Revisions currently deployed in MPs."]
    pub deployed_revisions: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRevision>,
        >,
    >,
    #[serde(rename = "deployedRoutes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current routes deployed in the ingress routing table. A route which is missing will appear in missing_routes."]
    pub deployed_routes: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRoute>,
        >,
    >,
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the instance reporting the status."]
    pub instance: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Revisions deployed in the MPs."]
pub struct GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRevision {
    #[serde(rename = "percentage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The percentage of MP replicas reporting this revision"]
    pub percentage: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The proxy revision reported as deployed."]
    pub revision: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A route deployed in the ingress routing table."]
pub struct GoogleCloudApigeeV1InstanceDeploymentStatusDeployedRoute {
    #[serde(rename = "basepath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The basepath in the routing table."]
    pub basepath: ::std::option::Option<::std::string::String>,
    #[serde(rename = "envgroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The envgroup where this route is installed."]
    pub envgroup: ::std::option::Option<::std::string::String>,
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The destination environment. This will be empty if the route is not yet reported."]
    pub environment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "percentage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The percentage of ingress replicas reporting this route."]
    pub percentage: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1KeyAliasReference {
    #[serde(rename = "aliasId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Alias ID. Must exist in the keystore referred to by the reference."]
    pub alias_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reference name in the following format: `organizations/{org}/environments/{env}/references/{reference}`"]
    pub reference: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A collection of key, value string pairs"]
pub struct GoogleCloudApigeeV1KeyValueMap {
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If `true` entry values will be encrypted."]
    pub encrypted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The id of the key value map."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Datastore for Certificates and Aliases."]
pub struct GoogleCloudApigeeV1Keystore {
    #[serde(rename = "aliases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Aliases in this keystore."]
    pub aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Resource ID for this keystore. Values must match the regular expression `[\\w[:space:]-.]{1,255}`."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1KeystoreConfig {
    #[serde(rename = "aliases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Aliases in the keystore."]
    pub aliases: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1AliasRevisionConfig>>,
    >,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name in the following format: `organizations/{org}/environments/{env}/keystores/{keystore}`"]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "the response for ListApiCategoriesRequest."]
pub struct GoogleCloudApigeeV1ListApiCategoriesResponse {
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details of categories."]
    pub data: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ApiCategoryData>>,
    >,
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID that can be used to find errors in the log files."]
    pub error_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the operation."]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID that can be used to find request details in the log files."]
    pub request_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the operation."]
    pub status: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1ListApiProductsResponse {
    #[serde(rename = "apiProduct")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Lists all API product names defined for an organization."]
    pub api_product:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ApiProduct>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1ListApiProxiesResponse {
    #[serde(rename = "proxies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxies:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ApiProxy>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1ListAppsResponse {
    #[serde(rename = "app")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub app: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1App>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for ListAsyncQueries."]
pub struct GoogleCloudApigeeV1ListAsyncQueriesResponse {
    #[serde(rename = "queries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The asynchronous queries belong to requested resource name."]
    pub queries:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1AsyncQuery>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This message encapsulates a list of custom report definitions"]
pub struct GoogleCloudApigeeV1ListCustomReportsResponse {
    #[serde(rename = "qualifier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub qualifier:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1CustomReport>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ListDataCollectors."]
pub struct GoogleCloudApigeeV1ListDataCollectorsResponse {
    #[serde(rename = "dataCollectors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Data collectors in the specified organization."]
    pub data_collectors:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1DataCollector>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Page token that you can include in a ListDataCollectors request to retrieve the next page. If omitted, no subsequent pages exist."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for ListDatastores"]
pub struct GoogleCloudApigeeV1ListDatastoresResponse {
    #[serde(rename = "datastores")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of datastores"]
    pub datastores:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Datastore>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1ListDebugSessionsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Page token that you can include in a ListDebugSessionsRequest to retrieve the next page. If omitted, no subsequent pages exist."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sessions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Session info that includes debug session ID and the first transaction creation timestamp."]
    pub sessions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Session>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1ListDeploymentsResponse {
    #[serde(rename = "deployments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of deployments."]
    pub deployments:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Deployment>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1ListDeveloperAppsResponse {
    #[serde(rename = "app")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of developer apps and their credentials."]
    pub app:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1DeveloperApp>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ListEnvironmentGroupAttachments."]
pub struct GoogleCloudApigeeV1ListEnvironmentGroupAttachmentsResponse {
    #[serde(rename = "environmentGroupAttachments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "EnvironmentGroupAttachments for the specified environment group."]
    pub environment_group_attachments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1EnvironmentGroupAttachment>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Page token that you can include in a ListEnvironmentGroupAttachments request to retrieve the next page. If omitted, no subsequent pages exist."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ListEnvironmentGroups."]
pub struct GoogleCloudApigeeV1ListEnvironmentGroupsResponse {
    #[serde(rename = "environmentGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "EnvironmentGroups in the specified organization."]
    pub environment_groups: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1EnvironmentGroup>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Page token that you can include in a ListEnvironmentGroups request to retrieve the next page. If omitted, no subsequent pages exist."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ListEnvironmentResources"]
pub struct GoogleCloudApigeeV1ListEnvironmentResourcesResponse {
    #[serde(rename = "resourceFile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of resources files."]
    pub resource_file:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ResourceFile>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for ListExports"]
pub struct GoogleCloudApigeeV1ListExportsResponse {
    #[serde(rename = "exports")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details of the export jobs."]
    pub exports:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Export>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1ListHybridIssuersResponse {
    #[serde(rename = "issuers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Lists of hybrid services and its trusted issuer email ids."]
    pub issuers: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ServiceIssuersMapping>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ListInstanceAttachments."]
pub struct GoogleCloudApigeeV1ListInstanceAttachmentsResponse {
    #[serde(rename = "attachments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Attachments for the instance."]
    pub attachments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1InstanceAttachment>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Page token that you can include in a ListInstanceAttachments request to retrieve the next page of content. If omitted, no subsequent pages exist."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ListInstances."]
pub struct GoogleCloudApigeeV1ListInstancesResponse {
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Instances in the specified organization."]
    pub instances:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Instance>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Page token that you can include in a ListInstance request to retrieve the next page of content. If omitted, no subsequent pages exist."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ListNatAddresses."]
pub struct GoogleCloudApigeeV1ListNatAddressesResponse {
    #[serde(rename = "natAddresses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of NAT Addresses for the instance."]
    pub nat_addresses:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1NatAddress>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Page token that you can include in a ListNatAddresses request to retrieve the next page of content. If omitted, no subsequent pages exist."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1ListOfDevelopersResponse {
    #[serde(rename = "developer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of developers."]
    pub developer:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Developer>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1ListOrganizationsResponse {
    #[serde(rename = "organizations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of Apigee organizations and associated GCP projects."]
    pub organizations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1OrganizationProjectMapping>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1ListSharedFlowsResponse {
    #[serde(rename = "sharedFlows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shared_flows:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1SharedFlow>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This message type encapsulates additional information about query execution."]
pub struct GoogleCloudApigeeV1Metadata {
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of error messages as strings."]
    pub errors: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "notices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of additional information such as data source, if result was truncated etc. E.g \"notices\": [ \"Source:Postgres\", \"PG Host:uappg0rw.e2e.apigeeks.net\", \"query served by:4b64601e-40de-4eb1-bfb9-eeee7ac929ed\", \"Table used: edge.api.uapgroup2.agg_api\" ]"]
    pub notices: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This message type encapsulates the metric data point. Example: { \"name\": \"sum(message_count)\", \"values\" : [ { \"timestamp\": 1549004400000, \"value\": \"39.0\" }, { \"timestamp\" : 1548997200000, \"value\" : \"0.0\" } ] } or { \"name\": \"sum(message_count)\", \"values\" : [\"39.0\"] }"]
pub struct GoogleCloudApigeeV1Metric {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains the metric name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of metric values. Possible value format: \"values\":[\"39.0\"] or \"values\":[ { \"value\": \"39.0\", \"timestamp\": 1232434354} ]"]
    pub values: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Apigee NAT(network address translation) address. A NAT address is a static external IP address used for Internet egress traffic."]
pub struct GoogleCloudApigeeV1NatAddress {
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The static IPV4 address."]
    pub ip_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Resource ID of the NAT address."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. State of the nat address."]
    pub state: ::std::option::Option<GoogleCloudApigeeV1NatAddressStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. State of the nat address."]
pub enum GoogleCloudApigeeV1NatAddressStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The resource is in an unspecified state."]
    StateUnspecified,
    #[serde(rename = "CREATING")]
    #[doc = "The NAT address is being created."]
    Creating,
    #[serde(rename = "RESERVED")]
    #[doc = "The NAT address is reserved but not yet used for Internet egress."]
    Reserved,
    #[serde(rename = "ACTIVE")]
    #[doc = "The NAT address is active and used for Internet egress."]
    Active,
    #[serde(rename = "DELETING")]
    #[doc = "The NAT address is being deleted."]
    Deleting,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Operation represents the pairing of REST resource path and the actions (verbs) allowed on the resource path."]
pub struct GoogleCloudApigeeV1Operation {
    #[serde(rename = "methods")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "methods refers to the REST verbs as in https://www.w3.org/Protocols/rfc2616/rfc2616-sec9.html. When none specified, all verb types are allowed."]
    pub methods: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. resource represents REST resource path associated with the proxy/remote service."]
    pub resource: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "OperationConfig binds the resources in a proxy or remote service with the allowed REST methods and its associated quota enforcement."]
pub struct GoogleCloudApigeeV1OperationConfig {
    #[serde(rename = "apiSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. API proxy or remote service name with which the resources, methods, and quota are associated."]
    pub api_source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom attributes associated with the operation."]
    pub attributes:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Attribute>>>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of resource/method pairs for the proxy/remote service, upon which quota will applied. **Note**: Currently, you can specify only a single resource/method pair. The call will fail if more than one resource/method pair is provided."]
    pub operations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Operation>>>,
    #[serde(rename = "quota")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Quota parameters to be enforced for the resources, methods, api_source combination. If none are specified, quota enforcement will not be done."]
    pub quota: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1Quota>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List of operation configuration details associated with Apigee API proxies or remote services. Remote services are non-Apigee proxies, such as Istio-Envoy."]
pub struct GoogleCloudApigeeV1OperationGroup {
    #[serde(rename = "operationConfigType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flag that specifes whether the configuration is for Apigee API proxy or a remote service. Valid values are `proxy` or `remoteservice`. Defaults to `proxy`. Set to `proxy` when Apigee API proxies are associated with the API product. Set to `remoteservice` when non-Apigee proxies like Istio-Envoy are associated with the API product."]
    pub operation_config_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. List of operation configurations for either Apigee API proxies or other remote services that are associated with this API product."]
    pub operation_configs: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1OperationConfig>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata describing an Operation."]
pub struct GoogleCloudApigeeV1OperationMetadata {
    #[serde(rename = "operationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub operation_type:
        ::std::option::Option<GoogleCloudApigeeV1OperationMetadataOperationTypeEnum>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub state: ::std::option::Option<GoogleCloudApigeeV1OperationMetadataStateEnum>,
    #[serde(rename = "targetResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the resource for which the operation is operating on."]
    pub target_resource_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GoogleCloudApigeeV1OperationMetadataOperationTypeEnum {
    #[serde(rename = "OPERATION_TYPE_UNSPECIFIED")]
    #[doc = ""]
    OperationTypeUnspecified,
    #[serde(rename = "INSERT")]
    #[doc = ""]
    Insert,
    #[serde(rename = "DELETE")]
    #[doc = ""]
    Delete,
    #[serde(rename = "UPDATE")]
    #[doc = ""]
    Update,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GoogleCloudApigeeV1OperationMetadataStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = ""]
    StateUnspecified,
    #[serde(rename = "NOT_STARTED")]
    #[doc = ""]
    NotStarted,
    #[serde(rename = "IN_PROGRESS")]
    #[doc = ""]
    InProgress,
    #[serde(rename = "FINISHED")]
    #[doc = ""]
    Finished,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1OptimizedStats {
    #[serde(rename = "Response")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field wraps the stats response for Js Optimized Scenario with a Response key. E.g. { \"Response\": { \"TimeUnit\": [], \"metaData\": { \"errors\": [], \"notices\": [ \"Source:Postgres\", \"Table used: edge.api.aaxgroup001.agg_api\", \"PG Host:ruappg08-ro.production.apigeeks.net\", \"query served by:80c4ebca-6a10-4a2e-8faf-c60c1ee306ca\" ] }, \"resultTruncated\": false, \"stats\": { \"data\": [ { \"identifier\": { \"names\": [ \"apiproxy\" ], \"values\": [ \"sirjee\" ] }, \"metric\": [ { \"env\": \"prod\", \"name\": \"sum(message_count)\", \"values\": [ 36.0 ] }, { \"env\": \"prod\", \"name\": \"sum(is_error)\", \"values\": [ 36.0 ] } ] } ] } } }"]
    pub response:
        ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1OptimizedStatsResponse>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This message type encapsulates a data node as represented below: { \"identifier\": { \"names\": [ \"apiproxy\" ], \"values\": [ \"sirjee\" ] }, \"metric\": [ { \"env\": \"prod\", \"name\": \"sum(message_count)\", \"values\": [ 36.0 ] } ] } OR { \"env\": \"prod\", \"name\": \"sum(message_count)\", \"values\": [ 36.0 ] } Depending on whether a dimension is present in the query or not the data node type can be a simple metric value or dimension identifier with list of metrics."]
pub struct GoogleCloudApigeeV1OptimizedStatsNode {
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This message type encapsulates a response format for Js Optimized Scenario."]
pub struct GoogleCloudApigeeV1OptimizedStatsResponse {
    #[serde(rename = "TimeUnit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains a list of time unit values. Time unit refers to an epoch timestamp value."]
    pub time_unit: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "metaData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains metadata information about the query executed"]
    pub meta_data: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1Metadata>>,
    #[serde(rename = "resultTruncated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This ia a boolean field to indicate if the results were truncated based on the limit parameter."]
    pub result_truncated: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "stats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains a stats results."]
    pub stats: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1OptimizedStatsNode>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1Organization {
    #[serde(rename = "analyticsRegion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Primary GCP region for analytics data storage. For valid values, see [Create an Apigee organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org)."]
    pub analytics_region: ::std::option::Option<::std::string::String>,
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Not used by Apigee."]
    pub attributes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "authorizedNetwork")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Compute Engine network used for Service Networking to be peered with Apigee runtime instances. See [Getting started with the Service Networking API](https://cloud.google.com/service-infrastructure/docs/service-networking/getting-started). Valid only when [RuntimeType](#RuntimeType) is set to `CLOUD`. The value can be updated only when there are no runtime instances. For example: `default`. Apigee also supports shared VPC (that is, the host network project is not the same as the one that is peering with Apigee). See [Shared VPC overview](https://cloud.google.com/vpc/docs/shared-vpc). To use a shared VPC network, use the following format: `projects/{host-project-id}/{region}/networks/{network-name}`. For example: `projects/my-sharedvpc-host/global/networks/mynetwork` **Note:** Not supported for Apigee hybrid."]
    pub authorized_network: ::std::option::Option<::std::string::String>,
    #[serde(rename = "billingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Billing type of the Apigee organization. See [Apigee pricing](https://cloud.google.com/apigee/pricing)."]
    pub billing_type: ::std::option::Option<GoogleCloudApigeeV1OrganizationBillingTypeEnum>,
    #[serde(rename = "caCertificate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Base64-encoded public certificate for the root CA of the Apigee organization. Valid only when [RuntimeType](#RuntimeType) is `CLOUD`."]
    pub ca_certificate: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time that the Apigee organization was created in milliseconds since epoch."]
    pub created_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customerName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Not used by Apigee."]
    pub customer_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the Apigee organization."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "environments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. List of environments in the Apigee organization."]
    pub environments: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time that the Apigee organization was last modified in milliseconds since epoch."]
    pub last_modified_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Name of the Apigee organization."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Project ID associated with the Apigee organization."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Properties defined in the Apigee organization profile."]
    pub properties: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1Properties>>,
    #[serde(rename = "runtimeDatabaseEncryptionKeyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud KMS key name used for encrypting the data that is stored and replicated across runtime instances. Update is not allowed after the organization is created. If not specified, a Google-Managed encryption key will be used. Valid only when [RuntimeType](#RuntimeType) is `CLOUD`. For example: \"projects/foo/locations/us/keyRings/bar/cryptoKeys/baz\". **Note:** Not supported for Apigee hybrid."]
    pub runtime_database_encryption_key_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "runtimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Runtime type of the Apigee organization based on the Apigee subscription purchased."]
    pub runtime_type: ::std::option::Option<GoogleCloudApigeeV1OrganizationRuntimeTypeEnum>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. State of the organization. Values other than ACTIVE means the resource is not ready to use."]
    pub state: ::std::option::Option<GoogleCloudApigeeV1OrganizationStateEnum>,
    #[serde(rename = "subscriptionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. DEPRECATED: This will eventually be replaced by BillingType. Subscription type of the Apigee organization. Valid values include trial (free, limited, and for evaluation purposes only) or paid (full subscription has been purchased). See [Apigee pricing](https://cloud.google.com/apigee/pricing/)."]
    pub subscription_type:
        ::std::option::Option<GoogleCloudApigeeV1OrganizationSubscriptionTypeEnum>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Not used by Apigee."]
    pub _type: ::std::option::Option<GoogleCloudApigeeV1OrganizationTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Billing type of the Apigee organization. See [Apigee pricing](https://cloud.google.com/apigee/pricing)."]
pub enum GoogleCloudApigeeV1OrganizationBillingTypeEnum {
    #[serde(rename = "BILLING_TYPE_UNSPECIFIED")]
    #[doc = "Billing type not specified."]
    BillingTypeUnspecified,
    #[serde(rename = "SUBSCRIPTION")]
    #[doc = "A pre-paid subscription to Apigee."]
    Subscription,
    #[serde(rename = "EVALUATION")]
    #[doc = "Free and limited access to Apigee for evaluation purposes only. only."]
    Evaluation,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Runtime type of the Apigee organization based on the Apigee subscription purchased."]
pub enum GoogleCloudApigeeV1OrganizationRuntimeTypeEnum {
    #[serde(rename = "RUNTIME_TYPE_UNSPECIFIED")]
    #[doc = "Runtime type not specified."]
    RuntimeTypeUnspecified,
    #[serde(rename = "CLOUD")]
    #[doc = "Google-managed Apigee runtime."]
    Cloud,
    #[serde(rename = "HYBRID")]
    #[doc = "User-managed Apigee hybrid runtime."]
    Hybrid,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. State of the organization. Values other than ACTIVE means the resource is not ready to use."]
pub enum GoogleCloudApigeeV1OrganizationStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Resource is in an unspecified state."]
    StateUnspecified,
    #[serde(rename = "CREATING")]
    #[doc = "Resource is being created."]
    Creating,
    #[serde(rename = "ACTIVE")]
    #[doc = "Resource is provisioned and ready to use."]
    Active,
    #[serde(rename = "DELETING")]
    #[doc = "The resource is being deleted."]
    Deleting,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. DEPRECATED: This will eventually be replaced by BillingType. Subscription type of the Apigee organization. Valid values include trial (free, limited, and for evaluation purposes only) or paid (full subscription has been purchased). See [Apigee pricing](https://cloud.google.com/apigee/pricing/)."]
pub enum GoogleCloudApigeeV1OrganizationSubscriptionTypeEnum {
    #[serde(rename = "SUBSCRIPTION_TYPE_UNSPECIFIED")]
    #[doc = "Subscription type not specified."]
    SubscriptionTypeUnspecified,
    #[serde(rename = "PAID")]
    #[doc = "Full subscription to Apigee has been purchased."]
    Paid,
    #[serde(rename = "TRIAL")]
    #[doc = "Subscription to Apigee is free, limited, and used for evaluation purposes only."]
    Trial,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Not used by Apigee."]
pub enum GoogleCloudApigeeV1OrganizationTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Subscription type not specified."]
    TypeUnspecified,
    #[serde(rename = "TYPE_TRIAL")]
    #[doc = "Subscription to Apigee is free, limited, and used for evaluation purposes only."]
    TypeTrial,
    #[serde(rename = "TYPE_PAID")]
    #[doc = "Full subscription to Apigee has been purchased. See [Apigee pricing](https://cloud.google.com/apigee/pricing/)."]
    TypePaid,
    #[serde(rename = "TYPE_INTERNAL")]
    #[doc = "For internal users only."]
    TypeInternal,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1OrganizationProjectMapping {
    #[serde(rename = "organization")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the Apigee organization."]
    pub organization: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of GCP projects associated with the Apigee organization."]
    pub project_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1PodStatus {
    #[serde(rename = "appVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Version of the application running in the pod."]
    pub app_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deploymentStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the deployment. Valid values include: - `deployed`: Successful. - `error` : Failed. - `pending` : Pod has not yet reported on the deployment."]
    pub deployment_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deploymentStatusTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time the deployment status was reported in milliseconds since epoch."]
    pub deployment_status_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deploymentTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time the proxy was deployed in milliseconds since epoch."]
    pub deployment_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "podName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the pod which is reporting the status."]
    pub pod_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "podStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Overall status of the pod (not this specific deployment). Valid values include: - `active`: Up to date. - `stale` : Recently out of date. Pods that have not reported status in a long time are excluded from the output."]
    pub pod_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "podStatusTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time the pod status was reported in milliseconds since epoch."]
    pub pod_status_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Code associated with the deployment status."]
    pub status_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statusCodeDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human-readable message associated with the status code."]
    pub status_code_details: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Point is a group of information collected by runtime plane at critical points of the message flow of the processed API request. This is a list of supported point IDs, categorized to three major buckets. For each category, debug points that we are currently supporting are listed below: - Flow status debug points: StateChange FlowInfo Condition Execution DebugMask Error - Flow control debug points: FlowCallout Paused Resumed FlowReturn BreakFlow Error - Runtime debug points: ScriptExecutor FlowCalloutStepDefinition CustomTarget StepDefinition Oauth2ServicePoint RaiseFault NodeJS The detail information of the given debug point is stored in a list of results."]
pub struct GoogleCloudApigeeV1Point {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of a step in the transaction."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of results extracted from a given debug point."]
    pub results:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Result>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message for compatibility with legacy Edge specification for Java Properties object in JSON."]
pub struct GoogleCloudApigeeV1Properties {
    #[serde(rename = "property")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of all properties in the object"]
    pub property:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Property>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single property entry in the Properties message."]
pub struct GoogleCloudApigeeV1Property {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The property key"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The property value"]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for ProvisionOrganization."]
pub struct GoogleCloudApigeeV1ProvisionOrganizationRequest {
    #[serde(rename = "analyticsRegion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Primary Cloud Platform region for analytics data storage. For valid values, see [Create an organization](https://cloud.google.com/apigee/docs/hybrid/latest/precog-provision). Defaults to `us-west1`."]
    pub analytics_region: ::std::option::Option<::std::string::String>,
    #[serde(rename = "authorizedNetwork")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the customer project's VPC network. If provided, the network needs to be peered through Service Networking. If none is provided, the organization will have access only to the public internet."]
    pub authorized_network: ::std::option::Option<::std::string::String>,
    #[serde(rename = "runtimeLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Platform location for the runtime instance. Defaults to `us-west1-a`."]
    pub runtime_location: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1Query {
    #[serde(rename = "csvDelimiter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Delimiter used in the CSV file, if `outputFormat` is set to `csv`. Defaults to the `,` (comma) character. Supported delimiter characters include comma (`,`), pipe (`|`), and tab (`\\t`)."]
    pub csv_delimiter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of dimensions. https://docs.apigee.com/api-platform/analytics/analytics-reference#dimensions"]
    pub dimensions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "envgroupHostname")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hostname needs to be specified if query intends to run at host level. This field is only allowed when query is submitted by CreateHostAsyncQuery where analytics data will be grouped by organization and hostname."]
    pub envgroup_hostname: ::std::option::Option<::std::string::String>,
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Boolean expression that can be used to filter data. Filter expressions can be combined using AND/OR terms and should be fully parenthesized to avoid ambiguity. See Analytics metrics, dimensions, and filters reference https://docs.apigee.com/api-platform/analytics/analytics-reference for more information on the fields available to filter on. For more information on the tokens that you use to build filter expressions, see Filter expression syntax. https://docs.apigee.com/api-platform/analytics/asynch-reports-api#filter-expression-syntax"]
    pub filter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "groupByTimeUnit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time unit used to group the result set. Valid values include: second, minute, hour, day, week, or month. If a query includes groupByTimeUnit, then the result is an aggregation based on the specified time unit and the resultant timestamp does not include milliseconds precision. If a query omits groupByTimeUnit, then the resultant timestamp includes milliseconds precision."]
    pub group_by_time_unit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum number of rows that can be returned in the result."]
    pub limit: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of Metrics."]
    pub metrics:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1QueryMetric>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Asynchronous Query Name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outputFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Valid values include: `csv` or `json`. Defaults to `json`. Note: Configure the delimiter for CSV output using the csvDelimiter property."]
    pub output_format: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reportDefinitionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Asynchronous Report ID."]
    pub report_definition_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Time range for the query. Can use the following predefined strings to specify the time range: `last60minutes` `last24hours` `last7days` Or, specify the timeRange as a structure describing start and end timestamps in the ISO format: yyyy-mm-ddThh:mm:ssZ. Example: \"timeRange\": { \"start\": \"2018-07-29T00:13:00Z\", \"end\": \"2018-08-01T00:18:00Z\" }"]
    pub time_range: ::std::option::Option<::serde_json::Value>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1QueryMetadata {
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Dimensions of the AsyncQuery."]
    pub dimensions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "endTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "End timestamp of the query range."]
    pub end_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metrics of the AsyncQuery. Example: [\"name:message_count,func:sum,alias:sum_message_count\"]"]
    pub metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "outputFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output format."]
    pub output_format: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Start timestamp of the query range."]
    pub start_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Query GroupBy time unit."]
    pub time_unit: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "More info about Metric: https://docs.apigee.com/api-platform/analytics/analytics-reference#metrics"]
pub struct GoogleCloudApigeeV1QueryMetric {
    #[serde(rename = "alias")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Alias for the metric. Alias will be used to replace metric name in query results."]
    pub alias: ::std::option::Option<::std::string::String>,
    #[serde(rename = "function")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Aggregation function: avg, min, max, or sum."]
    pub function: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Metric name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operator")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "One of `+`, `-`, `/`, `%`, `*`."]
    pub operator: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Operand value should be provided when operator is set."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Quota contains the essential parameters needed that can be applied on a proxy/remote service, resources and methods combination associated with this API product. While setting of Quota is optional, setting it prevents requests from exceeding the provisioned parameters."]
pub struct GoogleCloudApigeeV1Quota {
    #[serde(rename = "interval")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Time interval over which the number of request messages is calculated."]
    pub interval: ::std::option::Option<::std::string::String>,
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Upper limit allowed for the time interval and time unit specified. Requests exceeding this limit will be rejected."]
    pub limit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time unit defined for the `interval`. Valid values include `minute`, `hour`, `day`, or `month`. If `limit` and `interval` are valid, the default value is `hour`; otherwise, the default is null."]
    pub time_unit: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Reference configuration. References must refer to a keystore that also exists in the parent environment."]
pub struct GoogleCloudApigeeV1Reference {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A human-readable description of this reference."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The resource id of this reference. Values must match the regular expression [\\w\\s\\-.]+."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "refers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The id of the resource to which this reference refers. Must be the id of a resource that exists in the parent environment and is of the given resource_type."]
    pub refers: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of resource referred to by this reference. Valid values are 'KeyStore' or 'TrustStore'."]
    pub resource_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1ReferenceConfig {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the reference in the following format: `organizations/{org}/environments/{env}/references/{reference}`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the referenced resource in the following format: `organizations/{org}/environments/{env}/keystores/{keystore}` Only references to keystore resources are supported."]
    pub resource_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for ReportInstanceStatus."]
pub struct GoogleCloudApigeeV1ReportInstanceStatusRequest {
    #[serde(rename = "instanceUid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique ID for the instance which is guaranteed to be unique in case the user installs multiple hybrid runtimes with the same instance ID."]
    pub instance_uid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reportTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time the report was generated in the runtime. Used to prevent an old status from overwriting a newer one. An instance should space out it's status reports so that clock skew does not play a factor."]
    pub report_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status for config resources"]
    pub resources: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ResourceStatus>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Placeholder for future enhancements to status reporting protocol"]
pub struct GoogleCloudApigeeV1ReportInstanceStatusResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1ReportProperty {
    #[serde(rename = "property")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "name of the property"]
    pub property: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "property values"]
    pub value:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Attribute>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1ResourceConfig {
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location of the resource as a URI."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name in the following format: `organizations/{org}/environments/{env}/resourcefiles/{type}/{file}/revisions/{rev}` Only environment-scoped resource files are supported."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata about a resource file."]
pub struct GoogleCloudApigeeV1ResourceFile {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the resource file."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource file type. {{ resource_file_type }}"]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "List of resource files."]
pub struct GoogleCloudApigeeV1ResourceFiles {
    #[serde(rename = "resourceFile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of resource files."]
    pub resource_file:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1ResourceFile>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The status of a resource loaded in the runtime."]
pub struct GoogleCloudApigeeV1ResourceStatus {
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name. Currently only two resources are supported: EnvironmentGroup - organizations/{org}/envgroups/{envgroup} EnvironmentConfig - organizations/{org}/environments/{environment}/deployedConfig"]
    pub resource: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revisions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Revisions of the resource currently deployed in the instance."]
    pub revisions: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1RevisionStatus>>,
    >,
    #[serde(rename = "totalReplicas")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of replicas that should have this resource."]
    pub total_replicas: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The uid of the resource. In the unexpected case that the instance has multiple uids for the same name, they should be reported under separate ResourceStatuses."]
    pub uid: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Result is short for \"action result\", could be different types identified by \"action_result\" field. Supported types: 1. DebugInfo : generic debug info collected by runtime recorded as a list of properties. For example, the contents could be virtual host info, state change result, or execution metadata. Required fields : properties, timestamp 2. RequestMessage: information of a http request. Contains headers, request URI and http methods type.Required fields : headers, uri, verb 3. ResponseMessage: information of a http response. Contains headers, reason phrase and http status code. Required fields : headers, reasonPhrase, statusCode 4. ErrorMessage: information of a http error message. Contains detail error message, reason phrase and status code. Required fields : content, headers, reasonPhrase, statusCode 5. VariableAccess: a list of variable access actions, can be Get, Set and Remove. Required fields : accessList"]
pub struct GoogleCloudApigeeV1Result {
    #[serde(rename = "ActionResult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the action result. Can be one of the five: DebugInfo, RequestMessage, ResponseMessage, ErrorMessage, VariableAccess"]
    pub action_result: ::std::option::Option<::std::string::String>,
    #[serde(rename = "accessList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of variable access actions agaist the api proxy. Supported values: Get, Set, Remove."]
    pub access_list:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Access>>>,
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error message content. for example, \"content\" : \"{\\\"fault\\\":{\\\"faultstring\\\":\\\"API timed out\\\",\\\"detail\\\":{\\\"errorcode\\\":\\\"flow.APITimedOut\\\"}}}\""]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of HTTP headers. for example, '\"headers\" : [ { \"name\" : \"Content-Length\", \"value\" : \"83\" }, { \"name\" : \"Content-Type\", \"value\" : \"application/json\" } ]'"]
    pub headers:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Property>>>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name value pairs used for DebugInfo ActionResult."]
    pub properties: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1Properties>>,
    #[serde(rename = "reasonPhrase")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTTP response phrase"]
    pub reason_phrase: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTTP response code"]
    pub status_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp of when the result is recorded. Its format is dd-mm-yy hh:mm:ss:xxx. For example, `\"timestamp\" : \"12-08-19 00:31:59:960\"`"]
    pub timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uRI")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The relative path of the api proxy. for example, `\"uRI\" : \"/iloveapis\"`"]
    pub u_ri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "verb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTTP method verb"]
    pub verb: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The status of a specific resource revision."]
pub struct GoogleCloudApigeeV1RevisionStatus {
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Errors reported when attempting to load this revision."]
    pub errors:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1UpdateError>>>,
    #[serde(rename = "jsonSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The json content of the resource revision."]
    pub json_spec: ::std::option::Option<::std::string::String>,
    #[serde(rename = "replicas")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of replicas that have successfully loaded this revision."]
    pub replicas: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The revision of the resource."]
    pub revision_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1RoutingRule {
    #[serde(rename = "basepath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI path prefix used to route to the specified environment. May contain one or more wildcards. For example, path segments consisting of a single `*` character will match any string."]
    pub basepath: ::std::option::Option<::std::string::String>,
    #[serde(rename = "envGroupRevision")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The env group config revision_id when this rule was added or last updated. This value is set when the rule is created and will only update if the the environment_id changes. It is used to determine if the runtime is up to date with respect to this rule. This field is omitted from the IngressConfig unless the GetDeployedIngressConfig API is called with view=FULL."]
    pub env_group_revision: ::std::option::Option<::std::string::String>,
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of an environment bound to the environment group in the following format: `organizations/{org}/environments/{env}`."]
    pub environment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "receiver")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the proxy revision that is receiving this basepath in the following format: `organizations/{org}/apis/{api}/revisions/{rev}`. This field is omitted from the IngressConfig unless the GetDeployedIngressConfig API is called with view=FULL."]
    pub receiver: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unix timestamp when this rule was updated. This is updated whenever env_group_revision is updated. This field is omitted from the IngressConfig unless the GetDeployedIngressConfig API is called with view=FULL."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "NEXT ID: 8 RuntimeTraceConfig defines the configurations for distributed trace in an environment."]
pub struct GoogleCloudApigeeV1RuntimeTraceConfig {
    #[serde(rename = "endpoint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Endpoint of the exporter."]
    pub endpoint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "exporter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Exporter that is used to view the distributed trace captured using OpenCensus. An exporter sends traces to any backend that is capable of consuming them. Recorded spans can be exported by registered exporters."]
    pub exporter: ::std::option::Option<GoogleCloudApigeeV1RuntimeTraceConfigExporterEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the trace config in the following format: `organizations/{org}/environment/{env}/traceConfig`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "overrides")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of trace configuration overrides for spicific API proxies."]
    pub overrides: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1RuntimeTraceConfigOverride>>,
    >,
    #[serde(rename = "revisionCreateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp that the revision was created or updated."]
    pub revision_create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Revision number which can be used by the runtime to detect if the trace config has changed between two versions."]
    pub revision_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "samplingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Trace configuration for all API proxies in an environment."]
    pub sampling_config:
        ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1RuntimeTraceSamplingConfig>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Exporter that is used to view the distributed trace captured using OpenCensus. An exporter sends traces to any backend that is capable of consuming them. Recorded spans can be exported by registered exporters."]
pub enum GoogleCloudApigeeV1RuntimeTraceConfigExporterEnum {
    #[serde(rename = "EXPORTER_UNSPECIFIED")]
    #[doc = "Exporter unspecified"]
    ExporterUnspecified,
    #[serde(rename = "JAEGER")]
    #[doc = "Jaeger exporter"]
    Jaeger,
    #[serde(rename = "CLOUD_TRACE")]
    #[doc = "Cloudtrace exporter"]
    CloudTrace,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "NEXT ID: 7 Trace configuration override for a specific API proxy in an environment."]
pub struct GoogleCloudApigeeV1RuntimeTraceConfigOverride {
    #[serde(rename = "apiProxy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the API proxy that will have its trace configuration overridden following format: `organizations/{org}/apis/{api}`"]
    pub api_proxy: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the trace config override in the following format: `organizations/{org}/environment/{env}/traceConfig/overrides/{override}`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revisionCreateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp that the revision was created or updated."]
    pub revision_create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Revision number which can be used by the runtime to detect if the trace config override has changed between two versions."]
    pub revision_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "samplingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Trace configuration override for a specific API proxy in an environment."]
    pub sampling_config:
        ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1RuntimeTraceSamplingConfig>>,
    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique ID for the configuration override. The ID will only change if the override is deleted and recreated. Corresponds to name's \"override\" field."]
    pub uid: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "NEXT ID: 3 RuntimeTraceSamplingConfig represents the detail settings of distributed tracing. Only the fields that are defined in the distributed trace configuration can be overridden using the distribute trace configuration override APIs."]
pub struct GoogleCloudApigeeV1RuntimeTraceSamplingConfig {
    #[serde(rename = "sampler")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sampler of distributed tracing. OFF is the default value."]
    pub sampler: ::std::option::Option<GoogleCloudApigeeV1RuntimeTraceSamplingConfigSamplerEnum>,
    #[serde(rename = "samplingRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Field sampling rate. This value is only applicable when using the PROBABILITY sampler. The supported values are > 0 and <= 0.5."]
    pub sampling_rate: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Sampler of distributed tracing. OFF is the default value."]
pub enum GoogleCloudApigeeV1RuntimeTraceSamplingConfigSamplerEnum {
    #[serde(rename = "SAMPLER_UNSPECIFIED")]
    #[doc = "Sampler unspecified."]
    SamplerUnspecified,
    #[serde(rename = "OFF")]
    #[doc = "OFF means distributed trace is disabled, or the sampling probability is 0."]
    Off,
    #[serde(rename = "PROBABILITY")]
    #[doc = "PROBABILITY means traces are captured on a probability that defined by sampling_rate. The sampling rate is limited to 0 to 0.5 when this is set."]
    Probability,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for Schema call"]
pub struct GoogleCloudApigeeV1Schema {
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of schema fiels grouped as dimensions."]
    pub dimensions: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1SchemaSchemaElement>>,
    >,
    #[serde(rename = "meta")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional metadata associated with schema. This is a legacy field and usually consists of an empty array of strings."]
    pub meta: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of schema fields grouped as dimensions. These are fields that can be used with an aggregate function such as sum, avg, min, max."]
    pub metrics: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1SchemaSchemaElement>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message type for the schema element"]
pub struct GoogleCloudApigeeV1SchemaSchemaElement {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the field"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Property of the schema field E.g. { \"createTime\": \"2016-02-26T10:23:09.592Z\", \"custom\": \"false\", \"type\": \"string\" }"]
    pub properties:
        ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1SchemaSchemaProperty>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message type for schema property"]
pub struct GoogleCloudApigeeV1SchemaSchemaProperty {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creation time of the field"]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "custom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom is a flag signifying if the field was provided as part of the standard dataset or a custom field created by the customer"]
    pub custom: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Data type of the field."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1ServiceIssuersMapping {
    #[serde(rename = "emailIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of trusted issuer email ids."]
    pub email_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "String indicating the Apigee service name."]
    pub service: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Session carries the debug session id and its creation time."]
pub struct GoogleCloudApigeeV1Session {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The debug session ID."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestampMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The first transaction creation timestamp in millisecond, recoreded by UAP."]
    pub timestamp_ms: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The metadata describing a shared flow"]
pub struct GoogleCloudApigeeV1SharedFlow {
    #[serde(rename = "latestRevisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of the most recently created revision for this shared flow."]
    pub latest_revision_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metaData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata describing the shared flow."]
    pub meta_data: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1EntityMetadata>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the shared flow."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of revisions of this shared flow."]
    pub revision: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The metadata describing a shared flow revision."]
pub struct GoogleCloudApigeeV1SharedFlowRevision {
    #[serde(rename = "configurationVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version of the configuration schema to which this shared flow conforms. The only supported value currently is majorVersion 4 and minorVersion 0. This setting may be used in the future to enable evolution of the shared flow format."]
    pub configuration_version:
        ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1ConfigVersion>>,
    #[serde(rename = "contextInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A textual description of the shared flow revision."]
    pub context_info: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time at which this shared flow revision was created, in milliseconds since epoch."]
    pub created_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the shared flow revision."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human readable name of this shared flow."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityMetaDataAsProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Key-Value map of metadata about this shared flow revision."]
    pub entity_meta_data_as_properties:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time at which this shared flow revision was most recently modified, in milliseconds since epoch."]
    pub last_modified_at: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource ID of the parent shared flow."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "policies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of policy names included in this shared flow revision."]
    pub policies: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "resourceFiles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource files included in this shared flow revision."]
    pub resource_files: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1ResourceFiles>>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of the resources included in this shared flow revision formatted as \"{type}://{name}\"."]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource ID of this revision."]
    pub revision: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sharedFlows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of the shared flow names included in this shared flow revision."]
    pub shared_flows: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The string \"Application\""]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This message type encapsulates a stats response."]
pub struct GoogleCloudApigeeV1Stats {
    #[serde(rename = "environments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains a list of query results on environment level."]
    pub environments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1StatsEnvironmentStats>>,
    >,
    #[serde(rename = "hosts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains a list of query results grouped by host."]
    pub hosts: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1StatsHostStats>>,
    >,
    #[serde(rename = "metaData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains the metadata information."]
    pub meta_data: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1Metadata>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This message type encapsulates the environment wrapper: \"environments\": [ { \"metrics\": [ { \"name\": \"sum(message_count)\", \"values\": [ \"2.52056245E8\" ] } ], \"name\": \"prod\" } ]"]
pub struct GoogleCloudApigeeV1StatsEnvironmentStats {
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains the list of metrics grouped under dimensions."]
    pub dimensions: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1DimensionMetric>>,
    >,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "In the final response, only one of the following fields will be present based on the dimensions provided. If no dimensions are provided, then only a top level metrics is provided. If dimensions are included, then there will be a top level dimensions field under environments which will contain metrics values and the dimension name. Example: \"environments\": [ { \"dimensions\": [ { \"metrics\": [ { \"name\": \"sum(message_count)\", \"values\": [ \"2.14049521E8\" ] } ], \"name\": \"nit_proxy\" } ], \"name\": \"prod\" } ] OR \"environments\": [ { \"metrics\": [ { \"name\": \"sum(message_count)\", \"values\": [ \"2.19026331E8\" ] } ], \"name\": \"prod\" } ] This field contains the list of metric values."]
    pub metrics:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Metric>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This message type encapsulates the hostname wrapper: \"hosts\": [ { \"metrics\": [ { \"name\": \"sum(message_count)\", \"values\": [ \"2.52056245E8\" ] } ], \"name\": \"example.com\" } ]"]
pub struct GoogleCloudApigeeV1StatsHostStats {
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains the list of metrics grouped under dimensions."]
    pub dimensions: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1DimensionMetric>>,
    >,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "In the final response, only one of the following fields will be present based on the dimensions provided. If no dimensions are provided, then only a top level metrics is provided. If dimensions are included, then there will be a top level dimensions field under hostnames which will contain metrics values and the dimension name. Example: \"hosts\": [ { \"dimensions\": [ { \"metrics\": [ { \"name\": \"sum(message_count)\", \"values\": [ \"2.14049521E8\" ] } ], \"name\": \"nit_proxy\" } ], \"name\": \"example.com\" } ] OR \"hosts\": [ { \"metrics\": [ { \"name\": \"sum(message_count)\", \"values\": [ \"2.19026331E8\" ] } ], \"name\": \"example.com\" } ] This field contains the list of metric values."]
    pub metrics:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudApigeeV1Metric>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field contains the hostname used in query."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Pub/Sub subscription of an environment."]
pub struct GoogleCloudApigeeV1Subscription {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Full name of the Pub/Sub subcription. Use the following structure in your request: `subscription \"projects/foo/subscription/bar\"`"]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1SyncAuthorization {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entity tag (ETag) used for optimistic concurrency control as a way to help prevent simultaneous updates from overwriting each other. For example, when you call [getSyncAuthorization](organizations/getSyncAuthorization) an ETag is returned in the response. Pass that ETag when calling the [setSyncAuthorization](organizations/setSyncAuthorization) to ensure that you are updating the correct version. If you don't pass the ETag in the call to `setSyncAuthorization`, then the existing authorization is overwritten indiscriminately. **Note**: We strongly recommend that you use the ETag in the read-modify-write cycle to avoid race conditions."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "identities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Array of service accounts to grant access to control plane resources, each specified using the following format: `serviceAccount:` service-account-name. The service-account-name is formatted like an email address. For example: `my-synchronizer-manager-service_account@my_project_id.iam.gserviceaccount.com` You might specify multiple service accounts, for example, if you have multiple environments and wish to assign a unique service account to each one. The service accounts must have **Apigee Synchronizer Manager** role. See also [Create service accounts](https://cloud.google.com/apigee/docs/hybrid/latest/sa-about#create-the-service-accounts)."]
    pub identities: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "TargetServer configuration. TargetServers are used to decouple a proxy's TargetEndpoint HTTPTargetConnections from concrete URLs for backend services."]
pub struct GoogleCloudApigeeV1TargetServer {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A human-readable description of this TargetServer."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The host name this target connects to. Value must be a valid hostname as described by RFC-1123."]
    pub host: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Enabling/disabling a TargetServer is useful when TargetServers are used in load balancing configurations, and one or more TargetServers need to taken out of rotation periodically. Defaults to true."]
    pub is_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The resource id of this target server. Values must match the regular expression "]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The port number this target connects to on the given host. Value must be between 1 and 65535, inclusive."]
    pub port: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "sSLInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies TLS configuration info for this TargetServer. The JSON name is `sSLInfo` for legacy/backwards compatibility reasons -- Edge originally supported SSL, and the name is still used for TLS configuration."]
    pub s_sl_info: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1TlsInfo>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1TargetServerConfig {
    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Host name of the target server."]
    pub host: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target server revision name in the following format: `organizations/{org}/environments/{env}/targetservers/{targetserver}/revisions/{rev}`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Port number for the target server."]
    pub port: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "tlsInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "TLS settings for the target server."]
    pub tls_info: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1TlsInfoConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for TestDatastore"]
pub struct GoogleCloudApigeeV1TestDatastoreResponse {
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Error message of test connection failure"]
    pub error: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. It could be `completed` or `failed`"]
    pub state: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "TLS configuration information for VirtualHosts and TargetServers."]
pub struct GoogleCloudApigeeV1TlsInfo {
    #[serde(rename = "ciphers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The SSL/TLS cipher suites to be used. Must be one of the cipher suite names listed in: http://docs.oracle.com/javase/8/docs/technotes/guides/security/StandardNames.html#ciphersuites"]
    pub ciphers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "clientAuthEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Enables two-way TLS."]
    pub client_auth_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "commonName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The TLS Common Name of the certificate."]
    pub common_name: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1TlsInfoCommonName>>,
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Enables TLS. If false, neither one-way nor two-way TLS will be enabled."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "ignoreValidationErrors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, Edge ignores TLS certificate errors. Valid when configuring TLS for target servers and target endpoints, and when configuring virtual hosts that use 2-way TLS. When used with a target endpoint/target server, if the backend system uses SNI and returns a cert with a subject Distinguished Name (DN) that does not match the hostname, there is no way to ignore the error and the connection fails."]
    pub ignore_validation_errors: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "keyAlias")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required if `client_auth_enabled` is true. The resource ID for the alias containing the private key and cert."]
    pub key_alias: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keyStore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required if `client_auth_enabled` is true. The resource ID of the keystore. References not yet supported."]
    pub key_store: ::std::option::Option<::std::string::String>,
    #[serde(rename = "protocols")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The TLS versioins to be used."]
    pub protocols: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "trustStore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource ID of the truststore. References not yet supported."]
    pub trust_store: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1TlsInfoCommonName {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The TLS Common Name string of the certificate."]
    pub value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "wildcardMatch")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the cert should be matched against as a wildcard cert."]
    pub wildcard_match: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudApigeeV1TlsInfoConfig {
    #[serde(rename = "ciphers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of ciphers that are granted access."]
    pub ciphers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "clientAuthEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flag that specifies whether client-side authentication is enabled for the target server. Enables two-way TLS."]
    pub client_auth_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "commonName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common name to validate the target server against."]
    pub common_name: ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1CommonNameConfig>>,
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flag that specifies whether one-way TLS is enabled. Set to `true` to enable one-way TLS."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "ignoreValidationErrors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flag that specifies whether to ignore TLS certificate validation errors. Set to `true` to ignore errors."]
    pub ignore_validation_errors: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "keyAlias")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the alias used for client-side authentication in the following format: `organizations/{org}/environments/{env}/keystores/{keystore}/aliases/{alias}`"]
    pub key_alias: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keyAliasReference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reference name and alias pair to use for client-side authentication."]
    pub key_alias_reference:
        ::std::option::Option<::std::boxed::Box<GoogleCloudApigeeV1KeyAliasReference>>,
    #[serde(rename = "protocols")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of TLS protocols that are granted access."]
    pub protocols: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "trustStore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the keystore or keystore reference containing trusted certificates for the server in the following format: `organizations/{org}/environments/{env}/keystores/{keystore}` or `organizations/{org}/environments/{env}/references/{reference}`"]
    pub trust_store: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details on why a resource update failed in the runtime."]
pub struct GoogleCloudApigeeV1UpdateError {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status code."]
    pub code: ::std::option::Option<GoogleCloudApigeeV1UpdateErrorCodeEnum>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-friendly error message."]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sub resource specific to this error (e.g. a proxy deployed within the EnvironmentConfig). If empty the error refers to the top level resource."]
    pub resource: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A string that uniquely identifies the type of error. This provides a more reliable means to deduplicate errors across revisions and instances."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Status code."]
pub enum GoogleCloudApigeeV1UpdateErrorCodeEnum {
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { \"audit_configs\": [ { \"service\": \"allServices\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" }, { \"log_type\": \"ADMIN_READ\" } ] }, { \"service\": \"sampleservice.googleapis.com\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\" }, { \"log_type\": \"DATA_WRITE\", \"exempted_members\": [ \"user:aliya@example.com\" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging."]
pub struct GoogleIamV1AuditConfig {
    #[serde(rename = "auditLogConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration for logging of each type of permission."]
    pub audit_log_configs:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleIamV1AuditLogConfig>>>,
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services."]
    pub service: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Provides the configuration for logging a type of permissions. Example: { \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging."]
pub struct GoogleIamV1AuditLogConfig {
    #[serde(rename = "exemptedMembers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members."]
    pub exempted_members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "logType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The log type that this config enables."]
    pub log_type: ::std::option::Option<GoogleIamV1AuditLogConfigLogTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The log type that this config enables."]
pub enum GoogleIamV1AuditLogConfigLogTypeEnum {
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
pub struct GoogleIamV1Binding {
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the members in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub condition: ::std::option::Option<::std::boxed::Box<GoogleTypeExpr>>,
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
#[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/)."]
pub struct GoogleIamV1Policy {
    #[serde(rename = "auditConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies cloud audit logging configuration for this policy."]
    pub audit_configs:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleIamV1AuditConfig>>>,
    #[serde(rename = "bindings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member."]
    pub bindings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleIamV1Binding>>>,
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
#[doc = "Request message for `SetIamPolicy` method."]
pub struct GoogleIamV1SetIamPolicyRequest {
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them."]
    pub policy: ::std::option::Option<::std::boxed::Box<GoogleIamV1Policy>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: \"bindings, etag\"`"]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for `TestIamPermissions` method."]
pub struct GoogleIamV1TestIamPermissionsRequest {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of permissions to check for the `resource`. Permissions with wildcards (such as '*' or 'storage.*') are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions)."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for `TestIamPermissions` method."]
pub struct GoogleIamV1TestIamPermissionsResponse {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A subset of `TestPermissionsRequest.permissions` that the caller is allowed."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Operations.ListOperations."]
pub struct GoogleLongrunningListOperationsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The standard List next-page token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of operations that matches the specified filter in the request."]
    pub operations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleLongrunningOperation>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This resource represents a long-running operation that is the result of a network API call."]
pub struct GoogleLongrunningOperation {
    #[serde(rename = "done")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
    pub done: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error result of the operation in case of failure or cancellation."]
    pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
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
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct GoogleProtobufEmpty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes what preconditions have failed. For example, if an RPC failed because it required the Terms of Service to be acknowledged, it could list the terms of service violation in the PreconditionFailure message."]
pub struct GoogleRpcPreconditionFailure {
    #[serde(rename = "violations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes all precondition violations."]
    pub violations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleRpcPreconditionFailureViolation>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message type used to describe a single precondition failure."]
pub struct GoogleRpcPreconditionFailureViolation {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of how the precondition failed. Developers can use this description to understand how to fix the failure. For example: \"Terms of service not accepted\"."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subject")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The subject, relative to the type, that failed. For example, \"google.com/cloud\" relative to the \"TOS\" type would indicate which terms of service is being referenced."]
    pub subject: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of PreconditionFailure. We recommend using a service-specific enum type to define the supported precondition violation subjects. For example, \"TOS\" for \"Terms of Service violation\"."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
pub struct GoogleRpcStatus {
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
#[doc = "Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: \"Summary size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() < 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\" expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description: \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information."]
pub struct GoogleTypeExpr {
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
