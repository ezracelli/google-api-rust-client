#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of the final state \"abort\" and associated resource."]
pub struct AbortInfo {
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Causes that the analysis is aborted."]
    pub cause: ::std::option::Option<AbortInfoCauseEnum>,
    #[serde(rename = "resourceUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of the resource that caused the abort."]
    pub resource_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Causes that the analysis is aborted."]
pub enum AbortInfoCauseEnum {
    #[serde(rename = "CAUSE_UNSPECIFIED")]
    #[doc = "Cause is unspecified."]
    CauseUnspecified,
    #[serde(rename = "UNKNOWN_NETWORK")]
    #[doc = "Aborted due to unknown network. The reachability analysis cannot proceed because the user does not have access to the host project's network configurations, including firewall rules and routes. This happens when the project is a service project and the endpoints being traced are in the host project's network."]
    UnknownNetwork,
    #[serde(rename = "UNKNOWN_IP")]
    #[doc = "Aborted because the IP address(es) are unknown."]
    UnknownIp,
    #[serde(rename = "UNKNOWN_PROJECT")]
    #[doc = "Aborted because no project information can be derived from the test input."]
    UnknownProject,
    #[serde(rename = "PERMISSION_DENIED")]
    #[doc = "Aborted because the user lacks the permission to access all or part of the network configurations required to run the test."]
    PermissionDenied,
    #[serde(rename = "NO_SOURCE_LOCATION")]
    #[doc = "Aborted because no valid source endpoint is derived from the input test request."]
    NoSourceLocation,
    #[serde(rename = "INVALID_ARGUMENT")]
    #[doc = "Aborted because the source and/or destination endpoint specified in the test are invalid. The possible reasons that an endpoint is invalid include: malformed IP address; nonexistent instance or network URI; IP address not in the range of specified network URI; and instance not owning the network interface in the specified network."]
    InvalidArgument,
    #[serde(rename = "NO_EXTERNAL_IP")]
    #[doc = "Aborted because traffic is sent from a public IP to an instance without an external IP."]
    NoExternalIp,
    #[serde(rename = "UNINTENDED_DESTINATION")]
    #[doc = "Aborted because none of the traces matches destination information specified in the input test request."]
    UnintendedDestination,
    #[serde(rename = "TRACE_TOO_LONG")]
    #[doc = "Aborted because the number of steps in the trace exceeding a certain limit which may be caused by routing loop."]
    TraceTooLong,
    #[serde(rename = "INTERNAL_ERROR")]
    #[doc = "Aborted due to internal server error."]
    InternalError,
}
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
    #[serde(rename = "bindingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub binding_id: ::std::option::Option<::std::string::String>,
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
#[doc = "A Connectivity Test for a network reachability analysis."]
pub struct ConnectivityTest {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the test was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-supplied description of the Connectivity Test. Maximum of 512 characters."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Destination specification of the Connectivity Test. You can use a combination of destination IP address, Compute Engine VM instance, or VPC network to uniquely identify the destination location. Even if the destination IP address is not unique, the source IP location is unique. Usually, the analysis can infer the destination endpoint from route information. If the destination you specify is a VM instance and the instance has multiple network interfaces, then you must also specify either a destination IP address or VPC network to identify the destination interface. A reachability analysis proceeds even if the destination location is ambiguous. However, the result can include endpoints that you don't intend to test."]
    pub destination: ::std::option::Option<::std::boxed::Box<Endpoint>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of a Connectivity Test."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource labels to represent user-provided metadata."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Unique name of the resource using the form: `projects/{project_id}/locations/global/connectivityTests/{test_id}`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IP Protocol of the test. When not provided, \"TCP\" is assumed."]
    pub protocol: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reachabilityDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The reachability details of this test from the latest run. The details are updated when creating a new test, updating an existing test, or triggering a one-time rerun of an existing test."]
    pub reachability_details: ::std::option::Option<::std::boxed::Box<ReachabilityDetails>>,
    #[serde(rename = "relatedProjects")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Other projects that may be relevant for reachability analysis. This is applicable to scenarios where a test can cross project boundaries."]
    pub related_projects: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Source specification of the Connectivity Test. You can use a combination of source IP address, virtual machine (VM) instance, or Compute Engine network to uniquely identify the source location. Examples: If the source IP address is an internal IP address within a Google Cloud Virtual Private Cloud (VPC) network, then you must also specify the VPC network. Otherwise, specify the VM instance, which already contains its internal IP address and VPC network information. If the source of the test is within an on-premises network, then you must provide the destination VPC network. If the source endpoint is a Compute Engine VM instance with multiple network interfaces, the instance itself is not sufficient to identify the endpoint. So, you must also specify the source IP address or VPC network. A reachability analysis proceeds even if the source location is ambiguous. However, the test result may include endpoints that you don't intend to test."]
    pub source: ::std::option::Option<::std::boxed::Box<Endpoint>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the test's configuration was updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of the final state \"deliver\" and associated resource."]
