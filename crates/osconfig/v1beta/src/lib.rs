#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a single Apt package repository. This repository is added to a repo file that is stored at `/etc/apt/sources.list.d/google_osconfig.list`."]
pub struct AptRepository {
    #[serde(rename = "archiveType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of archive files in this repository. The default behavior is DEB."]
    pub archive_type: ::std::option::Option<AptRepositoryArchiveTypeEnum>,
    #[serde(rename = "components")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. List of components for this repository. Must contain at least one item."]
    pub components: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "distribution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Distribution of this repository."]
    pub distribution: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gpgKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI of the key file for this repository. The agent maintains a keyring at `/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg` containing all the keys in any applied guest policy."]
    pub gpg_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. URI for this repository."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of archive files in this repository. The default behavior is DEB."]
pub enum AptRepositoryArchiveTypeEnum {
    #[serde(rename = "ARCHIVE_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    ArchiveTypeUnspecified,
    #[serde(rename = "DEB")]
    #[doc = "DEB indicates that the archive contains binary files."]
    Deb,
    #[serde(rename = "DEB_SRC")]
    #[doc = "DEB_SRC indicates that the archive contains source files."]
    DebSrc,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Apt patching is completed by executing `apt-get update && apt-get upgrade`. Additional options can be set to control how this is executed."]
pub struct AptSettings {
    #[serde(rename = "excludes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of packages to exclude from update. These packages will be excluded"]
    pub excludes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "exclusivePackages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An exclusive list of packages to be updated. These are the only packages that will be updated. If these packages are not installed, they will be ignored. This field cannot be specified with any other patch configuration fields."]
    pub exclusive_packages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "By changing the type to DIST, the patching is performed using `apt-get dist-upgrade` instead."]
    pub _type: ::std::option::Option<AptSettingsTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "By changing the type to DIST, the patching is performed using `apt-get dist-upgrade` instead."]
pub enum AptSettingsTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "By default, upgrade will be performed."]
    TypeUnspecified,
    #[serde(rename = "DIST")]
    #[doc = "Runs `apt-get dist-upgrade`."]
    Dist,
    #[serde(rename = "UPGRADE")]
    #[doc = "Runs `apt-get upgrade`."]
    Upgrade,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An assignment represents the group or groups of VM instances that the policy applies to. If an assignment is empty, it applies to all VM instances. Otherwise, the targeted VM instances must meet all the criteria specified. So if both labels and zones are specified, the policy applies to VM instances with those labels and in those zones."]
pub struct Assignment {
    #[serde(rename = "groupLabels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targets instances matching at least one of these label sets. This allows an assignment to target disparate groups, for example \"env=prod or env=staging\"."]
    pub group_labels:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignmentGroupLabel>>>,
    #[serde(rename = "instanceNamePrefixes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targets VM instances whose name starts with one of these prefixes. Like labels, this is another way to group VM instances when targeting configs, for example prefix=\"prod-\". Only supported for project-level policies."]
    pub instance_name_prefixes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targets any of the instances specified. Instances are specified by their URI in the form `zones/[ZONE]/instances/[INSTANCE_NAME]`. Instance targeting is uncommon and is supported to facilitate the management of changes by the instance or to target specific VM instances for development and testing. Only supported for project-level policies and must reference instances within this project."]
    pub instances: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "osTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targets VM instances matching at least one of the following OS types. VM instances must match all supplied criteria for a given OsType to be included."]
    pub os_types: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AssignmentOsType>>>,
    #[serde(rename = "zones")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targets instances in any of these zones. Leave empty to target instances in any zone. Zonal targeting is uncommon and is supported to facilitate the management of changes by zone."]
    pub zones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a group of VM intances that can be identified as having all these labels, for example \"env=prod and app=web\"."]
pub struct AssignmentGroupLabel {
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Compute Engine instance labels that must be present for an instance to be included in this assignment group."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines the criteria for selecting VM Instances by OS type."]
pub struct AssignmentOsType {
    #[serde(rename = "osArchitecture")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targets VM instances with OS Inventory enabled and having the following OS architecture."]
    pub os_architecture: ::std::option::Option<::std::string::String>,
    #[serde(rename = "osShortName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targets VM instances with OS Inventory enabled and having the following OS short name, for example \"debian\" or \"windows\"."]
    pub os_short_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "osVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targets VM instances with OS Inventory enabled and having the following following OS version."]
    pub os_version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message for canceling a patch job."]
