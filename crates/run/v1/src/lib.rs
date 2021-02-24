#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information for connecting over HTTP(s)."]
pub struct Addressable {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
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
#[doc = "A domain that a user has been authorized to administer. To authorize use of a domain, verify ownership via [Webmaster Central](https://www.google.com/webmasters/verification/home)."]
pub struct AuthorizedDomain {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Relative name of the domain authorized for use. Example: `example.com`."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated Read only. Full path to the `AuthorizedDomain` resource in the API. Example: `projects/myproject/authorizedDomains/example.com`."]
    pub name: ::std::option::Option<::std::string::String>,
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
#[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported ConfigMapEnvSource selects a ConfigMap to populate the environment variables with. The contents of the target ConfigMap's Data field will represent the key-value pairs as environment variables."]
pub struct ConfigMapEnvSource {
    #[serde(rename = "localObjectReference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field should not be used directly as it is meant to be inlined directly into the message. Use the \"name\" field instead."]
    pub local_object_reference: ::std::option::Option<::std::boxed::Box<LocalObjectReference>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported The ConfigMap to select from."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "optional")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Specify whether the ConfigMap must be defined"]
    pub optional: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported Selects a key from a ConfigMap."]
pub struct ConfigMapKeySelector {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported The key to select."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "localObjectReference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field should not be used directly as it is meant to be inlined directly into the message. Use the \"name\" field instead."]
    pub local_object_reference: ::std::option::Option<::std::boxed::Box<LocalObjectReference>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported The ConfigMap to select from."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "optional")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Specify whether the ConfigMap or its key must be defined"]
    pub optional: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported Adapts a ConfigMap into a volume. The contents of the target ConfigMap's Data field will be presented in a volume as files using the keys in the Data field as the file names, unless the items element is populated with specific mappings of keys to paths."]
pub struct ConfigMapVolumeSource {
    #[serde(rename = "defaultMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Mode bits to use on created files by default. Must be a value between 0 and 0777. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set."]
    pub default_mode: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<KeyToPath>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported Name of the config."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "optional")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Specify whether the Secret or its keys must be defined."]
    pub optional: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration represents the \"floating HEAD\" of a linear history of Revisions, and optionally how the containers those revisions reference are built. Users create new Revisions by updating the Configuration's spec. The \"latest created\" revision's name is available under status, as is the \"latest ready\" revision's name. See also: https://github.com/knative/serving/blob/master/docs/spec/overview.md#configuration"]
pub struct Configuration {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API version for this call such as \"serving.knative.dev/v1\"."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource, in this case always \"Configuration\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata associated with this Configuration, including name, namespace, labels, and annotations."]
    pub metadata: ::std::option::Option<::std::boxed::Box<ObjectMeta>>,
    #[serde(rename = "spec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Spec holds the desired state of the Configuration (from the client)."]
    pub spec: ::std::option::Option<::std::boxed::Box<ConfigurationSpec>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status communicates the observed state of the Configuration (from the controller)."]
    pub status: ::std::option::Option<::std::boxed::Box<ConfigurationStatus>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ConfigurationSpec holds the desired state of the Configuration (from the client)."]
pub struct ConfigurationSpec {
    #[serde(rename = "template")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Template holds the latest specification for the Revision to be stamped out."]
    pub template: ::std::option::Option<::std::boxed::Box<RevisionTemplate>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ConfigurationStatus communicates the observed state of the Configuration (from the controller)."]
pub struct ConfigurationStatus {
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Conditions communicates information about ongoing/complete reconciliation processes that bring the \"spec\" inline with the observed state of the world."]
    pub conditions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudRunV1Condition>>>,
    #[serde(rename = "latestCreatedRevisionName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "LatestCreatedRevisionName is the last revision that was created from this Configuration. It might not be ready yet, for that use LatestReadyRevisionName."]
    pub latest_created_revision_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "latestReadyRevisionName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "LatestReadyRevisionName holds the name of the latest Revision stamped out from this Configuration that has had its \"Ready\" condition become \"True\"."]
    pub latest_ready_revision_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "observedGeneration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ObservedGeneration is the 'Generation' of the Configuration that was last processed by the controller. The observed generation is updated even if the controller failed to process the spec and create the Revision. Clients polling for completed reconciliation should poll until observedGeneration = metadata.generation, and the Ready condition's status is True or False."]
    pub observed_generation: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single application container. This specifies both the container to run, the command to run in the container and the arguments to supply to it. Note that additional arguments may be supplied by the system to the container at runtime."]
