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
            pub mod worker_pools {
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
                            #[serde(rename = "workerPoolId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. Immutable. The ID to use for the `WorkerPool`, which will become the final component of the resource name. This value should be 1-63 characters, and valid characters are /a-z-/."]
                            pub worker_pool_id: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod patch {
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
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A mask specifying which fields in `WorkerPool` should be updated."]
                            pub update_mask: ::std::option::Option<::std::string::String>,
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
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Files in the workspace to upload to Cloud Storage upon successful completion of all build steps."]
    pub struct ArtifactObjects {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cloud Storage bucket and optional object path, in the form \"gs://bucket/path/to/somewhere/\". (see [Bucket Name Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)). Files in the workspace matching any path pattern will be uploaded to Cloud Storage with this location as a prefix."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paths")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Path globs used to match files in the build's workspace."]
        pub paths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timing")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Stores timing information for pushing all artifact objects."]
        pub timing: ::std::option::Option<::std::boxed::Box<TimeSpan>>,
    }
    impl ArtifactObjects {
        pub fn builder() -> ArtifactObjectsBuilder {
            ArtifactObjectsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An artifact that was uploaded during a build. This is a single record in the artifact manifest JSON file."]
    pub struct ArtifactResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileHash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The file hash of the artifact."]
        pub file_hash: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FileHashes>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The path of an artifact in a Google Cloud Storage bucket, with the generation number. For example, `gs://mybucket/path/to/output.jar#generation`."]
        pub location: ::std::option::Option<::std::string::String>,
    }
    impl ArtifactResult {
        pub fn builder() -> ArtifactResultBuilder {
            ArtifactResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Artifacts produced by a build that should be uploaded upon successful completion of all build steps."]
    pub struct Artifacts {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "images")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of images to be pushed upon the successful completion of all build steps. The images will be pushed using the builder service account's credentials. The digests of the pushed images will be stored in the Build resource's results field. If any of the images fail to be pushed, the build is marked FAILURE."]
        pub images: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objects")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of objects to be uploaded to Cloud Storage upon successful completion of all build steps. Files in the workspace matching specified paths globs will be uploaded to the specified Cloud Storage location using the builder service account's credentials. The location and generation of the uploaded objects will be stored in the Build resource's results field. If any objects fail to be pushed, the build is marked FAILURE."]
        pub objects: ::std::option::Option<::std::boxed::Box<ArtifactObjects>>,
    }
    impl Artifacts {
        pub fn builder() -> ArtifactsBuilder {
            ArtifactsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A build resource in the Cloud Build API. At a high level, a `Build` describes where to find source code, how to build it (for example, the builder image to run on the source), and where to store the built artifacts. Fields can include the following variables, which will be expanded when the build is created: - $PROJECT_ID: the project ID of the build. - $PROJECT_NUMBER: the project number of the build. - $BUILD_ID: the autogenerated ID of the build. - $REPO_NAME: the source repository name specified by RepoSource. - $BRANCH_NAME: the branch name specified by RepoSource. - $TAG_NAME: the tag name specified by RepoSource. - $REVISION_ID or $COMMIT_SHA: the commit SHA specified by RepoSource or resolved from the specified branch or tag. - $SHORT_SHA: first 7 characters of $REVISION_ID or $COMMIT_SHA."]
    pub struct Build {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "artifacts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Artifacts produced by the build that should be uploaded upon successful completion of all build steps."]
        pub artifacts: ::std::option::Option<::std::boxed::Box<Artifacts>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "availableSecrets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Secrets and secret environment variables."]
        pub available_secrets: ::std::option::Option<::std::boxed::Box<Secrets>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buildTriggerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The ID of the `BuildTrigger` that triggered this build, if it was triggered automatically."]
        pub build_trigger_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time at which the request to create the build was received."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "finishTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time at which execution of the build was finished. The difference between finish_time and start_time is the duration of the build's execution."]
        pub finish_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Unique identifier of the build."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "images")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of images to be pushed upon the successful completion of all build steps. The images are pushed using the builder service account's credentials. The digests of the pushed images will be stored in the `Build` resource's results field. If any of the images fail to be pushed, the build status is marked `FAILURE`."]
        pub images: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. URL to logs for this build in Google Cloud Console."]
        pub log_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logsBucket")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage bucket where logs should be written (see [Bucket Name Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)). Logs file names will be of the format `${logs_bucket}/log-${build_id}.txt`."]
        pub logs_bucket: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The 'Build' name with format: `projects/{project}/locations/{location}/builds/{build}`, where {build} is a unique identifier generated by the service."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "options")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Special options for this build."]
        pub options: ::std::option::Option<::std::boxed::Box<BuildOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. ID of the project."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queueTtl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "TTL in queue for this build. If provided and the build is enqueued longer than this value, the build will expire and the build status will be `EXPIRED`. The TTL starts ticking from create_time."]
        pub queue_ttl: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Results of the build."]
        pub results: ::std::option::Option<::std::boxed::Box<Results>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secrets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Secrets to decrypt using Cloud Key Management Service. Note: Secret Manager is the recommended technique for managing sensitive data with Cloud Build. Use `available_secrets` to configure builds to access secrets from Secret Manager. For instructions, see: https://cloud.google.com/cloud-build/docs/securing-builds/use-secrets"]
        pub secrets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Secret>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceAccount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IAM service account whose credentials will be used at build runtime. Must be of the format `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`. ACCOUNT can be email address or uniqueId of the service account. This field is in beta."]
        pub service_account: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location of the source files to build."]
        pub source: ::std::option::Option<::std::boxed::Box<Source>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceProvenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A permanent fixed identifier for source."]
        pub source_provenance: ::std::option::Option<::std::boxed::Box<SourceProvenance>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time at which execution of the build was started."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Status of the build."]
        pub status: ::std::option::Option<BuildStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statusDetail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Customer-readable message about the current status."]
        pub status_detail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "steps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The operations to be performed on the workspace."]
        pub steps: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BuildStep>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "substitutions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Substitutions data for `Build` resource."]
        pub substitutions:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tags for annotation of a `Build`. These are not docker tags."]
        pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Amount of time that this build should be allowed to run, to second granularity. If this amount of time elapses, work on the build will cease and the build status will be `TIMEOUT`. `timeout` starts ticking from `startTime`. Default time is ten minutes."]
        pub timeout: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timing")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Stores timing information for phases of the build. Valid keys are: * BUILD: time to execute all build steps * PUSH: time to push all specified images. * FETCHSOURCE: time to fetch source. If the build does not specify source or images, these keys will not be included."]
        pub timing: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<TimeSpan>>,
        >,
    }
    impl Build {
        pub fn builder() -> BuildBuilder {
            BuildBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Status of the build."]
    pub enum BuildStatusEnum {
        #[serde(rename = "STATUS_UNKNOWN")]
        #[doc = "Status of the build is unknown."]
        StatusUnknown,
        #[serde(rename = "QUEUED")]
        #[doc = "Build or step is queued; work has not yet begun."]
        Queued,
        #[serde(rename = "WORKING")]
        #[doc = "Build or step is being executed."]
        Working,
        #[serde(rename = "SUCCESS")]
        #[doc = "Build or step finished successfully."]
        Success,
        #[serde(rename = "FAILURE")]
        #[doc = "Build or step failed to complete successfully."]
        Failure,
        #[serde(rename = "INTERNAL_ERROR")]
        #[doc = "Build or step failed due to an internal cause."]
        InternalError,
        #[serde(rename = "TIMEOUT")]
        #[doc = "Build or step took longer than was allowed."]
        Timeout,
        #[serde(rename = "CANCELLED")]
        #[doc = "Build or step was canceled by a user."]
        Cancelled,
        #[serde(rename = "EXPIRED")]
        #[doc = "Build was enqueued for longer than the value of `queue_ttl`."]
        Expired,
    }
    impl ::std::default::Default for BuildStatusEnum {
        fn default() -> Self {
            Self::StatusUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for build operations."]
    pub struct BuildOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "build")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The build that the operation is tracking."]
        pub _build: ::std::option::Option<::std::boxed::Box<Build>>,
    }
    impl BuildOperationMetadata {
        pub fn builder() -> BuildOperationMetadataBuilder {
            BuildOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Optional arguments to enable specific features of builds."]
    pub struct BuildOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskSizeGb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Requested disk size for the VM that runs the build. Note that this is *NOT* \"disk free\"; some of the space will be used by the operating system and build utilities. Also note that this is the minimum disk size that will be allocated for the build -- the build may run with a larger disk than requested. At present, the maximum disk size is 1000GB; builds that request more than the maximum are rejected with an error."]
        pub disk_size_gb: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dynamicSubstitutions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Option to specify whether or not to apply bash style string operations to the substitutions. NOTE: this is always enabled for triggered builds and cannot be overridden in the build configuration file."]
        pub dynamic_substitutions: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "env")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of global environment variable definitions that will exist for all build steps in this build. If a variable is defined in both globally and in a build step, the variable will use the build step value. The elements are of the form \"KEY=VALUE\" for the environment variable \"KEY\" being given the value \"VALUE\"."]
        pub env: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logStreamingOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Option to define build log streaming behavior to Google Cloud Storage."]
        pub log_streaming_option: ::std::option::Option<BuildOptionsLogStreamingOptionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logging")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Option to specify the logging mode, which determines if and where build logs are stored."]
        pub logging: ::std::option::Option<BuildOptionsLoggingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "machineType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Compute Engine machine type on which to run the build."]
        pub machine_type: ::std::option::Option<BuildOptionsMachineTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestedVerifyOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Requested verifiability options."]
        pub requested_verify_option: ::std::option::Option<BuildOptionsRequestedVerifyOptionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secretEnv")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of global environment variables, which are encrypted using a Cloud Key Management Service crypto key. These values must be specified in the build's `Secret`. These variables will be available to all build steps in this build."]
        pub secret_env: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceProvenanceHash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Requested hash for SourceProvenance."]
        pub source_provenance_hash:
            ::std::option::Option<::std::vec::Vec<BuildOptionsSourceProvenanceHashEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "substitutionOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Option to specify behavior when there is an error in the substitution checks. NOTE: this is always set to ALLOW_LOOSE for triggered builds and cannot be overridden in the build configuration file."]
        pub substitution_option: ::std::option::Option<BuildOptionsSubstitutionOptionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Global list of volumes to mount for ALL build steps Each volume is created as an empty volume prior to starting the build process. Upon completion of the build, volumes and their contents are discarded. Global volume names and paths cannot conflict with the volumes defined a build step. Using a global volume in a build with only one step is not valid as it is indicative of a build request with an incorrect configuration."]
        pub volumes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Volume>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerPool")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Option to specify a `WorkerPool` for the build. Format: projects/{project}/locations/{location}/workerPools/{workerPool} This field is in beta and is available only to restricted users."]
        pub worker_pool: ::std::option::Option<::std::string::String>,
    }
    impl BuildOptions {
        pub fn builder() -> BuildOptionsBuilder {
            BuildOptionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Option to define build log streaming behavior to Google Cloud Storage."]
    pub enum BuildOptionsLogStreamingOptionEnum {
        #[serde(rename = "STREAM_DEFAULT")]
        #[doc = "Service may automatically determine build log streaming behavior."]
        StreamDefault,
        #[serde(rename = "STREAM_ON")]
        #[doc = "Build logs should be streamed to Google Cloud Storage."]
        StreamOn,
        #[serde(rename = "STREAM_OFF")]
        #[doc = "Build logs should not be streamed to Google Cloud Storage; they will be written when the build is completed."]
        StreamOff,
    }
    impl ::std::default::Default for BuildOptionsLogStreamingOptionEnum {
        fn default() -> Self {
            Self::StreamDefault
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Option to specify the logging mode, which determines if and where build logs are stored."]
    pub enum BuildOptionsLoggingEnum {
        #[serde(rename = "LOGGING_UNSPECIFIED")]
        #[doc = "The service determines the logging mode. The default is `LEGACY`. Do not rely on the default logging behavior as it may change in the future."]
        LoggingUnspecified,
        #[serde(rename = "LEGACY")]
        #[doc = "Cloud Logging and Cloud Storage logging are enabled."]
        Legacy,
        #[serde(rename = "GCS_ONLY")]
        #[doc = "Only Cloud Storage logging is enabled."]
        GcsOnly,
        #[serde(rename = "STACKDRIVER_ONLY")]
        #[doc = "This option is the same as CLOUD_LOGGING_ONLY."]
        StackdriverOnly,
        #[serde(rename = "CLOUD_LOGGING_ONLY")]
        #[doc = "Only Cloud Logging is enabled. Note that logs for both the Cloud Console UI and Cloud SDK are based on Cloud Storage logs, so neither will provide logs if this option is chosen."]
        CloudLoggingOnly,
        #[serde(rename = "NONE")]
        #[doc = "Turn off all logging. No build logs will be captured."]
        None,
    }
    impl ::std::default::Default for BuildOptionsLoggingEnum {
        fn default() -> Self {
            Self::LoggingUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Compute Engine machine type on which to run the build."]
    pub enum BuildOptionsMachineTypeEnum {
        #[serde(rename = "UNSPECIFIED")]
        #[doc = "Standard machine type."]
        Unspecified,
        #[serde(rename = "N1_HIGHCPU_8")]
        #[doc = "Highcpu machine with 8 CPUs."]
        N1Highcpu8,
        #[serde(rename = "N1_HIGHCPU_32")]
        #[doc = "Highcpu machine with 32 CPUs."]
        N1Highcpu32,
        #[serde(rename = "E2_HIGHCPU_8")]
        #[doc = "Highcpu e2 machine with 8 CPUs."]
        E2Highcpu8,
        #[serde(rename = "E2_HIGHCPU_32")]
        #[doc = "Highcpu e2 machine with 32 CPUs."]
        E2Highcpu32,
    }
    impl ::std::default::Default for BuildOptionsMachineTypeEnum {
        fn default() -> Self {
            Self::Unspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Requested verifiability options."]
    pub enum BuildOptionsRequestedVerifyOptionEnum {
        #[serde(rename = "NOT_VERIFIED")]
        #[doc = "Not a verifiable build. (default)"]
        NotVerified,
        #[serde(rename = "VERIFIED")]
        #[doc = "Verified build."]
        Verified,
    }
    impl ::std::default::Default for BuildOptionsRequestedVerifyOptionEnum {
        fn default() -> Self {
            Self::NotVerified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum BuildOptionsSourceProvenanceHashEnum {
        #[serde(rename = "NONE")]
        #[doc = "No hash requested."]
        None,
        #[serde(rename = "SHA256")]
        #[doc = "Use a sha256 hash."]
        Sha256,
        #[serde(rename = "MD5")]
        #[doc = "Use a md5 hash."]
        Md5,
    }
    impl ::std::default::Default for BuildOptionsSourceProvenanceHashEnum {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Option to specify behavior when there is an error in the substitution checks. NOTE: this is always set to ALLOW_LOOSE for triggered builds and cannot be overridden in the build configuration file."]
    pub enum BuildOptionsSubstitutionOptionEnum {
        #[serde(rename = "MUST_MATCH")]
        #[doc = "Fails the build if error in substitutions checks, like missing a substitution in the template or in the map."]
        MustMatch,
        #[serde(rename = "ALLOW_LOOSE")]
        #[doc = "Do not fail the build if error in substitutions checks."]
        AllowLoose,
    }
    impl ::std::default::Default for BuildOptionsSubstitutionOptionEnum {
        fn default() -> Self {
            Self::MustMatch
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A step in the build pipeline."]
    pub struct BuildStep {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "args")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of arguments that will be presented to the step when it is started. If the image used to run the step's container has an entrypoint, the `args` are used as arguments to that entrypoint. If the image does not define an entrypoint, the first element in args is used as the entrypoint, and the remainder will be used as arguments."]
        pub args: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dir")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Working directory to use when running this step's container. If this value is a relative path, it is relative to the build's working directory. If this value is absolute, it may be outside the build's working directory, in which case the contents of the path may not be persisted across build step executions, unless a `volume` for that path is specified. If the build specifies a `RepoSource` with `dir` and a step with a `dir`, which specifies an absolute path, the `RepoSource` `dir` is ignored for the step's execution."]
        pub dir: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entrypoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entrypoint to be used instead of the build step image's default entrypoint. If unset, the image's default entrypoint is used."]
        pub entrypoint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "env")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of environment variable definitions to be used when running a step. The elements are of the form \"KEY=VALUE\" for the environment variable \"KEY\" being given the value \"VALUE\"."]
        pub env: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier for this build step, used in `wait_for` to reference this build step as a dependency."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the container image that will run this particular build step. If the image is available in the host's Docker daemon's cache, it will be run directly. If not, the host will attempt to pull the image first, using the builder service account's credentials if necessary. The Docker daemon's cache will already have the latest versions of all of the officially supported build steps ([https://github.com/GoogleCloudPlatform/cloud-builders](https://github.com/GoogleCloudPlatform/cloud-builders)). The Docker daemon will also have cached many of the layers for some popular images, like \"ubuntu\", \"debian\", but they will be refreshed at the time you attempt to use them. If you built an image in a previous build step, it will be stored in the host's Docker daemon's cache and is available to use as the name for a later build step."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pullTiming")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Stores timing information for pulling this build step's builder image only."]
        pub pull_timing: ::std::option::Option<::std::boxed::Box<TimeSpan>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secretEnv")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of environment variables which are encrypted using a Cloud Key Management Service crypto key. These values must be specified in the build's `Secret`."]
        pub secret_env: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Status of the build step. At this time, build step status is only updated on build completion; step status is not updated in real-time as the build progresses."]
        pub status: ::std::option::Option<BuildStepStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time limit for executing this build step. If not defined, the step has no time limit and will be allowed to continue to run until either it completes or the build itself times out."]
        pub timeout: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timing")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Stores timing information for executing this build step."]
        pub timing: ::std::option::Option<::std::boxed::Box<TimeSpan>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of volumes to mount into the build step. Each volume is created as an empty volume prior to execution of the build step. Upon completion of the build, volumes and their contents are discarded. Using a named volume in only one step is not valid as it is indicative of a build request with an incorrect configuration."]
        pub volumes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Volume>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "waitFor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID(s) of the step(s) that this build step depends on. This build step will not start until all the build steps in `wait_for` have completed successfully. If `wait_for` is empty, this build step will start when all previous build steps in the `Build.Steps` list have completed successfully."]
        pub wait_for: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl BuildStep {
        pub fn builder() -> BuildStepBuilder {
            BuildStepBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Status of the build step. At this time, build step status is only updated on build completion; step status is not updated in real-time as the build progresses."]
    pub enum BuildStepStatusEnum {
        #[serde(rename = "STATUS_UNKNOWN")]
        #[doc = "Status of the build is unknown."]
        StatusUnknown,
        #[serde(rename = "QUEUED")]
        #[doc = "Build or step is queued; work has not yet begun."]
        Queued,
        #[serde(rename = "WORKING")]
        #[doc = "Build or step is being executed."]
        Working,
        #[serde(rename = "SUCCESS")]
        #[doc = "Build or step finished successfully."]
        Success,
        #[serde(rename = "FAILURE")]
        #[doc = "Build or step failed to complete successfully."]
        Failure,
        #[serde(rename = "INTERNAL_ERROR")]
        #[doc = "Build or step failed due to an internal cause."]
        InternalError,
        #[serde(rename = "TIMEOUT")]
        #[doc = "Build or step took longer than was allowed."]
        Timeout,
        #[serde(rename = "CANCELLED")]
        #[doc = "Build or step was canceled by a user."]
        Cancelled,
        #[serde(rename = "EXPIRED")]
        #[doc = "Build was enqueued for longer than the value of `queue_ttl`."]
        Expired,
    }
    impl ::std::default::Default for BuildStepStatusEnum {
        fn default() -> Self {
            Self::StatusUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An image built by the pipeline."]
    pub struct BuiltImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "digest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Docker Registry 2.0 digest."]
        pub digest: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name used to push the container image to Google Container Registry, as presented to `docker push`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pushTiming")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Stores timing information for pushing the specified image."]
        pub push_timing: ::std::option::Option<::std::boxed::Box<TimeSpan>>,
    }
    impl BuiltImage {
        pub fn builder() -> BuiltImageBuilder {
            BuiltImageBuilder::default()
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
    #[doc = "Container message for hashes of byte content of files, used in SourceProvenance messages to verify integrity of source input to the build."]
    pub struct FileHashes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileHash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Collection of file hashes."]
        pub file_hash: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Hash>>>,
    }
    impl FileHashes {
        pub fn builder() -> FileHashesBuilder {
            FileHashesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "HTTPDelivery is the delivery configuration for an HTTP notification."]
    pub struct HttpDelivery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI to which JSON-containing HTTP POST requests should be sent."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl HttpDelivery {
        pub fn builder() -> HttpDeliveryBuilder {
            HttpDeliveryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Container message for hash values."]
    pub struct Hash {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of hash that was performed."]
        pub _type: ::std::option::Option<HashTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hash value."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl Hash {
        pub fn builder() -> HashBuilder {
            HashBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of hash that was performed."]
    pub enum HashTypeEnum {
        #[serde(rename = "NONE")]
        #[doc = "No hash requested."]
        None,
        #[serde(rename = "SHA256")]
        #[doc = "Use a sha256 hash."]
        Sha256,
        #[serde(rename = "MD5")]
        #[doc = "Use a md5 hash."]
        Md5,
    }
    impl ::std::default::Default for HashTypeEnum {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Pairs a set of secret environment variables mapped to encrypted values with the Cloud KMS key to use to decrypt the value."]
    pub struct InlineSecret {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "envMap")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map of environment variable name to its encrypted value. Secret environment variables must be unique across all of a build's secrets, and must be used by at least one build step. Values can be at most 64 KB in size. There can be at most 100 secret values across all of a build's secrets."]
        pub env_map:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmsKeyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of Cloud KMS crypto key to decrypt the encrypted value. In format: projects/*/locations/*/keyRings/*/cryptoKeys/*"]
        pub kms_key_name: ::std::option::Option<::std::string::String>,
    }
    impl InlineSecret {
        pub fn builder() -> InlineSecretBuilder {
            InlineSecretBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response containing existing `WorkerPools`."]
    pub struct ListWorkerPoolsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerPools")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "`WorkerPools` for the specified project."]
        pub worker_pools: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WorkerPool>>>,
    }
    impl ListWorkerPoolsResponse {
        pub fn builder() -> ListWorkerPoolsResponseBuilder {
            ListWorkerPoolsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Network describes the network configuration for a `WorkerPool`."]
    pub struct NetworkConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "peeredNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Immutable. The network definition that the workers are peered to. If this section is left empty, the workers will be peered to WorkerPool.project_id on the default network. Must be in the format `projects/{project}/global/networks/{network}`, where {project} is a project number, such as `12345`, and {network} is the name of a VPC network in the project."]
        pub peered_network: ::std::option::Option<::std::string::String>,
    }
    impl NetworkConfig {
        pub fn builder() -> NetworkConfigBuilder {
            NetworkConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Notification is the container which holds the data that is relevant to this particular notification."]
    pub struct Notification {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filter string to use for notification filtering. Currently, this is assumed to be a CEL program. See https://opensource.google/projects/cel for more."]
        pub filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpDelivery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for HTTP delivery."]
        pub http_delivery: ::std::option::Option<::std::boxed::Box<HttpDelivery>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slackDelivery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for Slack delivery."]
        pub slack_delivery: ::std::option::Option<::std::boxed::Box<SlackDelivery>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "smtpDelivery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for SMTP (email) delivery."]
        pub smtp_delivery: ::std::option::Option<::std::boxed::Box<SmtpDelivery>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "structDelivery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Escape hatch for users to supply custom delivery configs."]
        pub struct_delivery:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl Notification {
        pub fn builder() -> NotificationBuilder {
            NotificationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "NotifierConfig is the top-level configuration message."]
    pub struct NotifierConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The API version of this configuration format."]
        pub api_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of notifier to use (e.g. SMTPNotifier)."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata for referring to/handling/deploying this notifier."]
        pub metadata: ::std::option::Option<::std::boxed::Box<NotifierMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actual configuration for this notifier."]
        pub spec: ::std::option::Option<::std::boxed::Box<NotifierSpec>>,
    }
    impl NotifierConfig {
        pub fn builder() -> NotifierConfigBuilder {
            NotifierConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "NotifierMetadata contains the data which can be used to reference or describe this notifier."]
    pub struct NotifierMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The human-readable and user-given name for the notifier. For example: \"repo-merge-email-notifier\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The string representing the name and version of notifier to deploy. Expected to be of the form of \"/:\". For example: \"gcr.io/my-project/notifiers/smtp:1.2.34\"."]
        pub notifier: ::std::option::Option<::std::string::String>,
    }
    impl NotifierMetadata {
        pub fn builder() -> NotifierMetadataBuilder {
            NotifierMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "NotifierSecret is the container that maps a secret name (reference) to its Google Cloud Secret Manager resource path."]
    pub struct NotifierSecret {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name is the local name of the secret, such as the verbatim string \"my-smtp-password\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value is interpreted to be a resource path for fetching the actual (versioned) secret data for this secret. For example, this would be a Google Cloud Secret Manager secret version resource path like: \"projects/my-project/secrets/my-secret/versions/latest\"."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl NotifierSecret {
        pub fn builder() -> NotifierSecretBuilder {
            NotifierSecretBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "NotifierSecretRef contains the reference to a secret stored in the corresponding NotifierSpec."]
    pub struct NotifierSecretRef {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secretRef")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of `secret_ref` should be a `name` that is registered in a `Secret` in the `secrets` list of the `Spec`."]
        pub secret_ref: ::std::option::Option<::std::string::String>,
    }
    impl NotifierSecretRef {
        pub fn builder() -> NotifierSecretRefBuilder {
            NotifierSecretRefBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "NotifierSpec is the configuration container for notifications."]
    pub struct NotifierSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notification")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The configuration of this particular notifier."]
        pub notification: ::std::option::Option<::std::boxed::Box<Notification>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secrets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configurations for secret resources used by this particular notifier."]
        pub secrets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NotifierSecret>>>,
    }
    impl NotifierSpec {
        pub fn builder() -> NotifierSpecBuilder {
            NotifierSpecBuilder::default()
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
        #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "response")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
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
    #[doc = "Location of the source in a Google Cloud Source Repository."]
    pub struct RepoSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "branchName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Regex matching branches to build. The syntax of the regular expressions accepted is the syntax accepted by RE2 and described at https://github.com/google/re2/wiki/Syntax"]
        pub branch_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commitSha")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Explicit commit SHA to build."]
        pub commit_sha: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dir")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Directory, relative to the source root, in which to run the build. This must be a relative path. If a step's `dir` is specified and is an absolute path, this value is ignored for that step's execution."]
        pub dir: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invertRegex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only trigger a build if the revision regex does NOT match the revision regex."]
        pub invert_regex: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the project that owns the Cloud Source Repository. If omitted, the project ID requesting the build is assumed."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "repoName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the Cloud Source Repository."]
        pub repo_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "substitutions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Substitutions to use in a triggered build. Should only be used with RunBuildTrigger"]
        pub substitutions:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Regex matching tags to build. The syntax of the regular expressions accepted is the syntax accepted by RE2 and described at https://github.com/google/re2/wiki/Syntax"]
        pub tag_name: ::std::option::Option<::std::string::String>,
    }
    impl RepoSource {
        pub fn builder() -> RepoSourceBuilder {
            RepoSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Artifacts created by the build pipeline."]
    pub struct Results {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "artifactManifest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Path to the artifact manifest. Only populated when artifacts are uploaded."]
        pub artifact_manifest: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "artifactTiming")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time to push all non-container artifacts."]
        pub artifact_timing: ::std::option::Option<::std::boxed::Box<TimeSpan>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buildStepImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of build step digests, in the order corresponding to build step indices."]
        pub build_step_images: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buildStepOutputs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of build step outputs, produced by builder images, in the order corresponding to build step indices. [Cloud Builders](https://cloud.google.com/cloud-build/docs/cloud-builders) can produce this output by writing to `$BUILDER_OUTPUT/output`. Only the first 4KB of data is stored."]
        pub build_step_outputs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "images")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Container images that were built as a part of the build."]
        pub images: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BuiltImage>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numArtifacts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of artifacts uploaded. Only populated when artifacts are uploaded."]
        pub num_artifacts: ::std::option::Option<::std::string::String>,
    }
    impl Results {
        pub fn builder() -> ResultsBuilder {
            ResultsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SMTPDelivery is the delivery configuration for an SMTP (email) notification."]
    pub struct SmtpDelivery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fromAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is the SMTP account/email that appears in the `From:` of the email. If empty, it is assumed to be sender."]
        pub from_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "password")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The SMTP sender's password."]
        pub password: ::std::option::Option<::std::boxed::Box<NotifierSecretRef>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "port")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The SMTP port of the server."]
        pub port: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recipientAddresses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is the list of addresses to which we send the email (i.e. in the `To:` of the email)."]
        pub recipient_addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "senderAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is the SMTP account/email that is used to send the message."]
        pub sender_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "server")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The address of the SMTP server."]
        pub server: ::std::option::Option<::std::string::String>,
    }
    impl SmtpDelivery {
        pub fn builder() -> SmtpDeliveryBuilder {
            SmtpDeliveryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Pairs a set of secret environment variables containing encrypted values with the Cloud KMS key to use to decrypt the value. Note: Use `kmsKeyName` with `available_secrets` instead of using `kmsKeyName` with `secret`. For instructions see: https://cloud.google.com/cloud-build/docs/securing-builds/use-encrypted-credentials."]
    pub struct Secret {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmsKeyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cloud KMS key name to use to decrypt these envs."]
        pub kms_key_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secretEnv")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map of environment variable name to its encrypted value. Secret environment variables must be unique across all of a build's secrets, and must be used by at least one build step. Values can be at most 64 KB in size. There can be at most 100 secret values across all of a build's secrets."]
        pub secret_env:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl Secret {
        pub fn builder() -> SecretBuilder {
            SecretBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Pairs a secret environment variable with a SecretVersion in Secret Manager."]
    pub struct SecretManagerSecret {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "env")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Environment variable name to associate with the secret. Secret environment variables must be unique across all of a build's secrets, and must be used by at least one build step."]
        pub env: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of the SecretVersion. In format: projects/*/secrets/*/versions/*"]
        pub version_name: ::std::option::Option<::std::string::String>,
    }
    impl SecretManagerSecret {
        pub fn builder() -> SecretManagerSecretBuilder {
            SecretManagerSecretBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Secrets and secret environment variables."]
    pub struct Secrets {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Secrets encrypted with KMS key and the associated secret environment variable."]
        pub inline: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InlineSecret>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secretManager")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Secrets in Secret Manager and associated secret environment variable."]
        pub secret_manager:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SecretManagerSecret>>>,
    }
    impl Secrets {
        pub fn builder() -> SecretsBuilder {
            SecretsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SlackDelivery is the delivery configuration for delivering Slack messages via webhooks. See Slack webhook documentation at: https://api.slack.com/messaging/webhooks."]
    pub struct SlackDelivery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webhookUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The secret reference for the Slack webhook URI for sending messages to a channel."]
        pub webhook_uri: ::std::option::Option<::std::boxed::Box<NotifierSecretRef>>,
    }
    impl SlackDelivery {
        pub fn builder() -> SlackDeliveryBuilder {
            SlackDeliveryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Location of the source in a supported storage service."]
    pub struct Source {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "repoSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If provided, get the source from this location in a Cloud Source Repository."]
        pub repo_source: ::std::option::Option<::std::boxed::Box<RepoSource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storageSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If provided, get the source from this location in Google Cloud Storage."]
        pub storage_source: ::std::option::Option<::std::boxed::Box<StorageSource>>,
    }
    impl Source {
        pub fn builder() -> SourceBuilder {
            SourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provenance of the source. Ways to find the original source, or verify that some source was used for this build."]
    pub struct SourceProvenance {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileHashes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Hash(es) of the build source, which can be used to verify that the original source integrity was maintained in the build. Note that `FileHashes` will only be populated if `BuildOptions` has requested a `SourceProvenanceHash`. The keys to this map are file paths used as build source and the values contain the hash values for those files. If the build source came in a single package such as a gzipped tarfile (`.tar.gz`), the `FileHash` will be for the single path to that file."]
        pub file_hashes: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<FileHashes>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resolvedRepoSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A copy of the build's `source.repo_source`, if exists, with any revisions resolved."]
        pub resolved_repo_source: ::std::option::Option<::std::boxed::Box<RepoSource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resolvedStorageSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A copy of the build's `source.storage_source`, if exists, with any generations resolved."]
        pub resolved_storage_source: ::std::option::Option<::std::boxed::Box<StorageSource>>,
    }
    impl SourceProvenance {
        pub fn builder() -> SourceProvenanceBuilder {
            SourceProvenanceBuilder::default()
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
    #[doc = "Location of the source in an archive file in Google Cloud Storage."]
    pub struct StorageSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucket")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage bucket containing the source (see [Bucket Name Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements))."]
        pub bucket: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "generation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage generation for the object. If the generation is omitted, the latest generation will be used."]
        pub generation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "object")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage object containing the source. This object must be a gzipped archive file (`.tar.gz`) containing source to build."]
        pub object: ::std::option::Option<::std::string::String>,
    }
    impl StorageSource {
        pub fn builder() -> StorageSourceBuilder {
            StorageSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Start and end times for a build execution phase."]
    pub struct TimeSpan {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End of time span."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start of time span."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl TimeSpan {
        pub fn builder() -> TimeSpanBuilder {
            TimeSpanBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Volume describes a Docker container volume which is mounted into build steps in order to persist files across build step execution."]
    pub struct Volume {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the volume to mount. Volume names must be unique per build step and must be valid names for Docker volumes. Each named volume must be used by at least two build steps."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Path at which to mount the volume. Paths must be absolute and cannot conflict with other volume paths on the same build step or with certain reserved volume paths."]
        pub path: ::std::option::Option<::std::string::String>,
    }
    impl Volume {
        pub fn builder() -> VolumeBuilder {
            VolumeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "WorkerConfig defines the configuration to be used for a creating workers in the pool."]
    pub struct WorkerConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskSizeGb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Size of the disk attached to the worker, in GB. See https://cloud.google.com/compute/docs/disks/ If `0` is specified, Cloud Build will use a standard disk size."]
        pub disk_size_gb: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "machineType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Machine Type of the worker, such as n1-standard-1. See https://cloud.google.com/compute/docs/machine-types. If left blank, Cloud Build will use a standard unspecified machine to create the worker pool."]
        pub machine_type: ::std::option::Option<::std::string::String>,
    }
    impl WorkerConfig {
        pub fn builder() -> WorkerConfigBuilder {
            WorkerConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for a WorkerPool to run the builds. Workers are machines that Cloud Build uses to run your builds. By default, all workers run in a project owned by Cloud Build. To have full control over the workers that execute your builds -- such as enabling them to access private resources on your private network -- you can request Cloud Build to run the workers in your own project by creating a custom workers pool."]
    pub struct WorkerPool {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time at which the request to create the `WorkerPool` was received."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time at which the request to delete the `WorkerPool` was received."]
        pub delete_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The resource name of the `WorkerPool`. Format of the name is `projects/{project_id}/workerPools/{worker_pool_id}`, where the value of {worker_pool_id} is provided in the CreateWorkerPool request."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Network configuration for the `WorkerPool`."]
        pub network_config: ::std::option::Option<::std::boxed::Box<NetworkConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Immutable. The region where the `WorkerPool` runs. Only \"us-central1\" is currently supported. Note that `region` cannot be changed once the `WorkerPool` is created."]
        pub region: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. WorkerPool state."]
        pub state: ::std::option::Option<WorkerPoolStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time at which the request to update the `WorkerPool` was received."]
        pub update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workerConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Worker configuration for the `WorkerPool`."]
        pub worker_config: ::std::option::Option<::std::boxed::Box<WorkerConfig>>,
    }
    impl WorkerPool {
        pub fn builder() -> WorkerPoolBuilder {
            WorkerPoolBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. WorkerPool state."]
    pub enum WorkerPoolStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "State of the `WorkerPool` is unknown."]
        StateUnspecified,
        #[serde(rename = "CREATING")]
        #[doc = "`WorkerPool` is being created."]
        Creating,
        #[serde(rename = "RUNNING")]
        #[doc = "`WorkerPool` is running."]
        Running,
        #[serde(rename = "DELETING")]
        #[doc = "`WorkerPool` is being deleted: cancelling builds and draining workers."]
        Deleting,
        #[serde(rename = "DELETED")]
        #[doc = "`WorkerPool` is deleted."]
        Deleted,
    }
    impl ::std::default::Default for WorkerPoolStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
}
