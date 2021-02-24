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
pub mod resources {
    pub mod projects {
        pub mod resources {
            pub mod patch_deployments {
                pub mod methods {
                    pub mod create {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "patchDeploymentId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. A name for the patch deployment in the project. When creating a name the following rules apply: * Must contain only lowercase letters, numbers, and hyphens. * Must start with a letter. * Must be between 1-63 characters. * Must end with a number or a letter. * Must be unique within the project."]
                            pub patch_deployment_id: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The maximum number of patch deployments to return. Default is 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. A pagination token returned from a previous call to ListPatchDeployments that indicates where this listing should continue from."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod patch_jobs {
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If provided, this field specifies the criteria that must be met by patch jobs to be included in the response. Currently, filtering is only available on the patch_deployment field."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of instance status to return."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A pagination token returned from a previous call that indicates where this listing should continue from."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod instance_details {
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "filter")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A filter expression that filters results listed in the response. This field supports filtering results by instance zone, name, state, or `failure_reason`."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The maximum number of instance details records to return. Default is 100."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A pagination token returned from a previous call that indicates where this listing should continue from."]
                                    pub page_token: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
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
    #[doc = "Apt patching is completed by executing `apt-get update && apt-get upgrade`. Additional options can be set to control how this is executed."]
    pub struct AptSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of packages to exclude from update. These packages will be excluded"]
        pub excludes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exclusivePackages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An exclusive list of packages to be updated. These are the only packages that will be updated. If these packages are not installed, they will be ignored. This field cannot be specified with any other patch configuration fields."]
        pub exclusive_packages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "By changing the type to DIST, the patching is performed using `apt-get dist-upgrade` instead."]
        pub _type: ::std::option::Option<AptSettingsTypeEnum>,
    }
    impl AptSettings {
        pub fn builder() -> AptSettingsBuilder {
            AptSettingsBuilder::default()
        }
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
    impl ::std::default::Default for AptSettingsTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message for canceling a patch job."]
    pub struct CancelPatchJobRequest {}
    impl CancelPatchJobRequest {
        pub fn builder() -> CancelPatchJobRequestBuilder {
            CancelPatchJobRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
    pub struct Empty {}
    impl Empty {
        pub fn builder() -> EmptyBuilder {
            EmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A step that runs an executable for a PatchJob."]
    pub struct ExecStep {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linuxExecStepConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ExecStepConfig for all Linux VMs targeted by the PatchJob."]
        pub linux_exec_step_config: ::std::option::Option<::std::boxed::Box<ExecStepConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "windowsExecStepConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ExecStepConfig for all Windows VMs targeted by the PatchJob."]
        pub windows_exec_step_config: ::std::option::Option<::std::boxed::Box<ExecStepConfig>>,
    }
    impl ExecStep {
        pub fn builder() -> ExecStepBuilder {
            ExecStepBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Common configurations for an ExecStep."]
    pub struct ExecStepConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedSuccessCodes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defaults to [0]. A list of possible return values that the execution can return to indicate a success."]
        pub allowed_success_codes: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsObject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Cloud Storage object containing the executable."]
        pub gcs_object: ::std::option::Option<::std::boxed::Box<GcsObject>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "interpreter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The script interpreter to use to run the script. If no interpreter is specified the script will be executed directly, which will likely only succeed for scripts with [shebang lines] (https://en.wikipedia.org/wiki/Shebang_\\(Unix\\))."]
        pub interpreter: ::std::option::Option<ExecStepConfigInterpreterEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An absolute path to the executable on the VM."]
        pub local_path: ::std::option::Option<::std::string::String>,
    }
    impl ExecStepConfig {
        pub fn builder() -> ExecStepConfigBuilder {
            ExecStepConfigBuilder::default()
        }
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
    impl ::std::default::Default for ExecStepConfigInterpreterEnum {
        fn default() -> Self {
            Self::InterpreterUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request message to initiate patching across Compute Engine instances."]
    pub struct ExecutePatchJobRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the patch job. Length of the description is limited to 1024 characters."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display name for this patch job. This does not have to be unique."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dryRun")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this patch is a dry-run only, instances are contacted but will do nothing."]
        pub dry_run: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Duration of the patch job. After the duration ends, the patch job times out."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Instances to patch, either explicitly or filtered by some criteria such as zone or labels."]
        pub instance_filter: ::std::option::Option<::std::boxed::Box<PatchInstanceFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "patchConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Patch configuration being applied. If omitted, instances are patched using the default configurations."]
        pub patch_config: ::std::option::Option<::std::boxed::Box<PatchConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rollout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rollout strategy of the patch job."]
        pub rollout: ::std::option::Option<::std::boxed::Box<PatchRollout>>,
    }
    impl ExecutePatchJobRequest {
        pub fn builder() -> ExecutePatchJobRequestBuilder {
            ExecutePatchJobRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message encapsulating a value that can be either absolute (\"fixed\") or relative (\"percent\") to a value."]
    pub struct FixedOrPercent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fixed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies a fixed value."]
        pub fixed: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "percent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the relative value defined as a percentage, which will be multiplied by a reference value."]
        pub percent: ::std::option::Option<::std::primitive::i64>,
    }
    impl FixedOrPercent {
        pub fn builder() -> FixedOrPercentBuilder {
            FixedOrPercentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Cloud Storage object representation."]
    pub struct GcsObject {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucket")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Bucket of the Cloud Storage object."]
        pub bucket: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "generationNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Generation number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
        pub generation_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "object")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Name of the Cloud Storage object."]
        pub object: ::std::option::Option<::std::string::String>,
    }
    impl GcsObject {
        pub fn builder() -> GcsObjectBuilder {
            GcsObjectBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Googet patching is performed by running `googet update`."]
    pub struct GooSettings {}
    impl GooSettings {
        pub fn builder() -> GooSettingsBuilder {
            GooSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The inventory details of a VM."]
    pub struct Inventory {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Inventory items related to the VM keyed by an opaque unique identifier for each inventory item. The identifier is unique to each distinct and addressable inventory item and will change, when there is a new package version."]
        pub items: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<InventoryItem>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "osInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Base level operating system information for the VM."]
        pub os_info: ::std::option::Option<::std::boxed::Box<InventoryOsInfo>>,
    }
    impl Inventory {
        pub fn builder() -> InventoryBuilder {
            InventoryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single piece of inventory on a VM."]
    pub struct InventoryItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "availablePackage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Software package available to be installed on the VM instance."]
        pub available_package: ::std::option::Option<::std::boxed::Box<InventorySoftwarePackage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When this inventory item was first detected."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier for this item, unique across items for this VM."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "installedPackage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Software package present on the VM instance."]
        pub installed_package: ::std::option::Option<::std::boxed::Box<InventorySoftwarePackage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The origin of this inventory item."]
        pub origin_type: ::std::option::Option<InventoryItemOriginTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The specific type of inventory, correlating to its specific details."]
        pub _type: ::std::option::Option<InventoryItemTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When this inventory item was last modified."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl InventoryItem {
        pub fn builder() -> InventoryItemBuilder {
            InventoryItemBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The origin of this inventory item."]
    pub enum InventoryItemOriginTypeEnum {
        #[serde(rename = "ORIGIN_TYPE_UNSPECIFIED")]
        #[doc = "Invalid. An origin type must be specified."]
        OriginTypeUnspecified,
        #[serde(rename = "INVENTORY_REPORT")]
        #[doc = "This inventory item was discovered as the result of the agent reporting inventory via the reporting API."]
        InventoryReport,
    }
    impl ::std::default::Default for InventoryItemOriginTypeEnum {
        fn default() -> Self {
            Self::OriginTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The specific type of inventory, correlating to its specific details."]
    pub enum InventoryItemTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Invalid. An type must be specified."]
        TypeUnspecified,
        #[serde(rename = "INSTALLED_PACKAGE")]
        #[doc = "This represents a package that is installed on the VM."]
        InstalledPackage,
        #[serde(rename = "AVAILABLE_PACKAGE")]
        #[doc = "This represents an update that is available for a package."]
        AvailablePackage,
    }
    impl ::std::default::Default for InventoryItemTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Operating system information for the VM."]
    pub struct InventoryOsInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "architecture")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The system architecture of the operating system."]
        pub architecture: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hostname")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The VM hostname."]
        pub hostname: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kernelRelease")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kernel release of the operating system."]
        pub kernel_release: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kernelVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kernel version of the operating system."]
        pub kernel_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operating system long name. For example 'Debian GNU/Linux 9' or 'Microsoft Window Server 2019 Datacenter'."]
        pub long_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "osconfigAgentVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current version of the OS Config agent running on the VM."]
        pub osconfig_agent_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shortName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operating system short name. For example, 'windows' or 'debian'."]
        pub short_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the operating system."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl InventoryOsInfo {
        pub fn builder() -> InventoryOsInfoBuilder {
            InventoryOsInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Software package information of the operating system."]
    pub struct InventorySoftwarePackage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aptPackage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of an APT package. For details about the apt package manager, see https://wiki.debian.org/Apt."]
        pub apt_package: ::std::option::Option<::std::boxed::Box<InventoryVersionedPackage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cosPackage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of a COS package."]
        pub cos_package: ::std::option::Option<::std::boxed::Box<InventoryVersionedPackage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "googetPackage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of a Googet package. For details about the googet package manager, see https://github.com/google/googet."]
        pub googet_package: ::std::option::Option<::std::boxed::Box<InventoryVersionedPackage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "qfePackage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of a Windows Quick Fix engineering package. See https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering for info in Windows Quick Fix Engineering."]
        pub qfe_package:
            ::std::option::Option<::std::boxed::Box<InventoryWindowsQuickFixEngineeringPackage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "wuaPackage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of a Windows Update package. See https://docs.microsoft.com/en-us/windows/win32/api/_wua/ for information about Windows Update."]
        pub wua_package: ::std::option::Option<::std::boxed::Box<InventoryWindowsUpdatePackage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "yumPackage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Yum package info. For details about the yum package manager, see https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/6/html/deployment_guide/ch-yum."]
        pub yum_package: ::std::option::Option<::std::boxed::Box<InventoryVersionedPackage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zypperPackage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of a Zypper package. For details about the Zypper package manager, see https://en.opensuse.org/SDB:Zypper_manual."]
        pub zypper_package: ::std::option::Option<::std::boxed::Box<InventoryVersionedPackage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zypperPatch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of a Zypper patch. For details about the Zypper package manager, see https://en.opensuse.org/SDB:Zypper_manual."]
        pub zypper_patch: ::std::option::Option<::std::boxed::Box<InventoryZypperPatch>>,
    }
    impl InventorySoftwarePackage {
        pub fn builder() -> InventorySoftwarePackageBuilder {
            InventorySoftwarePackageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information related to the a standard versioned package. This includes package info for APT, Yum, Zypper, and Googet package managers."]
    pub struct InventoryVersionedPackage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "architecture")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The system architecture this package is intended for."]
        pub architecture: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the package."]
        pub package_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the package."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl InventoryVersionedPackage {
        pub fn builder() -> InventoryVersionedPackageBuilder {
            InventoryVersionedPackageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information related to a Quick Fix Engineering package. Fields are taken from Windows QuickFixEngineering Interface and match the source names: https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering"]
    pub struct InventoryWindowsQuickFixEngineeringPackage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "caption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short textual description of the QFE update."]
        pub caption: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A textual description of the QFE update."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hotFixId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier associated with a particular QFE update."]
        pub hot_fix_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "installTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date that the QFE update was installed. Mapped from installed_on field."]
        pub install_time: ::std::option::Option<::std::string::String>,
    }
    impl InventoryWindowsQuickFixEngineeringPackage {
        pub fn builder() -> InventoryWindowsQuickFixEngineeringPackageBuilder {
            InventoryWindowsQuickFixEngineeringPackageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details related to a Windows Update package. Field data and names are taken from Windows Update API IUpdate Interface: https://docs.microsoft.com/en-us/windows/win32/api/_wua/ Descriptive fields like title, and description are localized based on the locale of the VM being updated."]
    pub struct InventoryWindowsUpdatePackage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The categories that are associated with this update package."]
        pub categories: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<InventoryWindowsUpdatePackageWindowsUpdateCategory>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The localized description of the update package."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kbArticleIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A collection of Microsoft Knowledge Base article IDs that are associated with the update package."]
        pub kb_article_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastDeploymentChangeTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last published date of the update, in (UTC) date and time."]
        pub last_deployment_change_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "moreInfoUrls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A collection of URLs that provide more information about the update package."]
        pub more_info_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The revision number of this update package."]
        pub revision_number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A hyperlink to the language-specific support information for the update."]
        pub support_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The localized title of the update package."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Gets the identifier of an update package. Stays the same across revisions."]
        pub update_id: ::std::option::Option<::std::string::String>,
    }
    impl InventoryWindowsUpdatePackage {
        pub fn builder() -> InventoryWindowsUpdatePackageBuilder {
            InventoryWindowsUpdatePackageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Categories specified by the Windows Update."]
    pub struct InventoryWindowsUpdatePackageWindowsUpdateCategory {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of the windows update category."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the windows update category."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl InventoryWindowsUpdatePackageWindowsUpdateCategory {
        pub fn builder() -> InventoryWindowsUpdatePackageWindowsUpdateCategoryBuilder {
            InventoryWindowsUpdatePackageWindowsUpdateCategoryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details related to a Zypper Patch."]
    pub struct InventoryZypperPatch {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "category")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The category of the patch."]
        pub category: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "patchName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the patch."]
        pub patch_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The severity specified for this patch"]
        pub severity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "summary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Any summary information provided about this patch."]
        pub summary: ::std::option::Option<::std::string::String>,
    }
    impl InventoryZypperPatch {
        pub fn builder() -> InventoryZypperPatchBuilder {
            InventoryZypperPatchBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response message for listing patch deployments."]
    pub struct ListPatchDeploymentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A pagination token that can be used to get the next page of patch deployments."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "patchDeployments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of patch deployments."]
        pub patch_deployments:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PatchDeployment>>>,
    }
    impl ListPatchDeploymentsResponse {
        pub fn builder() -> ListPatchDeploymentsResponseBuilder {
            ListPatchDeploymentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response message for listing the instances details for a patch job."]
    pub struct ListPatchJobInstanceDetailsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A pagination token that can be used to get the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "patchJobInstanceDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of instance status."]
        pub patch_job_instance_details:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PatchJobInstanceDetails>>>,
    }
    impl ListPatchJobInstanceDetailsResponse {
        pub fn builder() -> ListPatchJobInstanceDetailsResponseBuilder {
            ListPatchJobInstanceDetailsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response message for listing patch jobs."]
    pub struct ListPatchJobsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A pagination token that can be used to get the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "patchJobs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of patch jobs."]
        pub patch_jobs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PatchJob>>>,
    }
    impl ListPatchJobsResponse {
        pub fn builder() -> ListPatchJobsResponseBuilder {
            ListPatchJobsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a monthly schedule. An example of a valid monthly schedule is \"on the third Tuesday of the month\" or \"on the 15th of the month\"."]
    pub struct MonthlySchedule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "monthDay")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. One day of the month. 1-31 indicates the 1st to the 31st day. -1 indicates the last day of the month. Months without the target day will be skipped. For example, a schedule to run \"every month on the 31st\" will not run in February, April, June, etc."]
        pub month_day: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "weekDayOfMonth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Week day in a month."]
        pub week_day_of_month: ::std::option::Option<::std::boxed::Box<WeekDayOfMonth>>,
    }
    impl MonthlySchedule {
        pub fn builder() -> MonthlyScheduleBuilder {
            MonthlyScheduleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Sets the time for a one time patch deployment. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
    pub struct OneTimeSchedule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executeTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The desired patch job execution time."]
        pub execute_time: ::std::option::Option<::std::string::String>,
    }
    impl OneTimeSchedule {
        pub fn builder() -> OneTimeScheduleBuilder {
            OneTimeScheduleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Patch configuration specifications. Contains details on how to apply the patch(es) to a VM instance."]
    pub struct PatchConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Apt update settings. Use this setting to override the default `apt` patch rules."]
        pub apt: ::std::option::Option<::std::boxed::Box<AptSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "goo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Goo update settings. Use this setting to override the default `goo` patch rules."]
        pub goo: ::std::option::Option<::std::boxed::Box<GooSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postStep")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `ExecStep` to run after the patch update."]
        pub post_step: ::std::option::Option<::std::boxed::Box<ExecStep>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "preStep")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `ExecStep` to run before the patch update."]
        pub pre_step: ::std::option::Option<::std::boxed::Box<ExecStep>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rebootConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Post-patch reboot settings."]
        pub reboot_config: ::std::option::Option<PatchConfigRebootConfigEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "windowsUpdate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Windows update settings. Use this override the default windows patch rules."]
        pub windows_update: ::std::option::Option<::std::boxed::Box<WindowsUpdateSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "yum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Yum update settings. Use this setting to override the default `yum` patch rules."]
        pub yum: ::std::option::Option<::std::boxed::Box<YumSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zypper")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Zypper update settings. Use this setting to override the default `zypper` patch rules."]
        pub zypper: ::std::option::Option<::std::boxed::Box<ZypperSettings>>,
    }
    impl PatchConfig {
        pub fn builder() -> PatchConfigBuilder {
            PatchConfigBuilder::default()
        }
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
    impl ::std::default::Default for PatchConfigRebootConfigEnum {
        fn default() -> Self {
            Self::RebootConfigUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Patch deployments are configurations that individual patch jobs use to complete a patch. These configurations include instance filter, package repository settings, and a schedule. For more information about creating and managing patch deployments, see [Scheduling patch jobs](https://cloud.google.com/compute/docs/os-patch-management/schedule-patch-jobs)."]
    pub struct PatchDeployment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time the patch deployment was created. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Description of the patch deployment. Length of the description is limited to 1024 characters."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Duration of the patch. After the duration ends, the patch times out."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. VM instances to patch."]
        pub instance_filter: ::std::option::Option<::std::boxed::Box<PatchInstanceFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastExecuteTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The last time a patch job was started by this deployment. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
        pub last_execute_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique name for the patch deployment resource in a project. The patch deployment name is in the form: `projects/{project_id}/patchDeployments/{patch_deployment_id}`. This field is ignored when you create a new patch deployment."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oneTimeSchedule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Schedule a one-time execution."]
        pub one_time_schedule: ::std::option::Option<::std::boxed::Box<OneTimeSchedule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "patchConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Patch configuration that is applied."]
        pub patch_config: ::std::option::Option<::std::boxed::Box<PatchConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recurringSchedule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Schedule recurring executions."]
        pub recurring_schedule: ::std::option::Option<::std::boxed::Box<RecurringSchedule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rollout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Rollout strategy of the patch job."]
        pub rollout: ::std::option::Option<::std::boxed::Box<PatchRollout>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time the patch deployment was last updated. Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl PatchDeployment {
        pub fn builder() -> PatchDeploymentBuilder {
            PatchDeploymentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A filter to target VM instances for patching. The targeted VMs must meet all criteria specified. So if both labels and zones are specified, the patch job targets only VMs with those labels and in those zones."]
    pub struct PatchInstanceFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "all")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target all VM instances in the project. If true, no other criteria is permitted."]
        pub all: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Targets VM instances matching ANY of these GroupLabels. This allows targeting of disparate groups of VM instances."]
        pub group_labels: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<PatchInstanceFilterGroupLabel>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceNamePrefixes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Targets VMs whose name starts with one of these prefixes. Similar to labels, this is another way to group VMs when targeting configs, for example prefix=\"prod-\"."]
        pub instance_name_prefixes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instances")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Targets any of the VM instances specified. Instances are specified by their URI in the form `zones/[ZONE]/instances/[INSTANCE_NAME]`, `projects/[PROJECT_ID]/zones/[ZONE]/instances/[INSTANCE_NAME]`, or `https://www.googleapis.com/compute/v1/projects/[PROJECT_ID]/zones/[ZONE]/instances/[INSTANCE_NAME]`"]
        pub instances: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zones")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Targets VM instances in ANY of these zones. Leave empty to target VM instances in any zone."]
        pub zones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl PatchInstanceFilter {
        pub fn builder() -> PatchInstanceFilterBuilder {
            PatchInstanceFilterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Targets a group of VM instances by using their [assigned labels](https://cloud.google.com/compute/docs/labeling-resources). Labels are key-value pairs. A `GroupLabel` is a combination of labels that is used to target VMs for a patch job. For example, a patch job can target VMs that have the following `GroupLabel`: `{\"env\":\"test\", \"app\":\"web\"}`. This means that the patch job is applied to VMs that have both the labels `env=test` and `app=web`."]
    pub struct PatchInstanceFilterGroupLabel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Compute Engine instance labels that must be present for a VM instance to be targeted by this filter."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl PatchInstanceFilterGroupLabel {
        pub fn builder() -> PatchInstanceFilterGroupLabelBuilder {
            PatchInstanceFilterGroupLabelBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A high level representation of a patch job that is either in progress or has completed. Instance details are not included in the job. To paginate through instance details, use ListPatchJobInstanceDetails. For more information about patch jobs, see [Creating patch jobs](https://cloud.google.com/compute/docs/os-patch-management/create-patch-job)."]
    pub struct PatchJob {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time this patch job was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the patch job. Length of the description is limited to 1024 characters."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display name for this patch job. This is not a unique identifier."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dryRun")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this patch job is a dry run, the agent reports that it has finished without running any updates on the VM instance."]
        pub dry_run: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Duration of the patch job. After the duration ends, the patch job times out."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this patch job failed, this message provides information about the failure."]
        pub error_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceDetailsSummary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Summary of instance details."]
        pub instance_details_summary:
            ::std::option::Option<::std::boxed::Box<PatchJobInstanceDetailsSummary>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instances to patch."]
        pub instance_filter: ::std::option::Option<::std::boxed::Box<PatchInstanceFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier for this patch job in the form `projects/*/patchJobs/*`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "patchConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Patch configuration being applied."]
        pub patch_config: ::std::option::Option<::std::boxed::Box<PatchConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "patchDeployment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Name of the patch deployment that created this patch job."]
        pub patch_deployment: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "percentComplete")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reflects the overall progress of the patch job in the range of 0.0 being no progress to 100.0 being complete."]
        pub percent_complete: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rollout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rollout strategy being applied."]
        pub rollout: ::std::option::Option<::std::boxed::Box<PatchRollout>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current state of the PatchJob."]
        pub state: ::std::option::Option<PatchJobStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last time this patch job was updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl PatchJob {
        pub fn builder() -> PatchJobBuilder {
            PatchJobBuilder::default()
        }
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
    impl ::std::default::Default for PatchJobStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Patch details for a VM instance. For more information about reviewing VM instance details, see [Listing all VM instance details for a specific patch job](https://cloud.google.com/compute/docs/os-patch-management/manage-patch-jobs#list-instance-details)."]
    pub struct PatchJobInstanceDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attemptCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of times the agent that the agent attempts to apply the patch."]
        pub attempt_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failureReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the patch fails, this field provides the reason."]
        pub failure_reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceSystemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier for the instance. This identifier is defined by the server."]
        pub instance_system_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The instance name in the form `projects/*/zones/*/instances/*`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Current state of instance patch."]
        pub state: ::std::option::Option<PatchJobInstanceDetailsStateEnum>,
    }
    impl PatchJobInstanceDetails {
        pub fn builder() -> PatchJobInstanceDetailsBuilder {
            PatchJobInstanceDetailsBuilder::default()
        }
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
    impl ::std::default::Default for PatchJobInstanceDetailsStateEnum {
        fn default() -> Self {
            Self::PatchStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A summary of the current patch state across all instances that this patch job affects. Contains counts of instances in different states. These states map to `InstancePatchState`. List patch job instance details to see the specific states of each instance."]
    pub struct PatchJobInstanceDetailsSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ackedInstanceCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of instances that have acked and will start shortly."]
        pub acked_instance_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applyingPatchesInstanceCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of instances that are applying patches."]
        pub applying_patches_instance_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadingPatchesInstanceCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of instances that are downloading patches."]
        pub downloading_patches_instance_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failedInstanceCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of instances that failed."]
        pub failed_instance_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inactiveInstanceCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of instances that are inactive."]
        pub inactive_instance_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "noAgentDetectedInstanceCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of instances that do not appear to be running the agent. Check to ensure that the agent is installed, running, and able to communicate with the service."]
        pub no_agent_detected_instance_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notifiedInstanceCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of instances notified about patch job."]
        pub notified_instance_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pendingInstanceCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of instances pending patch job."]
        pub pending_instance_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postPatchStepInstanceCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of instances that are running the post-patch step."]
        pub post_patch_step_instance_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prePatchStepInstanceCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of instances that are running the pre-patch step."]
        pub pre_patch_step_instance_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rebootingInstanceCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of instances rebooting."]
        pub rebooting_instance_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startedInstanceCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of instances that have started."]
        pub started_instance_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "succeededInstanceCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of instances that have completed successfully."]
        pub succeeded_instance_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "succeededRebootRequiredInstanceCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of instances that require reboot."]
        pub succeeded_reboot_required_instance_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timedOutInstanceCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of instances that exceeded the time out while applying the patch."]
        pub timed_out_instance_count: ::std::option::Option<::std::string::String>,
    }
    impl PatchJobInstanceDetailsSummary {
        pub fn builder() -> PatchJobInstanceDetailsSummaryBuilder {
            PatchJobInstanceDetailsSummaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Patch rollout configuration specifications. Contains details on the concurrency control when applying patch(es) to all targeted VMs."]
    pub struct PatchRollout {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disruptionBudget")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number (or percentage) of VMs per zone to disrupt at any given moment. The number of VMs calculated from multiplying the percentage by the total number of VMs in a zone is rounded up. During patching, a VM is considered disrupted from the time the agent is notified to begin until patching has completed. This disruption time includes the time to complete reboot and any post-patch steps. A VM contributes to the disruption budget if its patching operation fails either when applying the patches, running pre or post patch steps, or if it fails to respond with a success notification before timing out. VMs that are not running or do not have an active agent do not count toward this disruption budget. For zone-by-zone rollouts, if the disruption budget in a zone is exceeded, the patch job stops, because continuing to the next zone requires completion of the patch process in the previous zone. For example, if the disruption budget has a fixed value of `10`, and 8 VMs fail to patch in the current zone, the patch job continues to patch 2 VMs at a time until the zone is completed. When that zone is completed successfully, patching begins with 10 VMs at a time in the next zone. If 10 VMs in the next zone fail to patch, the patch job stops."]
        pub disruption_budget: ::std::option::Option<::std::boxed::Box<FixedOrPercent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mode of the patch rollout."]
        pub mode: ::std::option::Option<PatchRolloutModeEnum>,
    }
    impl PatchRollout {
        pub fn builder() -> PatchRolloutBuilder {
            PatchRolloutBuilder::default()
        }
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
    impl ::std::default::Default for PatchRolloutModeEnum {
        fn default() -> Self {
            Self::ModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Sets the time for recurring patch deployments."]
    pub struct RecurringSchedule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The end time at which a recurring patch deployment schedule is no longer active."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frequency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The frequency unit of this recurring schedule."]
        pub frequency: ::std::option::Option<RecurringScheduleFrequencyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastExecuteTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time the last patch job ran successfully."]
        pub last_execute_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "monthly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Schedule with monthly executions."]
        pub monthly: ::std::option::Option<::std::boxed::Box<MonthlySchedule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextExecuteTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time the next patch job is scheduled to run."]
        pub next_execute_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The time that the recurring schedule becomes effective. Defaults to `create_time` of the patch deployment."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOfDay")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Time of the day to run a recurring deployment."]
        pub time_of_day: ::std::option::Option<::std::boxed::Box<TimeOfDay>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Defines the time zone that `time_of_day` is relative to. The rules for daylight saving time are determined by the chosen time zone."]
        pub time_zone: ::std::option::Option<::std::boxed::Box<TimeZone>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "weekly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Schedule with weekly executions."]
        pub weekly: ::std::option::Option<::std::boxed::Box<WeeklySchedule>>,
    }
    impl RecurringSchedule {
        pub fn builder() -> RecurringScheduleBuilder {
            RecurringScheduleBuilder::default()
        }
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
    impl ::std::default::Default for RecurringScheduleFrequencyEnum {
        fn default() -> Self {
            Self::FrequencyUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`."]
    pub struct TimeOfDay {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hours")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
        pub hours: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minutes of hour of day. Must be from 0 to 59."]
        pub minutes: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nanos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
        pub nanos: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds."]
        pub seconds: ::std::option::Option<::std::primitive::i64>,
    }
    impl TimeOfDay {
        pub fn builder() -> TimeOfDayBuilder {
            TimeOfDayBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a time zone from the [IANA Time Zone Database](https://www.iana.org/time-zones)."]
    pub struct TimeZone {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IANA Time Zone Database time zone, e.g. \"America/New_York\"."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. IANA Time Zone Database version number, e.g. \"2019a\"."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl TimeZone {
        pub fn builder() -> TimeZoneBuilder {
            TimeZoneBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents one week day in a month. An example is \"the 4th Sunday\"."]
    pub struct WeekDayOfMonth {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dayOfWeek")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A day of the week."]
        pub day_of_week: ::std::option::Option<WeekDayOfMonthDayOfWeekEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "weekOrdinal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Week number in a month. 1-4 indicates the 1st to 4th week of the month. -1 indicates the last week of the month."]
        pub week_ordinal: ::std::option::Option<::std::primitive::i64>,
    }
    impl WeekDayOfMonth {
        pub fn builder() -> WeekDayOfMonthBuilder {
            WeekDayOfMonthBuilder::default()
        }
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
    impl ::std::default::Default for WeekDayOfMonthDayOfWeekEnum {
        fn default() -> Self {
            Self::DayOfWeekUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a weekly schedule."]
    pub struct WeeklySchedule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dayOfWeek")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Day of the week."]
        pub day_of_week: ::std::option::Option<WeeklyScheduleDayOfWeekEnum>,
    }
    impl WeeklySchedule {
        pub fn builder() -> WeeklyScheduleBuilder {
            WeeklyScheduleBuilder::default()
        }
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
    impl ::std::default::Default for WeeklyScheduleDayOfWeekEnum {
        fn default() -> Self {
            Self::DayOfWeekUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Windows patching is performed using the Windows Update Agent."]
    pub struct WindowsUpdateSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "classifications")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only apply updates of these windows update classifications. If empty, all updates are applied."]
        pub classifications:
            ::std::option::Option<::std::vec::Vec<WindowsUpdateSettingsClassificationsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of KBs to exclude from update."]
        pub excludes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exclusivePatches")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An exclusive list of kbs to be updated. These are the only patches that will be updated. This field must not be used with other patch configurations."]
        pub exclusive_patches: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl WindowsUpdateSettings {
        pub fn builder() -> WindowsUpdateSettingsBuilder {
            WindowsUpdateSettingsBuilder::default()
        }
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
    impl ::std::default::Default for WindowsUpdateSettingsClassificationsEnum {
        fn default() -> Self {
            Self::ClassificationUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Yum patching is performed by executing `yum update`. Additional options can be set to control how this is executed. Note that not all settings are supported on all platforms."]
    pub struct YumSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of packages to exclude from update. These packages are excluded by using the yum `--exclude` flag."]
        pub excludes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exclusivePackages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An exclusive list of packages to be updated. These are the only packages that will be updated. If these packages are not installed, they will be ignored. This field must not be specified with any other patch configuration fields."]
        pub exclusive_packages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Will cause patch to run `yum update-minimal` instead."]
        pub minimal: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "security")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Adds the `--security` flag to `yum update`. Not supported on all platforms."]
        pub security: ::std::option::Option<::std::primitive::bool>,
    }
    impl YumSettings {
        pub fn builder() -> YumSettingsBuilder {
            YumSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Zypper patching is performed by running `zypper patch`. See also https://en.opensuse.org/SDB:Zypper_manual."]
    pub struct ZypperSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Install only patches with these categories. Common categories include security, recommended, and feature."]
        pub categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of patches to exclude from update."]
        pub excludes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exclusivePatches")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An exclusive list of patches to be updated. These are the only patches that will be installed using 'zypper patch patch:' command. This field must not be used with any other patch configuration fields."]
        pub exclusive_patches: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Install only patches with these severities. Common severities include critical, important, moderate, and low."]
        pub severities: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "withOptional")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Adds the `--with-optional` flag to `zypper patch`."]
        pub with_optional: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "withUpdate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Adds the `--with-update` flag, to `zypper patch`."]
        pub with_update: ::std::option::Option<::std::primitive::bool>,
    }
    impl ZypperSettings {
        pub fn builder() -> ZypperSettingsBuilder {
            ZypperSettingsBuilder::default()
        }
    }
}