pub struct Container {
    #[serde(rename = "args")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: supported Cloud Run for Anthos: supported Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell"]
    pub args: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub command: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "env")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: supported Cloud Run for Anthos: supported List of environment variables to set in the container."]
    pub env: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EnvVar>>>,
    #[serde(rename = "envFrom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated."]
    pub env_from: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EnvFromSource>>>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: only supports containers from Google Container Registry Cloud Run for Anthos: supported URL of the Container image. More info: https://kubernetes.io/docs/concepts/containers/images"]
    pub image: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imagePullPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images"]
    pub image_pull_policy: ::std::option::Option<::std::string::String>,
    #[serde(rename = "livenessProbe")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Periodic probe of container liveness. Container will be restarted if the probe fails. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub liveness_probe: ::std::option::Option<::std::boxed::Box<Probe>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Name of the container specified as a DNS_LABEL."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ports")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) List of ports to expose from the container. Only a single port can be specified. The specified ports must be listening on all interfaces (0.0.0.0) within the container to be accessible. If omitted, a port number will be chosen and passed to the container through the PORT environment variable for the container to listen on."]
    pub ports: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContainerPort>>>,
    #[serde(rename = "readinessProbe")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub readiness_probe: ::std::option::Option<::std::boxed::Box<Probe>>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: supported Cloud Run for Anthos: supported Compute Resources required by this container. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources"]
    pub resources: ::std::option::Option<::std::boxed::Box<ResourceRequirements>>,
    #[serde(rename = "securityContext")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Security options the pod should run with. More info: https://kubernetes.io/docs/concepts/policy/security-context/ More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/"]
    pub security_context: ::std::option::Option<::std::boxed::Box<SecurityContext>>,
    #[serde(rename = "terminationMessagePath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log."]
    pub termination_message_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "terminationMessagePolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated."]
    pub termination_message_policy: ::std::option::Option<::std::string::String>,
    #[serde(rename = "volumeMounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Pod volumes to mount into the container's filesystem."]
    pub volume_mounts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<VolumeMount>>>,
    #[serde(rename = "workingDir")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image."]
    pub working_dir: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ContainerPort represents a network port in a single container."]
pub struct ContainerPort {
    #[serde(rename = "containerPort")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Port number the container listens on. This must be a valid port number, 0 < x < 65536."]
    pub container_port: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported If specified, used to specify which protocol to use. Allowed values are \"http1\" and \"h2c\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Protocol for port. Must be \"TCP\". Defaults to \"TCP\"."]
    pub protocol: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Resource to hold the state and status of a user's domain mapping. NOTE: This resource is currently in Beta."]
pub struct DomainMapping {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API version for this call such as \"domains.cloudrun.com/v1\"."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource, in this case \"DomainMapping\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata associated with this BuildTemplate."]
    pub metadata: ::std::option::Option<::std::boxed::Box<ObjectMeta>>,
    #[serde(rename = "spec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The spec for this DomainMapping."]
    pub spec: ::std::option::Option<::std::boxed::Box<DomainMappingSpec>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current status of the DomainMapping."]
    pub status: ::std::option::Option<::std::boxed::Box<DomainMappingStatus>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The desired state of the Domain Mapping."]
pub struct DomainMappingSpec {
    #[serde(rename = "certificateMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The mode of the certificate."]
    pub certificate_mode: ::std::option::Option<DomainMappingSpecCertificateModeEnum>,
    #[serde(rename = "forceOverride")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, the mapping will override any mapping set before this spec was set. It is recommended that the user leaves this empty to receive an error warning about a potential conflict and only set it once the respective UI has given such a warning."]
    pub force_override: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "routeName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the Knative Route that this DomainMapping applies to. The route must exist."]
    pub route_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The mode of the certificate."]
pub enum DomainMappingSpecCertificateModeEnum {
    #[serde(rename = "CERTIFICATE_MODE_UNSPECIFIED")]
    #[doc = ""]
    CertificateModeUnspecified,
    #[serde(rename = "NONE")]
    #[doc = "Do not provision an HTTPS certificate."]
    None,
    #[serde(rename = "AUTOMATIC")]
    #[doc = "Automatically provisions an HTTPS certificate via GoogleCA or LetsEncrypt."]
    Automatic,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The current state of the Domain Mapping."]
pub struct DomainMappingStatus {
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Array of observed DomainMappingConditions, indicating the current state of the DomainMapping."]
    pub conditions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudRunV1Condition>>>,
    #[serde(rename = "mappedRouteName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the route that the mapping currently points to."]
    pub mapped_route_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "observedGeneration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ObservedGeneration is the 'Generation' of the DomainMapping that was last processed by the controller. Clients polling for completed reconciliation should poll until observedGeneration = metadata.generation and the Ready condition's status is True or False."]
    pub observed_generation: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "resourceRecords")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource records required to configure this domain mapping. These records must be added to the domain's DNS configuration in order to serve the application via this domain mapping."]
    pub resource_records: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResourceRecord>>>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run on GKE: supported Holds the URL that will serve the traffic of the DomainMapping. +optional"]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported EnvFromSource represents the source of a set of ConfigMaps"]
pub struct EnvFromSource {
    #[serde(rename = "configMapRef")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported The ConfigMap to select from"]
    pub config_map_ref: ::std::option::Option<::std::boxed::Box<ConfigMapEnvSource>>,
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported An optional identifier to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER."]
    pub prefix: ::std::option::Option<::std::string::String>,
    #[serde(rename = "secretRef")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported The Secret to select from"]
    pub secret_ref: ::std::option::Option<::std::boxed::Box<SecretEnvSource>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "EnvVar represents an environment variable present in a Container."]
pub struct EnvVar {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the environment variable. Must be a C_IDENTIFIER."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Variable references $(VAR_NAME) are expanded using the previous defined environment variables in the container and any route environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to \"\"."]
    pub value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "valueFrom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Source for the environment variable's value. Cannot be used if value is not empty."]
    pub value_from: ::std::option::Option<::std::boxed::Box<EnvVarSource>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported EnvVarSource represents a source for the value of an EnvVar."]
