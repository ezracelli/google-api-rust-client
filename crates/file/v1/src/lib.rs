#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Cloud Filestore backup."]
pub struct Backup {
    #[serde(rename = "capacityGb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Capacity of the source file share when the backup was created."]
    pub capacity_gb: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the backup was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "downloadBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Amount of bytes that will be downloaded if the backup is restored. This may be different than storage bytes, since sequential backups of the same disk will share storage."]
    pub download_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource labels to represent user provided metadata."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the backup, in the format projects/{project_number}/locations/{location_id}/backups/{backup_id}."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceFileShare")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the file share in the source Cloud Filestore instance that the backup is created from."]
    pub source_file_share: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceInstance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the source Cloud Filestore instance, in the format projects/{project_number}/locations/{location_id}/instances/{instance_id}, used to create this backup."]
    pub source_instance: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceInstanceTier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The service tier of the source Cloud Filestore instance that this backup is created from."]
    pub source_instance_tier: ::std::option::Option<BackupSourceInstanceTierEnum>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The backup state."]
    pub state: ::std::option::Option<BackupStateEnum>,
    #[serde(rename = "storageBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The size of the storage used by the backup. As backups share storage, this number is expected to change with backup creation/deletion."]
    pub storage_bytes: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The service tier of the source Cloud Filestore instance that this backup is created from."]
pub enum BackupSourceInstanceTierEnum {
    #[serde(rename = "TIER_UNSPECIFIED")]
    #[doc = "Not set."]
    TierUnspecified,
    #[serde(rename = "STANDARD")]
    #[doc = "STANDARD tier."]
    Standard,
    #[serde(rename = "PREMIUM")]
    #[doc = "PREMIUM tier."]
    Premium,
    #[serde(rename = "BASIC_HDD")]
    #[doc = "BASIC instances offer a maximum capacity of 63.9 TB. BASIC_HDD is an alias for STANDARD Tier, offering economical performance backed by HDD."]
    BasicHdd,
    #[serde(rename = "BASIC_SSD")]
    #[doc = "BASIC instances offer a maximum capacity of 63.9 TB. BASIC_SSD is an alias for PREMIUM Tier, and offers improved performance backed by SSD."]
    BasicSsd,
    #[serde(rename = "HIGH_SCALE_SSD")]
    #[doc = "HIGH_SCALE instances offer expanded capacity and performance scaling capabilities."]
    HighScaleSsd,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The backup state."]
pub enum BackupStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "State not set."]
    StateUnspecified,
    #[serde(rename = "CREATING")]
    #[doc = "Backup is being created."]
    Creating,
    #[serde(rename = "FINALIZING")]
    #[doc = "Backup has been taken and the operation is being finalized. At this point, changes to the file share will not be reflected in the backup."]
    Finalizing,
    #[serde(rename = "READY")]
    #[doc = "Backup is available for use."]
    Ready,
    #[serde(rename = "DELETING")]
    #[doc = "Backup is being deleted."]
    Deleting,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for Operations.CancelOperation."]
