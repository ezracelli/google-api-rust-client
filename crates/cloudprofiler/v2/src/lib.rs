#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "CreateProfileRequest describes a profile resource online creation request. The deployment field must be populated. The profile_type specifies the list of profile types supported by the agent. The creation call will hang until a profile of one of these types needs to be collected."]
pub struct CreateProfileRequest {
    #[serde(rename = "deployment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deployment details."]
    pub deployment: ::std::option::Option<::std::boxed::Box<Deployment>>,
    #[serde(rename = "profileType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "One or more profile types that the agent is capable of providing."]
    pub profile_type: ::std::option::Option<::std::vec::Vec<CreateProfileRequestProfileTypeEnum>>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deployment contains the deployment identification information."]
pub struct Deployment {
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Labels identify the deployment within the user universe and same target. Validation regex for label names: `^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?$`. Value for an individual label must be <= 512 bytes, the total size of all label names and values must be <= 1024 bytes. Label named \"language\" can be used to record the programming language of the profiled deployment. The standard choices for the value include \"java\", \"go\", \"python\", \"ruby\", \"nodejs\", \"php\", \"dotnet\". For deployments running on Google Cloud Platform, \"zone\" or \"region\" label should be present describing the deployment location. An example of a zone is \"us-central1-a\", an example of a region is \"us-central1\" or \"us-central\"."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Project ID is the ID of a cloud project. Validation regex: `^a-z{4,61}[a-z0-9]$`."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target is the service name used to group related deployments: * Service name for GAE Flex / Standard. * Cluster and container name for GKE. * User-specified string for direct GCE profiling (e.g. Java). * Job name for Dataflow. Validation regex: `^[a-z]([-a-z0-9_.]{0,253}[a-z0-9])?$`."]
    pub target: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Profile resource."]
pub struct Profile {
    #[serde(rename = "deployment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deployment this profile corresponds to."]
    pub deployment: ::std::option::Option<::std::boxed::Box<Deployment>>,
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Duration of the profiling session. Input (for the offline mode) or output (for the online mode). The field represents requested profiling duration. It may slightly differ from the effective profiling duration, which is recorded in the profile data, in case the profiling can't be stopped immediately (e.g. in case stopping the profiling is handled asynchronously)."]
    pub duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input only. Labels associated to this specific profile. These labels will get merged with the deployment labels for the final data set. See documentation on deployment labels for validation rules and limits."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Opaque, server-assigned, unique ID for this profile."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "profileBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input only. Profile bytes, as a gzip compressed serialized proto, the format is https://github.com/google/pprof/blob/master/proto/profile.proto."]
    pub profile_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "profileType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of profile. For offline mode, this must be specified when creating the profile. For online mode it is assigned and returned by the server."]
    pub profile_type: ::std::option::Option<ProfileProfileTypeEnum>,
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