pub struct EnvVarSource {
    #[serde(rename = "configMapKeyRef")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Selects a key of a ConfigMap."]
    pub config_map_key_ref: ::std::option::Option<::std::boxed::Box<ConfigMapKeySelector>>,
    #[serde(rename = "secretKeyRef")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Selects a key of a secret in the pod's namespace"]
    pub secret_key_ref: ::std::option::Option<::std::boxed::Box<SecretKeySelector>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported ExecAction describes a \"run in container\" action."]
pub struct ExecAction {
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Command is the command line to execute inside the container, the working directory for the command is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy."]
    pub command: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
#[doc = "Condition defines a generic condition for a Resource"]
pub struct GoogleCloudRunV1Condition {
    #[serde(rename = "lastTransitionTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Last time the condition transitioned from one status to another."]
    pub last_transition_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Human readable message indicating details about the current status."]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. One-word CamelCase reason for the condition's last transition."]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. How to interpret failures of this condition, one of Error, Warning, Info"]
    pub severity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the condition, one of True, False, Unknown."]
    pub status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "type is used to communicate the status of the reconciliation process. See also: https://github.com/knative/serving/blob/master/docs/spec/errors.md#error-conditions-and-reporting Types common to all resources include: * \"Ready\": True when the Resource is ready."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported HTTPGetAction describes an action based on HTTP Get requests."]
pub struct HttpGetAction {
    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Host name to connect to, defaults to the pod IP. You probably want to set \"Host\" in httpHeaders instead."]
    pub host: ::std::option::Option<::std::string::String>,
    #[serde(rename = "httpHeaders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Custom headers to set in the request. HTTP allows repeated headers."]
    pub http_headers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HttpHeader>>>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Path to access on the HTTP server."]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scheme")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Scheme to use for connecting to the host. Defaults to HTTP."]
    pub scheme: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported HTTPHeader describes a custom header to be used in HTTP probes"]
pub struct HttpHeader {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported The header field name"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported The header field value"]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported Maps a string key to a path within a volume."]
pub struct KeyToPath {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported The key to project."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Mode bits to use on this file, must be a value between 0000 and 0777. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set."]
    pub mode: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported The relative path of the file to map the key to. May not be an absolute path. May not contain the path element '..'. May not start with the string '..'."]
    pub path: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of Authorized Domains."]
pub struct ListAuthorizedDomainsResponse {
    #[serde(rename = "domains")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The authorized domains belonging to the user."]
    pub domains: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuthorizedDomain>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token for fetching the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ListConfigurationsResponse is a list of Configuration resources."]
pub struct ListConfigurationsResponse {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API version for this call such as \"serving.knative.dev/v1\"."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of Configurations."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Configuration>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this resource, in this case \"ConfigurationList\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata associated with this Configuration list."]
    pub metadata: ::std::option::Option<::std::boxed::Box<ListMeta>>,
    #[serde(rename = "unreachable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locations that could not be reached."]
    pub unreachable: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ListDomainMappingsResponse is a list of DomainMapping resources."]
pub struct ListDomainMappingsResponse {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API version for this call such as \"domains.cloudrun.com/v1\"."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of DomainMappings."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DomainMapping>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this resource, in this case \"DomainMappingList\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata associated with this DomainMapping list."]
    pub metadata: ::std::option::Option<::std::boxed::Box<ListMeta>>,
    #[serde(rename = "unreachable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locations that could not be reached."]
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
#[doc = "ListMeta describes metadata that synthetic resources must have, including lists and various status objects. A resource may have only one of {ObjectMeta, ListMeta}."]
pub struct ListMeta {
    #[serde(rename = "continue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "continue may be set if the user set a limit on the number of items returned, and indicates that the server has more data available. The value is opaque and may be used to issue another request to the endpoint that served this list to retrieve the next set of available objects. Continuing a list may not be possible if the server configuration has changed or more than a few minutes have passed. The resourceVersion field returned when using this continue value will be identical to the value in the first response."]
    pub _continue: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "String that identifies the server's internal version of this object that can be used by clients to determine when objects have changed. Value must be treated as opaque by clients and passed unmodified back to the server. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#concurrency-control-and-consistency +optional"]
    pub resource_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "SelfLink is a URL representing this object. Populated by the system. Read-only. +optional"]
    pub self_link: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ListRevisionsResponse is a list of Revision resources."]
pub struct ListRevisionsResponse {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API version for this call such as \"serving.knative.dev/v1\"."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of Revisions."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Revision>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this resource, in this case \"RevisionList\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata associated with this revision list."]
    pub metadata: ::std::option::Option<::std::boxed::Box<ListMeta>>,
    #[serde(rename = "unreachable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locations that could not be reached."]
    pub unreachable: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ListRoutesResponse is a list of Route resources."]
pub struct ListRoutesResponse {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API version for this call such as \"serving.knative.dev/v1\"."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of Routes."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Route>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this resource, in this case always \"RouteList\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata associated with this Route list."]
    pub metadata: ::std::option::Option<::std::boxed::Box<ListMeta>>,
    #[serde(rename = "unreachable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locations that could not be reached."]
    pub unreachable: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of Service resources."]
