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
            pub mod profiles {
                pub mod methods {
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
                            #[doc = "Field mask used to specify the fields to be overwritten. Currently only profile_bytes and labels fields are supported by UpdateProfile, so only those fields can be specified in the mask. When no mask is provided, all fields are overwritten."]
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
    #[doc = "CreateProfileRequest describes a profile resource online creation request. The deployment field must be populated. The profile_type specifies the list of profile types supported by the agent. The creation call will hang until a profile of one of these types needs to be collected."]
    pub struct CreateProfileRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deployment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deployment details."]
        pub deployment: ::std::option::Option<::std::boxed::Box<Deployment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "One or more profile types that the agent is capable of providing."]
        pub profile_type:
            ::std::option::Option<::std::vec::Vec<CreateProfileRequestProfileTypeEnum>>,
    }
    impl CreateProfileRequest {
        pub fn builder() -> CreateProfileRequestBuilder {
            CreateProfileRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum CreateProfileRequestProfileTypeEnum {
        #[serde(rename = "PROFILE_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified profile type."]
        ProfileTypeUnspecified,
        #[serde(rename = "CPU")]
        #[doc = "Thread CPU time sampling."]
        Cpu,
        #[serde(rename = "WALL")]
        #[doc = "Wallclock time sampling. More expensive as stops all threads."]
        Wall,
        #[serde(rename = "HEAP")]
        #[doc = "In-use heap profile. Represents a snapshot of the allocations that are live at the time of the profiling."]
        Heap,
        #[serde(rename = "THREADS")]
        #[doc = "Single-shot collection of all thread stacks."]
        Threads,
        #[serde(rename = "CONTENTION")]
        #[doc = "Synchronization contention profile."]
        Contention,
        #[serde(rename = "PEAK_HEAP")]
        #[doc = "Peak heap profile."]
        PeakHeap,
        #[serde(rename = "HEAP_ALLOC")]
        #[doc = "Heap allocation profile. It represents the aggregation of all allocations made over the duration of the profile. All allocations are included, including those that might have been freed by the end of the profiling interval. The profile is in particular useful for garbage collecting languages to understand which parts of the code create most of the garbage collection pressure to see if those can be optimized."]
        HeapAlloc,
    }
    impl ::std::default::Default for CreateProfileRequestProfileTypeEnum {
        fn default() -> Self {
            Self::ProfileTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deployment contains the deployment identification information."]
    pub struct Deployment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Labels identify the deployment within the user universe and same target. Validation regex for label names: `^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?$`. Value for an individual label must be <= 512 bytes, the total size of all label names and values must be <= 1024 bytes. Label named \"language\" can be used to record the programming language of the profiled deployment. The standard choices for the value include \"java\", \"go\", \"python\", \"ruby\", \"nodejs\", \"php\", \"dotnet\". For deployments running on Google Cloud Platform, \"zone\" or \"region\" label should be present describing the deployment location. An example of a zone is \"us-central1-a\", an example of a region is \"us-central1\" or \"us-central\"."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Project ID is the ID of a cloud project. Validation regex: `^a-z{4,61}[a-z0-9]$`."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target is the service name used to group related deployments: * Service name for GAE Flex / Standard. * Cluster and container name for GKE. * User-specified string for direct GCE profiling (e.g. Java). * Job name for Dataflow. Validation regex: `^[a-z]([-a-z0-9_.]{0,253}[a-z0-9])?$`."]
        pub target: ::std::option::Option<::std::string::String>,
    }
    impl Deployment {
        pub fn builder() -> DeploymentBuilder {
            DeploymentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Profile resource."]
    pub struct Profile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deployment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deployment this profile corresponds to."]
        pub deployment: ::std::option::Option<::std::boxed::Box<Deployment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Duration of the profiling session. Input (for the offline mode) or output (for the online mode). The field represents requested profiling duration. It may slightly differ from the effective profiling duration, which is recorded in the profile data, in case the profiling can't be stopped immediately (e.g. in case stopping the profiling is handled asynchronously)."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input only. Labels associated to this specific profile. These labels will get merged with the deployment labels for the final data set. See documentation on deployment labels for validation rules and limits."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Opaque, server-assigned, unique ID for this profile."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input only. Profile bytes, as a gzip compressed serialized proto, the format is https://github.com/google/pprof/blob/master/proto/profile.proto."]
        pub profile_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of profile. For offline mode, this must be specified when creating the profile. For online mode it is assigned and returned by the server."]
        pub profile_type: ::std::option::Option<ProfileProfileTypeEnum>,
    }
    impl Profile {
        pub fn builder() -> ProfileBuilder {
            ProfileBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of profile. For offline mode, this must be specified when creating the profile. For online mode it is assigned and returned by the server."]
    pub enum ProfileProfileTypeEnum {
        #[serde(rename = "PROFILE_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified profile type."]
        ProfileTypeUnspecified,
        #[serde(rename = "CPU")]
        #[doc = "Thread CPU time sampling."]
        Cpu,
        #[serde(rename = "WALL")]
        #[doc = "Wallclock time sampling. More expensive as stops all threads."]
        Wall,
        #[serde(rename = "HEAP")]
        #[doc = "In-use heap profile. Represents a snapshot of the allocations that are live at the time of the profiling."]
        Heap,
        #[serde(rename = "THREADS")]
        #[doc = "Single-shot collection of all thread stacks."]
        Threads,
        #[serde(rename = "CONTENTION")]
        #[doc = "Synchronization contention profile."]
        Contention,
        #[serde(rename = "PEAK_HEAP")]
        #[doc = "Peak heap profile."]
        PeakHeap,
        #[serde(rename = "HEAP_ALLOC")]
        #[doc = "Heap allocation profile. It represents the aggregation of all allocations made over the duration of the profile. All allocations are included, including those that might have been freed by the end of the profiling interval. The profile is in particular useful for garbage collecting languages to understand which parts of the code create most of the garbage collection pressure to see if those can be optimized."]
        HeapAlloc,
    }
    impl ::std::default::Default for ProfileProfileTypeEnum {
        fn default() -> Self {
            Self::ProfileTypeUnspecified
        }
    }
}