pub struct CancelPatchJobRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The effective guest policy that applies to a VM instance."]
pub struct EffectiveGuestPolicy {
    #[serde(rename = "packageRepositories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of package repository configurations assigned to the VM instance."]
    pub package_repositories: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<EffectiveGuestPolicySourcedPackageRepository>>,
    >,
    #[serde(rename = "packages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of package configurations assigned to the VM instance."]
    pub packages: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<EffectiveGuestPolicySourcedPackage>>,
    >,
    #[serde(rename = "softwareRecipes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of recipes assigned to the VM instance."]
    pub software_recipes: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<EffectiveGuestPolicySourcedSoftwareRecipe>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A guest policy package including its source."]
pub struct EffectiveGuestPolicySourcedPackage {
    #[serde(rename = "package")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A software package to configure on the VM instance."]
    pub package: ::std::option::Option<::std::boxed::Box<Package>>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the guest policy providing this config."]
    pub source: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A guest policy package repository including its source."]
pub struct EffectiveGuestPolicySourcedPackageRepository {
    #[serde(rename = "packageRepository")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A software package repository to configure on the VM instance."]
    pub package_repository: ::std::option::Option<::std::boxed::Box<PackageRepository>>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the guest policy providing this config."]
    pub source: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A guest policy recipe including its source."]
pub struct EffectiveGuestPolicySourcedSoftwareRecipe {
    #[serde(rename = "softwareRecipe")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A software recipe to configure on the VM instance."]
    pub software_recipe: ::std::option::Option<::std::boxed::Box<SoftwareRecipe>>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the guest policy providing this config."]
    pub source: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A step that runs an executable for a PatchJob."]
pub struct ExecStep {
    #[serde(rename = "linuxExecStepConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ExecStepConfig for all Linux VMs targeted by the PatchJob."]
    pub linux_exec_step_config: ::std::option::Option<::std::boxed::Box<ExecStepConfig>>,
    #[serde(rename = "windowsExecStepConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ExecStepConfig for all Windows VMs targeted by the PatchJob."]
    pub windows_exec_step_config: ::std::option::Option<::std::boxed::Box<ExecStepConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Common configurations for an ExecStep."]
pub struct ExecStepConfig {
    #[serde(rename = "allowedSuccessCodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defaults to [0]. A list of possible return values that the execution can return to indicate a success."]
    pub allowed_success_codes: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "gcsObject")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Google Cloud Storage object containing the executable."]
    pub gcs_object: ::std::option::Option<::std::boxed::Box<GcsObject>>,
    #[serde(rename = "interpreter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The script interpreter to use to run the script. If no interpreter is specified the script will be executed directly, which will likely only succeed for scripts with [shebang lines] (https://en.wikipedia.org/wiki/Shebang_\\(Unix\\))."]
    pub interpreter: ::std::option::Option<ExecStepConfigInterpreterEnum>,
    #[serde(rename = "localPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An absolute path to the executable on the VM."]
    pub local_path: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The script interpreter to use to run the script. If no interpreter is specified the script will be executed directly, which will likely only succeed for scripts with [shebang lines] (https://en.wikipedia.org/wiki/Shebang_\\(Unix\\))."]
pub enum ExecStepConfigInterpreterEnum {
    #[serde(rename = "INTERPRETER_UNSPECIFIED")]
    #[doc = "Invalid for a Windows ExecStepConfig. For a Linux ExecStepConfig, the interpreter will be parsed from the shebang line of the script if unspecified."]
    InterpreterUnspecified,
    #[serde(rename = "SHELL")]
    #[doc = "Indicates that the script is run with `/bin/sh` on Linux and `cmd` on Windows."]
    Shell,
    #[serde(rename = "POWERSHELL")]
    #[doc = "Indicates that the file is run with PowerShell flags `-NonInteractive`, `-NoProfile`, and `-ExecutionPolicy Bypass`."]
    Powershell,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request message to initiate patching across Compute Engine instances."]
pub struct ExecutePatchJobRequest {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the patch job. Length of the description is limited to 1024 characters."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display name for this patch job. This does not have to be unique."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dryRun")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this patch is a dry-run only, instances are contacted but will do nothing."]
    pub dry_run: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Duration of the patch job. After the duration ends, the patch job times out."]
    pub duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "instanceFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Instances to patch, either explicitly or filtered by some criteria such as zone or labels."]
    pub instance_filter: ::std::option::Option<::std::boxed::Box<PatchInstanceFilter>>,
    #[serde(rename = "patchConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Patch configuration being applied. If omitted, instances are patched using the default configurations."]
    pub patch_config: ::std::option::Option<::std::boxed::Box<PatchConfig>>,
    #[serde(rename = "rollout")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rollout strategy of the patch job."]
    pub rollout: ::std::option::Option<::std::boxed::Box<PatchRollout>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message encapsulating a value that can be either absolute (\"fixed\") or relative (\"percent\") to a value."]
pub struct FixedOrPercent {
    #[serde(rename = "fixed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies a fixed value."]
    pub fixed: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "percent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the relative value defined as a percentage, which will be multiplied by a reference value."]
    pub percent: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Google Cloud Storage object representation."]
pub struct GcsObject {
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Bucket of the Google Cloud Storage object."]
    pub bucket: ::std::option::Option<::std::string::String>,
    #[serde(rename = "generationNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Generation number of the Google Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
    pub generation_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "object")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Name of the Google Cloud Storage object."]
    pub object: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a Goo package repository. These is added to a repo file that is stored at C:/ProgramData/GooGet/repos/google_osconfig.repo."]
pub struct GooRepository {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the repository."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The url of the repository."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Googet patching is performed by running `googet update`."]
pub struct GooSettings {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An OS Config resource representing a guest configuration policy. These policies represent the desired state for VM instance guest environments including packages to install or remove, package repository configurations, and software to install."]
pub struct GuestPolicy {
    #[serde(rename = "assignment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Specifies the VM instances that are assigned to this policy. This allows you to target sets or groups of VM instances by different parameters such as labels, names, OS, or zones. If left empty, all VM instances underneath this policy are targeted. At the same level in the resource hierarchy (that is within a project), the service prevents the creation of multiple policies that conflict with each other. For more information, see how the service [handles assignment conflicts](/compute/docs/os-config-management/create-guest-policy#handle-conflicts)."]
    pub assignment: ::std::option::Option<::std::boxed::Box<Assignment>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time this guest policy was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the guest policy. Length of the description is limited to 1024 characters."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The etag for this guest policy. If this is provided on update, it must match the server's etag."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Unique name of the resource in this project using one of the following forms: `projects/{project_number}/guestPolicies/{guest_policy_id}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "packageRepositories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of package repositories to configure on the VM instance. This is done before any other configs are applied so they can use these repos. Package repositories are only configured if the corresponding package manager(s) are available."]
    pub package_repositories:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PackageRepository>>>,
    #[serde(rename = "packages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The software packages to be managed by this policy."]
    pub packages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Package>>>,
    #[serde(rename = "recipes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of Recipes to install on the VM instance."]
    pub recipes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SoftwareRecipe>>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Last time this guest policy was updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response message for listing guest policies."]
pub struct ListGuestPoliciesResponse {
    #[serde(rename = "guestPolicies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of GuestPolicies."]
    pub guest_policies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GuestPolicy>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A pagination token that can be used to get the next page of guest policies."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response message for listing patch deployments."]
pub struct ListPatchDeploymentsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A pagination token that can be used to get the next page of patch deployments."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "patchDeployments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of patch deployments."]
    pub patch_deployments:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PatchDeployment>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response message for listing the instances details for a patch job."]
pub struct ListPatchJobInstanceDetailsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A pagination token that can be used to get the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "patchJobInstanceDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of instance status."]
    pub patch_job_instance_details:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PatchJobInstanceDetails>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response message for listing patch jobs."]
pub struct ListPatchJobsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A pagination token that can be used to get the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "patchJobs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of patch jobs."]
    pub patch_jobs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PatchJob>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request message for getting the effective guest policy assigned to the instance."]
pub struct LookupEffectiveGuestPolicyRequest {
    #[serde(rename = "osArchitecture")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Architecture of OS running on the instance. The OS Config agent only provides this field for targeting if OS Inventory is enabled for that instance."]
    pub os_architecture: ::std::option::Option<::std::string::String>,
    #[serde(rename = "osShortName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Short name of the OS running on the instance. The OS Config agent only provides this field for targeting if OS Inventory is enabled for that instance."]
    pub os_short_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "osVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Version of the OS running on the instance. The OS Config agent only provides this field for targeting if OS Inventory is enabled for that VM instance."]
    pub os_version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a monthly schedule. An example of a valid monthly schedule is \"on the third Tuesday of the month\" or \"on the 15th of the month\"."]
pub struct MonthlySchedule {
    #[serde(rename = "monthDay")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. One day of the month. 1-31 indicates the 1st to the 31st day. -1 indicates the last day of the month. Months without the target day will be skipped. For example, a schedule to run \"every month on the 31st\" will not run in February, April, June, etc."]
    pub month_day: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "weekDayOfMonth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Week day in a month."]
    pub week_day_of_month: ::std::option::Option<::std::boxed::Box<WeekDayOfMonth>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Sets the time for a one time patch deployment. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
pub struct OneTimeSchedule {
    #[serde(rename = "executeTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The desired patch job execution time."]
    pub execute_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Package is a reference to the software package to be installed or removed. The agent on the VM instance uses the system package manager to apply the config. These are the commands that the agent uses to install or remove packages. Apt install: `apt-get update && apt-get -y install package1 package2 package3` remove: `apt-get -y remove package1 package2 package3` Yum install: `yum -y install package1 package2 package3` remove: `yum -y remove package1 package2 package3` Zypper install: `zypper install package1 package2 package3` remove: `zypper rm package1 package2` Googet install: `googet -noconfirm install package1 package2 package3` remove: `googet -noconfirm remove package1 package2 package3`"]
pub struct Package {
    #[serde(rename = "desiredState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired_state the agent should maintain for this package. The default is to ensure the package is installed."]
    pub desired_state: ::std::option::Option<PackageDesiredStateEnum>,
    #[serde(rename = "manager")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of package manager that can be used to install this package. If a system does not have the package manager, the package is not installed or removed no error message is returned. By default, or if you specify `ANY`, the agent attempts to install and remove this package using the default package manager. This is useful when creating a policy that applies to different types of systems. The default behavior is ANY."]
    pub manager: ::std::option::Option<PackageManagerEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the package. A package is uniquely identified for conflict validation by checking the package name and the manager(s) that the package targets."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The desired_state the agent should maintain for this package. The default is to ensure the package is installed."]
pub enum PackageDesiredStateEnum {
    #[serde(rename = "DESIRED_STATE_UNSPECIFIED")]
    #[doc = "The default is to ensure the package is installed."]
    DesiredStateUnspecified,
    #[serde(rename = "INSTALLED")]
    #[doc = "The agent ensures that the package is installed."]
    Installed,
    #[serde(rename = "UPDATED")]
    #[doc = "The agent ensures that the package is installed and periodically checks for and install any updates."]
    Updated,
    #[serde(rename = "REMOVED")]
    #[doc = "The agent ensures that the package is not installed and uninstall it if detected."]
    Removed,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of package manager that can be used to install this package. If a system does not have the package manager, the package is not installed or removed no error message is returned. By default, or if you specify `ANY`, the agent attempts to install and remove this package using the default package manager. This is useful when creating a policy that applies to different types of systems. The default behavior is ANY."]
pub enum PackageManagerEnum {
    #[serde(rename = "MANAGER_UNSPECIFIED")]
    #[doc = "The default behavior is ANY."]
    ManagerUnspecified,
    #[serde(rename = "ANY")]
    #[doc = "Apply this package config using the default system package manager."]
    Any,
    #[serde(rename = "APT")]
    #[doc = "Apply this package config only if Apt is available on the system."]
    Apt,
    #[serde(rename = "YUM")]
    #[doc = "Apply this package config only if Yum is available on the system."]
    Yum,
    #[serde(rename = "ZYPPER")]
    #[doc = "Apply this package config only if Zypper is available on the system."]
    Zypper,
    #[serde(rename = "GOO")]
    #[doc = "Apply this package config only if GooGet is available on the system."]
    Goo,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A package repository."]
pub struct PackageRepository {
    #[serde(rename = "apt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An Apt Repository."]
    pub apt: ::std::option::Option<::std::boxed::Box<AptRepository>>,
    #[serde(rename = "goo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Goo Repository."]
    pub goo: ::std::option::Option<::std::boxed::Box<GooRepository>>,
    #[serde(rename = "yum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Yum Repository."]
    pub yum: ::std::option::Option<::std::boxed::Box<YumRepository>>,
    #[serde(rename = "zypper")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Zypper Repository."]
    pub zypper: ::std::option::Option<::std::boxed::Box<ZypperRepository>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Patch configuration specifications. Contains details on how to apply the patch(es) to a VM instance."]
pub struct PatchConfig {
    #[serde(rename = "apt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Apt update settings. Use this setting to override the default `apt` patch rules."]
    pub apt: ::std::option::Option<::std::boxed::Box<AptSettings>>,
    #[serde(rename = "goo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Goo update settings. Use this setting to override the default `goo` patch rules."]
    pub goo: ::std::option::Option<::std::boxed::Box<GooSettings>>,
    #[serde(rename = "postStep")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `ExecStep` to run after the patch update."]
    pub post_step: ::std::option::Option<::std::boxed::Box<ExecStep>>,
    #[serde(rename = "preStep")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `ExecStep` to run before the patch update."]
    pub pre_step: ::std::option::Option<::std::boxed::Box<ExecStep>>,
    #[serde(rename = "rebootConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Post-patch reboot settings."]
    pub reboot_config: ::std::option::Option<PatchConfigRebootConfigEnum>,
    #[serde(rename = "windowsUpdate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Windows update settings. Use this override the default windows patch rules."]
    pub windows_update: ::std::option::Option<::std::boxed::Box<WindowsUpdateSettings>>,
    #[serde(rename = "yum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Yum update settings. Use this setting to override the default `yum` patch rules."]
    pub yum: ::std::option::Option<::std::boxed::Box<YumSettings>>,
    #[serde(rename = "zypper")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Zypper update settings. Use this setting to override the default `zypper` patch rules."]
    pub zypper: ::std::option::Option<::std::boxed::Box<ZypperSettings>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Post-patch reboot settings."]
pub enum PatchConfigRebootConfigEnum {
    #[serde(rename = "REBOOT_CONFIG_UNSPECIFIED")]
    #[doc = "The default behavior is DEFAULT."]
    RebootConfigUnspecified,
    #[serde(rename = "DEFAULT")]
    #[doc = "The agent decides if a reboot is necessary by checking signals such as registry keys on Windows or `/var/run/reboot-required` on APT based systems. On RPM based systems, a set of core system package install times are compared with system boot time."]
    Default,
    #[serde(rename = "ALWAYS")]
    #[doc = "Always reboot the machine after the update completes."]
    Always,
    #[serde(rename = "NEVER")]
    #[doc = "Never reboot the machine after the update completes."]
    Never,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Patch deployments are configurations that individual patch jobs use to complete a patch. These configurations include instance filter, package repository settings, and a schedule. For more information about creating and managing patch deployments, see [Scheduling patch jobs](https://cloud.google.com/compute/docs/os-patch-management/schedule-patch-jobs)."]
pub struct PatchDeployment {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time the patch deployment was created. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Description of the patch deployment. Length of the description is limited to 1024 characters."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Duration of the patch. After the duration ends, the patch times out."]
    pub duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "instanceFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. VM instances to patch."]
    pub instance_filter: ::std::option::Option<::std::boxed::Box<PatchInstanceFilter>>,
    #[serde(rename = "lastExecuteTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The last time a patch job was started by this deployment. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
    pub last_execute_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique name for the patch deployment resource in a project. The patch deployment name is in the form: `projects/{project_id}/patchDeployments/{patch_deployment_id}`. This field is ignored when you create a new patch deployment."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "oneTimeSchedule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Schedule a one-time execution."]
    pub one_time_schedule: ::std::option::Option<::std::boxed::Box<OneTimeSchedule>>,
    #[serde(rename = "patchConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Patch configuration that is applied."]
    pub patch_config: ::std::option::Option<::std::boxed::Box<PatchConfig>>,
    #[serde(rename = "recurringSchedule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Schedule recurring executions."]
    pub recurring_schedule: ::std::option::Option<::std::boxed::Box<RecurringSchedule>>,
    #[serde(rename = "rollout")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Rollout strategy of the patch job."]
    pub rollout: ::std::option::Option<::std::boxed::Box<PatchRollout>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time the patch deployment was last updated. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A filter to target VM instances for patching. The targeted VMs must meet all criteria specified. So if both labels and zones are specified, the patch job targets only VMs with those labels and in those zones."]
pub struct PatchInstanceFilter {
    #[serde(rename = "all")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target all VM instances in the project. If true, no other criteria is permitted."]
    pub all: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "groupLabels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targets VM instances matching at least one of these label sets. This allows targeting of disparate groups, for example \"env=prod or env=staging\"."]
    pub group_labels:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PatchInstanceFilterGroupLabel>>>,
    #[serde(rename = "instanceNamePrefixes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targets VMs whose name starts with one of these prefixes. Similar to labels, this is another way to group VMs when targeting configs, for example prefix=\"prod-\"."]
    pub instance_name_prefixes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targets any of the VM instances specified. Instances are specified by their URI in the form `zones/[ZONE]/instances/[INSTANCE_NAME]`, `projects/[PROJECT_ID]/zones/[ZONE]/instances/[INSTANCE_NAME]`, or `https://www.googleapis.com/compute/v1/projects/[PROJECT_ID]/zones/[ZONE]/instances/[INSTANCE_NAME]`"]
    pub instances: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "zones")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targets VM instances in ANY of these zones. Leave empty to target VM instances in any zone."]
    pub zones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a group of VMs that can be identified as having all these labels, for example \"env=prod and app=web\"."]
pub struct PatchInstanceFilterGroupLabel {
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Compute Engine instance labels that must be present for a VM instance to be targeted by this filter."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A high level representation of a patch job that is either in progress or has completed. Instance details are not included in the job. To paginate through instance details, use `ListPatchJobInstanceDetails`. For more information about patch jobs, see [Creating patch jobs](https://cloud.google.com/compute/docs/os-patch-management/create-patch-job)."]
pub struct PatchJob {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time this patch job was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the patch job. Length of the description is limited to 1024 characters."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display name for this patch job. This is not a unique identifier."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dryRun")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this patch job is a dry run, the agent reports that it has finished without running any updates on the VM instance."]
    pub dry_run: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Duration of the patch job. After the duration ends, the patch job times out."]
    pub duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this patch job failed, this message provides information about the failure."]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "instanceDetailsSummary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Summary of instance details."]
    pub instance_details_summary:
        ::std::option::Option<::std::boxed::Box<PatchJobInstanceDetailsSummary>>,
    #[serde(rename = "instanceFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Instances to patch."]
    pub instance_filter: ::std::option::Option<::std::boxed::Box<PatchInstanceFilter>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier for this patch job in the form `projects/*/patchJobs/*`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "patchConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Patch configuration being applied."]
    pub patch_config: ::std::option::Option<::std::boxed::Box<PatchConfig>>,
    #[serde(rename = "patchDeployment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Name of the patch deployment that created this patch job."]
    pub patch_deployment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "percentComplete")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reflects the overall progress of the patch job in the range of 0.0 being no progress to 100.0 being complete."]
    pub percent_complete: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "rollout")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rollout strategy being applied."]
    pub rollout: ::std::option::Option<::std::boxed::Box<PatchRollout>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current state of the PatchJob."]
    pub state: ::std::option::Option<PatchJobStateEnum>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Last time this patch job was updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The current state of the PatchJob."]
pub enum PatchJobStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "State must be specified."]
    StateUnspecified,
    #[serde(rename = "STARTED")]
    #[doc = "The patch job was successfully initiated."]
    Started,
    #[serde(rename = "INSTANCE_LOOKUP")]
    #[doc = "The patch job is looking up instances to run the patch on."]
    InstanceLookup,
    #[serde(rename = "PATCHING")]
    #[doc = "Instances are being patched."]
    Patching,
    #[serde(rename = "SUCCEEDED")]
    #[doc = "Patch job completed successfully."]
    Succeeded,
    #[serde(rename = "COMPLETED_WITH_ERRORS")]
    #[doc = "Patch job completed but there were errors."]
    CompletedWithErrors,
    #[serde(rename = "CANCELED")]
    #[doc = "The patch job was canceled."]
    Canceled,
    #[serde(rename = "TIMED_OUT")]
    #[doc = "The patch job timed out."]
    TimedOut,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Patch details for a VM instance. For more information about reviewing VM instance details, see [Listing all VM instance details for a specific patch job](https://cloud.google.com/compute/docs/os-patch-management/manage-patch-jobs#list-instance-details)."]
pub struct PatchJobInstanceDetails {
    #[serde(rename = "attemptCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of times the agent that the agent attempts to apply the patch."]
    pub attempt_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the patch fails, this field provides the reason."]
    pub failure_reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "instanceSystemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier for the instance. This identifier is defined by the server."]
    pub instance_system_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The instance name in the form `projects/*/zones/*/instances/*`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Current state of instance patch."]
    pub state: ::std::option::Option<PatchJobInstanceDetailsStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Current state of instance patch."]
pub enum PatchJobInstanceDetailsStateEnum {
    #[serde(rename = "PATCH_STATE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    PatchStateUnspecified,
    #[serde(rename = "PENDING")]
    #[doc = "The instance is not yet notified."]
    Pending,
    #[serde(rename = "INACTIVE")]
    #[doc = "Instance is inactive and cannot be patched."]
    Inactive,
    #[serde(rename = "NOTIFIED")]
    #[doc = "The instance is notified that it should be patched."]
    Notified,
    #[serde(rename = "STARTED")]
    #[doc = "The instance has started the patching process."]
    Started,
    #[serde(rename = "DOWNLOADING_PATCHES")]
    #[doc = "The instance is downloading patches."]
    DownloadingPatches,
    #[serde(rename = "APPLYING_PATCHES")]
    #[doc = "The instance is applying patches."]
    ApplyingPatches,
    #[serde(rename = "REBOOTING")]
    #[doc = "The instance is rebooting."]
    Rebooting,
    #[serde(rename = "SUCCEEDED")]
    #[doc = "The instance has completed applying patches."]
    Succeeded,
    #[serde(rename = "SUCCEEDED_REBOOT_REQUIRED")]
    #[doc = "The instance has completed applying patches but a reboot is required."]
    SucceededRebootRequired,
    #[serde(rename = "FAILED")]
    #[doc = "The instance has failed to apply the patch."]
    Failed,
    #[serde(rename = "ACKED")]
    #[doc = "The instance acked the notification and will start shortly."]
    Acked,
    #[serde(rename = "TIMED_OUT")]
    #[doc = "The instance exceeded the time out while applying the patch."]
    TimedOut,
    #[serde(rename = "RUNNING_PRE_PATCH_STEP")]
    #[doc = "The instance is running the pre-patch step."]
    RunningPrePatchStep,
    #[serde(rename = "RUNNING_POST_PATCH_STEP")]
    #[doc = "The instance is running the post-patch step."]
    RunningPostPatchStep,
    #[serde(rename = "NO_AGENT_DETECTED")]
    #[doc = "The service could not detect the presence of the agent. Check to ensure that the agent is installed, running, and able to communicate with the service."]
    NoAgentDetected,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A summary of the current patch state across all instances that this patch job affects. Contains counts of instances in different states. These states map to `InstancePatchState`. List patch job instance details to see the specific states of each instance."]
pub struct PatchJobInstanceDetailsSummary {
    #[serde(rename = "ackedInstanceCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of instances that have acked and will start shortly."]
    pub acked_instance_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "applyingPatchesInstanceCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of instances that are applying patches."]
    pub applying_patches_instance_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "downloadingPatchesInstanceCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of instances that are downloading patches."]
    pub downloading_patches_instance_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "failedInstanceCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of instances that failed."]
    pub failed_instance_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inactiveInstanceCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of instances that are inactive."]
    pub inactive_instance_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "noAgentDetectedInstanceCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of instances that do not appear to be running the agent. Check to ensure that the agent is installed, running, and able to communicate with the service."]
    pub no_agent_detected_instance_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notifiedInstanceCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of instances notified about patch job."]
    pub notified_instance_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pendingInstanceCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of instances pending patch job."]
    pub pending_instance_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postPatchStepInstanceCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of instances that are running the post-patch step."]
    pub post_patch_step_instance_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "prePatchStepInstanceCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of instances that are running the pre-patch step."]
    pub pre_patch_step_instance_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rebootingInstanceCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of instances rebooting."]
    pub rebooting_instance_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startedInstanceCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of instances that have started."]
    pub started_instance_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "succeededInstanceCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of instances that have completed successfully."]
    pub succeeded_instance_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "succeededRebootRequiredInstanceCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of instances that require reboot."]
    pub succeeded_reboot_required_instance_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timedOutInstanceCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of instances that exceeded the time out while applying the patch."]
    pub timed_out_instance_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Patch rollout configuration specifications. Contains details on the concurrency control when applying patch(es) to all targeted VMs."]