pub struct DeliverInfo {
    #[serde(rename = "resourceUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of the resource that the packet is delivered to."]
    pub resource_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target type where the packet is delivered to."]
    pub target: ::std::option::Option<DeliverInfoTargetEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Target type where the packet is delivered to."]
pub enum DeliverInfoTargetEnum {
    #[serde(rename = "TARGET_UNSPECIFIED")]
    #[doc = "Target not specified."]
    TargetUnspecified,
    #[serde(rename = "INSTANCE")]
    #[doc = "Target is a Compute Engine instance."]
    Instance,
    #[serde(rename = "INTERNET")]
    #[doc = "Target is the Internet."]
    Internet,
    #[serde(rename = "GOOGLE_API")]
    #[doc = "Target is a Google API."]
    GoogleApi,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of the final state \"drop\" and associated resource."]
pub struct DropInfo {
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cause that the packet is dropped."]
    pub cause: ::std::option::Option<DropInfoCauseEnum>,
    #[serde(rename = "resourceUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of the resource that caused the drop."]
    pub resource_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Cause that the packet is dropped."]
pub enum DropInfoCauseEnum {
    #[serde(rename = "CAUSE_UNSPECIFIED")]
    #[doc = "Cause is unspecified."]
    CauseUnspecified,
    #[serde(rename = "UNKNOWN_EXTERNAL_ADDRESS")]
    #[doc = "Destination external address cannot be resolved to a known target."]
    UnknownExternalAddress,
    #[serde(rename = "FOREIGN_IP_DISALLOWED")]
    #[doc = "a Compute Engine instance can only send or receive a packet with a foreign IP if ip_forward is enabled."]
    ForeignIpDisallowed,
    #[serde(rename = "FIREWALL_RULE")]
    #[doc = "Dropped due to a firewall rule unless allowed due to connection tracking."]
    FirewallRule,
    #[serde(rename = "NO_ROUTE")]
    #[doc = "Dropped due to no routes."]
    NoRoute,
    #[serde(rename = "ROUTE_BLACKHOLE")]
    #[doc = "Dropped due to invalid route. Route's next hop is a blackhole."]
    RouteBlackhole,
    #[serde(rename = "ROUTE_WRONG_NETWORK")]
    #[doc = "Packet is sent to a wrong (unintended) network. Example: user traces a packet from VM1:Network1 to VM2:Network2, however, the route configured in Network1 sends the packet destined for VM2's IP addresss to Network3."]
    RouteWrongNetwork,
    #[serde(rename = "PRIVATE_TRAFFIC_TO_INTERNET")]
    #[doc = "Packet with internal destination address sent to Internet gateway."]
    PrivateTrafficToInternet,
    #[serde(rename = "PRIVATE_GOOGLE_ACCESS_DISALLOWED")]
    #[doc = "Instance with only an internal IP tries to access Google API and Services, and private Google access is not enabled."]
    PrivateGoogleAccessDisallowed,
    #[serde(rename = "NO_EXTERNAL_ADDRESS")]
    #[doc = "Instance with only internal IP tries to access external hosts, but Cloud NAT is not enabled in the subnet, unless special configurations on a VM allows this connection. See [Special Configurations for VM instances](/vpc/docs/special-configurations) for details."]
    NoExternalAddress,
    #[serde(rename = "UNKNOWN_INTERNAL_ADDRESS")]
    #[doc = "Destination internal address cannot be resolved to a known target."]
    UnknownInternalAddress,
    #[serde(rename = "FORWARDING_RULE_MISMATCH")]
    #[doc = "Forwarding rule's protocol and ports do not match the packet header."]
    ForwardingRuleMismatch,
    #[serde(rename = "FORWARDING_RULE_NO_INSTANCES")]
    #[doc = "Forwarding rule does not have backends configured."]
    ForwardingRuleNoInstances,
    #[serde(rename = "FIREWALL_BLOCKING_LOAD_BALANCER_BACKEND_HEALTH_CHECK")]
    #[doc = "Firewalls block the health check probes to the backends and cause the backends to be unavailable for traffic from the load balancer. See [Health check firewall rules](/load-balancing/docs/ health-checks#firewall_rules) for more details."]
    FirewallBlockingLoadBalancerBackendHealthCheck,
    #[serde(rename = "INSTANCE_NOT_RUNNING")]
    #[doc = "Packet is sent from or to a Compute Engine instance that is not in a running state."]
    InstanceNotRunning,
    #[serde(rename = "TRAFFIC_TYPE_BLOCKED")]
    #[doc = "The type of traffic is blocked and the user cannot configure a firewall rule to enable it. See [Always blocked traffic](/vpc/docs/firewalls# blockedtraffic) for more details."]
    TrafficTypeBlocked,
    #[serde(rename = "GKE_MASTER_UNAUTHORIZED_ACCESS")]
    #[doc = "Access to GKE master's endpoint is not authorized. See [Access to the cluster endpoints](/kubernetes-engine/docs/how-to/ private-clusters#access_to_the_cluster_endpoints) for more details."]
    GkeMasterUnauthorizedAccess,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Source or destination of the Connectivity Test."]
pub struct Endpoint {
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Compute Engine instance URI."]
    pub instance: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IP address of the endpoint, which can be an external or internal IP. An IPv6 address is only allowed when the test's destination is a [global load balancer VIP](/load-balancing/docs/load-balancing-overview)."]
    pub ip_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "network")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Compute Engine network URI."]
    pub network: ::std::option::Option<::std::string::String>,
    #[serde(rename = "networkType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the network where the endpoint is located. Applicable only to source endpoint, as destination network type can be inferred from the source."]
    pub network_type: ::std::option::Option<EndpointNetworkTypeEnum>,
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IP protocol port of the endpoint. Only applicable when protocol is TCP or UDP."]
    pub port: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Project ID where the endpoint is located. The Project ID can be derived from the URI if you provide a VM instance or network URI. The following are two cases where you must provide the project ID: 1. Only the IP address is specified, and the IP address is within a GCP project. 2. When you are using Shared VPC and the IP address that you provide is from the service project. In this case, the network that the IP address resides in is defined in the host project."]
    pub project_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of the network where the endpoint is located. Applicable only to source endpoint, as destination network type can be inferred from the source."]
pub enum EndpointNetworkTypeEnum {
    #[serde(rename = "NETWORK_TYPE_UNSPECIFIED")]
    #[doc = "Default type if unspecified."]
    NetworkTypeUnspecified,
    #[serde(rename = "GCP_NETWORK")]
    #[doc = "A network hosted within Google Cloud Platform. To receive more detailed output, specify the URI for the source or destination network."]
    GcpNetwork,
    #[serde(rename = "NON_GCP_NETWORK")]
    #[doc = "A network hosted outside of Google Cloud Platform. This can be an on-premises network, or a network hosted by another cloud provider."]
    NonGcpNetwork,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "For display only. The specification of the endpoints for the test. EndpointInfo is derived from source and destination Endpoint and validated by the backend data plane model."]
pub struct EndpointInfo {
    #[serde(rename = "destinationIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Destination IP address."]
    pub destination_ip: ::std::option::Option<::std::string::String>,
    #[serde(rename = "destinationNetworkUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of the network where this packet is sent to."]
    pub destination_network_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "destinationPort")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Destination port. Only valid when protocol is TCP or UDP."]
    pub destination_port: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IP protocol in string format, for example: \"TCP\", \"UDP\", \"ICMP\"."]
    pub protocol: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Source IP address."]
    pub source_ip: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceNetworkUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of the network where this packet originates from."]
    pub source_network_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourcePort")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Source port. Only valid when protocol is TCP or UDP."]
    pub source_port: ::std::option::Option<::std::primitive::i64>,
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
#[doc = "For display only. Metadata associated with a Compute Engine firewall rule."]
pub struct FirewallInfo {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Possible values: ALLOW, DENY"]
    pub action: ::std::option::Option<::std::string::String>,
    #[serde(rename = "direction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Possible values: INGRESS, EGRESS"]
    pub direction: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of a Compute Engine firewall rule."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "networkUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of a Compute Engine network."]
    pub network_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Priority of the firewall rule."]
    pub priority: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "targetServiceAccounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target service accounts of the firewall rule."]
    pub target_service_accounts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "targetTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target tags of the firewall rule."]
    pub target_tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of a Compute Engine firewall rule. Implied default rule does not have URI."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of the final state \"forward\" and associated resource."]
