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
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An individual endpoint that provides a service. The service must already exist to create an endpoint."]
pub struct Endpoint {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. An IPv4 or IPv6 address. Service Directory rejects bad addresses like: * `8.8.8` * `8.8.8.8:53` * `test:bad:address` * `[::1]` * `[::1]:8080` Limited to 45 characters."]
    pub address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "annotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Annotations for the endpoint. This data can be consumed by service clients. Restrictions: * The entire annotations dictionary may contain up to 512 characters, spread accoss all key-value pairs. Annotations that go beyond this limit are rejected * Valid annotation keys have two segments: an optional prefix and name, separated by a slash (/). The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between. The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots (.), not longer than 253 characters in total, followed by a slash (/) Annotations that fails to meet these requirements are rejected. * The `(*.)google.com/` and `(*.)googleapis.com/` prefixes are reserved for system annotations managed by Service Directory. If the user tries to write to these keyspaces, those entries are silently ignored by the system Note: This field is equivalent to the `metadata` field in the v1beta1 API. They have the same syntax and read/write to the same location in Service Directory."]
    pub annotations:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The resource name for the endpoint in the format `projects/*/locations/*/namespaces/*/services/*/endpoints/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Service Directory rejects values outside of `[0, 65535]`."]
    pub port: ::std::option::Option<::std::primitive::i64>,
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
#[doc = "Request message for `GetIamPolicy` method."]
pub struct GetIamPolicyRequest {
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`."]
    pub options: ::std::option::Option<::std::boxed::Box<GetPolicyOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Encapsulates settings provided to GetIamPolicy."]
pub struct GetPolicyOptions {
    #[serde(rename = "requestedPolicyVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The policy format version to be returned. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional bindings must specify version 3. Policies without any conditional bindings may specify any valid value or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub requested_policy_version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for RegistrationService.ListEndpoints."]
pub struct ListEndpointsResponse {
    #[serde(rename = "endpoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of endpoints."]
    pub endpoints: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Endpoint>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
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
#[doc = "The response message for RegistrationService.ListNamespaces."]
pub struct ListNamespacesResponse {
    #[serde(rename = "namespaces")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of namespaces."]
    pub namespaces: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Namespace>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for RegistrationService.ListServices."]
pub struct ListServicesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of services."]
    pub services: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Service>>>,
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
#[doc = "A container for services. Namespaces allow administrators to group services together and define permissions for a collection of services."]
pub struct Namespace {
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Resource labels associated with this namespace. No more than 64 user labels can be associated with a given resource. Label keys and values can be no longer than 63 characters."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The resource name for the namespace in the format `projects/*/locations/*/namespaces/*`."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/)."]
pub struct Policy {
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
#[doc = "The request message for LookupService.ResolveService. Looks up a service by its name, returns the service and its endpoints."]
pub struct ResolveServiceRequest {
    #[serde(rename = "endpointFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The filter applied to the endpoints of the resolved service. General `filter` string syntax: ` ()` * `` can be `name`, `address`, `port`, or `annotations.` for map field * `` can be `<`, `>`, `<=`, `>=`, `!=`, `=`, `:`. Of which `:` means `HAS`, and is roughly the same as `=` * `` must be the same data type as field * `` can be `AND`, `OR`, `NOT` Examples of valid filters: * `annotations.owner` returns endpoints that have a annotation with the key `owner`, this is the same as `annotations:owner` * `annotations.protocol=gRPC` returns endpoints that have key/value `protocol=gRPC` * `address=192.108.1.105` returns endpoints that have this address * `port>8080` returns endpoints that have port number larger than 8080 * `name>projects/my-project/locations/us-east1/namespaces/my-namespace/services/my-service/endpoints/endpoint-c` returns endpoints that have name that is alphabetically later than the string, so \"endpoint-e\" is returned but \"endpoint-a\" is not * `annotations.owner!=sd AND annotations.foo=bar` returns endpoints that have `owner` in annotation key but value is not `sd` AND have key/value `foo=bar` * `doesnotexist.foo=bar` returns an empty list. Note that endpoint doesn't have a field called \"doesnotexist\". Since the filter does not match any endpoint, it returns no results For more information about filtering, see [API Filtering](https://aip.dev/160)."]
    pub endpoint_filter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxEndpoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The maximum number of endpoints to return. Defaults to 25. Maximum is 100. If a value less than one is specified, the Default is used. If a value greater than the Maximum is specified, the Maximum is used."]
    pub max_endpoints: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for LookupService.ResolveService."]
pub struct ResolveServiceResponse {
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service: ::std::option::Option<::std::boxed::Box<Service>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An individual service. A service contains a name and optional metadata. A service must exist before endpoints can be added to it."]
pub struct Service {
    #[serde(rename = "annotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Annotations for the service. This data can be consumed by service clients. Restrictions: * The entire annotations dictionary may contain up to 512 characters, spread accoss all key-value pairs. Annotations that go beyond this limit are rejected * Valid annotation keys have two segments: an optional prefix and name, separated by a slash (/). The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between. The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots (.), not longer than 253 characters in total, followed by a slash (/). Annotations that fails to meet these requirements are rejected * The `(*.)google.com/` and `(*.)googleapis.com/` prefixes are reserved for system annotations managed by Service Directory. If the user tries to write to these keyspaces, those entries are silently ignored by the system Note: This field is equivalent to the `metadata` field in the v1beta1 API. They have the same syntax and read/write to the same location in Service Directory."]
    pub annotations:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "endpoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Endpoints associated with this service. Returned on LookupService.ResolveService. Control plane clients should use RegistrationService.ListEndpoints."]
    pub endpoints: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Endpoint>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The resource name for the service in the format `projects/*/locations/*/namespaces/*/services/*`."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for `SetIamPolicy` method."]
pub struct SetIamPolicyRequest {
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them."]
    pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
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