pub struct CancelOperationRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Time window specified for daily operations."]
pub struct DailyCycle {
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Duration of the time window, set by service producer."]
    pub duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time within the day to start the operations."]
    pub start_time: ::std::option::Option<::std::boxed::Box<TimeOfDay>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."]
pub struct Date {
    #[serde(rename = "day")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub day: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "month")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub month: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "year")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub year: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "DenyMaintenancePeriod definition. Maintenance is forbidden within the deny period. The start_date must be less than the end_date."]
pub struct DenyMaintenancePeriod {
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deny period end date. This can be: * A full date, with non-zero year, month and day values. * A month and day value, with a zero year. Allows recurring deny periods each year. Date matching this period will have to be before the end."]
    pub end_date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deny period start date. This can be: * A full date, with non-zero year, month and day values. * A month and day value, with a zero year. Allows recurring deny periods each year. Date matching this period will have to be the same or after the start."]
    pub start_date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time in UTC when the Blackout period starts on start_date and ends on end_date. This can be: * Full time. * All zeros for 00:00:00 UTC"]
    pub time: ::std::option::Option<::std::boxed::Box<TimeOfDay>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "File share configuration for the instance."]
pub struct FileShareConfig {
    #[serde(rename = "capacityGb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "File share capacity in gigabytes (GB). Cloud Filestore defines 1 GB as 1024^3 bytes."]
    pub capacity_gb: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the file share (must be 16 characters or less)."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nfsExportOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Nfs Export Options. There is a limit of 10 export options per file share."]
    pub nfs_export_options:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NfsExportOptions>>>,
    #[serde(rename = "sourceBackup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the backup, in the format projects/{project_number}/locations/{location_id}/backups/{backup_id}, that this file share has been restored from."]
    pub source_backup: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudSaasacceleratorManagementProvidersV1Instance {
    #[serde(rename = "consumerDefinedName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "consumer_defined_name is the name that is set by the consumer. On the other hand Name field represents system-assigned id of an instance so consumers are not necessarily aware of it. consumer_defined_name is used for notification/UI purposes for consumer to recognize their instances."]
    pub consumer_defined_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Timestamp when the resource was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Resource labels to represent user provided metadata. Each label is a key-value pair, where both the key and the value are arbitrary strings provided by the user."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "maintenancePolicyNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. The MaintenancePolicies that have been attached to the instance. The key must be of the type name of the oneof policy name defined in MaintenancePolicy, and the referenced policy must define the same policy type. For complete details of MaintenancePolicy, please refer to go/cloud-saas-mw-ug."]
    pub maintenance_policy_names:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "maintenanceSchedules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MaintenanceSchedule contains the scheduling information of published maintenance schedule with same key as software_versions."]
    pub maintenance_schedules: ::std::option::Option<
        ::std::collections::BTreeMap<
            String,
            ::std::boxed::Box<GoogleCloudSaasacceleratorManagementProvidersV1MaintenanceSchedule>,
        >,
    >,
    #[serde(rename = "maintenanceSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The MaintenanceSettings associated with instance."]
    pub maintenance_settings: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudSaasacceleratorManagementProvidersV1MaintenanceSettings>,
    >,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique name of the resource. It uses the form: `projects/{project_id}/locations/{location_id}/instances/{instance_id}`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "producerMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Custom string attributes used primarily to expose producer-specific information in monitoring dashboards. See go/get-instance-metadata."]
    pub producer_metadata:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "provisionedResources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The list of data plane resources provisioned for this instance, e.g. compute VMs. See go/get-instance-metadata."]
    pub provisioned_resources: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudSaasacceleratorManagementProvidersV1ProvisionedResource>,
        >,
    >,
    #[serde(rename = "slmInstanceTemplate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Link to the SLM instance template. Only populated when updating SLM instances via SSA's Actuation service adaptor. Service producers with custom control plane (e.g. Cloud SQL) doesn't need to populate this field. Instead they should use software_versions."]
    pub slm_instance_template: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sloMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. SLO metadata for instance classification in the Standardized dataplane SLO platform. See go/cloud-ssa-standard-slo for feature description."]
    pub slo_metadata: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudSaasacceleratorManagementProvidersV1SloMetadata>,
    >,
    #[serde(rename = "softwareVersions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Software versions that are used to deploy this instance. This can be mutated by rollout services."]
    pub software_versions:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Current lifecycle state of the resource (e.g. if it's being created or ready to use)."]
    pub state:
        ::std::option::Option<GoogleCloudSaasacceleratorManagementProvidersV1InstanceStateEnum>,
    #[serde(rename = "tenantProjectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. ID of the associated GCP tenant project. See go/get-instance-metadata."]
    pub tenant_project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Timestamp when the resource was last modified."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Current lifecycle state of the resource (e.g. if it's being created or ready to use)."]
pub enum GoogleCloudSaasacceleratorManagementProvidersV1InstanceStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Unspecified state."]
    StateUnspecified,
    #[serde(rename = "CREATING")]
    #[doc = "Instance is being created."]
    Creating,
    #[serde(rename = "READY")]
    #[doc = "Instance has been created and is ready to use."]
    Ready,
    #[serde(rename = "UPDATING")]
    #[doc = "Instance is being updated."]
    Updating,
    #[serde(rename = "REPAIRING")]
    #[doc = "Instance is unheathy and under repair."]
    Repairing,
    #[serde(rename = "DELETING")]
    #[doc = "Instance is being deleted."]
    Deleting,
    #[serde(rename = "ERROR")]
    #[doc = "Instance encountered an error and is in indeterministic state."]
    Error,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Maintenance schedule which is exposed to customer and potentially end user, indicating published upcoming future maintenance schedule"]