pub struct ForwardInfo {
    #[serde(rename = "resourceUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of the resource that the packet is forwarded to."]
    pub resource_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target type where this packet is forwarded to."]
    pub target: ::std::option::Option<ForwardInfoTargetEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Target type where this packet is forwarded to."]
pub enum ForwardInfoTargetEnum {
    #[serde(rename = "TARGET_UNSPECIFIED")]
    #[doc = "Target not specified."]
    TargetUnspecified,
    #[serde(rename = "PEERING_VPC")]
    #[doc = "Forwarded to a VPC peering network."]
    PeeringVpc,
    #[serde(rename = "VPN_GATEWAY")]
    #[doc = "Forwarded to a Cloud VPN gateway."]
    VpnGateway,
    #[serde(rename = "INTERCONNECT")]
    #[doc = "Forwarded to an Cloud Interconnect connection."]
    Interconnect,
    #[serde(rename = "GKE_MASTER")]
    #[doc = "Forwarded to a Google Kubernetes Engine Container cluster master."]
    GkeMaster,
    #[serde(rename = "IMPORTED_CUSTOM_ROUTE_NEXT_HOP")]
    #[doc = "Forwarded to the next hop of a custom route imported from a peering VPC."]
    ImportedCustomRouteNextHop,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "For display only. Metadata associated with a Compute Engine forwarding rule."]
pub struct ForwardingRuleInfo {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of a Compute Engine forwarding rule."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "matchedPortRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Port range defined in the forwarding rule that matches the test."]
    pub matched_port_range: ::std::option::Option<::std::string::String>,
    #[serde(rename = "matchedProtocol")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Protocol defined in the forwarding rule that matches the test."]
    pub matched_protocol: ::std::option::Option<::std::string::String>,
    #[serde(rename = "networkUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Network URI. Only valid for Internal Load Balancer."]
    pub network_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target type of the forwarding rule."]
    pub target: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of a Compute Engine forwarding rule."]
    pub uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "vip")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "VIP of the forwarding rule."]
    pub vip: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "For display only. Metadata associated with a Compute Engine instance."]