pub struct ListServicesResponse {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API version for this call such as \"serving.knative.dev/v1\"."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of Services."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Service>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this resource, in this case \"ServiceList\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata associated with this Service list."]
    pub metadata: ::std::option::Option<::std::boxed::Box<ListMeta>>,
    #[serde(rename = "unreachable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locations that could not be reached."]
    pub unreachable: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace."]
pub struct LocalObjectReference {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names"]
    pub name: ::std::option::Option<::std::string::String>,
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
#[doc = "k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMeta is metadata that all persisted resources must have, which includes all objects users must create."]
pub struct ObjectMeta {
    #[serde(rename = "annotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations"]
    pub annotations:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "clusterName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported The name of the cluster which the object belongs to. This is used to distinguish resources with same name and namespace in different clusters. This field is not set anywhere right now and apiserver is going to ignore it if set in create or update request."]
    pub cluster_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creationTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) CreationTimestamp is a timestamp representing the server time when this object was created. It is not guaranteed to be set in happens-before order across separate operations. Clients may not set this value. It is represented in RFC3339 form and is in UTC. Populated by the system. Read-only. Null for lists. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata"]
    pub creation_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deletionGracePeriodSeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Number of seconds allowed for this object to gracefully terminate before it will be removed from the system. Only set when deletionTimestamp is also set. May only be shortened. Read-only."]
    pub deletion_grace_period_seconds: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "deletionTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported DeletionTimestamp is RFC 3339 date and time at which this resource will be deleted. This field is set by the server when a graceful deletion is requested by the user, and is not directly settable by a client. The resource is expected to be deleted (no longer visible from resource lists, and not reachable by name) after the time in this field, once the finalizers list is empty. As long as the finalizers list contains items, deletion is blocked. Once the deletionTimestamp is set, this value may not be unset or be set further into the future, although it may be shortened or the resource may be deleted prior to this time. For example, a user may request that a pod is deleted in 30 seconds. The Kubelet will react by sending a graceful termination signal to the containers in the pod. After that 30 seconds, the Kubelet will send a hard termination signal (SIGKILL) to the container and after cleanup, remove the pod from the API. In the presence of network partitions, this object may still exist after this timestamp, until an administrator or automated process can determine the resource is fully terminated. If not set, graceful deletion of the object has not been requested. Populated by the system when a graceful deletion is requested. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata"]
    pub deletion_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "finalizers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Must be empty before the object is deleted from the registry. Each entry is an identifier for the responsible component that will remove the entry from the list. If the deletionTimestamp of the object is non-nil, entries in this list can only be removed. +patchStrategy=merge"]
    pub finalizers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "generateName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported GenerateName is an optional prefix, used by the server, to generate a unique name ONLY IF the Name field has not been provided. If this field is used, the name returned to the client will be different than the name passed. This value will also be combined with a unique suffix. The provided value has the same validation rules as the Name field, and may be truncated by the length of the suffix required to make the value unique on the server. If this field is specified and the generated name exists, the server will NOT return a 409 - instead, it will either return 201 Created or 500 with Reason ServerTimeout indicating a unique name could not be found in the time allotted, and the client should retry (optionally after the time indicated in the Retry-After header). Applied only if Name is not specified. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#idempotency string generateName = 2;"]
    pub generate_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "generation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) A sequence number representing a specific generation of the desired state. Populated by the system. Read-only."]
    pub generation: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and routes. More info: http://kubernetes.io/docs/user-guide/labels"]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name must be unique within a namespace, within a Cloud Run region. Is required when creating resources, although some resources may allow a client to request the generation of an appropriate name automatically. Name is primarily intended for creation idempotence and configuration definition. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/identifiers#names +optional"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Namespace defines the space within each name must be unique, within a Cloud Run region. In Cloud Run the namespace must be equal to either the project ID or project number."]
    pub namespace: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ownerReferences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported List of objects that own this object. If ALL objects in the list have been deleted, this object will be garbage collected."]
    pub owner_references: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OwnerReference>>>,
    #[serde(rename = "resourceVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) An opaque value that represents the internal version of this object that can be used by clients to determine when objects have changed. May be used for optimistic concurrency, change detection, and the watch operation on a resource or set of resources. Clients must treat these values as opaque and passed unmodified back to the server. They may only be valid for a particular resource or set of resources. Populated by the system. Read-only. Value must be treated as opaque by clients. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency"]
    pub resource_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) SelfLink is a URL representing this object. Populated by the system. Read-only. string selfLink = 4;"]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) UID is the unique in time and space value for this object. It is typically generated by the server on successful creation of a resource and is not allowed to change on PUT operations. Populated by the system. Read-only. More info: http://kubernetes.io/docs/user-guide/identifiers#uids"]
    pub uid: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "OwnerReference contains enough information to let you identify an owning object. Currently, an owning object must be in the same namespace, so there is no namespace field."]
