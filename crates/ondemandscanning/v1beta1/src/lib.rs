#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An alias to a repo revision."]
pub struct AliasContext {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The alias kind."]
    pub kind: ::std::option::Option<AliasContextKindEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The alias name."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The alias kind."]
pub enum AliasContextKindEnum {
    #[serde(rename = "KIND_UNSPECIFIED")]
    #[doc = "Unknown."]
    KindUnspecified,
    #[serde(rename = "FIXED")]
    #[doc = "Git tag."]
    Fixed,
    #[serde(rename = "MOVABLE")]
    #[doc = "Git branch."]
    Movable,
    #[serde(rename = "OTHER")]
    #[doc = "Used to specify non-standard aliases. For example, if a Git repo has a ref named \"refs/foo/bar\"."]
    Other,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "AnalyzePackagesMetadata contains metadata for an active scan of a container image."]
pub struct AnalyzePackagesMetadata {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When the scan was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource URI of the container image being scanned."]
    pub resource_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "AnalyzePackagesRequest is the request to analyze a list of packages and create Vulnerability Occurrences for it."]
pub struct AnalyzePackagesRequest {
    #[serde(rename = "packages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The packages to analyze."]
    pub packages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PackageData>>>,
    #[serde(rename = "resourceUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The resource URI of the container image being scanned."]
    pub resource_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "AnalyzePackagesResponse contains the information necessary to find results for the given scan."]
pub struct AnalyzePackagesResponse {
    #[serde(rename = "scan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the scan resource created by this successful scan."]
    pub scan: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Artifact describes a build product."]
pub struct Artifact {
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hash or checksum value of a binary, or Docker Registry 2.0 digest of a container."]
    pub checksum: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Artifact ID, if any; for container images, this will be a URL by digest like `gcr.io/projectID/imagename@sha256:123456`."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Related artifact names. This may be the path to a binary or jar file, or in the case of a container build, the name used to push the container image to Google Container Registry, as presented to `docker push`. Note that a single Artifact ID can have multiple names, for example if two tags are applied to one image."]
    pub names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Occurrence that represents a single \"attestation\". The authenticity of an attestation can be verified using the attached signature. If the verifier trusts the public key of the signer, then verifying the signature is sufficient to establish trust. In this circumstance, the authority to which this attestation is attached is primarily useful for lookup (how to find this attestation if you already know the authority and artifact to be verified) and intent (for which authority this attestation was intended to sign."]
pub struct AttestationOccurrence {
    #[serde(rename = "jwts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "One or more JWTs encoding a self-contained attestation. Each JWT encodes the payload that it verifies within the JWT itself. Verifier implementation SHOULD ignore the `serialized_payload` field when verifying these JWTs. If only JWTs are present on this AttestationOccurrence, then the `serialized_payload` SHOULD be left empty. Each JWT SHOULD encode a claim specific to the `resource_uri` of this Occurrence, but this is not validated by Grafeas metadata API implementations. The JWT itself is opaque to Grafeas."]
    pub jwts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Jwt>>>,
    #[serde(rename = "serializedPayload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The serialized payload that is verified by one or more `signatures`."]
    pub serialized_payload: ::std::option::Option<::std::string::String>,
    #[serde(rename = "signatures")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "One or more signatures over `serialized_payload`. Verifier implementations should consider this attestation message verified if at least one `signature` verifies `serialized_payload`. See `Signature` in common.proto for more details on signature structure and verification."]
    pub signatures: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Signature>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of a build occurrence."]
pub struct BuildOccurrence {
    #[serde(rename = "provenance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The actual provenance for the build."]
    pub provenance: ::std::option::Option<::std::boxed::Box<BuildProvenance>>,
    #[serde(rename = "provenanceBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Serialized JSON representation of the provenance, used in generating the build signature in the corresponding build note. After verifying the signature, `provenance_bytes` can be unmarshalled and compared to the provenance to confirm that it is unchanged. A base64-encoded string representation of the provenance bytes is used for the signature in order to interoperate with openssl which expects this format for signature verification. The serialized form is captured both to avoid ambiguity in how the provenance is marshalled to json as well to prevent incompatibilities with future changes."]
    pub provenance_bytes: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Provenance of a build. Contains all information needed to verify the full details about the build from source to completion."]
pub struct BuildProvenance {
    #[serde(rename = "buildOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Special options applied to this build. This is a catch-all field where build providers can enter any desired additional details."]
    pub build_options:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "builderVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Version string of the builder at the time this build was executed."]
    pub builder_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "builtArtifacts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output of the build."]
    pub built_artifacts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Artifact>>>,
    #[serde(rename = "commands")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Commands requested by the build."]
    pub commands: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Command>>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time at which the build was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creator")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "E-mail address of the user who initiated this build. Note that this was the user's e-mail address at the time the build was initiated; this address may not represent the same end-user for all time."]
    pub creator: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time at which execution of the build was finished."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Unique identifier of the build."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "logsUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URI where any logs for this provenance were written."]
    pub logs_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the project."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceProvenance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details of the Source input to the build."]
    pub source_provenance: ::std::option::Option<::std::boxed::Box<Source>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time at which execution of the build was started."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "triggerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Trigger identifier if the build was triggered automatically; empty if not."]
    pub trigger_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The category to which the update belongs."]