pub struct InstanceInfo {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of a Compute Engine instance."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "externalIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "External IP address of the network interface."]
    pub external_ip: ::std::option::Option<::std::string::String>,
    #[serde(rename = "interface")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the network interface of a Compute Engine instance."]
    pub interface: ::std::option::Option<::std::string::String>,
    #[serde(rename = "internalIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Internal IP address of the network interface."]
    pub internal_ip: ::std::option::Option<::std::string::String>,
    #[serde(rename = "networkTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Network tags configured on the instance."]
    pub network_tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "networkUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of a Compute Engine network."]
    pub network_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Service account authorized for the instance."]
    pub service_account: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of a Compute Engine instance."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `ListConnectivityTests` method."]
pub struct ListConnectivityTestsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Page token to fetch the next set of Connectivity Tests."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of Connectivity Tests."]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ConnectivityTest>>>,
    #[serde(rename = "unreachable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locations that could not be reached (when querying all locations with `-`)."]
    pub unreachable: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
#[doc = "For display only. Metadata associated with a specific load balancer backend."]
pub struct LoadBalancerBackend {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of a Compute Engine instance or network endpoint."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "healthCheckAllowingFirewallRules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of firewall rule URIs allowing probes from health check IP ranges."]
    pub health_check_allowing_firewall_rules:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "healthCheckBlockingFirewallRules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of firewall rule URIs blocking probes from health check IP ranges."]
    pub health_check_blocking_firewall_rules:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "healthCheckFirewallState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "State of the health check firewall configuration."]
    pub health_check_firewall_state:
        ::std::option::Option<LoadBalancerBackendHealthCheckFirewallStateEnum>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of a Compute Engine instance or network endpoint."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "State of the health check firewall configuration."]
pub enum LoadBalancerBackendHealthCheckFirewallStateEnum {
    #[serde(rename = "HEALTH_CHECK_FIREWALL_STATE_UNSPECIFIED")]
    #[doc = "State is unspecified. Default state if not populated."]
    HealthCheckFirewallStateUnspecified,
    #[serde(rename = "CONFIGURED")]
    #[doc = "There are configured firewall rules to allow health check probes to the backend."]
    Configured,
    #[serde(rename = "MISCONFIGURED")]
    #[doc = "There are firewall rules configured to allow partial health check ranges or block all health check ranges. If a health check probe is sent from denied IP ranges, the health check to the backend will fail. Then, the backend will be marked unhealthy and will not receive traffic sent to the load balancer."]
    Misconfigured,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "For display only. Metadata associated with a load balancer."]