pub struct PatchRollout {
    #[serde(rename = "disruptionBudget")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum number (or percentage) of VMs per zone to disrupt at any given moment. The number of VMs calculated from multiplying the percentage by the total number of VMs in a zone is rounded up. During patching, a VM is considered disrupted from the time the agent is notified to begin until patching has completed. This disruption time includes the time to complete reboot and any post-patch steps. A VM contributes to the disruption budget if its patching operation fails either when applying the patches, running pre or post patch steps, or if it fails to respond with a success notification before timing out. VMs that are not running or do not have an active agent do not count toward this disruption budget. For zone-by-zone rollouts, if the disruption budget in a zone is exceeded, the patch job stops, because continuing to the next zone requires completion of the patch process in the previous zone. For example, if the disruption budget has a fixed value of `10`, and 8 VMs fail to patch in the current zone, the patch job continues to patch 2 VMs at a time until the zone is completed. When that zone is completed successfully, patching begins with 10 VMs at a time in the next zone. If 10 VMs in the next zone fail to patch, the patch job stops."]
    pub disruption_budget: ::std::option::Option<::std::boxed::Box<FixedOrPercent>>,
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mode of the patch rollout."]
    pub mode: ::std::option::Option<PatchRolloutModeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Mode of the patch rollout."]
pub enum PatchRolloutModeEnum {
    #[serde(rename = "MODE_UNSPECIFIED")]
    #[doc = "Mode must be specified."]
    ModeUnspecified,
    #[serde(rename = "ZONE_BY_ZONE")]
    #[doc = "Patches are applied one zone at a time. The patch job begins in the region with the lowest number of targeted VMs. Within the region, patching begins in the zone with the lowest number of targeted VMs. If multiple regions (or zones within a region) have the same number of targeted VMs, a tie-breaker is achieved by sorting the regions or zones in alphabetical order."]
    ZoneByZone,
    #[serde(rename = "CONCURRENT_ZONES")]
    #[doc = "Patches are applied to VMs in all zones at the same time."]
    ConcurrentZones,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Sets the time for recurring patch deployments."]
pub struct RecurringSchedule {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The end time at which a recurring patch deployment schedule is no longer active."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The frequency unit of this recurring schedule."]
    pub frequency: ::std::option::Option<RecurringScheduleFrequencyEnum>,
    #[serde(rename = "lastExecuteTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the last patch job ran successfully."]
    pub last_execute_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "monthly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Schedule with monthly executions."]
    pub monthly: ::std::option::Option<::std::boxed::Box<MonthlySchedule>>,
    #[serde(rename = "nextExecuteTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the next patch job is scheduled to run."]
    pub next_execute_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The time that the recurring schedule becomes effective. Defaults to `create_time` of the patch deployment."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeOfDay")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Time of the day to run a recurring deployment."]
    pub time_of_day: ::std::option::Option<::std::boxed::Box<TimeOfDay>>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Defines the time zone that `time_of_day` is relative to. The rules for daylight saving time are determined by the chosen time zone."]
    pub time_zone: ::std::option::Option<::std::boxed::Box<TimeZone>>,
    #[serde(rename = "weekly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Schedule with weekly executions."]
    pub weekly: ::std::option::Option<::std::boxed::Box<WeeklySchedule>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The frequency unit of this recurring schedule."]
