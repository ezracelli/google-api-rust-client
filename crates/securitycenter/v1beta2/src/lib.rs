#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration of a module."]
pub struct Config {
    #[serde(rename = "moduleEnablementState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of enablement for the module at its level of the resource hierarchy."]
    pub module_enablement_state: ::std::option::Option<ConfigModuleEnablementStateEnum>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration value for the module. The absence of this field implies its inheritance from the parent."]
    pub value: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of enablement for the module at its level of the resource hierarchy."]
pub enum ConfigModuleEnablementStateEnum {
    #[serde(rename = "ENABLEMENT_STATE_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    EnablementStateUnspecified,
    #[serde(rename = "INHERITED")]
    #[doc = "State is inherited from the parent resource."]
    Inherited,
    #[serde(rename = "ENABLED")]
    #[doc = "State is enabled."]
    Enabled,
    #[serde(rename = "DISABLED")]
    #[doc = "State is disabled."]
    Disabled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Resource capturing the settings for the Container Threat Detection service."]
pub struct ContainerThreatDetectionSettings {
    #[serde(rename = "modules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configurations including the state of enablement for the service's different modules. The absence of a module in the map implies its configuration is inherited from its parent's."]
    pub modules:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Config>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the ContainerThreatDetectionSettings. Formats: * organizations/{organization}/containerThreatDetectionSettings * folders/{folder}/containerThreatDetectionSettings * projects/{project}/containerThreatDetectionSettings * projects/{project}/locations/{location}/clusters/{cluster}/containerThreatDetectionSettings"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The service account used by Container Threat Detection for scanning. Service accounts are scoped at the project level meaning this field will be empty at any level above a project."]
    pub service_account: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceEnablementState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED."]
    pub service_enablement_state:
        ::std::option::Option<ContainerThreatDetectionSettingsServiceEnablementStateEnum>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the settings were last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED."]
pub enum ContainerThreatDetectionSettingsServiceEnablementStateEnum {
    #[serde(rename = "ENABLEMENT_STATE_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    EnablementStateUnspecified,
    #[serde(rename = "INHERITED")]
    #[doc = "State is inherited from the parent resource."]
    Inherited,
    #[serde(rename = "ENABLED")]
    #[doc = "State is enabled."]
    Enabled,
    #[serde(rename = "DISABLED")]
    #[doc = "State is disabled."]
    Disabled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of a subscription."]
pub struct Details {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time the subscription has or will end."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time the subscription has or will start."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of subscription"]
    pub _type: ::std::option::Option<DetailsTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of subscription"]
pub enum DetailsTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    TypeUnspecified,
    #[serde(rename = "STANDARD")]
    #[doc = "The standard subscription."]
    Standard,
    #[serde(rename = "TRIAL")]
    #[doc = "The trial subscription."]
    Trial,
    #[serde(rename = "ALPHA")]
    #[doc = "The alpha subscription."]
    Alpha,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Resource capturing the settings for the Event Threat Detection service."]
pub struct EventThreatDetectionSettings {
    #[serde(rename = "modules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configurations including the state of enablement for the service's different modules. The absence of a module in the map implies its configuration is inherited from its parent's."]
    pub modules:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Config>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the EventThreatDetectionSettings. Formats: * organizations/{organization}/eventThreatDetectionSettings * folders/{folder}/eventThreatDetectionSettings * projects/{project}/eventThreatDetectionSettings"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceEnablementState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED."]
    pub service_enablement_state:
        ::std::option::Option<EventThreatDetectionSettingsServiceEnablementStateEnum>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the settings were last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED."]
pub enum EventThreatDetectionSettingsServiceEnablementStateEnum {
    #[serde(rename = "ENABLEMENT_STATE_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    EnablementStateUnspecified,
    #[serde(rename = "INHERITED")]
    #[doc = "State is inherited from the parent resource."]
    Inherited,
    #[serde(rename = "ENABLED")]
    #[doc = "State is enabled."]
    Enabled,
    #[serde(rename = "DISABLED")]
    #[doc = "State is disabled."]
    Disabled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Security Command Center finding. A finding is a record of assessment data like security, risk, health, or privacy, that is ingested into Security Command Center for presentation, notification, analysis, policy testing, and enforcement. For example, a cross-site scripting (XSS) vulnerability in an App Engine application is a finding."]
pub struct Finding {
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The additional taxonomy group within findings from a given source. This field is immutable after creation time. Example: \"XSS_FLASH_INJECTION\""]
    pub category: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the finding was created in Security Command Center."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "eventTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the event took place, or when an update to the finding occurred. For example, if the finding represents an open firewall it would capture the time the detector believes the firewall became open. The accuracy is determined by the detector. If the finding were to be resolved afterward, this time would reflect when the finding was resolved."]
    pub event_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "externalUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI that, if available, points to a web page outside of Security Command Center where additional information about the finding can be found. This field is guaranteed to be either empty or a well formed URL."]
    pub external_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The relative resource name of this finding. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: \"organizations/{organization_id}/sources/{source_id}/findings/{finding_id}\""]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The relative resource name of the source the finding belongs to. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name This field is immutable after creation time. For example: \"organizations/{organization_id}/sources/{source_id}\""]
    pub parent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For findings on Google Cloud resources, the full resource name of the Google Cloud resource this finding is for. See: https://cloud.google.com/apis/design/resource_names#full_resource_name When the finding is for a non-Google Cloud resource, the resourceName can be a customer or partner defined string. This field is immutable after creation time."]
    pub resource_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "securityMarks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. User specified security marks. These marks are entirely managed by the user and come from the SecurityMarks resource that belongs to the finding."]
    pub security_marks: ::std::option::Option<::std::boxed::Box<SecurityMarks>>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The severity of the finding. This field is managed by the source that writes the finding."]
    pub severity: ::std::option::Option<FindingSeverityEnum>,
    #[serde(rename = "sourceProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Source specific properties. These properties are managed by the source that writes the finding. The key names in the source_properties map must be between 1 and 255 characters, and must start with a letter and contain alphanumeric characters or underscores only."]
    pub source_properties:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of the finding."]
    pub state: ::std::option::Option<FindingStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The severity of the finding. This field is managed by the source that writes the finding."]
pub enum FindingSeverityEnum {
    #[serde(rename = "SEVERITY_UNSPECIFIED")]
    #[doc = "This value is used for findings when a source doesn't write a severity value."]
    SeverityUnspecified,
    #[serde(rename = "CRITICAL")]
    #[doc = "Vulnerability: A critical vulnerability is easily discoverable by an external actor, exploitable, and results in the direct ability to execute arbitrary code, exfiltrate data, and otherwise gain additional access and privileges to cloud resources and workloads. Examples include publicly accessible unprotected user data, public SSH access with weak or no passwords, etc. Threat: Indicates a threat that is able to access, modify, or delete data or execute unauthorized code within existing resources."]
    Critical,
    #[serde(rename = "HIGH")]
    #[doc = "Vulnerability: A high risk vulnerability can be easily discovered and exploited in combination with other vulnerabilities in order to gain direct access and the ability to execute arbitrary code, exfiltrate data, and otherwise gain additional access and privileges to cloud resources and workloads. An example is a database with weak or no passwords that is only accessible internally. This database could easily be compromised by an actor that had access to the internal network. Threat: Indicates a threat that is able to create new computational resources in an environment but not able to access data or execute code in existing resources."]
    High,
    #[serde(rename = "MEDIUM")]
    #[doc = "Vulnerability: A medium risk vulnerability could be used by an actor to gain access to resources or privileges that enable them to eventually (through multiple steps or a complex exploit) gain access and the ability to execute arbitrary code or exfiltrate data. An example is a service account with access to more projects than it should have. If an actor gains access to the service account, they could potentially use that access to manipulate a project the service account was not intended to. Threat: Indicates a threat that is able to cause operational impact but may not access data or execute unauthorized code."]
    Medium,
    #[serde(rename = "LOW")]
    #[doc = "Vulnerability: A low risk vulnerability hampers a security organization’s ability to detect vulnerabilities or active threats in their deployment, or prevents the root cause investigation of security issues. An example is monitoring and logs being disabled for resource configurations and access. Threat: Indicates a threat that has obtained minimal access to an environment but is not able to access data, execute code, or create resources."]
    Low,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of the finding."]
pub enum FindingStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Unspecified state."]
    StateUnspecified,
    #[serde(rename = "ACTIVE")]
    #[doc = "The finding requires attention and has not been addressed yet."]
    Active,
    #[serde(rename = "INACTIVE")]
    #[doc = "The finding has been fixed, triaged as a non-issue or otherwise addressed and is no longer active."]
    Inactive,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message that contains the resource name and display name of a folder resource."]
pub struct Folder {
    #[serde(rename = "resourceFolder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Full resource name of this folder. See: https://cloud.google.com/apis/design/resource_names#full_resource_name"]
    pub resource_folder: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceFolderDisplayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user defined display name for this folder."]
    pub resource_folder_display_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud SCC's Notification"]
pub struct GoogleCloudSecuritycenterV1NotificationMessage {
    #[serde(rename = "finding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If it's a Finding based notification config, this field will be populated."]
    pub finding: ::std::option::Option<::std::boxed::Box<Finding>>,
    #[serde(rename = "notificationConfigName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the notification config that generated current notification."]
    pub notification_config_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Cloud resource tied to this notification's Finding."]
    pub resource: ::std::option::Option<::std::boxed::Box<GoogleCloudSecuritycenterV1Resource>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information related to the Google Cloud resource."]
pub struct GoogleCloudSecuritycenterV1Resource {
    #[serde(rename = "folders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Contains a Folder message for each folder in the assets ancestry. The first folder is the deepest nested folder, and the last folder is the folder directly under the Organization."]
    pub folders: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Folder>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full resource name of the resource. See: https://cloud.google.com/apis/design/resource_names#full_resource_name"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full resource name of resource's parent."]
    pub parent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parentDisplayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human readable name of resource's parent."]
    pub parent_display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full resource name of project that the resource belongs to."]
    pub project: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectDisplayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human readable name of project that the resource belongs to."]
    pub project_display_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response of asset discovery run"]
pub struct GoogleCloudSecuritycenterV1RunAssetDiscoveryResponse {
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The duration between asset discovery run start and end"]
    pub duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of an asset discovery run."]
    pub state: ::std::option::Option<GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of an asset discovery run."]
pub enum GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Asset discovery run state was unspecified."]
    StateUnspecified,
    #[serde(rename = "COMPLETED")]
    #[doc = "Asset discovery run completed successfully."]
    Completed,
    #[serde(rename = "SUPERSEDED")]
    #[doc = "Asset discovery run was cancelled with tasks still pending, as another run for the same organization was started with a higher priority."]
    Superseded,
    #[serde(rename = "TERMINATED")]
    #[doc = "Asset discovery run was killed and terminated."]
    Terminated,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response of asset discovery run"]
pub struct GoogleCloudSecuritycenterV1beta1RunAssetDiscoveryResponse {
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The duration between asset discovery run start and end"]
    pub duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of an asset discovery run."]
    pub state:
        ::std::option::Option<GoogleCloudSecuritycenterV1beta1RunAssetDiscoveryResponseStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of an asset discovery run."]
pub enum GoogleCloudSecuritycenterV1beta1RunAssetDiscoveryResponseStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Asset discovery run state was unspecified."]
    StateUnspecified,
    #[serde(rename = "COMPLETED")]
    #[doc = "Asset discovery run completed successfully."]
    Completed,
    #[serde(rename = "SUPERSEDED")]
    #[doc = "Asset discovery run was cancelled with tasks still pending, as another run for the same organization was started with a higher priority."]
    Superseded,
    #[serde(rename = "TERMINATED")]
    #[doc = "Asset discovery run was killed and terminated."]
    Terminated,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Security Command Center finding. A finding is a record of assessment data (security, risk, health or privacy) ingested into Security Command Center for presentation, notification, analysis, policy testing, and enforcement. For example, an XSS vulnerability in an App Engine application is a finding."]
pub struct GoogleCloudSecuritycenterV1p1beta1Finding {
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The additional taxonomy group within findings from a given source. This field is immutable after creation time. Example: \"XSS_FLASH_INJECTION\""]
    pub category: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the finding was created in Security Command Center."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "eventTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the event took place, or when an update to the finding occurred. For example, if the finding represents an open firewall it would capture the time the detector believes the firewall became open. The accuracy is determined by the detector. If the finding were to be resolved afterward, this time would reflect when the finding was resolved."]
    pub event_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "externalUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI that, if available, points to a web page outside of Security Command Center where additional information about the finding can be found. This field is guaranteed to be either empty or a well formed URL."]
    pub external_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The relative resource name of this finding. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: \"organizations/{organization_id}/sources/{source_id}/findings/{finding_id}\""]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The relative resource name of the source the finding belongs to. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name This field is immutable after creation time. For example: \"organizations/{organization_id}/sources/{source_id}\""]
    pub parent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For findings on Google Cloud resources, the full resource name of the Google Cloud resource this finding is for. See: https://cloud.google.com/apis/design/resource_names#full_resource_name When the finding is for a non-Google Cloud resource, the resourceName can be a customer or partner defined string. This field is immutable after creation time."]
    pub resource_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "securityMarks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. User specified security marks. These marks are entirely managed by the user and come from the SecurityMarks resource that belongs to the finding."]
    pub security_marks:
        ::std::option::Option<::std::boxed::Box<GoogleCloudSecuritycenterV1p1beta1SecurityMarks>>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The severity of the finding. This field is managed by the source that writes the finding."]
    pub severity: ::std::option::Option<GoogleCloudSecuritycenterV1p1beta1FindingSeverityEnum>,
    #[serde(rename = "sourceProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Source specific properties. These properties are managed by the source that writes the finding. The key names in the source_properties map must be between 1 and 255 characters, and must start with a letter and contain alphanumeric characters or underscores only."]
    pub source_properties:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of the finding."]
    pub state: ::std::option::Option<GoogleCloudSecuritycenterV1p1beta1FindingStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The severity of the finding. This field is managed by the source that writes the finding."]
pub enum GoogleCloudSecuritycenterV1p1beta1FindingSeverityEnum {
    #[serde(rename = "SEVERITY_UNSPECIFIED")]
    #[doc = "No severity specified. The default value."]
    SeverityUnspecified,
    #[serde(rename = "CRITICAL")]
    #[doc = "Critical severity."]
    Critical,
    #[serde(rename = "HIGH")]
    #[doc = "High severity."]
    High,
    #[serde(rename = "MEDIUM")]
    #[doc = "Medium severity."]
    Medium,
    #[serde(rename = "LOW")]
    #[doc = "Low severity."]
    Low,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of the finding."]
pub enum GoogleCloudSecuritycenterV1p1beta1FindingStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Unspecified state."]
    StateUnspecified,
    #[serde(rename = "ACTIVE")]
    #[doc = "The finding requires attention and has not been addressed yet."]
    Active,
    #[serde(rename = "INACTIVE")]
    #[doc = "The finding has been fixed, triaged as a non-issue or otherwise addressed and is no longer active."]
    Inactive,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message that contains the resource name and display name of a folder resource."]
pub struct GoogleCloudSecuritycenterV1p1beta1Folder {
    #[serde(rename = "resourceFolder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Full resource name of this folder. See: https://cloud.google.com/apis/design/resource_names#full_resource_name"]
    pub resource_folder: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceFolderDisplayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user defined display name for this folder."]
    pub resource_folder_display_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Security Command Center's Notification"]
pub struct GoogleCloudSecuritycenterV1p1beta1NotificationMessage {
    #[serde(rename = "finding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If it's a Finding based notification config, this field will be populated."]
    pub finding:
        ::std::option::Option<::std::boxed::Box<GoogleCloudSecuritycenterV1p1beta1Finding>>,
    #[serde(rename = "notificationConfigName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the notification config that generated current notification."]
    pub notification_config_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Cloud resource tied to the notification."]
    pub resource:
        ::std::option::Option<::std::boxed::Box<GoogleCloudSecuritycenterV1p1beta1Resource>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information related to the Google Cloud resource."]
pub struct GoogleCloudSecuritycenterV1p1beta1Resource {
    #[serde(rename = "folders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Contains a Folder message for each folder in the assets ancestry. The first folder is the deepest nested folder, and the last folder is the folder directly under the Organization."]
    pub folders: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudSecuritycenterV1p1beta1Folder>>,
    >,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full resource name of the resource. See: https://cloud.google.com/apis/design/resource_names#full_resource_name"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full resource name of resource's parent."]
    pub parent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parentDisplayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human readable name of resource's parent."]
    pub parent_display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full resource name of project that the resource belongs to."]
    pub project: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectDisplayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human readable name of project that the resource belongs to."]
    pub project_display_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response of asset discovery run"]
pub struct GoogleCloudSecuritycenterV1p1beta1RunAssetDiscoveryResponse {
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The duration between asset discovery run start and end"]
    pub duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of an asset discovery run."]
    pub state:
        ::std::option::Option<GoogleCloudSecuritycenterV1p1beta1RunAssetDiscoveryResponseStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of an asset discovery run."]
pub enum GoogleCloudSecuritycenterV1p1beta1RunAssetDiscoveryResponseStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Asset discovery run state was unspecified."]
    StateUnspecified,
    #[serde(rename = "COMPLETED")]
    #[doc = "Asset discovery run completed successfully."]
    Completed,
    #[serde(rename = "SUPERSEDED")]
    #[doc = "Asset discovery run was cancelled with tasks still pending, as another run for the same organization was started with a higher priority."]
    Superseded,
    #[serde(rename = "TERMINATED")]
    #[doc = "Asset discovery run was killed and terminated."]
    Terminated,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "User specified security marks that are attached to the parent Security Command Center resource. Security marks are scoped within a Security Command Center organization -- they can be modified and viewed by all users who have proper permissions on the organization."]
pub struct GoogleCloudSecuritycenterV1p1beta1SecurityMarks {
    #[serde(rename = "marks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mutable user specified security marks belonging to the parent resource. Constraints are as follows: * Keys and values are treated as case insensitive * Keys must be between 1 - 256 characters (inclusive) * Keys must be letters, numbers, underscores, or dashes * Values have leading and trailing whitespace trimmed, remaining characters must be between 1 - 4096 characters (inclusive)"]
    pub marks: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The relative resource name of the SecurityMarks. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Examples: \"organizations/{organization_id}/assets/{asset_id}/securityMarks\" \"organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks\"."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Resource capturing the settings for Security Center."]
pub struct SecurityCenterSettings {
    #[serde(rename = "logSinkProject")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the project to send logs to. This project must be part of the organization this resource resides in. The format is `projects/{project_id}`. An empty value disables logging. This value is only referenced by services that support log sink. Please refer to the documentation for an updated list of compatible services."]
    pub log_sink_project: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the SecurityCenterSettings. Format: organizations/{organization}/securityCenterSettings"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orgServiceAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The organization level service account to be used for security center components."]
    pub org_service_account: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Resource capturing the settings for the Security Health Analytics service."]
pub struct SecurityHealthAnalyticsSettings {
    #[serde(rename = "modules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configurations including the state of enablement for the service's different modules. The absence of a module in the map implies its configuration is inherited from its parent's."]
    pub modules:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Config>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the SecurityHealthAnalyticsSettings. Formats: * organizations/{organization}/securityHealthAnalyticsSettings * folders/{folder}/securityHealthAnalyticsSettings * projects/{project}/securityHealthAnalyticsSettings"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The service account used by Security Health Analytics detectors."]
    pub service_account: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceEnablementState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED."]
    pub service_enablement_state:
        ::std::option::Option<SecurityHealthAnalyticsSettingsServiceEnablementStateEnum>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the settings were last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED."]
pub enum SecurityHealthAnalyticsSettingsServiceEnablementStateEnum {
    #[serde(rename = "ENABLEMENT_STATE_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    EnablementStateUnspecified,
    #[serde(rename = "INHERITED")]
    #[doc = "State is inherited from the parent resource."]
    Inherited,
    #[serde(rename = "ENABLED")]
    #[doc = "State is enabled."]
    Enabled,
    #[serde(rename = "DISABLED")]
    #[doc = "State is disabled."]
    Disabled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "User specified security marks that are attached to the parent Security Command Center resource. Security marks are scoped within a Security Command Center organization -- they can be modified and viewed by all users who have proper permissions on the organization."]
pub struct SecurityMarks {
    #[serde(rename = "marks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mutable user specified security marks belonging to the parent resource. Constraints are as follows: * Keys and values are treated as case insensitive * Keys must be between 1 - 256 characters (inclusive) * Keys must be letters, numbers, underscores, or dashes * Values have leading and trailing whitespace trimmed, remaining characters must be between 1 - 4096 characters (inclusive)"]
    pub marks: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The relative resource name of the SecurityMarks. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Examples: \"organizations/{organization_id}/assets/{asset_id}/securityMarks\" \"organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks\"."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Resource capturing the state of an organization's subscription."]
pub struct Subscription {
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The details of the most recent active subscription. If there has never been a subscription this will be empty."]
    pub details: ::std::option::Option<::std::boxed::Box<Details>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the subscription. Format: organizations/{organization}/subscription"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tier of SCC features this organization currently has access to."]
    pub tier: ::std::option::Option<SubscriptionTierEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The tier of SCC features this organization currently has access to."]
pub enum SubscriptionTierEnum {
    #[serde(rename = "TIER_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    TierUnspecified,
    #[serde(rename = "STANDARD")]
    #[doc = "The standard tier."]
    Standard,
    #[serde(rename = "PREMIUM")]
    #[doc = "The premium tier."]
    Premium,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Resource capturing the settings for the Web Security Scanner service."]
pub struct WebSecurityScannerSettings {
    #[serde(rename = "modules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configurations including the state of enablement for the service's different modules. The absence of a module in the map implies its configuration is inherited from its parent's."]
    pub modules:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Config>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the WebSecurityScannerSettings. Formats: * organizations/{organization}/webSecurityScannerSettings * folders/{folder}/webSecurityScannerSettings * projects/{project}/webSecurityScannerSettings"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceEnablementState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED."]
    pub service_enablement_state:
        ::std::option::Option<WebSecurityScannerSettingsServiceEnablementStateEnum>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the settings were last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of enablement for the service at its level of the resource hierarchy. A DISABLED state will override all module enablement_states to DISABLED."]
pub enum WebSecurityScannerSettingsServiceEnablementStateEnum {
    #[serde(rename = "ENABLEMENT_STATE_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    EnablementStateUnspecified,
    #[serde(rename = "INHERITED")]
    #[doc = "State is inherited from the parent resource."]
    Inherited,
    #[serde(rename = "ENABLED")]
    #[doc = "State is enabled."]
    Enabled,
    #[serde(rename = "DISABLED")]
    #[doc = "State is disabled."]
    Disabled,
}