pub struct LoadBalancerInfo {
    #[serde(rename = "backendType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of load balancer's backend configuration."]
    pub backend_type: ::std::option::Option<LoadBalancerInfoBackendTypeEnum>,
    #[serde(rename = "backendUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Backend configuration URI."]
    pub backend_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "backends")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information for the loadbalancer backends."]
    pub backends: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LoadBalancerBackend>>>,
    #[serde(rename = "healthCheckUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of the health check for the load balancer."]
    pub health_check_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "loadBalancerType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the load balancer."]
    pub load_balancer_type: ::std::option::Option<LoadBalancerInfoLoadBalancerTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of load balancer's backend configuration."]
pub enum LoadBalancerInfoBackendTypeEnum {
    #[serde(rename = "BACKEND_TYPE_UNSPECIFIED")]
    #[doc = "Type is unspecified."]
    BackendTypeUnspecified,
    #[serde(rename = "BACKEND_SERVICE")]
    #[doc = "Backend Service as the load balancer's backend."]
    BackendService,
    #[serde(rename = "TARGET_POOL")]
    #[doc = "Target Pool as the load balancer's backend."]
    TargetPool,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of the load balancer."]
pub enum LoadBalancerInfoLoadBalancerTypeEnum {
    #[serde(rename = "LOAD_BALANCER_TYPE_UNSPECIFIED")]
    #[doc = "Type is unspecified."]
    LoadBalancerTypeUnspecified,
    #[serde(rename = "INTERNAL_TCP_UDP")]
    #[doc = "Internal TCP/UDP load balancer."]
    InternalTcpUdp,
    #[serde(rename = "NETWORK_TCP_UDP")]
    #[doc = "Network TCP/UDP load balancer."]
    NetworkTcpUdp,
    #[serde(rename = "HTTP_PROXY")]
    #[doc = "HTTP(S) proxy load balancer."]
    HttpProxy,
    #[serde(rename = "TCP_PROXY")]
    #[doc = "TCP proxy load balancer."]
    TcpProxy,
    #[serde(rename = "SSL_PROXY")]
    #[doc = "SSL proxy load balancer."]
    SslProxy,
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
#[doc = "For display only. Metadata associated with a Compute Engine network."]
pub struct NetworkInfo {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of a Compute Engine network."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "matchedIpRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IP range that matches the test."]
    pub matched_ip_range: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of a Compute Engine network."]
    pub uri: ::std::option::Option<::std::string::String>,
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
#[doc = "Metadata describing an Operation"]
pub struct OperationMetadata {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API version."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cancelRequested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies if cancellation was requested for the operation."]
    pub cancel_requested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time the operation was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time the operation finished running."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statusDetail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human-readable status of the operation, if any."]
    pub status_detail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target of the operation - for example projects/project-1/locations/global/connectivityTests/test-1"]
    pub target: ::std::option::Option<::std::string::String>,
    #[serde(rename = "verb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the verb executed by the operation."]
    pub verb: ::std::option::Option<::std::string::String>,
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
#[doc = "The details of reachability state from the latest run."]
pub struct ReachabilityDetails {
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The details of a failure or a cancellation of reachability analysis."]
    pub error: ::std::option::Option<::std::boxed::Box<Status>>,
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The overall reachability result of the test."]
    pub result: ::std::option::Option<ReachabilityDetailsResultEnum>,
    #[serde(rename = "traces")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Result may contain a list of traces if a test has multiple possible paths in the network, such as when destination endpoint is a load balancer with multiple backends."]
    pub traces: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Trace>>>,
    #[serde(rename = "verifyTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time the reachability state was verified."]
    pub verify_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The overall reachability result of the test."]
pub enum ReachabilityDetailsResultEnum {
    #[serde(rename = "RESULT_UNSPECIFIED")]
    #[doc = "Result is not specified."]
    ResultUnspecified,
    #[serde(rename = "REACHABLE")]
    #[doc = "Packet originating from source is expected to reach destination."]
    Reachable,
    #[serde(rename = "UNREACHABLE")]
    #[doc = "Packet originating from source is expected to be dropped before reaching destination."]
    Unreachable,
    #[serde(rename = "AMBIGUOUS")]
    #[doc = "If the source and destination endpoint does not uniquely identify the test location in the network, and the reachability result contains multiple traces with mixed reachable and unreachable states, then this result is returned."]
    Ambiguous,
    #[serde(rename = "UNDETERMINED")]
    #[doc = "The reachability could not be determined. Possible reasons are: * Analysis is aborted due to permission error. User does not have read permission to the projects listed in the test. * Analysis is aborted due to internal errors. * Analysis is partially complete based on configurations where the user has permission. The Final state indicates that the packet is forwarded to another network where the user has no permission to access the configurations."]
    Undetermined,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the `RerunConnectivityTest` method."]
pub struct RerunConnectivityTestRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "For display only. Metadata associated with a Compute Engine route."]
pub struct RouteInfo {
    #[serde(rename = "destIpRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Destination IP range of the route."]
    pub dest_ip_range: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of a Compute Engine route."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "instanceTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Instance tags of the route."]
    pub instance_tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "networkUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of a Compute Engine network."]
    pub network_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextHop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Next hop of the route."]
    pub next_hop: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextHopType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of next hop."]
    pub next_hop_type: ::std::option::Option<RouteInfoNextHopTypeEnum>,
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Priority of the route."]
    pub priority: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "routeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of route."]
    pub route_type: ::std::option::Option<RouteInfoRouteTypeEnum>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of a Compute Engine route. Dynamic route from cloud router does not have a URI. Advertised route from Google Cloud VPC to on-premises network also does not have a URI."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of next hop."]
pub enum RouteInfoNextHopTypeEnum {
    #[serde(rename = "NEXT_HOP_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified type. Default value."]
    NextHopTypeUnspecified,
    #[serde(rename = "NEXT_HOP_IP")]
    #[doc = "Next hop is an IP address."]
    NextHopIp,
    #[serde(rename = "NEXT_HOP_INSTANCE")]
    #[doc = "Next hop is a Compute Engine instance."]
    NextHopInstance,
    #[serde(rename = "NEXT_HOP_NETWORK")]
    #[doc = "Next hop is a VPC network gateway."]
    NextHopNetwork,
    #[serde(rename = "NEXT_HOP_PEERING")]
    #[doc = "Next hop is a peering VPC."]
    NextHopPeering,
    #[serde(rename = "NEXT_HOP_INTERCONNECT")]
    #[doc = "Next hop is an interconnect."]
    NextHopInterconnect,
    #[serde(rename = "NEXT_HOP_VPN_TUNNEL")]
    #[doc = "Next hop is a VPN tunnel."]
    NextHopVpnTunnel,
    #[serde(rename = "NEXT_HOP_VPN_GATEWAY")]
    #[doc = "Next hop is a VPN Gateway. This scenario only happens when tracing connectivity from an on-premises network to GCP through a VPN. The analysis simulates a packet departing from the on-premises network through a VPN tunnel and arrives at a Cloud VPN gateway."]
    NextHopVpnGateway,
    #[serde(rename = "NEXT_HOP_INTERNET_GATEWAY")]
    #[doc = "Next hop is an internet gateway."]
    NextHopInternetGateway,
    #[serde(rename = "NEXT_HOP_BLACKHOLE")]
    #[doc = "Next hop is blackhole; that is, the next hop either does not exist or is not running."]
    NextHopBlackhole,
    #[serde(rename = "NEXT_HOP_ILB")]
    #[doc = "Next hop is the forwarding rule of an Internal Load Balancer."]
    NextHopIlb,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of route."]
pub enum RouteInfoRouteTypeEnum {
    #[serde(rename = "ROUTE_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified type. Default value."]
    RouteTypeUnspecified,
    #[serde(rename = "SUBNET")]
    #[doc = "Route is a subnet route automatically created by the system."]
    Subnet,
    #[serde(rename = "STATIC")]
    #[doc = "Static route created by the user including the default route to the Internet."]
    Static,
    #[serde(rename = "DYNAMIC")]
    #[doc = "Dynamic route exchanged between BGP peers."]
    Dynamic,
    #[serde(rename = "PEERING_SUBNET")]
    #[doc = "A subnet route received from peering network."]
    PeeringSubnet,
    #[serde(rename = "PEERING_STATIC")]
    #[doc = "A static route received from peering network."]
    PeeringStatic,
    #[serde(rename = "PEERING_DYNAMIC")]
    #[doc = "A dynamic route received from peering network."]
    PeeringDynamic,
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
#[doc = "A simulated forwarding path is composed of multiple steps. Each step has a well-defined state and an associated configuration."]
pub struct Step {
    #[serde(rename = "abort")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display info of the final state \"abort\" and reason."]
    pub abort: ::std::option::Option<::std::boxed::Box<AbortInfo>>,
    #[serde(rename = "causesDrop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This is a step that leads to the final state Drop."]
    pub causes_drop: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "deliver")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display info of the final state \"deliver\" and reason."]
    pub deliver: ::std::option::Option<::std::boxed::Box<DeliverInfo>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of the step. Usually this is a summary of the state."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "drop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display info of the final state \"drop\" and reason."]
    pub drop: ::std::option::Option<::std::boxed::Box<DropInfo>>,
    #[serde(rename = "endpoint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display info of the source and destination under analysis. The endpiont info in an intermediate state may differ with the initial input, as it might be modified by state like NAT, or Connection Proxy."]
    pub endpoint: ::std::option::Option<::std::boxed::Box<EndpointInfo>>,
    #[serde(rename = "firewall")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display info of a Compute Engine firewall rule."]
    pub firewall: ::std::option::Option<::std::boxed::Box<FirewallInfo>>,
    #[serde(rename = "forward")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display info of the final state \"forward\" and reason."]
    pub forward: ::std::option::Option<::std::boxed::Box<ForwardInfo>>,
    #[serde(rename = "forwardingRule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display info of a Compute Engine forwarding rule."]
    pub forwarding_rule: ::std::option::Option<::std::boxed::Box<ForwardingRuleInfo>>,
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display info of a Compute Engine instance."]
    pub instance: ::std::option::Option<::std::boxed::Box<InstanceInfo>>,
    #[serde(rename = "loadBalancer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display info of the load balancers."]
    pub load_balancer: ::std::option::Option<::std::boxed::Box<LoadBalancerInfo>>,
    #[serde(rename = "network")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display info of a GCP network."]
    pub network: ::std::option::Option<::std::boxed::Box<NetworkInfo>>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Project ID that contains the configuration this step is validating."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "route")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display info of a Compute Engine route."]
    pub route: ::std::option::Option<::std::boxed::Box<RouteInfo>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Each step is in one of the pre-defined states."]
    pub state: ::std::option::Option<StepStateEnum>,
    #[serde(rename = "vpnGateway")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display info of a Compute Engine VPN gateway."]
    pub vpn_gateway: ::std::option::Option<::std::boxed::Box<VpnGatewayInfo>>,
    #[serde(rename = "vpnTunnel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display info of a Compute Engine VPN tunnel."]
    pub vpn_tunnel: ::std::option::Option<::std::boxed::Box<VpnTunnelInfo>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Each step is in one of the pre-defined states."]