pub struct GoogleCloudSaasacceleratorManagementProvidersV1MaintenanceSchedule {
    #[serde(rename = "canReschedule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Can this scheduled update be rescheduled? By default, it's true and API needs to do explicitly check whether it's set, if it's set as false explicitly, it's false"]
    pub can_reschedule: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The scheduled end time for the maintenance."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rolloutManagementPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The rollout management policy this maintenance schedule is associated with. When doing reschedule update request, the reschedule should be against this given policy."]
    pub rollout_management_policy: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scheduleDeadlineTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "schedule_deadline_time is the time deadline any schedule start time cannot go beyond, including reschedule. It's normally the initial schedule start time plus maintenance window length (1 day or 1 week). Maintenance cannot be scheduled to start beyond this deadline."]
    pub schedule_deadline_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The scheduled start time for the maintenance."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Maintenance settings associated with instance. Allows service producers and end users to assign settings that controls maintenance on this instance."]
pub struct GoogleCloudSaasacceleratorManagementProvidersV1MaintenanceSettings {
    #[serde(rename = "exclude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Exclude instance from maintenance. When true, rollout service will not attempt maintenance on the instance. Rollout service will include the instance in reported rollout progress as not attempted."]
    pub exclude: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isRollback")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If the update call is triggered from rollback, set the value as true."]
    pub is_rollback: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "maintenancePolicies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The MaintenancePolicies that have been attached to the instance. The key must be of the type name of the oneof policy name defined in MaintenancePolicy, and the embedded policy must define the same policy type. For complete details of MaintenancePolicy, please refer to go/cloud-saas-mw-ug. If only the name is needed (like in the deprecated Instance.maintenance_policy_names field) then only populate MaintenancePolicy.name."]
    pub maintenance_policies: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<MaintenancePolicy>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Node information for custom per-node SLO implementations. SSA does not support per-node SLO, but producers can populate per-node information in SloMetadata for custom precomputations. SSA Eligibility Exporter will emit per-node metric based on this information."]
pub struct GoogleCloudSaasacceleratorManagementProvidersV1NodeSloMetadata {
    #[serde(rename = "exclusions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "By default node is eligible if instance is eligible. But individual node might be excluded from SLO by adding entry here. For semantic see SloMetadata.exclusions. If both instance and node level exclusions are present for time period, the node level's reason will be reported by Eligibility Exporter."]
    pub exclusions: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudSaasacceleratorManagementProvidersV1SloExclusion>,
        >,
    >,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location of the node, if different from instance location."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of the node. This should be equal to SaasInstanceNode.node_id."]
    pub node_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "PerSliSloEligibility is a mapping from an SLI name to eligibility."]