pub enum RecurringScheduleFrequencyEnum {
    #[serde(rename = "FREQUENCY_UNSPECIFIED")]
    #[doc = "Invalid. A frequency must be specified."]
    FrequencyUnspecified,
    #[serde(rename = "WEEKLY")]
    #[doc = "Indicates that the frequency should be expressed in terms of weeks."]
    Weekly,
    #[serde(rename = "MONTHLY")]
    #[doc = "Indicates that the frequency should be expressed in terms of months."]
    Monthly,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A software recipe is a set of instructions for installing and configuring a piece of software. It consists of a set of artifacts that are downloaded, and a set of steps that install, configure, and/or update the software. Recipes support installing and updating software from artifacts in the following formats: Zip archive, Tar archive, Windows MSI, Debian package, and RPM package. Additionally, recipes support executing a script (either defined in a file or directly in this api) in bash, sh, cmd, and powershell. Updating a software recipe If a recipe is assigned to an instance and there is a recipe with the same name but a lower version already installed and the assigned state of the recipe is `UPDATED`, then the recipe is updated to the new version. Script Working Directories Each script or execution step is run in its own temporary directory which is deleted after completing the step."]
pub struct SoftwareRecipe {
    #[serde(rename = "artifacts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resources available to be used in the steps in the recipe."]
    pub artifacts:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SoftwareRecipeArtifact>>>,
    #[serde(rename = "desiredState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Default is INSTALLED. The desired state the agent should maintain for this recipe. INSTALLED: The software recipe is installed on the instance but won't be updated to new versions. UPDATED: The software recipe is installed on the instance. The recipe is updated to a higher version, if a higher version of the recipe is assigned to this instance. REMOVE: Remove is unsupported for software recipes and attempts to create or update a recipe to the REMOVE state is rejected."]
    pub desired_state: ::std::option::Option<SoftwareRecipeDesiredStateEnum>,
    #[serde(rename = "installSteps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Actions to be taken for installing this recipe. On failure it stops executing steps and does not attempt another installation. Any steps taken (including partially completed steps) are not rolled back."]
    pub install_steps:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SoftwareRecipeStep>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Unique identifier for the recipe. Only one recipe with a given name is installed on an instance. Names are also used to identify resources which helps to determine whether guest policies have conflicts. This means that requests to create multiple recipes with the same name and version are rejected since they could potentially have conflicting assignments."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateSteps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Actions to be taken for updating this recipe. On failure it stops executing steps and does not attempt another update for this recipe. Any steps taken (including partially completed steps) are not rolled back."]
    pub update_steps: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SoftwareRecipeStep>>>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version of this software recipe. Version can be up to 4 period separated numbers (e.g. 12.34.56.78)."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Default is INSTALLED. The desired state the agent should maintain for this recipe. INSTALLED: The software recipe is installed on the instance but won't be updated to new versions. UPDATED: The software recipe is installed on the instance. The recipe is updated to a higher version, if a higher version of the recipe is assigned to this instance. REMOVE: Remove is unsupported for software recipes and attempts to create or update a recipe to the REMOVE state is rejected."]