pub enum StepStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Unspecified state."]
    StateUnspecified,
    #[serde(rename = "START_FROM_INSTANCE")]
    #[doc = "Initial state: packet originating from a Compute Engine instance. An InstanceInfo will be populated with starting instance info."]
    StartFromInstance,
    #[serde(rename = "START_FROM_INTERNET")]
    #[doc = "Initial state: packet originating from Internet. The endpoint info will be populated."]
    StartFromInternet,
    #[serde(rename = "START_FROM_PRIVATE_NETWORK")]
    #[doc = "Initial state: packet originating from a VPC or on-premises network with internal source IP. If the source is a VPC network visible to the user, a NetworkInfo will be populated with details of the network."]
    StartFromPrivateNetwork,
    #[serde(rename = "APPLY_INGRESS_FIREWALL_RULE")]
    #[doc = "Config checking state: verify ingress firewall rule."]
    ApplyIngressFirewallRule,
    #[serde(rename = "APPLY_EGRESS_FIREWALL_RULE")]
    #[doc = "Config checking state: verify egress firewall rule."]
    ApplyEgressFirewallRule,
    #[serde(rename = "APPLY_ROUTE")]
    #[doc = "Config checking state: verify route."]
    ApplyRoute,
    #[serde(rename = "APPLY_FORWARDING_RULE")]
    #[doc = "Config checking state: match forwarding rule."]
    ApplyForwardingRule,
    #[serde(rename = "SPOOFING_APPROVED")]
    #[doc = "Config checking state: packet sent or received under foreign IP address and allowed."]
    SpoofingApproved,
    #[serde(rename = "ARRIVE_AT_INSTANCE")]
    #[doc = "Forwarding state: arriving at a Compute Engine instance."]
    ArriveAtInstance,
    #[serde(rename = "ARRIVE_AT_INTERNAL_LOAD_BALANCER")]
    #[doc = "Forwarding state: arriving at a Compute Engine internal load balancer."]
    ArriveAtInternalLoadBalancer,
    #[serde(rename = "ARRIVE_AT_EXTERNAL_LOAD_BALANCER")]
    #[doc = "Forwarding state: arriving at a Compute Engine external load balancer."]
    ArriveAtExternalLoadBalancer,
    #[serde(rename = "ARRIVE_AT_VPN_GATEWAY")]
    #[doc = "Forwarding state: arriving at a Cloud VPN gateway."]
    ArriveAtVpnGateway,
    #[serde(rename = "ARRIVE_AT_VPN_TUNNEL")]
    #[doc = "Forwarding state: arriving at a Cloud VPN tunnel."]
    ArriveAtVpnTunnel,
    #[serde(rename = "NAT")]
    #[doc = "Transition state: packet header translated."]
    Nat,
    #[serde(rename = "PROXY_CONNECTION")]
    #[doc = "Transition state: original connection is terminated and a new proxied connection is initiated."]
    ProxyConnection,
    #[serde(rename = "DELIVER")]
    #[doc = "Final state: packet delivered."]
    Deliver,
    #[serde(rename = "DROP")]
    #[doc = "Final state: packet dropped."]
    Drop,
    #[serde(rename = "FORWARD")]
    #[doc = "Final state: packet forwarded to a network with an unknown configuration."]
    Forward,
    #[serde(rename = "ABORT")]
    #[doc = "Final state: analysis is aborted."]
    Abort,
    #[serde(rename = "VIEWER_PERMISSION_MISSING")]
    #[doc = "Special state: viewer of the test result does not have permission to see the configuration in this step."]
    ViewerPermissionMissing,
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
#[doc = "Trace represents one simulated packet forwarding path. - Each trace contains multiple ordered steps. - Each step is in a particular state and has an associated configuration. - State is categorized as a final or non-final state. - Each final state has a reason associated with it. - Each trace must end with a final state (the last step). |---------------------Trace----------------------| Step1(State) Step2(State) --- StepN(State(final)) "]
pub struct Trace {
    #[serde(rename = "endpointInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Derived from the source and destination endpoints definition, and validated by the data plane model. If there are multiple traces starting from different source locations, then the endpoint_info may be different between traces."]
    pub endpoint_info: ::std::option::Option<::std::boxed::Box<EndpointInfo>>,
    #[serde(rename = "steps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A trace of a test contains multiple steps from the initial state to the final state (delivered, dropped, forwarded, or aborted). The steps are ordered by the processing sequence within the simulated network state machine. It is critical to preserve the order of the steps and avoid reordering or sorting them."]
    pub steps: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Step>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "For display only. Metadata associated with a Compute Engine VPN gateway."]