pub struct GoogleCloudSaasacceleratorManagementProvidersV1PerSliSloEligibility {
    #[serde(rename = "eligibilities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry in the eligibilities map specifies an eligibility for a particular SLI for the given instance. The SLI key in the name must be a valid SLI name specified in the Eligibility Exporter binary flags otherwise an error will be emitted by Eligibility Exporter and the oncaller will be alerted. If an SLI has been defined in the binary flags but the eligibilities map does not contain it, the corresponding SLI time series will not be emitted by the Eligibility Exporter. This ensures a smooth rollout and compatibility between the data produced by different versions of the Eligibility Exporters. If eligibilities map contains a key for an SLI which has not been declared in the binary flags, there will be an error message emitted in the Eligibility Exporter log and the metric for the SLI in question will not be emitted."]
    pub eligibilities: ::std::option::Option<
        ::std::collections::BTreeMap<
            String,
            ::std::boxed::Box<GoogleCloudSaasacceleratorManagementProvidersV1SloEligibility>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes provisioned dataplane resources."]
pub struct GoogleCloudSaasacceleratorManagementProvidersV1ProvisionedResource {
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the resource. This can be either a GCP resource or a custom one (e.g. another cloud provider's VM). For GCP compute resources use singular form of the names listed in GCP compute API documentation (https://cloud.google.com/compute/docs/reference/rest/v1/), prefixed with 'compute-', for example: 'compute-instance', 'compute-disk', 'compute-autoscaler'."]
    pub resource_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL identifying the resource, e.g. \"https://www.googleapis.com/compute/v1/projects/...)\"."]
    pub resource_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SloEligibility is a tuple containing eligibility value: true if an instance is eligible for SLO calculation or false if it should be excluded from all SLO-related calculations along with a user-defined reason."]
pub struct GoogleCloudSaasacceleratorManagementProvidersV1SloEligibility {
    #[serde(rename = "eligible")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether an instance is eligible or ineligible."]
    pub eligible: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-defined reason for the current value of instance eligibility. Usually, this can be directly mapped to the internal state. An empty reason is allowed."]
    pub reason: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SloExclusion represents an exclusion in SLI calculation applies to all SLOs."]
pub struct GoogleCloudSaasacceleratorManagementProvidersV1SloExclusion {
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Exclusion duration. No restrictions on the possible values. When an ongoing operation is taking longer than initially expected, an existing entry in the exclusion list can be updated by extending the duration. This is supported by the subsystem exporting eligibility data as long as such extension is committed at least 10 minutes before the original exclusion expiration - otherwise it is possible that there will be \"gaps\" in the exclusion application in the exported timeseries."]
    pub duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human-readable reason for the exclusion. This should be a static string (e.g. \"Disruptive update in progress\") and should not contain dynamically generated data (e.g. instance name). Can be left empty."]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sliName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of an SLI that this exclusion applies to. Can be left empty, signaling that the instance should be excluded from all SLIs."]
    pub sli_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Start time of the exclusion. No alignment (e.g. to a full minute) needed."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SloMetadata contains resources required for proper SLO classification of the instance."]
pub struct GoogleCloudSaasacceleratorManagementProvidersV1SloMetadata {
    #[serde(rename = "eligibility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Global per-instance SLI eligibility which applies to all defined SLIs. Exactly one of 'eligibility' and 'per_sli_eligibility' fields must be used."]
    pub eligibility: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudSaasacceleratorManagementProvidersV1SloEligibility>,
    >,
    #[serde(rename = "exclusions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of SLO exclusion windows. When multiple entries in the list match (matching the exclusion time-window against current time point) the exclusion reason used in the first matching entry will be published. It is not needed to include expired exclusion in this list, as only the currently applicable exclusions are taken into account by the eligibility exporting subsystem (the historical state of exclusions will be reflected in the historically produced timeseries regardless of the current state). This field can be used to mark the instance as temporary ineligible for the purpose of SLO calculation. For permanent instance SLO exclusion, use of custom instance eligibility is recommended. See 'eligibility' field below."]
    pub exclusions: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudSaasacceleratorManagementProvidersV1SloExclusion>,
        >,
    >,
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. List of nodes. Some producers need to use per-node metadata to calculate SLO. This field allows such producers to publish per-node SLO meta data, which will be consumed by SSA Eligibility Exporter and published in the form of per node metric to Monarch."]
    pub nodes: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudSaasacceleratorManagementProvidersV1NodeSloMetadata>,
        >,
    >,
    #[serde(rename = "perSliEligibility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Multiple per-instance SLI eligibilities which apply for individual SLIs. Exactly one of 'eligibility' and 'per_sli_eligibility' fields must be used."]
    pub per_sli_eligibility: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudSaasacceleratorManagementProvidersV1PerSliSloEligibility>,
    >,
    #[serde(rename = "tier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the SLO tier the Instance belongs to. This name will be expected to match the tiers specified in the service SLO configuration. Field is mandatory and must not be empty."]
    pub tier: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Cloud Filestore instance."]
pub struct Instance {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the instance was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of the instance (2048 characters or less)."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Server-specified ETag for the instance resource to prevent simultaneous updates from overwriting each other."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileShares")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "File system shares on the instance. For this version, only a single file share is supported."]
    pub file_shares: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FileShareConfig>>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource labels to represent user provided metadata."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the instance, in the format projects/{project}/locations/{location}/instances/{instance}."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "networks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "VPC networks to which the instance is connected. For this version, only a single network is supported."]
    pub networks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NetworkConfig>>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The instance state."]
    pub state: ::std::option::Option<InstanceStateEnum>,
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Additional information about the instance state, if available."]
    pub status_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The service tier of the instance."]
    pub tier: ::std::option::Option<InstanceTierEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The instance state."]
pub enum InstanceStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "State not set."]
    StateUnspecified,
    #[serde(rename = "CREATING")]
    #[doc = "The instance is being created."]
    Creating,
    #[serde(rename = "READY")]
    #[doc = "The instance is available for use."]
    Ready,
    #[serde(rename = "REPAIRING")]
    #[doc = "Work is being done on the instance. You can get further details from the `statusMessage` field of the `Instance` resource."]
    Repairing,
    #[serde(rename = "DELETING")]
    #[doc = "The instance is shutting down."]
    Deleting,
    #[serde(rename = "ERROR")]
    #[doc = "The instance is experiencing an issue and might be unusable. You can get further details from the `statusMessage` field of the `Instance` resource."]
    Error,
    #[serde(rename = "RESTORING")]
    #[doc = "The instance is restoring a backup to an existing file share and may be unusable during this time."]
    Restoring,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The service tier of the instance."]
pub enum InstanceTierEnum {
    #[serde(rename = "TIER_UNSPECIFIED")]
    #[doc = "Not set."]
    TierUnspecified,
    #[serde(rename = "STANDARD")]
    #[doc = "STANDARD tier."]
    Standard,
    #[serde(rename = "PREMIUM")]
    #[doc = "PREMIUM tier."]
    Premium,
    #[serde(rename = "BASIC_HDD")]
    #[doc = "BASIC instances offer a maximum capacity of 63.9 TB. BASIC_HDD is an alias for STANDARD Tier, offering economical performance backed by HDD."]
    BasicHdd,
    #[serde(rename = "BASIC_SSD")]
    #[doc = "BASIC instances offer a maximum capacity of 63.9 TB. BASIC_SSD is an alias for PREMIUM Tier, and offers improved performance backed by SSD."]
    BasicSsd,
    #[serde(rename = "HIGH_SCALE_SSD")]
    #[doc = "HIGH_SCALE instances offer expanded capacity and performance scaling capabilities."]
    HighScaleSsd,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ListBackupsResponse is the result of ListBackupsRequest."]
pub struct ListBackupsResponse {
    #[serde(rename = "backups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of backups in the project for the specified location. If the {location} value in the request is \"-\", the response contains a list of backups from all locations. If any location is unreachable, the response will only return backups in reachable locations and the \"unreachable\" field will be populated with a list of unreachable locations."]
    pub backups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Backup>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token you can use to retrieve the next page of results. Not returned if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "unreachable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locations that could not be reached."]
    pub unreachable: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ListInstancesResponse is the result of ListInstancesRequest."]
pub struct ListInstancesResponse {
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of instances in the project for the specified location. If the {location} value in the request is \"-\", the response contains a list of instances from all locations. If any location is unreachable, the response will only return instances in reachable locations and the \"unreachable\" field will be populated with a list of unreachable locations."]
    pub instances: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Instance>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token you can use to retrieve the next page of results. Not returned if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
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
#[doc = "Defines policies to service maintenance events."]
pub struct MaintenancePolicy {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the resource was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Description of what this policy is for. Create/Update methods return INVALID_ARGUMENT if the length is greater than 512."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Resource labels to represent user provided metadata. Each label is a key-value pair, where both the key and the value are arbitrary strings provided by the user."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. MaintenancePolicy name using the form: `projects/{project_id}/locations/{location_id}/maintenancePolicies/{maintenance_policy_id}` where {project_id} refers to a GCP consumer project ID, {location_id} refers to a GCP region/zone, {maintenance_policy_id} must be 1-63 characters long and match the regular expression `[a-z0-9]([-a-z0-9]*[a-z0-9])?`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The state of the policy."]
    pub state: ::std::option::Option<MaintenancePolicyStateEnum>,
    #[serde(rename = "updatePolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maintenance policy applicable to instance update."]
    pub update_policy: ::std::option::Option<::std::boxed::Box<UpdatePolicy>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the resource was updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. The state of the policy."]
pub enum MaintenancePolicyStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Unspecified state."]
    StateUnspecified,
    #[serde(rename = "READY")]
    #[doc = "Resource is ready to be used."]
    Ready,
    #[serde(rename = "DELETING")]
    #[doc = "Resource is being deleted. It can no longer be attached to instances."]
    Deleting,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "MaintenanceWindow definition."]
pub struct MaintenanceWindow {
    #[serde(rename = "dailyCycle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Daily cycle."]
    pub daily_cycle: ::std::option::Option<::std::boxed::Box<DailyCycle>>,
    #[serde(rename = "weeklyCycle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Weekly cycle."]
    pub weekly_cycle: ::std::option::Option<::std::boxed::Box<WeeklyCycle>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Network configuration for the instance."]
pub struct NetworkConfig {
    #[serde(rename = "ipAddresses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. IPv4 addresses in the format {octet 1}.{octet 2}.{octet 3}.{octet 4} or IPv6 addresses in the format {block 1}:{block 2}:{block 3}:{block 4}:{block 5}:{block 6}:{block 7}:{block 8}."]
    pub ip_addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "modes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Internet protocol versions for which the instance has IP addresses assigned. For this version, only MODE_IPV4 is supported."]
    pub modes: ::std::option::Option<::std::vec::Vec<NetworkConfigModesEnum>>,
    #[serde(rename = "network")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the Google Compute Engine [VPC network](/compute/docs/networks-and-firewalls#networks) to which the instance is connected."]
    pub network: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reservedIpRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A /29 CIDR block in one of the [internal IP address ranges](https://www.arin.net/knowledge/address_filters.html) that identifies the range of IP addresses reserved for this instance. For example, 10.0.0.0/29 or 192.168.0.0/29. The range you specify can't overlap with either existing subnets or assigned IP address ranges for other Cloud Filestore instances in the selected VPC network."]
    pub reserved_ip_range: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum NetworkConfigModesEnum {
    #[serde(rename = "ADDRESS_MODE_UNSPECIFIED")]
    #[doc = "Internet protocol not set."]
    AddressModeUnspecified,
    #[serde(rename = "MODE_IPV4")]
    #[doc = "Use the IPv4 internet protocol."]
    ModeIpv4,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "NFS export options specifications."]
pub struct NfsExportOptions {
    #[serde(rename = "accessMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Either READ_ONLY, for allowing only read requests on the exported directory, or READ_WRITE, for allowing both read and write requests. The default is READ_WRITE."]
    pub access_mode: ::std::option::Option<NfsExportOptionsAccessModeEnum>,
    #[serde(rename = "anonGid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An integer representing the anonymous group id with a default value of 65534. Anon_gid may only be set with squash_mode of ROOT_SQUASH. An error will be returned if this field is specified for other squash_mode settings."]
    pub anon_gid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "anonUid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An integer representing the anonymous user id with a default value of 65534. Anon_uid may only be set with squash_mode of ROOT_SQUASH. An error will be returned if this field is specified for other squash_mode settings."]
    pub anon_uid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ipRanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of either an IPv4 addresses in the format {octet 1}.{octet 2}.{octet 3}.{octet 4} or CIDR ranges in the format {octet 1}.{octet 2}.{octet 3}.{octet 4}/{mask size} which may mount the file share. Overlapping IP ranges are not allowed, both within and across NfsExportOptions. An error will be returned. The limit is 64 IP ranges/addresses for each FileShareConfig among all NfsExportOptions."]
    pub ip_ranges: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "squashMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Either NO_ROOT_SQUASH, for allowing root access on the exported directory, or ROOT_SQUASH, for not allowing root access. The default is NO_ROOT_SQUASH."]
    pub squash_mode: ::std::option::Option<NfsExportOptionsSquashModeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Either READ_ONLY, for allowing only read requests on the exported directory, or READ_WRITE, for allowing both read and write requests. The default is READ_WRITE."]
pub enum NfsExportOptionsAccessModeEnum {
    #[serde(rename = "ACCESS_MODE_UNSPECIFIED")]
    #[doc = "AccessMode not set."]
    AccessModeUnspecified,
    #[serde(rename = "READ_ONLY")]
    #[doc = "The client can only read the file share."]
    ReadOnly,
    #[serde(rename = "READ_WRITE")]
    #[doc = "The client can read and write the file share (default)."]
    ReadWrite,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Either NO_ROOT_SQUASH, for allowing root access on the exported directory, or ROOT_SQUASH, for not allowing root access. The default is NO_ROOT_SQUASH."]
pub enum NfsExportOptionsSquashModeEnum {
    #[serde(rename = "SQUASH_MODE_UNSPECIFIED")]
    #[doc = "SquashMode not set."]
    SquashModeUnspecified,
    #[serde(rename = "NO_ROOT_SQUASH")]
    #[doc = "The Root user has root access to the file share (default)."]
    NoRootSquash,
    #[serde(rename = "ROOT_SQUASH")]
    #[doc = "The Root user has squashed access to the anonymous uid/gid."]
    RootSquash,
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
#[doc = "Represents the metadata of the long-running operation."]
pub struct OperationMetadata {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] API version used to start the operation."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cancelRequested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Identifies whether the user has requested cancellation of the operation. Operations that have successfully been cancelled have Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."]
    pub cancel_requested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The time the operation was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The time the operation finished running."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statusDetail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Human-readable status of the operation, if any."]
    pub status_detail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Server-defined resource path for the target of the operation."]
    pub target: ::std::option::Option<::std::string::String>,
    #[serde(rename = "verb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Name of the verb executed by the operation."]
    pub verb: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "RestoreInstanceRequest restores an existing instances's file share from a backup."]
pub struct RestoreInstanceRequest {
    #[serde(rename = "fileShare")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Name of the file share in the Cloud Filestore instance that the backup is being restored to."]
    pub file_share: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceBackup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the backup, in the format projects/{project_number}/locations/{location_id}/backups/{backup_id}."]
    pub source_backup: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configure the schedule."]
pub struct Schedule {
    #[serde(rename = "day")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allows to define schedule that runs specified day of the week."]
    pub day: ::std::option::Option<ScheduleDayEnum>,
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Duration of the time window, set by service producer."]
    pub duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time within the window to start the operations."]
    pub start_time: ::std::option::Option<::std::boxed::Box<TimeOfDay>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Allows to define schedule that runs specified day of the week."]
pub enum ScheduleDayEnum {
    #[serde(rename = "DAY_OF_WEEK_UNSPECIFIED")]
    #[doc = "The day of the week is unspecified."]
    DayOfWeekUnspecified,
    #[serde(rename = "MONDAY")]
    #[doc = "Monday"]
    Monday,
    #[serde(rename = "TUESDAY")]
    #[doc = "Tuesday"]
    Tuesday,
    #[serde(rename = "WEDNESDAY")]
    #[doc = "Wednesday"]
    Wednesday,
    #[serde(rename = "THURSDAY")]
    #[doc = "Thursday"]
    Thursday,
    #[serde(rename = "FRIDAY")]
    #[doc = "Friday"]
    Friday,
    #[serde(rename = "SATURDAY")]
    #[doc = "Saturday"]
    Saturday,
    #[serde(rename = "SUNDAY")]
    #[doc = "Sunday"]
    Sunday,
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
#[doc = "Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`."]
pub struct TimeOfDay {
    #[serde(rename = "hours")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub hours: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "minutes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minutes of hour of day. Must be from 0 to 59."]
    pub minutes: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "nanos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub nanos: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "seconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub seconds: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Maintenance policy applicable to instance updates."]
pub struct UpdatePolicy {
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Relative scheduling channel applied to resource."]
    pub channel: ::std::option::Option<UpdatePolicyChannelEnum>,
    #[serde(rename = "denyMaintenancePeriods")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deny Maintenance Period that is applied to resource to indicate when maintenance is forbidden. User can specify zero or more non-overlapping deny periods. For V1, Maximum number of deny_maintenance_periods is expected to be one."]
    pub deny_maintenance_periods:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DenyMaintenancePeriod>>>,
    #[serde(rename = "window")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Maintenance window that is applied to resources covered by this policy."]
    pub window: ::std::option::Option<::std::boxed::Box<MaintenanceWindow>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Relative scheduling channel applied to resource."]
pub enum UpdatePolicyChannelEnum {
    #[serde(rename = "UPDATE_CHANNEL_UNSPECIFIED")]
    #[doc = "Unspecified channel."]
    UpdateChannelUnspecified,
    #[serde(rename = "EARLIER")]
    #[doc = "Early channel within a customer project."]
    Earlier,
    #[serde(rename = "LATER")]
    #[doc = "Later channel within a customer project."]
    Later,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Time window specified for weekly operations."]
pub struct WeeklyCycle {
    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User can specify multiple windows in a week. Minimum of 1 window."]
    pub schedule: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Schedule>>>,
}