pub enum SoftwareRecipeDesiredStateEnum {
    #[serde(rename = "DESIRED_STATE_UNSPECIFIED")]
    #[doc = "The default is to ensure the package is installed."]
    DesiredStateUnspecified,
    #[serde(rename = "INSTALLED")]
    #[doc = "The agent ensures that the package is installed."]
    Installed,
    #[serde(rename = "UPDATED")]
    #[doc = "The agent ensures that the package is installed and periodically checks for and install any updates."]
    Updated,
    #[serde(rename = "REMOVED")]
    #[doc = "The agent ensures that the package is not installed and uninstall it if detected."]
    Removed,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies a resource to be used in the recipe."]
pub struct SoftwareRecipeArtifact {
    #[serde(rename = "allowInsecure")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defaults to false. When false, recipes are subject to validations based on the artifact type: Remote: A checksum must be specified, and only protocols with transport-layer security are permitted. GCS: An object generation number must be specified."]
    pub allow_insecure: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "gcs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Google Cloud Storage artifact."]
    pub gcs: ::std::option::Option<::std::boxed::Box<SoftwareRecipeArtifactGcs>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Id of the artifact, which the installation and update steps of this recipe can reference. Artifacts in a recipe cannot have the same id."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "remote")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A generic remote artifact."]
    pub remote: ::std::option::Option<::std::boxed::Box<SoftwareRecipeArtifactRemote>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies an artifact available as a Google Cloud Storage object."]