pub struct VpnGatewayInfo {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of a VPN gateway."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IP address of the VPN gateway."]
    pub ip_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "networkUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of a Compute Engine network where the VPN gateway is configured."]
    pub network_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of a GCP region where this VPN gateway is configured."]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of a VPN gateway."]
    pub uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "vpnTunnelUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A VPN tunnel that is associated with this VPN gateway. There may be multiple VPN tunnels configured on a VPN gateway, and only the one relevant to the test is displayed."]
    pub vpn_tunnel_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "For display only. Metadata associated with a Compute Engine VPN tunnel."]
pub struct VpnTunnelInfo {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of a VPN tunnel."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "networkUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of a Compute Engine network where the VPN tunnel is configured."]
    pub network_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of a GCP region where this VPN tunnel is configured."]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(rename = "remoteGateway")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of a VPN gateway at remote end of the tunnel."]
    pub remote_gateway: ::std::option::Option<::std::string::String>,
    #[serde(rename = "remoteGatewayIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Remote VPN gateway's IP address."]
    pub remote_gateway_ip: ::std::option::Option<::std::string::String>,
    #[serde(rename = "routingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the routing policy."]
    pub routing_type: ::std::option::Option<VpnTunnelInfoRoutingTypeEnum>,
    #[serde(rename = "sourceGateway")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of the VPN gateway at local end of the tunnel."]
    pub source_gateway: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceGatewayIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Local VPN gateway's IP address."]
    pub source_gateway_ip: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of a VPN tunnel."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of the routing policy."]
pub enum VpnTunnelInfoRoutingTypeEnum {
    #[serde(rename = "ROUTING_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified type. Default value."]
    RoutingTypeUnspecified,
    #[serde(rename = "ROUTE_BASED")]
    #[doc = "Route based VPN."]
    RouteBased,
    #[serde(rename = "POLICY_BASED")]
    #[doc = "Policy based routing."]
    PolicyBased,
    #[serde(rename = "DYNAMIC")]
    #[doc = "Dynamic (BGP) routing."]
    Dynamic,
}