pub struct OwnerReference {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API version of the referent."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "blockOwnerDeletion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, AND if the owner has the \"foregroundDeletion\" finalizer, then the owner cannot be deleted from the key-value store until this reference is removed. Defaults to false. To set this field, a user needs \"delete\" permission of the owner, otherwise 422 (Unprocessable Entity) will be returned. +optional"]
    pub block_owner_deletion: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "controller")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, this reference points to the managing controller. +optional"]
    pub controller: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds"]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "UID of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#uids"]
    pub uid: ::std::option::Option<::std::string::String>,
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
#[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported Probe describes a health check to be performed against a container to determine whether it is alive or ready to receive traffic."]
pub struct Probe {
    #[serde(rename = "exec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported One and only one of the following should be specified. Exec specifies the action to take. A field inlined from the Handler message."]
    pub exec: ::std::option::Option<::std::boxed::Box<ExecAction>>,
    #[serde(rename = "failureThreshold")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1."]
    pub failure_threshold: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "httpGet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported HTTPGet specifies the http request to perform. A field inlined from the Handler message."]
    pub http_get: ::std::option::Option<::std::boxed::Box<HttpGetAction>>,
    #[serde(rename = "initialDelaySeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub initial_delay_seconds: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "periodSeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1."]
    pub period_seconds: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "successThreshold")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness. Minimum value is 1."]
    pub success_threshold: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "tcpSocket")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported A field inlined from the Handler message."]
    pub tcp_socket: ::std::option::Option<::std::boxed::Box<TcpSocketAction>>,
    #[serde(rename = "timeoutSeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    pub timeout_seconds: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A DNS resource record."]
pub struct ResourceRecord {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Relative name of the object affected by this record. Only applicable for `CNAME` records. Example: 'www'."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rrdata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Data for this record. Values vary by record type, as defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1)."]
    pub rrdata: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource record type. Example: `AAAA`."]
    pub _type: ::std::option::Option<ResourceRecordTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Resource record type. Example: `AAAA`."]
pub enum ResourceRecordTypeEnum {
    #[serde(rename = "RECORD_TYPE_UNSPECIFIED")]
    #[doc = "An unknown resource record."]
    RecordTypeUnspecified,
    #[serde(rename = "A")]
    #[doc = "An A resource record. Data is an IPv4 address."]
    A,
    #[serde(rename = "AAAA")]
    #[doc = "An AAAA resource record. Data is an IPv6 address."]
    Aaaa,
    #[serde(rename = "CNAME")]
    #[doc = "A CNAME resource record. Data is a domain name to be aliased."]
    Cname,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ResourceRequirements describes the compute resource requirements."]
pub struct ResourceRequirements {
    #[serde(rename = "limits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: Only memory and CPU are supported. Note: The only supported values for CPU are '1', '2', and '4'. Setting 4 CPU requires at least 2Gi of memory. Cloud Run for Anthos: supported Limits describes the maximum amount of compute resources allowed. The values of the map is string form of the 'quantity' k8s type: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go"]
    pub limits: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "requests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: Only memory and CPU are supported. Note: The only supported values for CPU are '1' and '2'. Cloud Run for Anthos: supported Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. The values of the map is string form of the 'quantity' k8s type: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go"]
    pub requests:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Revision is an immutable snapshot of code and configuration. A revision references a container image. Revisions are created by updates to a Configuration. See also: https://github.com/knative/serving/blob/master/docs/spec/overview.md#revision"]
pub struct Revision {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API version for this call such as \"serving.knative.dev/v1\"."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this resource, in this case \"Revision\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata associated with this Revision, including name, namespace, labels, and annotations."]
    pub metadata: ::std::option::Option<::std::boxed::Box<ObjectMeta>>,
    #[serde(rename = "spec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Spec holds the desired state of the Revision (from the client)."]
    pub spec: ::std::option::Option<::std::boxed::Box<RevisionSpec>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status communicates the observed state of the Revision (from the controller)."]
    pub status: ::std::option::Option<::std::boxed::Box<RevisionStatus>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "RevisionSpec holds the desired state of the Revision (from the client)."]
pub struct RevisionSpec {
    #[serde(rename = "containerConcurrency")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) ContainerConcurrency specifies the maximum allowed in-flight (concurrent) requests per container instance of the Revision. Cloud Run fully managed: supported, defaults to 80 Cloud Run for Anthos: supported, defaults to 0, which means concurrency to the application is not limited, and the system decides the target concurrency for the autoscaler."]
    pub container_concurrency: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "containers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Containers holds the single container that defines the unit of execution for this Revision. In the context of a Revision, we disallow a number of fields on this Container, including: name and lifecycle. In Cloud Run, only a single container may be provided. The runtime contract is documented here: https://github.com/knative/serving/blob/master/docs/runtime-contract.md"]
    pub containers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Container>>>,
    #[serde(rename = "serviceAccountName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Email address of the IAM service account associated with the revision of the service. The service account represents the identity of the running revision, and determines what permissions the revision has. If not provided, the revision will use the project's default service account."]
    pub service_account_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeoutSeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "TimeoutSeconds holds the max duration the instance is allowed for responding to a request. Cloud Run fully managed: defaults to 300 seconds (5 minutes). Maximum allowed value is 900 seconds (15 minutes). Cloud Run for Anthos: defaults to 300 seconds (5 minutes). Maximum allowed value is configurable by the cluster operator."]
    pub timeout_seconds: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "volumes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub volumes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Volume>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "RevisionStatus communicates the observed state of the Revision (from the controller)."]
