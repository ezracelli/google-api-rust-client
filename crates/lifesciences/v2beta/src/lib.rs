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
        serde_json::from_str(&"\"json\"").unwrap()
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
            pub mod locations {
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
                            #[doc = "The standard list filter."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list page size."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list page token."]
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
                    pub mod operations {
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
                                    #[doc = "A string for filtering Operations. The following filter fields are supported: * createTime: The time this job was created * events: The set of event (names) that have occurred while running the pipeline. The : operator can be used to determine if a particular event has occurred. * error: If the pipeline is running, this value is NULL. Once the pipeline finishes, the value is the standard Google error code. * labels.key or labels.\"key with space\" where key is a label key. * done: If the pipeline is running, this value is false. Once the pipeline finishes, the value is true."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The maximum number of results to return. The maximum value is 256."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The standard list page token."]
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
    #[doc = "Carries information about an accelerator that can be attached to a VM."]
    pub struct Accelerator {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How many accelerators of this type to attach."]
        pub count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The accelerator type string (for example, \"nvidia-tesla-k80\"). Only NVIDIA GPU accelerators are currently supported. If an NVIDIA GPU is attached, the required runtime libraries will be made available to all containers under `/usr/local/nvidia`. The driver version to install must be specified using the NVIDIA driver version parameter on the virtual machine specification. Note that attaching a GPU increases the worker VM startup time by a few minutes."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl Accelerator {
        pub fn builder() -> AcceleratorBuilder {
            AcceleratorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies a single action that runs a Docker container."]
    pub struct Action {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alwaysRun")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "By default, after an action fails, no further actions are run. This flag indicates that this action must be run even if the pipeline has already failed. This is useful for actions that copy output files off of the VM or for debugging. Note that no actions will be run if image prefetching fails."]
        pub always_run: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockExternalNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Prevents the container from accessing the external network."]
        pub block_external_network: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commands")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, overrides the `CMD` specified in the container. If the container also has an `ENTRYPOINT` the values are used as entrypoint arguments. Otherwise, they are used as a command and arguments to run inside the container."]
        pub commands: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional name for the container. The container hostname will be set to this name, making it useful for inter-container communication. The name must contain only upper and lowercase alphanumeric characters and hyphens and cannot start with a hyphen."]
        pub container_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "credentials")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the specified image is hosted on a private registry other than Google Container Registry, the credentials required to pull the image must be specified here as an encrypted secret. The secret must decrypt to a JSON-encoded dictionary containing both `username` and `password` keys."]
        pub credentials: ::std::option::Option<::std::boxed::Box<Secret>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableImagePrefetch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All container images are typically downloaded before any actions are executed. This helps prevent typos in URIs or issues like lack of disk space from wasting large amounts of compute resources. If set, this flag prevents the worker from downloading the image until just before the action is executed."]
        pub disable_image_prefetch: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableStandardErrorCapture")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A small portion of the container's standard error stream is typically captured and returned inside the `ContainerStoppedEvent`. Setting this flag disables this functionality."]
        pub disable_standard_error_capture: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableFuse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enable access to the FUSE device for this action. Filesystems can then be mounted into disks shared with other actions. The other actions do not need the `enable_fuse` flag to access the mounted filesystem. This has the effect of causing the container to be executed with `CAP_SYS_ADMIN` and exposes `/dev/fuse` to the container, so use it only for containers you trust."]
        pub enable_fuse: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entrypoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, overrides the `ENTRYPOINT` specified in the container."]
        pub entrypoint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The environment to pass into the container. This environment is merged with values specified in the google.cloud.lifesciences.v2beta.Pipeline message, overwriting any duplicate values. In addition to the values passed here, a few other values are automatically injected into the environment. These cannot be hidden or overwritten. `GOOGLE_PIPELINE_FAILED` will be set to \"1\" if the pipeline failed because an action has exited with a non-zero status (and did not have the `IGNORE_EXIT_STATUS` flag set). This can be used to determine if additional debug or logging actions should execute. `GOOGLE_LAST_EXIT_STATUS` will be set to the exit status of the last non-background action that executed. This can be used by workflow engine authors to determine whether an individual action has succeeded or failed."]
        pub environment:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ignoreExitStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Normally, a non-zero exit status causes the pipeline to fail. This flag allows execution of other actions to continue instead."]
        pub ignore_exit_status: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The URI to pull the container image from. Note that all images referenced by actions in the pipeline are pulled before the first action runs. If multiple actions reference the same image, it is only pulled once, ensuring that the same image is used for all actions in a single pipeline. The image URI can be either a complete host and image specification (e.g., quay.io/biocontainers/samtools), a library and image name (e.g., google/cloud-sdk) or a bare image name ('bash') to pull from the default library. No schema is required in any of these cases. If the specified image is not public, the service account specified for the Virtual Machine must have access to pull the images from GCR, or appropriate credentials must be specified in the google.cloud.lifesciences.v2beta.Action.credentials field."]
        pub image_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Labels to associate with the action. This field is provided to assist workflow engine authors in identifying actions (for example, to indicate what sort of action they perform, such as localization or debugging). They are returned in the operation metadata, but are otherwise ignored."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of mounts to make available to the action. In addition to the values specified here, every action has a special virtual disk mounted under `/google` that contains log files and other operational components. - /google/logs All logs written during the pipeline execution. - /google/logs/output The combined standard output and standard error of all actions run as part of the pipeline execution. - /google/logs/action/*/stdout The complete contents of each individual action's standard output. - /google/logs/action/*/stderr The complete contents of each individual action's standard error output. "]
        pub mounts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Mount>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pidNamespace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional identifier for a PID namespace to run the action inside. Multiple actions should use the same string to share a namespace. If unspecified, a separate isolated namespace is used."]
        pub pid_namespace: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "portMappings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of containers to host port mappings for this container. If the container already specifies exposed ports, use the `PUBLISH_EXPOSED_PORTS` flag instead. The host port number must be less than 65536. If it is zero, an unused random port is assigned. To determine the resulting port number, consult the `ContainerStartedEvent` in the operation metadata."]
        pub port_mappings:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishExposedPorts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Exposes all ports specified by `EXPOSE` statements in the container. To discover the host side port numbers, consult the `ACTION_STARTED` event in the operation metadata."]
        pub publish_exposed_ports: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "runInBackground")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This flag allows an action to continue running in the background while executing subsequent actions. This is useful to provide services to other actions (or to provide debugging support tools like SSH servers)."]
        pub run_in_background: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum amount of time to give the action to complete. If the action fails to complete before the timeout, it will be terminated and the exit status will be non-zero. The pipeline will continue or terminate based on the rules defined by the `ALWAYS_RUN` and `IGNORE_EXIT_STATUS` flags."]
        pub timeout: ::std::option::Option<::std::string::String>,
    }
    impl Action {
        pub fn builder() -> ActionBuilder {
            ActionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for Operations.CancelOperation."]
    pub struct CancelOperationRequest {}
    impl CancelOperationRequest {
        pub fn builder() -> CancelOperationRequestBuilder {
            CancelOperationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event generated when a container is forcibly terminated by the worker. Currently, this only occurs when the container outlives the timeout specified by the user."]
    pub struct ContainerKilledEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The numeric ID of the action that started the container."]
        pub action_id: ::std::option::Option<::std::primitive::i64>,
    }
    impl ContainerKilledEvent {
        pub fn builder() -> ContainerKilledEventBuilder {
            ContainerKilledEventBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event generated when a container starts."]
    pub struct ContainerStartedEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The numeric ID of the action that started this container."]
        pub action_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The public IP address that can be used to connect to the container. This field is only populated when at least one port mapping is present. If the instance was created with a private address, this field will be empty even if port mappings exist."]
        pub ip_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "portMappings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The container-to-host port mappings installed for this container. This set will contain any ports exposed using the `PUBLISH_EXPOSED_PORTS` flag as well as any specified in the `Action` definition."]
        pub port_mappings:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::primitive::i64>>,
    }
    impl ContainerStartedEvent {
        pub fn builder() -> ContainerStartedEventBuilder {
            ContainerStartedEventBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event generated when a container exits."]
    pub struct ContainerStoppedEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The numeric ID of the action that started this container."]
        pub action_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exitStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The exit status of the container."]
        pub exit_status: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stderr")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tail end of any content written to standard error by the container. If the content emits large amounts of debugging noise or contains sensitive information, you can prevent the content from being printed by setting the `DISABLE_STANDARD_ERROR_CAPTURE` flag. Note that only a small amount of the end of the stream is captured here. The entire stream is stored in the `/google/logs` directory mounted into each action, and can be copied off the machine as described elsewhere."]
        pub stderr: ::std::option::Option<::std::string::String>,
    }
    impl ContainerStoppedEvent {
        pub fn builder() -> ContainerStoppedEventBuilder {
            ContainerStoppedEventBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event generated whenever a resource limitation or transient error delays execution of a pipeline that was otherwise ready to run."]
    pub struct DelayedEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cause")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A textual description of the cause of the delay. The string can change without notice because it is often generated by another service (such as Compute Engine)."]
        pub cause: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the delay was caused by a resource shortage, this field lists the Compute Engine metrics that are preventing this operation from running (for example, `CPUS` or `INSTANCES`). If the particular metric is not known, a single `UNKNOWN` metric will be present."]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl DelayedEvent {
        pub fn builder() -> DelayedEventBuilder {
            DelayedEventBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Carries information about a disk that can be attached to a VM. See https://cloud.google.com/compute/docs/disks/performance for more information about disk type, size, and performance considerations. Specify either `Volume` or `Disk`, but not both."]
    pub struct Disk {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A user-supplied name for the disk. Used when mounting the disk into actions. The name must contain only upper and lowercase alphanumeric characters and hyphens and cannot start with a hyphen."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sizeGb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size, in GB, of the disk to attach. If the size is not specified, a default is chosen to ensure reasonable I/O performance. If the disk type is specified as `local-ssd`, multiple local drives are automatically combined to provide the requested size. Note, however, that each physical SSD is 375GB in size, and no more than 8 drives can be attached to a single instance."]
        pub size_gb: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional image to put on the disk before attaching it to the VM."]
        pub source_image: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Compute Engine disk type. If unspecified, `pd-standard` is used."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl Disk {
        pub fn builder() -> DiskBuilder {
            DiskBuilder::default()
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
    #[doc = "Carries information about events that occur during pipeline execution."]
    pub struct Event {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerKilled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "See google.cloud.lifesciences.v2beta.ContainerKilledEvent."]
        pub container_killed: ::std::option::Option<::std::boxed::Box<ContainerKilledEvent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerStarted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "See google.cloud.lifesciences.v2beta.ContainerStartedEvent."]
        pub container_started: ::std::option::Option<::std::boxed::Box<ContainerStartedEvent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerStopped")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "See google.cloud.lifesciences.v2beta.ContainerStoppedEvent."]
        pub container_stopped: ::std::option::Option<::std::boxed::Box<ContainerStoppedEvent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delayed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "See google.cloud.lifesciences.v2beta.DelayedEvent."]
        pub delayed: ::std::option::Option<::std::boxed::Box<DelayedEvent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A human-readable description of the event. Note that these strings can change at any time without notice. Any application logic must use the information in the `details` field."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "See google.cloud.lifesciences.v2beta.FailedEvent."]
        pub failed: ::std::option::Option<::std::boxed::Box<FailedEvent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pullStarted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "See google.cloud.lifesciences.v2beta.PullStartedEvent."]
        pub pull_started: ::std::option::Option<::std::boxed::Box<PullStartedEvent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pullStopped")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "See google.cloud.lifesciences.v2beta.PullStoppedEvent."]
        pub pull_stopped: ::std::option::Option<::std::boxed::Box<PullStoppedEvent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the event occurred."]
        pub timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unexpectedExitStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "See google.cloud.lifesciences.v2beta.UnexpectedExitStatusEvent."]
        pub unexpected_exit_status:
            ::std::option::Option<::std::boxed::Box<UnexpectedExitStatusEvent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerAssigned")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "See google.cloud.lifesciences.v2beta.WorkerAssignedEvent."]
        pub worker_assigned: ::std::option::Option<::std::boxed::Box<WorkerAssignedEvent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerReleased")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "See google.cloud.lifesciences.v2beta.WorkerReleasedEvent."]
        pub worker_released: ::std::option::Option<::std::boxed::Box<WorkerReleasedEvent>>,
    }
    impl Event {
        pub fn builder() -> EventBuilder {
            EventBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for an existing disk to be attached to the VM."]
    pub struct ExistingDisk {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disk")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If `disk` contains slashes, the Cloud Life Sciences API assumes that it is a complete URL for the disk. If `disk` does not contain slashes, the Cloud Life Sciences API assumes that the disk is a zonal disk and a URL will be generated of the form `zones//disks/`, where `` is the zone in which the instance is allocated. The disk must be ext4 formatted. If all `Mount` references to this disk have the `read_only` flag set to true, the disk will be attached in `read-only` mode and can be shared with other instances. Otherwise, the disk will be available for writing but cannot be shared."]
        pub disk: ::std::option::Option<::std::string::String>,
    }
    impl ExistingDisk {
        pub fn builder() -> ExistingDiskBuilder {
            ExistingDiskBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event generated when the execution of a pipeline has failed. Note that other events can continue to occur after this event."]
    pub struct FailedEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cause")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The human-readable description of the cause of the failure."]
        pub cause: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google standard error code that best describes this failure."]
        pub code: ::std::option::Option<FailedEventCodeEnum>,
    }
    impl FailedEvent {
        pub fn builder() -> FailedEventBuilder {
            FailedEventBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The Google standard error code that best describes this failure."]
    pub enum FailedEventCodeEnum {
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
    impl ::std::default::Default for FailedEventCodeEnum {
        fn default() -> Self {
            Self::Ok
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Locations.ListLocations."]
    pub struct ListLocationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of locations that matches the specified filter in the request."]
        pub locations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Location>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListLocationsResponse {
        pub fn builder() -> ListLocationsResponseBuilder {
            ListLocationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Operations.ListOperations."]
    pub struct ListOperationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of operations that matches the specified filter in the request."]
        pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
    }
    impl ListOperationsResponse {
        pub fn builder() -> ListOperationsResponseBuilder {
            ListOperationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A resource that represents Google Cloud Platform location."]
    pub struct Location {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The friendly name for this location, typically a nearby city name. For example, \"Tokyo\"."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cross-service attributes for the location. For example {\"cloud.googleapis.com/region\": \"us-east1\"}"]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The canonical id for this location. For example: `\"us-east1\"`."]
        pub location_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Service-specific metadata. For example the available capacity at the given location."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name for the location, which may vary between implementations. For example: `\"projects/example-project/locations/us-east1\"`"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Location {
        pub fn builder() -> LocationBuilder {
            LocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Carries information about the pipeline execution that is returned in the long running operation's metadata field."]
    pub struct Metadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the operation was created by the API."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which execution was completed and resources were cleaned up."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "events")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of events that have happened so far during the execution of this operation."]
        pub events: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Event>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user-defined labels associated with this operation."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pipeline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pipeline this operation represents."]
        pub pipeline: ::std::option::Option<::std::boxed::Box<Pipeline>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pubSubTopic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the Cloud Pub/Sub topic where notifications of operation status changes are sent."]
        pub pub_sub_topic: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The first time at which resources were allocated to execute the pipeline."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl Metadata {
        pub fn builder() -> MetadataBuilder {
            MetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Carries information about a particular disk mount inside a container."]
    pub struct Mount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disk")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the disk to mount, as specified in the resources section."]
        pub disk: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The path to mount the disk inside the container."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "readOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, the disk is mounted read-only inside the container."]
        pub read_only: ::std::option::Option<::std::primitive::bool>,
    }
    impl Mount {
        pub fn builder() -> MountBuilder {
            MountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for an `NFSMount` to be attached to the VM."]
    pub struct NfsMount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A target NFS mount. The target must be specified as `address:/mount\"."]
        pub target: ::std::option::Option<::std::string::String>,
    }
    impl NfsMount {
        pub fn builder() -> NfsMountBuilder {
            NfsMountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "VM networking options."]
    pub struct Network {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The network name to attach the VM's network interface to. The value will be prefixed with `global/networks/` unless it contains a `/`, in which case it is assumed to be a fully specified network resource URL. If unspecified, the global default network is used."]
        pub network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subnetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the specified network is configured for custom subnet creation, the name of the subnetwork to attach the instance to must be specified here. The value is prefixed with `regions/*/subnetworks/` unless it contains a `/`, in which case it is assumed to be a fully specified subnetwork resource URL. If the `*` character appears in the value, it is replaced with the region that the virtual machine has been allocated in."]
        pub subnetwork: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "usePrivateAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set to true, do not attach a public IP address to the VM. Note that without a public IP address, additional configuration is required to allow the VM to access Google services. See https://cloud.google.com/vpc/docs/configure-private-google-access for more information."]
        pub use_private_address: ::std::option::Option<::std::primitive::bool>,
    }
    impl Network {
        pub fn builder() -> NetworkBuilder {
            NetworkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This resource represents a long-running operation that is the result of a network API call."]
    pub struct Operation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "done")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
        pub done: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error result of the operation in case of failure or cancellation."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An Metadata object. This will always be returned with the Operation."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The server-assigned name for the operation. This may be passed to the other operation methods to retrieve information about the operation's status."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "response")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An Empty object."]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl Operation {
        pub fn builder() -> OperationBuilder {
            OperationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for a persistent disk to be attached to the VM. See https://cloud.google.com/compute/docs/disks/performance for more information about disk type, size, and performance considerations."]
    pub struct PersistentDisk {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sizeGb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size, in GB, of the disk to attach. If the size is not specified, a default is chosen to ensure reasonable I/O performance. If the disk type is specified as `local-ssd`, multiple local drives are automatically combined to provide the requested size. Note, however, that each physical SSD is 375GB in size, and no more than 8 drives can be attached to a single instance."]
        pub size_gb: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An image to put on the disk before attaching it to the VM."]
        pub source_image: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Compute Engine disk type. If unspecified, `pd-standard` is used."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl PersistentDisk {
        pub fn builder() -> PersistentDiskBuilder {
            PersistentDiskBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies a series of actions to execute, expressed as Docker containers."]
    pub struct Pipeline {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of actions to execute, in the order they are specified."]
        pub actions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Action>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The environment to pass into every action. Each action can also specify additional environment variables but cannot delete an entry from this map (though they can overwrite it with a different value)."]
        pub environment:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resources required for execution."]
        pub resources: ::std::option::Option<::std::boxed::Box<Resources>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum amount of time to give the pipeline to complete. This includes the time spent waiting for a worker to be allocated. If the pipeline fails to complete before the timeout, it will be cancelled and the error code will be set to DEADLINE_EXCEEDED. If unspecified, it will default to 7 days."]
        pub timeout: ::std::option::Option<::std::string::String>,
    }
    impl Pipeline {
        pub fn builder() -> PipelineBuilder {
            PipelineBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event generated when the worker starts pulling an image."]
    pub struct PullStartedEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of the image that was pulled."]
        pub image_uri: ::std::option::Option<::std::string::String>,
    }
    impl PullStartedEvent {
        pub fn builder() -> PullStartedEventBuilder {
            PullStartedEventBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event generated when the worker stops pulling an image."]
    pub struct PullStoppedEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of the image that was pulled."]
        pub image_uri: ::std::option::Option<::std::string::String>,
    }
    impl PullStoppedEvent {
        pub fn builder() -> PullStoppedEventBuilder {
            PullStoppedEventBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The system resources for the pipeline run. At least one zone or region must be specified or the pipeline run will fail."]
    pub struct Resources {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of regions allowed for VM allocation. If set, the `zones` field must not be set."]
        pub regions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "virtualMachine")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The virtual machine specification."]
        pub virtual_machine: ::std::option::Option<::std::boxed::Box<VirtualMachine>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zones")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of zones allowed for VM allocation. If set, the `regions` field must not be set."]
        pub zones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Resources {
        pub fn builder() -> ResourcesBuilder {
            ResourcesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The arguments to the `RunPipeline` method. The requesting user must have the `iam.serviceAccounts.actAs` permission for the Cloud Life Sciences service account or the request will fail."]
    pub struct RunPipelineRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-defined labels to associate with the returned operation. These labels are not propagated to any Google Cloud Platform resources used by the operation, and can be modified at any time. To associate labels with resources created while executing the operation, see the appropriate resource message (for example, `VirtualMachine`)."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pipeline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The description of the pipeline to run."]
        pub pipeline: ::std::option::Option<::std::boxed::Box<Pipeline>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pubSubTopic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of an existing Pub/Sub topic. The server will publish messages to this topic whenever the status of the operation changes. The Life Sciences Service Agent account must have publisher permissions to the specified topic or notifications will not be sent."]
        pub pub_sub_topic: ::std::option::Option<::std::string::String>,
    }
    impl RunPipelineRequest {
        pub fn builder() -> RunPipelineRequestBuilder {
            RunPipelineRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response to the RunPipeline method, returned in the operation's result field on success."]
    pub struct RunPipelineResponse {}
    impl RunPipelineResponse {
        pub fn builder() -> RunPipelineResponseBuilder {
            RunPipelineResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Holds encrypted information that is only decrypted and stored in RAM by the worker VM when running the pipeline."]
    pub struct Secret {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cipherText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of the cipherText response from the `encrypt` method. This field is intentionally unaudited."]
        pub cipher_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the Cloud KMS key that will be used to decrypt the secret value. The VM service account must have the required permissions and authentication scopes to invoke the `decrypt` method on the specified key."]
        pub key_name: ::std::option::Option<::std::string::String>,
    }
    impl Secret {
        pub fn builder() -> SecretBuilder {
            SecretBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Carries information about a Google Cloud service account."]
    pub struct ServiceAccount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email address of the service account. If not specified, the default Compute Engine service account for the project will be used."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scopes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of scopes to be enabled for this service account on the VM, in addition to the cloud-platform API scope that will be added by default."]
        pub scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ServiceAccount {
        pub fn builder() -> ServiceAccountBuilder {
            ServiceAccountBuilder::default()
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
    #[doc = "An event generated when the execution of a container results in a non-zero exit status that was not otherwise ignored. Execution will continue, but only actions that are flagged as `ALWAYS_RUN` will be executed. Other actions will be skipped."]
    pub struct UnexpectedExitStatusEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The numeric ID of the action that started the container."]
        pub action_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exitStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The exit status of the container."]
        pub exit_status: ::std::option::Option<::std::primitive::i64>,
    }
    impl UnexpectedExitStatusEvent {
        pub fn builder() -> UnexpectedExitStatusEventBuilder {
            UnexpectedExitStatusEventBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Carries information about a Compute Engine VM resource."]
    pub struct VirtualMachine {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accelerators")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of accelerators to attach to the VM."]
        pub accelerators: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Accelerator>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bootDiskSizeGb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the boot disk, in GB. The boot disk must be large enough to accommodate all of the Docker images from each action in the pipeline at the same time. If not specified, a small but reasonable default value is used."]
        pub boot_disk_size_gb: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bootImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The host operating system image to use. Currently, only Container-Optimized OS images can be used. The default value is `projects/cos-cloud/global/images/family/cos-stable`, which selects the latest stable release of Container-Optimized OS. This option is provided to allow testing against the beta release of the operating system to ensure that the new version does not interact negatively with production pipelines. To test a pipeline against the beta release of Container-Optimized OS, use the value `projects/cos-cloud/global/images/family/cos-beta`."]
        pub boot_image: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuPlatform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The CPU platform to request. An instance based on a newer platform can be allocated, but never one with fewer capabilities. The value of this parameter must be a valid Compute Engine CPU platform name (such as \"Intel Skylake\"). This parameter is only useful for carefully optimized work loads where the CPU platform has a significant impact. For more information about the effect of this parameter, see https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform."]
        pub cpu_platform: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of disks to create and attach to the VM. Specify either the `volumes[]` field or the `disks[]` field, but not both."]
        pub disks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Disk>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dockerCacheImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Compute Engine Disk Images to use as a Docker cache. The disks will be mounted into the Docker folder in a way that the images present in the cache will not need to be pulled. The digests of the cached images must match those of the tags used or the latest version will still be pulled. The root directory of the ext4 image must contain `image` and `overlay2` directories copied from the Docker directory of a VM where the desired Docker images have already been pulled. Any images pulled that are not cached will be stored on the first cache disk instead of the boot disk. Only a single image is supported."]
        pub docker_cache_images: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableStackdriverMonitoring")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether Stackdriver monitoring should be enabled on the VM."]
        pub enable_stackdriver_monitoring: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional set of labels to apply to the VM and any attached disk resources. These labels must adhere to the [name and value restrictions](https://cloud.google.com/compute/docs/labeling-resources) on VM labels imposed by Compute Engine. Labels keys with the prefix 'google-' are reserved for use by Google. Labels applied at creation time to the VM. Applied on a best-effort basis to attached disk resources shortly after VM creation."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "machineType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The machine type of the virtual machine to create. Must be the short name of a standard machine type (such as \"n1-standard-1\") or a custom machine type (such as \"custom-1-4096\", where \"1\" indicates the number of vCPUs and \"4096\" indicates the memory in MB). See [Creating an instance with a custom machine type](https://cloud.google.com/compute/docs/instances/creating-instance-with-custom-machine-type#create) for more specifications on creating a custom machine type."]
        pub machine_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The VM network configuration."]
        pub network: ::std::option::Option<::std::boxed::Box<Network>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nvidiaDriverVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The NVIDIA driver version to use when attaching an NVIDIA GPU accelerator. The version specified here must be compatible with the GPU libraries contained in the container being executed, and must be one of the drivers hosted in the `nvidia-drivers-us-public` bucket on Google Cloud Storage."]
        pub nvidia_driver_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "preemptible")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, allocate a preemptible VM."]
        pub preemptible: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceAccount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The service account to install on the VM. This account does not need any permissions other than those required by the pipeline."]
        pub service_account: ::std::option::Option<::std::boxed::Box<ServiceAccount>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of disks and other storage to create or attach to the VM. Specify either the `volumes[]` field or the `disks[]` field, but not both."]
        pub volumes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Volume>>>,
    }
    impl VirtualMachine {
        pub fn builder() -> VirtualMachineBuilder {
            VirtualMachineBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Carries information about storage that can be attached to a VM. Specify either `Volume` or `Disk`, but not both."]
    pub struct Volume {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "existingDisk")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for a existing disk."]
        pub existing_disk: ::std::option::Option<::std::boxed::Box<ExistingDisk>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nfsMount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for an NFS mount."]
        pub nfs_mount: ::std::option::Option<::std::boxed::Box<NfsMount>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "persistentDisk")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for a persistent disk."]
        pub persistent_disk: ::std::option::Option<::std::boxed::Box<PersistentDisk>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volume")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A user-supplied name for the volume. Used when mounting the volume into `Actions`. The name must contain only upper and lowercase alphanumeric characters and hyphens and cannot start with a hyphen."]
        pub volume: ::std::option::Option<::std::string::String>,
    }
    impl Volume {
        pub fn builder() -> VolumeBuilder {
            VolumeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event generated after a worker VM has been assigned to run the pipeline."]
    pub struct WorkerAssignedEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The worker's instance name."]
        pub instance: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "machineType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The machine type that was assigned for the worker."]
        pub machine_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zone the worker is running in."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl WorkerAssignedEvent {
        pub fn builder() -> WorkerAssignedEventBuilder {
            WorkerAssignedEventBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event generated when the worker VM that was assigned to the pipeline has been released (deleted)."]
    pub struct WorkerReleasedEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The worker's instance name."]
        pub instance: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zone the worker was running in."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl WorkerReleasedEvent {
        pub fn builder() -> WorkerReleasedEventBuilder {
            WorkerReleasedEventBuilder::default()
        }
    }
}