pub struct Category {
    #[serde(rename = "categoryId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of the category."]
    pub category_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The localized name of the category."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A CloudRepoSourceContext denotes a particular revision in a Google Cloud Source Repo."]
pub struct CloudRepoSourceContext {
    #[serde(rename = "aliasContext")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An alias, which may be a branch or tag."]
    pub alias_context: ::std::option::Option<::std::boxed::Box<AliasContext>>,
    #[serde(rename = "repoId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the repo."]
    pub repo_id: ::std::option::Option<::std::boxed::Box<RepoId>>,
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A revision ID."]
    pub revision_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Command describes a step performed as part of the build pipeline."]
pub struct Command {
    #[serde(rename = "args")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Command-line arguments used when executing this command."]
    pub args: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "dir")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Working directory (relative to project source root) used when running this command."]
    pub dir: ::std::option::Option<::std::string::String>,
    #[serde(rename = "env")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Environment variables set before running this command."]
    pub env: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional unique identifier for this command, used in wait_for to reference this command as a dependency."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Name of the command, as presented on the command line, or if the command is packaged as a Docker container, as presented to `docker pull`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "waitFor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID(s) of the command(s) that this command depends on."]
    pub wait_for: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The period during which some deployable was active in a runtime."]
pub struct DeploymentOccurrence {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Address of the runtime element hosting this deployment."]
    pub address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration used to create this deployment."]
    pub config: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deployTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Beginning of the lifetime of this deployment."]
    pub deploy_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Platform hosting this deployment."]
    pub platform: ::std::option::Option<DeploymentOccurrencePlatformEnum>,
    #[serde(rename = "resourceUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Resource URI for the artifact being deployed taken from the deployable field with the same name."]
    pub resource_uri: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "undeployTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "End of the lifetime of this deployment."]
    pub undeploy_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identity of the user that triggered this deployment."]
    pub user_email: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Platform hosting this deployment."]
pub enum DeploymentOccurrencePlatformEnum {
    #[serde(rename = "PLATFORM_UNSPECIFIED")]
    #[doc = "Unknown."]
    PlatformUnspecified,
    #[serde(rename = "GKE")]
    #[doc = "Google Container Engine."]
    Gke,
    #[serde(rename = "FLEX")]
    #[doc = "Google App Engine: Flexible Environment."]
    Flex,
    #[serde(rename = "CUSTOM")]
    #[doc = "Custom user-defined platform."]
    Custom,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Provides information about the analysis status of a discovered resource."]
pub struct DiscoveryOccurrence {
    #[serde(rename = "analysisStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of discovery for the resource."]
    pub analysis_status: ::std::option::Option<DiscoveryOccurrenceAnalysisStatusEnum>,
    #[serde(rename = "analysisStatusError")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When an error is encountered this will contain a LocalizedMessage under details to show to the user. The LocalizedMessage is output only and populated by the API."]
    pub analysis_status_error: ::std::option::Option<::std::boxed::Box<Status>>,
    #[serde(rename = "continuousAnalysis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the resource is continuously analyzed."]
    pub continuous_analysis: ::std::option::Option<DiscoveryOccurrenceContinuousAnalysisEnum>,
    #[serde(rename = "cpe")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The CPE of the resource being scanned."]
    pub cpe: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastScanTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last time this resource was scanned."]
    pub last_scan_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The status of discovery for the resource."]
pub enum DiscoveryOccurrenceAnalysisStatusEnum {
    #[serde(rename = "ANALYSIS_STATUS_UNSPECIFIED")]
    #[doc = "Unknown."]
    AnalysisStatusUnspecified,
    #[serde(rename = "PENDING")]
    #[doc = "Resource is known but no action has been taken yet."]
    Pending,
    #[serde(rename = "SCANNING")]
    #[doc = "Resource is being analyzed."]
    Scanning,
    #[serde(rename = "FINISHED_SUCCESS")]
    #[doc = "Analysis has finished successfully."]
    FinishedSuccess,
    #[serde(rename = "FINISHED_FAILED")]
    #[doc = "Analysis has finished unsuccessfully, the analysis itself is in a bad state."]
    FinishedFailed,
    #[serde(rename = "FINISHED_UNSUPPORTED")]
    #[doc = "The resource is known not to be supported"]
    FinishedUnsupported,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Whether the resource is continuously analyzed."]
pub enum DiscoveryOccurrenceContinuousAnalysisEnum {
    #[serde(rename = "CONTINUOUS_ANALYSIS_UNSPECIFIED")]
    #[doc = "Unknown."]
    ContinuousAnalysisUnspecified,
    #[serde(rename = "ACTIVE")]
    #[doc = "The resource is continuously analyzed."]
    Active,
    #[serde(rename = "INACTIVE")]
    #[doc = "The resource is ignored for continuous analysis."]
    Inactive,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Container message for hashes of byte content of files, used in source messages to verify integrity of source input to the build."]
pub struct FileHashes {
    #[serde(rename = "fileHash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Collection of file hashes."]
    pub file_hash: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Hash>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A set of properties that uniquely identify a given Docker image."]
pub struct Fingerprint {
    #[serde(rename = "v1Name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The layer ID of the final layer in the Docker image's v1 representation."]
    pub v1_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "v2Blob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ordered list of v2 blobs that represent a given image."]
    pub v2_blob: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "v2Name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of the image's v2 blobs computed via: [bottom] := v2_blobbottom := sha256(v2_blob[N] + \" \" + v2_name[N+1]) Only the name of the final blob is kept."]
    pub v2_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A SourceContext referring to a Gerrit project."]
pub struct GerritSourceContext {
    #[serde(rename = "aliasContext")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An alias, which may be a branch or tag."]
    pub alias_context: ::std::option::Option<::std::boxed::Box<AliasContext>>,
    #[serde(rename = "gerritProject")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full project name within the host. Projects may be nested, so \"project/subproject\" is a valid project name. The \"repo name\" is the hostURI/project."]
    pub gerrit_project: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hostUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI of a running Gerrit instance."]
    pub host_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A revision (commit) ID."]
    pub revision_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A GitSourceContext denotes a particular revision in a third party Git repository (e.g., GitHub)."]
pub struct GitSourceContext {
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Git commit hash."]
    pub revision_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Git repository URL."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Container message for hash values."]
pub struct Hash {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of hash that was performed, e.g. \"SHA-256\"."]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The hash value."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The unique identifier of the update."]
pub struct Identity {
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The revision number of the update."]
    pub revision: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "updateId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The revision independent identifier of the update."]
    pub update_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of the derived image portion of the DockerImage relationship. This image would be produced from a Dockerfile with FROM ."]
pub struct ImageOccurrence {
    #[serde(rename = "baseResourceUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. This contains the base image URL for the derived image occurrence."]
    pub base_resource_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "distance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The number of layers by which this image differs from the associated image basis."]
    pub distance: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The fingerprint of the derived image."]
    pub fingerprint: ::std::option::Option<::std::boxed::Box<Fingerprint>>,
    #[serde(rename = "layerInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This contains layer-specific metadata, if populated it has length \"distance\" and is ordered with [distance] being the layer immediately following the base image and [1] being the final layer."]
    pub layer_info: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Layer>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Jwt {
    #[serde(rename = "compactJwt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The compact encoding of a JWS, which is always three base64 encoded strings joined by periods. For details, see: https://tools.ietf.org/html/rfc7515.html#section-3.1"]
    pub compact_jwt: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Layer holds metadata specific to a layer of a Docker image."]
pub struct Layer {
    #[serde(rename = "arguments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The recovered arguments to the Dockerfile directive."]
    pub arguments: ::std::option::Option<::std::string::String>,
    #[serde(rename = "directive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The recovered Dockerfile directive used to construct this layer. See https://docs.docker.com/engine/reference/builder/ for more information."]
    pub directive: ::std::option::Option<::std::string::String>,
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
#[doc = "ListVulnerabilitiesResponse contains a single page of vulnerabilities resulting from a scan."]
pub struct ListVulnerabilitiesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A page token that can be used in a subsequent call to ListVulnerabilities to continue retrieving results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "occurrences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of Vulnerability Occurrences resulting from a scan."]
    pub occurrences: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Occurrence>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An occurrence of a particular package installation found within a system's filesystem. E.g., glibc was found in `/var/lib/dpkg/status`."]
pub struct Location {
    #[serde(rename = "cpeUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The CPE URI in [CPE format](https://cpe.mitre.org/specification/) denoting the package manager version distributing a package."]
    pub cpe_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The path from which we gathered that this package/version is installed."]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version installed at this location."]
    pub version: ::std::option::Option<::std::boxed::Box<Version>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An instance of an analysis type that has been found on a resource."]
pub struct Occurrence {
    #[serde(rename = "attestation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes an attestation of an artifact."]
    pub attestation: ::std::option::Option<::std::boxed::Box<AttestationOccurrence>>,
    #[serde(rename = "build")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes a verifiable build."]
    pub build: ::std::option::Option<::std::boxed::Box<BuildOccurrence>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time this occurrence was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deployment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes the deployment of an artifact on a runtime."]
    pub deployment: ::std::option::Option<::std::boxed::Box<DeploymentOccurrence>>,
    #[serde(rename = "discovery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes when a resource was discovered."]
    pub discovery: ::std::option::Option<::std::boxed::Box<DiscoveryOccurrence>>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes how this resource derives from the basis in the associated note."]
    pub image: ::std::option::Option<::std::boxed::Box<ImageOccurrence>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests."]
    pub kind: ::std::option::Option<OccurrenceKindEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "noteName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The analysis note associated with this occurrence, in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. This field can be used as a filter in list requests."]
    pub note_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "package")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes the installation of a package on the linked resource."]
    pub package: ::std::option::Option<::std::boxed::Box<PackageOccurrence>>,
    #[serde(rename = "remediation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of actions that can be taken to remedy the note."]
    pub remediation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. A URI that represents the resource for which the occurrence applies. For example, `https://gcr.io/project/image@sha256:123abc` for a Docker image."]
    pub resource_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time this occurrence was last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "upgrade")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes an available package upgrade on the linked resource."]
    pub upgrade: ::std::option::Option<::std::boxed::Box<UpgradeOccurrence>>,
    #[serde(rename = "vulnerability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes a security vulnerability."]
    pub vulnerability: ::std::option::Option<::std::boxed::Box<VulnerabilityOccurrence>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests."]
pub enum OccurrenceKindEnum {
    #[serde(rename = "NOTE_KIND_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    NoteKindUnspecified,
    #[serde(rename = "VULNERABILITY")]
    #[doc = "The note and occurrence represent a package vulnerability."]
    Vulnerability,
    #[serde(rename = "BUILD")]
    #[doc = "The note and occurrence assert build provenance."]
    Build,
    #[serde(rename = "IMAGE")]
    #[doc = "This represents an image basis relationship."]
    Image,
    #[serde(rename = "PACKAGE")]
    #[doc = "This represents a package installed via a package manager."]
    Package,
    #[serde(rename = "DEPLOYMENT")]
    #[doc = "The note and occurrence track deployment events."]
    Deployment,
    #[serde(rename = "DISCOVERY")]
    #[doc = "The note and occurrence track the initial discovery status of a resource."]
    Discovery,
    #[serde(rename = "ATTESTATION")]
    #[doc = "This represents a logical \"role\" that can attest to artifacts."]
    Attestation,
    #[serde(rename = "UPGRADE")]
    #[doc = "This represents an available package upgrade."]
    Upgrade,
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
pub struct PackageData {
    #[serde(rename = "cpeUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cpe_uri in [cpe format] (https://cpe.mitre.org/specification/) in which the vulnerability may manifest. Examples include distro or storage location for vulnerable jar."]
    pub cpe_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "os")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The OS affected by a vulnerability This field is deprecated and the information is in cpe_uri"]
    pub os: ::std::option::Option<::std::string::String>,
    #[serde(rename = "osVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version of the OS This field is deprecated and the information is in cpe_uri"]
    pub os_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "package")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The package being analysed for vulnerabilities"]
    pub package: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version of the package being analysed"]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A detail for a distro and package this vulnerability occurrence was found in and its associated fix (if one is available)."]
pub struct PackageIssue {
    #[serde(rename = "affectedCpeUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The [CPE URI](https://cpe.mitre.org/specification/) this vulnerability was found in."]
    pub affected_cpe_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "affectedPackage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The package this vulnerability was found in."]
    pub affected_package: ::std::option::Option<::std::string::String>,
    #[serde(rename = "affectedVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The version of the package that is installed on the resource affected by this vulnerability."]
    pub affected_version: ::std::option::Option<::std::boxed::Box<Version>>,
    #[serde(rename = "fixAvailable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether a fix is available for this package."]
    pub fix_available: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "fixedCpeUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The [CPE URI](https://cpe.mitre.org/specification/) this vulnerability was fixed in. It is possible for this to be different from the affected_cpe_uri."]
    pub fixed_cpe_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fixedPackage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The package this vulnerability was fixed in. It is possible for this to be different from the affected_package."]
    pub fixed_package: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fixedVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The version of the package this vulnerability was fixed in. Setting this to VersionKind.MAXIMUM means no fix is yet available."]
    pub fixed_version: ::std::option::Option<::std::boxed::Box<Version>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details on how a particular software package was installed on a system."]
pub struct PackageOccurrence {
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. All of the places within the filesystem versions of this package have been found."]
    pub location: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Location>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of the installed package."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Selects a repo using a Google Cloud Platform project ID (e.g., winged-cargo-31) and a repo name within that project."]
pub struct ProjectRepoId {
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the project."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "repoName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the repo. Leave empty for the default repo."]
    pub repo_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for any related URL information."]
pub struct RelatedUrl {
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Label to describe usage of the URL."]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specific URL associated with the resource."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A unique identifier for a Cloud Repo."]
pub struct RepoId {
    #[serde(rename = "projectRepoId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A combination of a project ID and a repo name."]
    pub project_repo_id: ::std::option::Option<::std::boxed::Box<ProjectRepoId>>,
    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A server-assigned, globally unique identifier."]
    pub uid: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Verifiers (e.g. Kritis implementations) MUST verify signatures with respect to the trust anchors defined in policy (e.g. a Kritis policy). Typically this means that the verifier has been configured with a map from `public_key_id` to public key material (and any required parameters, e.g. signing algorithm). In particular, verification implementations MUST NOT treat the signature `public_key_id` as anything more than a key lookup hint. The `public_key_id` DOES NOT validate or authenticate a public key; it only provides a mechanism for quickly selecting a public key ALREADY CONFIGURED on the verifier through a trusted channel. Verification implementations MUST reject signatures in any of the following circumstances: * The `public_key_id` is not recognized by the verifier. * The public key that `public_key_id` refers to does not verify the signature with respect to the payload. The `signature` contents SHOULD NOT be \"attached\" (where the payload is included with the serialized `signature` bytes). Verifiers MUST ignore any \"attached\" payload and only verify signatures with respect to explicitly provided payload (e.g. a `payload` field on the proto message that holds this Signature, or the canonical serialization of the proto message that holds this signature)."]
pub struct Signature {
    #[serde(rename = "publicKeyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier for the public key that verifies this signature. * The `public_key_id` is required. * The `public_key_id` SHOULD be an RFC3986 conformant URI. * When possible, the `public_key_id` SHOULD be an immutable reference, such as a cryptographic digest. Examples of valid `public_key_id`s: OpenPGP V4 public key fingerprint: * \"openpgp4fpr:74FAF3B861BDA0870C7B6DEF607E48D2A663AEEA\" See https://www.iana.org/assignments/uri-schemes/prov/openpgp4fpr for more details on this scheme. RFC6920 digest-named SubjectPublicKeyInfo (digest of the DER serialization): * \"ni:///sha-256;cD9o9Cq6LG3jD0iKXqEi_vdjJGecm_iXkbqVoScViaU\" * \"nih:///sha-256;703f68f42aba2c6de30f488a5ea122fef76324679c9bf89791ba95a1271589a5\""]
    pub public_key_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "signature")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content of the signature, an opaque bytestring. The payload that this signature verifies MUST be unambiguously provided with the Signature during verification. A wrapper message might provide the payload explicitly. Alternatively, a message might have a canonical serialization that can always be unambiguously computed to derive the payload."]
    pub signature: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Source describes the location of the source used for the build."]
pub struct Source {
    #[serde(rename = "additionalContexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If provided, some of the source code used for the build may be found in these locations, in the case where the source repository had multiple remotes or submodules. This list will not include the context specified in the context field."]
    pub additional_contexts:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SourceContext>>>,
    #[serde(rename = "artifactStorageSourceUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If provided, the input binary artifacts for the build came from this location."]
    pub artifact_storage_source_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If provided, the source code used for the build came from this location."]
    pub context: ::std::option::Option<::std::boxed::Box<SourceContext>>,
    #[serde(rename = "fileHashes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hash(es) of the build source, which can be used to verify that the original source integrity was maintained in the build. The keys to this map are file paths used as build source and the values contain the hash values for those files. If the build source came in a single package such as a gzipped tarfile (.tar.gz), the FileHash will be for the single path to that file."]
    pub file_hashes:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<FileHashes>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A SourceContext is a reference to a tree of files. A SourceContext together with a path point to a unique revision of a single file or directory."]
pub struct SourceContext {
    #[serde(rename = "cloudRepo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A SourceContext referring to a revision in a Google Cloud Source Repo."]
    pub cloud_repo: ::std::option::Option<::std::boxed::Box<CloudRepoSourceContext>>,
    #[serde(rename = "gerrit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A SourceContext referring to a Gerrit project."]
    pub gerrit: ::std::option::Option<::std::boxed::Box<GerritSourceContext>>,
    #[serde(rename = "git")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A SourceContext referring to any third party Git repo (e.g., GitHub)."]
    pub git: ::std::option::Option<::std::boxed::Box<GitSourceContext>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Labels with user defined metadata."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
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
#[doc = "The Upgrade Distribution represents metadata about the Upgrade for each operating system (CPE). Some distributions have additional metadata around updates, classifying them into various categories and severities."]
pub struct UpgradeDistribution {
    #[serde(rename = "classification")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The operating system classification of this Upgrade, as specified by the upstream operating system upgrade feed. For Windows the classification is one of the category_ids listed at https://docs.microsoft.com/en-us/previous-versions/windows/desktop/ff357803(v=vs.85)"]
    pub classification: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cpeUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required - The specific operating system this metadata applies to. See https://cpe.mitre.org/specification/."]
    pub cpe_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cve")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cve tied to this Upgrade."]
    pub cve: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The severity as specified by the upstream operating system."]
    pub severity: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An Upgrade Occurrence represents that a specific resource_url could install a specific upgrade. This presence is supplied via local sources (i.e. it is present in the mirror and the running system has noticed its availability). For Windows, both distribution and windows_update contain information for the Windows update."]
pub struct UpgradeOccurrence {
    #[serde(rename = "distribution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the upgrade for available for the specific operating system for the resource_url. This allows efficient filtering, as well as making it easier to use the occurrence."]
    pub distribution: ::std::option::Option<::std::boxed::Box<UpgradeDistribution>>,
    #[serde(rename = "package")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for non-Windows OS. The package this Upgrade is for."]
    pub package: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parsedVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for non-Windows OS. The version of the package in a machine + human readable form."]
    pub parsed_version: ::std::option::Option<::std::boxed::Box<Version>>,
    #[serde(rename = "windowsUpdate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for Windows OS. Represents the metadata about the Windows update."]
    pub windows_update: ::std::option::Option<::std::boxed::Box<WindowsUpdate>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Version contains structured information about the version of a package."]
pub struct Version {
    #[serde(rename = "epoch")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used to correct mistakes in the version numbering scheme."]
    pub epoch: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "fullName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human readable version string. This string is of the form :- and is only set when kind is NORMAL."]
    pub full_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inclusive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this version is specifying part of an inclusive range. Grafeas does not have the capability to specify version ranges; instead we have fields that specify start version and end versions. At times this is insufficient - we also need to specify whether the version is included in the range or is excluded from the range. This boolean is expected to be set to true when the version is included in a range."]
    pub inclusive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Distinguishes between sentinel MIN/MAX versions and normal versions."]
    pub kind: ::std::option::Option<VersionKindEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required only when version kind is NORMAL. The main part of the version name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The iteration of the package build from the above version."]
    pub revision: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Distinguishes between sentinel MIN/MAX versions and normal versions."]
pub enum VersionKindEnum {
    #[serde(rename = "VERSION_KIND_UNSPECIFIED")]
    #[doc = "Unknown."]
    VersionKindUnspecified,
    #[serde(rename = "NORMAL")]
    #[doc = "A standard package version."]
    Normal,
    #[serde(rename = "MINIMUM")]
    #[doc = "A special version representing negative infinity."]
    Minimum,
    #[serde(rename = "MAXIMUM")]
    #[doc = "A special version representing positive infinity."]
    Maximum,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An occurrence of a severity vulnerability on a resource."]
pub struct VulnerabilityOccurrence {
    #[serde(rename = "cvssScore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The CVSS score of this vulnerability. CVSS score is on a scale of 0 - 10 where 0 indicates low severity and 10 indicates high severity."]
    pub cvss_score: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "effectiveSeverity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The distro assigned severity for this vulnerability when it is available, otherwise this is the note provider assigned severity."]
    pub effective_severity: ::std::option::Option<VulnerabilityOccurrenceEffectiveSeverityEnum>,
    #[serde(rename = "fixAvailable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether at least one of the affected packages has a fix available."]
    pub fix_available: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "longDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A detailed description of this vulnerability."]
    pub long_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "packageIssue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The set of affected locations and their fixes (if available) within the associated resource."]
    pub package_issue: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PackageIssue>>>,
    #[serde(rename = "relatedUrls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. URLs related to this vulnerability."]
    pub related_urls: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RelatedUrl>>>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The note provider assigned severity of this vulnerability."]
    pub severity: ::std::option::Option<VulnerabilityOccurrenceSeverityEnum>,
    #[serde(rename = "shortDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A one sentence description of this vulnerability."]
    pub short_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of package; whether native or non native (e.g., ruby gems, node.js packages, etc.)."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The distro assigned severity for this vulnerability when it is available, otherwise this is the note provider assigned severity."]
pub enum VulnerabilityOccurrenceEffectiveSeverityEnum {
    #[serde(rename = "SEVERITY_UNSPECIFIED")]
    #[doc = "Unknown."]
    SeverityUnspecified,
    #[serde(rename = "MINIMAL")]
    #[doc = "Minimal severity."]
    Minimal,
    #[serde(rename = "LOW")]
    #[doc = "Low severity."]
    Low,
    #[serde(rename = "MEDIUM")]
    #[doc = "Medium severity."]
    Medium,
    #[serde(rename = "HIGH")]
    #[doc = "High severity."]
    High,
    #[serde(rename = "CRITICAL")]
    #[doc = "Critical severity."]
    Critical,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The note provider assigned severity of this vulnerability."]
pub enum VulnerabilityOccurrenceSeverityEnum {
    #[serde(rename = "SEVERITY_UNSPECIFIED")]
    #[doc = "Unknown."]
    SeverityUnspecified,
    #[serde(rename = "MINIMAL")]
    #[doc = "Minimal severity."]
    Minimal,
    #[serde(rename = "LOW")]
    #[doc = "Low severity."]
    Low,
    #[serde(rename = "MEDIUM")]
    #[doc = "Medium severity."]
    Medium,
    #[serde(rename = "HIGH")]
    #[doc = "High severity."]
    High,
    #[serde(rename = "CRITICAL")]
    #[doc = "Critical severity."]
    Critical,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Windows Update represents the metadata about the update for the Windows operating system. The fields in this message come from the Windows Update API documented at https://docs.microsoft.com/en-us/windows/win32/api/wuapi/nn-wuapi-iupdate."]
pub struct WindowsUpdate {
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of categories to which the update belongs."]
    pub categories: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Category>>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The localized description of the update."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "identity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required - The unique identifier for the update."]
    pub identity: ::std::option::Option<::std::boxed::Box<Identity>>,
    #[serde(rename = "kbArticleIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Microsoft Knowledge Base article IDs that are associated with the update."]
    pub kb_article_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "lastPublishedTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last published timestamp of the update."]
    pub last_published_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "supportUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The hyperlink to the support information for the update."]
    pub support_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The localized title of the update."]
    pub title: ::std::option::Option<::std::string::String>,
}