pub struct RevisionStatus {
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Conditions communicates information about ongoing/complete reconciliation processes that bring the \"spec\" inline with the observed state of the world. As a Revision is being prepared, it will incrementally update conditions. Revision-specific conditions include: * \"ResourcesAvailable\": True when underlying resources have been provisioned. * \"ContainerHealthy\": True when the Revision readiness check completes. * \"Active\": True when the Revision may receive traffic."]
    pub conditions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudRunV1Condition>>>,
    #[serde(rename = "imageDigest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ImageDigest holds the resolved digest for the image specified within .Spec.Container.Image. The digest is resolved during the creation of Revision. This field holds the digest value regardless of whether a tag or digest was originally specified in the Container object."]
    pub image_digest: ::std::option::Option<::std::string::String>,
    #[serde(rename = "logUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the generated logging url for this particular revision based on the revision url template specified in the controller's config. +optional"]
    pub log_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "observedGeneration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ObservedGeneration is the 'Generation' of the Revision that was last processed by the controller. Clients polling for completed reconciliation should poll until observedGeneration = metadata.generation, and the Ready condition's status is True or False."]
    pub observed_generation: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Not currently used by Cloud Run."]
    pub service_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "RevisionTemplateSpec describes the data a revision should have when created from a template. Based on: https://github.com/kubernetes/api/blob/e771f807/core/v1/types.go#L3179-L3190"]
pub struct RevisionTemplate {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional metadata for this Revision, including labels and annotations. Name will be generated by the Configuration. The following annotation keys set properties of the created revision: * `autoscaling.knative.dev/minScale` sets the minimum number of instances. * `autoscaling.knative.dev/maxScale` sets the maximum number of instances. * `run.googleapis.com/cloudsql-instances` sets Cloud SQL connections. Multiple values should be comma separated. * `run.googleapis.com/vpc-access-connector` sets a Serverless VPC Access connector. * `run.googleapis.com/vpc-access-egress` sets VPC egress. Supported values are `all` and `private-ranges-only`."]
    pub metadata: ::std::option::Option<::std::boxed::Box<ObjectMeta>>,
    #[serde(rename = "spec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "RevisionSpec holds the desired state of the Revision (from the client)."]
    pub spec: ::std::option::Option<::std::boxed::Box<RevisionSpec>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Route is responsible for configuring ingress over a collection of Revisions. Some of the Revisions a Route distributes traffic over may be specified by referencing the Configuration responsible for creating them; in these cases the Route is additionally responsible for monitoring the Configuration for \"latest ready\" revision changes, and smoothly rolling out latest revisions. See also: https://github.com/knative/serving/blob/master/docs/spec/overview.md#route Cloud Run currently supports referencing a single Configuration to automatically deploy the \"latest ready\" Revision from that Configuration."]
pub struct Route {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API version for this call such as \"serving.knative.dev/v1\"."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of this resource, in this case always \"Route\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata associated with this Route, including name, namespace, labels, and annotations."]
    pub metadata: ::std::option::Option<::std::boxed::Box<ObjectMeta>>,
    #[serde(rename = "spec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Spec holds the desired state of the Route (from the client)."]
    pub spec: ::std::option::Option<::std::boxed::Box<RouteSpec>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status communicates the observed state of the Route (from the controller)."]
    pub status: ::std::option::Option<::std::boxed::Box<RouteStatus>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "RouteSpec holds the desired state of the Route (from the client)."]
pub struct RouteSpec {
    #[serde(rename = "traffic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Traffic specifies how to distribute traffic over a collection of Knative Revisions and Configurations. Cloud Run currently supports a single configurationName."]
    pub traffic: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TrafficTarget>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "RouteStatus communicates the observed state of the Route (from the controller)."]
pub struct RouteStatus {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Similar to url, information on where the service is available on HTTP."]
    pub address: ::std::option::Option<::std::boxed::Box<Addressable>>,
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Conditions communicates information about ongoing/complete reconciliation processes that bring the \"spec\" inline with the observed state of the world."]
    pub conditions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudRunV1Condition>>>,
    #[serde(rename = "observedGeneration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ObservedGeneration is the 'Generation' of the Route that was last processed by the controller. Clients polling for completed reconciliation should poll until observedGeneration = metadata.generation and the Ready condition's status is True or False. Note that providing a trafficTarget that only has a configurationName will result in a Route that does not increment either its metadata.generation or its observedGeneration, as new \"latest ready\" revisions from the Configuration are processed without an update to the Route's spec."]
    pub observed_generation: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "traffic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Traffic holds the configured traffic distribution. These entries will always contain RevisionName references. When ConfigurationName appears in the spec, this will hold the LatestReadyRevisionName that we last observed."]
    pub traffic: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TrafficTarget>>>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL holds the url that will distribute traffic over the provided traffic targets. It generally has the form: https://{route-hash}-{project-hash}-{cluster-level-suffix}.a.run.app"]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported SecretEnvSource selects a Secret to populate the environment variables with. The contents of the target Secret's Data field will represent the key-value pairs as environment variables."]
pub struct SecretEnvSource {
    #[serde(rename = "localObjectReference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field should not be used directly as it is meant to be inlined directly into the message. Use the \"name\" field instead."]
    pub local_object_reference: ::std::option::Option<::std::boxed::Box<LocalObjectReference>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported The Secret to select from."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "optional")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Specify whether the Secret must be defined"]
    pub optional: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported SecretKeySelector selects a key of a Secret."]