pub struct SoftwareRecipeArtifactGcs {
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Bucket of the Google Cloud Storage object. Given an example URL: `https://storage.googleapis.com/my-bucket/foo/bar#1234567` this value would be `my-bucket`."]
    pub bucket: ::std::option::Option<::std::string::String>,
    #[serde(rename = "generation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Must be provided if allow_insecure is false. Generation number of the Google Cloud Storage object. `https://storage.googleapis.com/my-bucket/foo/bar#1234567` this value would be `1234567`."]
    pub generation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "object")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the Google Cloud Storage object. As specified [here] (https://cloud.google.com/storage/docs/naming#objectnames) Given an example URL: `https://storage.googleapis.com/my-bucket/foo/bar#1234567` this value would be `foo/bar`."]
    pub object: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies an artifact available via some URI."]
pub struct SoftwareRecipeArtifactRemote {
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Must be provided if `allow_insecure` is `false`. SHA256 checksum in hex format, to compare to the checksum of the artifact. If the checksum is not empty and it doesn't match the artifact then the recipe installation fails before running any of the steps."]
    pub checksum: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI from which to fetch the object. It should contain both the protocol and path following the format {protocol}://{location}."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An action that can be taken as part of installing or updating a recipe."]
pub struct SoftwareRecipeStep {
    #[serde(rename = "archiveExtraction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Extracts an archive into the specified directory."]
    pub archive_extraction:
        ::std::option::Option<::std::boxed::Box<SoftwareRecipeStepExtractArchive>>,
    #[serde(rename = "dpkgInstallation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Installs a deb file via dpkg."]
    pub dpkg_installation: ::std::option::Option<::std::boxed::Box<SoftwareRecipeStepInstallDpkg>>,
    #[serde(rename = "fileCopy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Copies a file onto the instance."]
    pub file_copy: ::std::option::Option<::std::boxed::Box<SoftwareRecipeStepCopyFile>>,
    #[serde(rename = "fileExec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Executes an artifact or local file."]
    pub file_exec: ::std::option::Option<::std::boxed::Box<SoftwareRecipeStepExecFile>>,
    #[serde(rename = "msiInstallation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Installs an MSI file."]
    pub msi_installation: ::std::option::Option<::std::boxed::Box<SoftwareRecipeStepInstallMsi>>,
    #[serde(rename = "rpmInstallation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Installs an rpm file via the rpm utility."]
    pub rpm_installation: ::std::option::Option<::std::boxed::Box<SoftwareRecipeStepInstallRpm>>,
    #[serde(rename = "scriptRun")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Runs commands in a shell."]
    pub script_run: ::std::option::Option<::std::boxed::Box<SoftwareRecipeStepRunScript>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Copies the artifact to the specified path on the instance."]
pub struct SoftwareRecipeStepCopyFile {
    #[serde(rename = "artifactId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The id of the relevant artifact in the recipe."]
    pub artifact_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The absolute path on the instance to put the file."]
    pub destination: ::std::option::Option<::std::string::String>,
    #[serde(rename = "overwrite")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to allow this step to overwrite existing files. If this is false and the file already exists the file is not overwritten and the step is considered a success. Defaults to false."]
    pub overwrite: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Consists of three octal digits which represent, in order, the permissions of the owner, group, and other users for the file (similarly to the numeric mode used in the linux chmod utility). Each digit represents a three bit number with the 4 bit corresponding to the read permissions, the 2 bit corresponds to the write bit, and the one bit corresponds to the execute permission. Default behavior is 755. Below are some examples of permissions and their associated values: read, write, and execute: 7 read and execute: 5 read and write: 6 read only: 4"]
    pub permissions: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Executes an artifact or local file."]
pub struct SoftwareRecipeStepExecFile {
    #[serde(rename = "allowedExitCodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defaults to [0]. A list of possible return values that the program can return to indicate a success."]
    pub allowed_exit_codes: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "args")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Arguments to be passed to the provided executable."]
    pub args: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "artifactId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of the relevant artifact in the recipe."]
    pub artifact_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "localPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The absolute path of the file on the local filesystem."]
    pub local_path: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Extracts an archive of the type specified in the specified directory."]
pub struct SoftwareRecipeStepExtractArchive {
    #[serde(rename = "artifactId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The id of the relevant artifact in the recipe."]
    pub artifact_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Directory to extract archive to. Defaults to `/` on Linux or `C:\\` on Windows."]
    pub destination: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of the archive to extract."]
    pub _type: ::std::option::Option<SoftwareRecipeStepExtractArchiveTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of the archive to extract."]
pub enum SoftwareRecipeStepExtractArchiveTypeEnum {
    #[serde(rename = "ARCHIVE_TYPE_UNSPECIFIED")]
    #[doc = "Indicates that the archive type isn't specified."]
    ArchiveTypeUnspecified,
    #[serde(rename = "TAR")]
    #[doc = "Indicates that the archive is a tar archive with no encryption."]
    Tar,
    #[serde(rename = "TAR_GZIP")]
    #[doc = "Indicates that the archive is a tar archive with gzip encryption."]
    TarGzip,
    #[serde(rename = "TAR_BZIP")]
    #[doc = "Indicates that the archive is a tar archive with bzip encryption."]
    TarBzip,
    #[serde(rename = "TAR_LZMA")]
    #[doc = "Indicates that the archive is a tar archive with lzma encryption."]
    TarLzma,
    #[serde(rename = "TAR_XZ")]
    #[doc = "Indicates that the archive is a tar archive with xz encryption."]
    TarXz,
    #[serde(rename = "ZIP")]
    #[doc = "Indicates that the archive is a zip archive."]
    Zip,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Installs a deb via dpkg."]
pub struct SoftwareRecipeStepInstallDpkg {
    #[serde(rename = "artifactId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The id of the relevant artifact in the recipe."]
    pub artifact_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Installs an MSI file."]
pub struct SoftwareRecipeStepInstallMsi {
    #[serde(rename = "allowedExitCodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Return codes that indicate that the software installed or updated successfully. Behaviour defaults to [0]"]
    pub allowed_exit_codes: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "artifactId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The id of the relevant artifact in the recipe."]
    pub artifact_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "flags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The flags to use when installing the MSI defaults to [\"/i\"] (i.e. the install flag)."]
    pub flags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Installs an rpm file via the rpm utility."]
pub struct SoftwareRecipeStepInstallRpm {
    #[serde(rename = "artifactId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The id of the relevant artifact in the recipe."]
    pub artifact_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Runs a script through an interpreter."]
pub struct SoftwareRecipeStepRunScript {
    #[serde(rename = "allowedExitCodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Return codes that indicate that the software installed or updated successfully. Behaviour defaults to [0]"]
    pub allowed_exit_codes: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "interpreter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The script interpreter to use to run the script. If no interpreter is specified the script is executed directly, which likely only succeed for scripts with [shebang lines](https://en.wikipedia.org/wiki/Shebang_\\(Unix\\))."]
    pub interpreter: ::std::option::Option<SoftwareRecipeStepRunScriptInterpreterEnum>,
    #[serde(rename = "script")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The shell script to be executed."]
    pub script: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The script interpreter to use to run the script. If no interpreter is specified the script is executed directly, which likely only succeed for scripts with [shebang lines](https://en.wikipedia.org/wiki/Shebang_\\(Unix\\))."]
pub enum SoftwareRecipeStepRunScriptInterpreterEnum {
    #[serde(rename = "INTERPRETER_UNSPECIFIED")]
    #[doc = "Default value for ScriptType."]
    InterpreterUnspecified,
    #[serde(rename = "SHELL")]
    #[doc = "Indicates that the script is run with `/bin/sh` on Linux and `cmd` on windows."]
    Shell,
    #[serde(rename = "POWERSHELL")]
    #[doc = "Indicates that the script is run with powershell."]
    Powershell,
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
#[doc = "Represents a time zone from the [IANA Time Zone Database](https://www.iana.org/time-zones)."]
pub struct TimeZone {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IANA Time Zone Database time zone, e.g. \"America/New_York\"."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. IANA Time Zone Database version number, e.g. \"2019a\"."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents one week day in a month. An example is \"the 4th Sunday\"."]
pub struct WeekDayOfMonth {
    #[serde(rename = "dayOfWeek")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A day of the week."]
    pub day_of_week: ::std::option::Option<WeekDayOfMonthDayOfWeekEnum>,
    #[serde(rename = "weekOrdinal")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Week number in a month. 1-4 indicates the 1st to 4th week of the month. -1 indicates the last week of the month."]
    pub week_ordinal: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. A day of the week."]
pub enum WeekDayOfMonthDayOfWeekEnum {
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
#[doc = "Represents a weekly schedule."]
pub struct WeeklySchedule {
    #[serde(rename = "dayOfWeek")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Day of the week."]
    pub day_of_week: ::std::option::Option<WeeklyScheduleDayOfWeekEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Day of the week."]
pub enum WeeklyScheduleDayOfWeekEnum {
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
#[doc = "Windows patching is performed using the Windows Update Agent."]
pub struct WindowsUpdateSettings {
    #[serde(rename = "classifications")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only apply updates of these windows update classifications. If empty, all updates are applied."]
    pub classifications:
        ::std::option::Option<::std::vec::Vec<WindowsUpdateSettingsClassificationsEnum>>,
    #[serde(rename = "excludes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of KBs to exclude from update."]
    pub excludes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "exclusivePatches")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An exclusive list of kbs to be updated. These are the only patches that will be updated. This field must not be used with other patch configurations."]
    pub exclusive_patches: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum WindowsUpdateSettingsClassificationsEnum {
    #[serde(rename = "CLASSIFICATION_UNSPECIFIED")]
    #[doc = "Invalid. If classifications are included, they must be specified."]
    ClassificationUnspecified,
    #[serde(rename = "CRITICAL")]
    #[doc = "\"A widely released fix for a specific problem that addresses a critical, non-security-related bug.\" [1]"]
    Critical,
    #[serde(rename = "SECURITY")]
    #[doc = "\"A widely released fix for a product-specific, security-related vulnerability. Security vulnerabilities are rated by their severity. The severity rating is indicated in the Microsoft security bulletin as critical, important, moderate, or low.\" [1]"]
    Security,
    #[serde(rename = "DEFINITION")]
    #[doc = "\"A widely released and frequent software update that contains additions to a product's definition database. Definition databases are often used to detect objects that have specific attributes, such as malicious code, phishing websites, or junk mail.\" [1]"]
    Definition,
    #[serde(rename = "DRIVER")]
    #[doc = "\"Software that controls the input and output of a device.\" [1]"]
    Driver,
    #[serde(rename = "FEATURE_PACK")]
    #[doc = "\"New product functionality that is first distributed outside the context of a product release and that is typically included in the next full product release.\" [1]"]
    FeaturePack,
    #[serde(rename = "SERVICE_PACK")]
    #[doc = "\"A tested, cumulative set of all hotfixes, security updates, critical updates, and updates. Additionally, service packs may contain additional fixes for problems that are found internally since the release of the product. Service packs my also contain a limited number of customer-requested design changes or features.\" [1]"]
    ServicePack,
    #[serde(rename = "TOOL")]
    #[doc = "\"A utility or feature that helps complete a task or set of tasks.\" [1]"]
    Tool,
    #[serde(rename = "UPDATE_ROLLUP")]
    #[doc = "\"A tested, cumulative set of hotfixes, security updates, critical updates, and updates that are packaged together for easy deployment. A rollup generally targets a specific area, such as security, or a component of a product, such as Internet Information Services (IIS).\" [1]"]
    UpdateRollup,
    #[serde(rename = "UPDATE")]
    #[doc = "\"A widely released fix for a specific problem. An update addresses a noncritical, non-security-related bug.\" [1]"]
    Update,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a single Yum package repository. This repository is added to a repo file that is stored at `/etc/yum.repos.d/google_osconfig.repo`."]
pub struct YumRepository {
    #[serde(rename = "baseUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The location of the repository directory."]
    pub base_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name of the repository."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gpgKeys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URIs of GPG keys."]
    pub gpg_keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A one word, unique name for this repository. This is the `repo id` in the Yum config file and also the `display_name` if `display_name` is omitted. This id is also used as the unique identifier when checking for guest policy conflicts."]
    pub id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Yum patching is performed by executing `yum update`. Additional options can be set to control how this is executed. Note that not all settings are supported on all platforms."]
pub struct YumSettings {
    #[serde(rename = "excludes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of packages to exclude from update. These packages are excluded by using the yum `--exclude` flag."]
    pub excludes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "exclusivePackages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An exclusive list of packages to be updated. These are the only packages that will be updated. If these packages are not installed, they will be ignored. This field must not be specified with any other patch configuration fields."]
    pub exclusive_packages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "minimal")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Will cause patch to run `yum update-minimal` instead."]
    pub minimal: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "security")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Adds the `--security` flag to `yum update`. Not supported on all platforms."]
    pub security: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a single Zypper package repository. This repository is added to a repo file that is stored at `/etc/zypp/repos.d/google_osconfig.repo`."]
pub struct ZypperRepository {
    #[serde(rename = "baseUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The location of the repository directory."]
    pub base_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name of the repository."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gpgKeys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URIs of GPG keys."]
    pub gpg_keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A one word, unique name for this repository. This is the `repo id` in the zypper config file and also the `display_name` if `display_name` is omitted. This id is also used as the unique identifier when checking for guest policy conflicts."]
    pub id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Zypper patching is performed by running `zypper patch`. See also https://en.opensuse.org/SDB:Zypper_manual."]
pub struct ZypperSettings {
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Install only patches with these categories. Common categories include security, recommended, and feature."]
    pub categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "excludes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of patches to exclude from update."]
    pub excludes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "exclusivePatches")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An exclusive list of patches to be updated. These are the only patches that will be installed using 'zypper patch patch:' command. This field must not be used with any other patch configuration fields."]
    pub exclusive_patches: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "severities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Install only patches with these severities. Common severities include critical, important, moderate, and low."]
    pub severities: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "withOptional")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Adds the `--with-optional` flag to `zypper patch`."]
    pub with_optional: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "withUpdate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Adds the `--with-update` flag, to `zypper patch`."]
    pub with_update: ::std::option::Option<::std::primitive::bool>,
}