pub struct SecretKeySelector {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported The key of the secret to select from. Must be a valid secret key."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "localObjectReference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field should not be used directly as it is meant to be inlined directly into the message. Use the \"name\" field instead."]
    pub local_object_reference: ::std::option::Option<::std::boxed::Box<LocalObjectReference>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported The name of the secret in the pod's namespace to select from."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "optional")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Specify whether the Secret or its key must be defined"]
    pub optional: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported The contents of the target Secret's Data field will be presented in a volume as files using the keys in the Data field as the file names."]
pub struct SecretVolumeSource {
    #[serde(rename = "defaultMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Mode bits to use on created files by default. Must be a value between 0000 and 0777. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set. NOTE: This is an integer representation of the mode bits. So, the integer value should look exactly as the chmod numeric notation, i.e. Unix chmod \"777\" (a=rwx) should have the integer value 777."]
    pub default_mode: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<KeyToPath>>>,
    #[serde(rename = "optional")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Specify whether the Secret or its keys must be defined."]
    pub optional: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "secretName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported Name of the secret in the container's namespace to use."]
    pub secret_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported SecurityContext holds security configuration that will be applied to a container. Some fields are present in both SecurityContext and PodSecurityContext. When both are set, the values in SecurityContext take precedence."]
pub struct SecurityContext {
    #[serde(rename = "runAsUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence."]
    pub run_as_user: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Service acts as a top-level container that manages a set of Routes and Configurations which implement a network service. Service exists to provide a singular abstraction which can be access controlled, reasoned about, and which encapsulates software lifecycle decisions such as rollout policy and team resource ownership. Service acts only as an orchestrator of the underlying Routes and Configurations (much as a kubernetes Deployment orchestrates ReplicaSets). The Service's controller will track the statuses of its owned Configuration and Route, reflecting their statuses and conditions as its own. See also: https://github.com/knative/serving/blob/master/docs/spec/overview.md#service"]
pub struct Service {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API version for this call such as \"serving.knative.dev/v1\"."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of resource, in this case \"Service\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata associated with this Service, including name, namespace, labels, and annotations."]
    pub metadata: ::std::option::Option<::std::boxed::Box<ObjectMeta>>,
    #[serde(rename = "spec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Spec holds the desired state of the Service (from the client)."]
    pub spec: ::std::option::Option<::std::boxed::Box<ServiceSpec>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status communicates the observed state of the Service (from the controller)."]
    pub status: ::std::option::Option<::std::boxed::Box<ServiceStatus>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ServiceSpec holds the desired state of the Route (from the client), which is used to manipulate the underlying Route and Configuration(s)."]
pub struct ServiceSpec {
    #[serde(rename = "template")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Template holds the latest specification for the Revision to be stamped out."]
    pub template: ::std::option::Option<::std::boxed::Box<RevisionTemplate>>,
    #[serde(rename = "traffic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Traffic specifies how to distribute traffic over a collection of Knative Revisions and Configurations."]
    pub traffic: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TrafficTarget>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The current state of the Service. Output only."]
pub struct ServiceStatus {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "From RouteStatus. Similar to url, information on where the service is available on HTTP."]
    pub address: ::std::option::Option<::std::boxed::Box<Addressable>>,
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Conditions communicates information about ongoing/complete reconciliation processes that bring the \"spec\" inline with the observed state of the world. Service-specific conditions include: * \"ConfigurationsReady\": true when the underlying Configuration is ready. * \"RoutesReady\": true when the underlying Route is ready. * \"Ready\": true when both the underlying Route and Configuration are ready."]
    pub conditions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudRunV1Condition>>>,
    #[serde(rename = "latestCreatedRevisionName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "From ConfigurationStatus. LatestCreatedRevisionName is the last revision that was created from this Service's Configuration. It might not be ready yet, for that use LatestReadyRevisionName."]
    pub latest_created_revision_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "latestReadyRevisionName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "From ConfigurationStatus. LatestReadyRevisionName holds the name of the latest Revision stamped out from this Service's Configuration that has had its \"Ready\" condition become \"True\"."]
    pub latest_ready_revision_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "observedGeneration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ObservedGeneration is the 'Generation' of the Route that was last processed by the controller. Clients polling for completed reconciliation should poll until observedGeneration = metadata.generation and the Ready condition's status is True or False."]
    pub observed_generation: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "traffic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "From RouteStatus. Traffic holds the configured traffic distribution. These entries will always contain RevisionName references. When ConfigurationName appears in the spec, this will hold the LatestReadyRevisionName that we last observed."]
    pub traffic: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TrafficTarget>>>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "From RouteStatus. URL holds the url that will distribute traffic over the provided traffic targets. It generally has the form https://{route-hash}-{project-hash}-{cluster-level-suffix}.a.run.app"]
    pub url: ::std::option::Option<::std::string::String>,
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
#[doc = "Status is a return value for calls that don't return other objects"]
pub struct Status {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Suggested HTTP return code for this status, 0 if not set. +optional"]
    pub code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Extended data associated with the reason. Each reason may define its own extended details. This field is optional and the data returned is not guaranteed to conform to any schema except that defined by the reason type. +optional"]
    pub details: ::std::option::Option<::std::boxed::Box<StatusDetails>>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A human-readable description of the status of this operation. +optional"]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds +optional"]
    pub metadata: ::std::option::Option<::std::boxed::Box<ListMeta>>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A machine-readable description of why this operation is in the \"Failure\" status. If this value is empty there is no information available. A Reason clarifies an HTTP status code but does not override it. +optional"]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the operation. One of: \"Success\" or \"Failure\". More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status +optional"]
    pub status: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "StatusCause provides more information about an api.Status failure, including cases when multiple errors are encountered."]
pub struct StatusCause {
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The field of the resource that has caused this error, as named by its JSON serialization. May include dot and postfix notation for nested attributes. Arrays are zero-indexed. Fields may appear more than once in an array of causes due to fields having multiple errors. Optional. Examples: \"name\" - the field \"name\" on the current resource \"items[0].name\" - the field \"name\" on the first array entry in \"items\" +optional"]
    pub field: ::std::option::Option<::std::string::String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A human-readable description of the cause of the error. This field may be presented as-is to a reader. +optional"]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A machine-readable description of the cause of the error. If this value is empty there is no information available. +optional"]
    pub reason: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "StatusDetails is a set of additional properties that MAY be set by the server to provide additional information about a response. The Reason field of a Status object defines what attributes will be set. Clients must ignore fields that do not match the defined type of each attribute, and should assume that any attribute may be empty, invalid, or under defined."]
pub struct StatusDetails {
    #[serde(rename = "causes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Causes array includes more details associated with the StatusReason failure. Not all StatusReasons may provide detailed causes. +optional"]
    pub causes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StatusCause>>>,
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The group attribute of the resource associated with the status StatusReason. +optional"]
    pub group: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind attribute of the resource associated with the status StatusReason. On some operations may differ from the requested resource Kind. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds +optional"]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name attribute of the resource associated with the status StatusReason (when there is a single name which can be described). +optional"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "retryAfterSeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If specified, the time in seconds before the operation should be retried. Some errors may indicate the client must take an alternate action - for those errors this field may indicate how long to wait before taking the alternate action. +optional"]
    pub retry_after_seconds: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "UID of the resource. (when there is a single resource which can be described). More info: http://kubernetes.io/docs/user-guide/identifiers#uids +optional"]
    pub uid: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported TCPSocketAction describes an action based on opening a socket"]
pub struct TcpSocketAction {
    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Optional: Host name to connect to, defaults to the pod IP."]
    pub host: ::std::option::Option<::std::string::String>,
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME. This field is currently limited to integer types only because of proto's inability to properly support the IntOrString golang type."]
    pub port: ::std::option::Option<::std::primitive::i64>,
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
#[doc = "TrafficTarget holds a single entry of the routing table for a Route."]
pub struct TrafficTarget {
    #[serde(rename = "configurationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ConfigurationName of a configuration to whose latest revision we will send this portion of traffic. When the \"status.latestReadyRevisionName\" of the referenced configuration changes, we will automatically migrate traffic from the prior \"latest ready\" revision to the new one. This field is never set in Route's status, only its spec. This is mutually exclusive with RevisionName. Cloud Run currently supports a single ConfigurationName."]
    pub configuration_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "latestRevision")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "LatestRevision may be optionally provided to indicate that the latest ready Revision of the Configuration should be used for this traffic target. When provided LatestRevision must be true if RevisionName is empty; it must be false when RevisionName is non-empty. +optional"]
    pub latest_revision: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "percent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Percent specifies percent of the traffic to this Revision or Configuration. This defaults to zero if unspecified. Cloud Run currently requires 100 percent for a single ConfigurationName TrafficTarget entry."]
    pub percent: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "revisionName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "RevisionName of a specific revision to which to send this portion of traffic. This is mutually exclusive with ConfigurationName. Providing RevisionName in spec is not currently supported by Cloud Run."]
    pub revision_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tag is optionally used to expose a dedicated url for referencing this target exclusively. +optional"]
    pub tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. URL displays the URL for accessing tagged traffic targets. URL is displayed in status, and is disallowed on spec. URL must contain a scheme (e.g. http://) and a hostname, but may not contain anything else (e.g. basic auth, url path, etc. Not currently supported in Cloud Run."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported Volume represents a named volume in a container."]
pub struct Volume {
    #[serde(rename = "configMap")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported"]
    pub config_map: ::std::option::Option<::std::boxed::Box<ConfigMapVolumeSource>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported Volume's name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "secret")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported"]
    pub secret: ::std::option::Option<::std::boxed::Box<SecretVolumeSource>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported VolumeMount describes a mounting of a Volume within a container."]
pub struct VolumeMount {
    #[serde(rename = "mountPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported Path within the container at which the volume should be mounted. Must not contain ':'."]
    pub mount_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Run fully managed: not supported Cloud Run for Anthos: supported This must match the Name of a Volume."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Only true is accepted. Defaults to true."]
    pub read_only: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "subPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Cloud Run fully managed: not supported Cloud Run for Anthos: supported Path within the volume from which the container's volume should be mounted. Defaults to \"\" (volume's root)."]
    pub sub_path: ::std::option::Option<::std::string::String>,
}
